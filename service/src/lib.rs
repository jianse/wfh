mod api;

mod common;
mod entities;
pub mod error;
mod job;
mod logging;
pub mod tls;

use crate::job::JobRegistry;
// use crate::tls::MyConnectInfo;
use crate::tls::TlsListener;
use axum::Router;
use axum::http::StatusCode;
use axum::response::Redirect;
use axum::routing::post;
use axum::serve::Listener;
use error::Result;
use http::HeaderValue;
use http::Uri;
use http::uri::Authority;
use http::uri::Scheme;
use logging::init_logger;
use migration::Migrator;
use migration::MigratorTrait;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use settings::TlsSettings;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::{self};
use tokio_rustls::TlsAcceptor;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::error;
use tracing::info;

#[derive(Clone)]
struct AppStateRaw {
    conn: DatabaseConnection,
    http_client: reqwest::Client,
}

async fn shutdown_signal(mut rx: Receiver<()>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
        _ = rx.recv() =>{}
    }
}

pub(crate) type AppState = Arc<AppStateRaw>;

pub async fn serve() -> Result<()> {
    let _guard = init_logger();
    let settings = settings::get();
    #[cfg(debug_assertions)]
    {
        dbg!(&settings);
    }
    let mut opt = ConnectOptions::new(&settings.db.database_url);
    opt.sqlx_logging_level(log::LevelFilter::Debug);

    let conn = Database::connect(opt).await?;

    Migrator::up(&conn, None).await?;

    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        // .read_timeout(Duration::from_secs(5))
        // .connect_timeout(Duration::from_secs(1))
        .build()?;

    let state = Arc::new(AppStateRaw { conn, http_client });
    let (tx, rx) = broadcast::channel::<()>(10);
    let rx2 = tx.subscribe();

    // start control endpoint
    tokio::spawn(start_ctl_endpoint(tx.clone(), rx2));

    // start background jobs
    let job = JobRegistry::new(state.clone(), shutdown_signal(tx.subscribe()));
    tokio::task::spawn(job.start());

    // load public export on startup
    tokio::spawn({
        let state = state.clone();
        async move {
            let res = api::load_deps("data", "cache", &state.http_client, false).await;
            if let Err(err) = res {
                error!("failed to load dependencies: {}", err);
            }
        }
    });
    // api::load_deps("data", &state.http_client, false);

    // start other endpoints
    match &settings.server.tls {
        None => {
            let listener =
                TcpListener::bind((settings.server.addr.to_owned(), settings.server.port)).await?;
            info!("HTTP server listening on {}", listener.local_addr()?);
            // let rx3
            serve_app(listener, state.clone(), rx).await
        }

        Some(tls_opt) => {
            // setup http server
            let listener =
                TcpListener::bind((settings.server.addr.to_owned(), settings.server.port)).await?;
            info!("HTTP server listening on {}", listener.local_addr()?);

            // setup https server
            let rustls_config = tls::rustls_server_config(&tls_opt.key, &tls_opt.cert);
            let tls_acceptor = TlsAcceptor::from(rustls_config);
            let bind = (settings.server.addr.to_owned(), tls_opt.port);
            let tcp_listener = TcpListener::bind(bind).await?;
            info!("HTTPS server listening on {}.", tcp_listener.local_addr()?);
            let tls_listener = TlsListener::new(tls_acceptor, tcp_listener);

            let rx3 = tx.subscribe();
            // await on both
            tokio::try_join!(
                serve_app_or_redirect(tls_opt, listener, state.clone(), rx3),
                serve_app(tls_listener, state.clone(), rx),
            )?;
            Ok(())
        }
    }
}

async fn start_ctl_endpoint(tx: Sender<()>, rx: Receiver<()>) -> Result<()> {
    let listener = TcpListener::bind(("localhost", 54321)).await?;
    let app = control_router(tx);
    info!("control endpoint started at http://localhost:54321");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(rx))
        .await;
    Ok(())
}

async fn serve_app_or_redirect<L>(
    opt: &TlsSettings,
    listener: L,
    state: Arc<AppStateRaw>,
    rx: Receiver<()>,
) -> Result<()>
where
    L: Listener,
    L::Addr: std::fmt::Display + std::fmt::Debug,
{
    let app = if opt.redirect_to_https {
        info!("redirect http to https is enabled");
        let host = settings::get().server.host.to_owned();
        redirect_to_https_router(host, opt.port)
    } else {
        app_router(state)
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(rx))
        .await;
    Ok(())
}

fn app_router(state: AppState) -> Router {
    let mut app = spa::spa(
        Router::new()
            .nest("/api", api::api_routes(state.clone()))
            .with_state(state),
    );

    if let Some(cors) = settings::get().cors.clone() {
        let mut cors_layer = CorsLayer::new();
        if let Some(origins) = cors.allow_origin {
            cors_layer = match origins {
                settings::AllowOrigin::Any => cors_layer.allow_origin(Any),
                settings::AllowOrigin::Exact(origin) => {
                    cors_layer.allow_origin(origin.parse::<HeaderValue>().unwrap())
                }
                settings::AllowOrigin::List(origins) => {
                    let origins = origins
                        .iter()
                        .map(|s| s.parse::<HeaderValue>().unwrap())
                        .collect::<Vec<HeaderValue>>();
                    cors_layer.allow_origin(origins)
                }
            };
        };

        if let Some(a) = cors.allow_credentials {
            cors_layer = cors_layer.allow_credentials(a)
        };

        if let Some(headers) = cors.allow_headers {
            cors_layer = match headers {
                settings::AllowHeaders::Any => cors_layer.allow_headers(Any),
                settings::AllowHeaders::Exact(header_name) => {
                    cors_layer.allow_headers([header_name])
                }
                settings::AllowHeaders::List(header_names) => {
                    cors_layer.allow_headers(header_names)
                }
            }
        };

        if let Some(methods) = cors.allow_methods {
            cors_layer = match methods {
                settings::AllowMethods::Any => cors_layer.allow_methods(Any),
                settings::AllowMethods::Exact(method) => cors_layer.allow_methods(method),
                settings::AllowMethods::List(methods) => cors_layer.allow_methods(methods),
            }
        };

        app = app.layer(cors_layer);
    };

    app.layer(TraceLayer::new_for_http())
}

#[allow(unused)]
fn control_router(tx: Sender<()>) -> Router {
    Router::new().route(
        "/shutdown",
        post(|| async move {
            let _ = tx.send(()).unwrap();
            (StatusCode::OK, "shuting down")
        }),
    )
}

fn redirect_to_https_router(host: String, port: u16) -> Router {
    let a = Authority::from_maybe_shared(format!("{host}:{port}")).unwrap();
    Router::new().fallback(async move |uri: Uri| {
        let mut builder = Uri::builder().scheme(Scheme::HTTPS).authority(a);
        if let Some(pq) = uri.path_and_query() {
            builder = builder.path_and_query(pq.to_owned());
        };

        let r_url = match builder.build() {
            Ok(u) => u.to_string(),
            Err(e) => {
                error!("build redirect uri error : {e}");
                format!("https://{host}:{port}/")
            }
        };

        info!("redirect from {uri} to {r_url}");
        Redirect::temporary(r_url.as_str())
    })
}

async fn serve_app<L>(listener: L, state: AppState, rx: Receiver<()>) -> Result<()>
where
    L: Listener<Addr = SocketAddr>,
{
    let app = app_router(state);

    let service = app.into_make_service_with_connect_info::<SocketAddr>();

    axum::serve(listener, service)
        .with_graceful_shutdown(shutdown_signal(rx))
        .await;
    Ok(())
}

#[cfg(test)]
mod tests {

    use std::net::SocketAddr;

    use axum::Router;
    use axum::extract::ConnectInfo;
    use axum::routing::any;
    // use axum_client_ip::ClientIp;
    use eyre::Result;
    use tokio::net::TcpListener;
    use tokio_rustls::TlsAcceptor;

    use crate::tls::TlsListener;
    use crate::tls::rustls_server_config;

    #[tokio::test]
    async fn test_app() -> Result<()> {
        let tcp = TcpListener::bind(("localhost", 8080)).await?;
        let conf = rustls_server_config("self_signed_certs/key.pem", "self_signed_certs/cert.pem");
        let acceptor = TlsAcceptor::from(conf);
        let tls = TlsListener::new(acceptor, tcp);
        let app = Router::new().route("/", any(hello));

        let service = app.into_make_service_with_connect_info::<SocketAddr>();

        axum::serve(tls, service)
            // .with_graceful_shutdown(shutdown_signal(rx))
            .await;
        Ok(())
    }

    async fn hello(ConnectInfo(info): ConnectInfo<SocketAddr>) -> String {
        format!("Hello, {}!", info)
    }
}
