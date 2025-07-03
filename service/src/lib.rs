mod api;

mod entities;
pub mod error;
mod migrator;
pub mod tls;

use crate::tls::TlsListener;
use axum::response::Redirect;
use axum::serve::Listener;
use axum::{Router, http::StatusCode, routing::post};
use error::Result;
use http::uri::{Authority, Scheme};
use http::{HeaderValue, Uri};
use migrator::Migrator;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use settings::TlsSettings;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio_rustls::TlsAcceptor;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing::level_filters::LevelFilter;

#[derive(Clone)]
struct AppStateRaw {
    conn: DatabaseConnection,
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
    let settings = settings::get();
    init_logger();
    dbg!(settings);
    let mut opt = ConnectOptions::new(&settings.db.database_url);
    opt.sqlx_logging_level(log::LevelFilter::Debug);

    let conn = Database::connect(opt).await?;

    Migrator::up(&conn, None).await?;

    let state = Arc::new(AppStateRaw { conn });
    let (tx, rx) = broadcast::channel::<()>(10);
    let rx2 = tx.subscribe();

    tokio::spawn(start_ctl_endpoint(tx.clone(), rx2));

    match &settings.server.tls {
        None => {
            let listener =
                TcpListener::bind((settings.server.host.to_owned(), settings.server.port)).await?;
            info!("HTTP server listening on {}", listener.local_addr()?);
            // let rx3
            serve_app(listener, state.clone(), rx).await
        }

        Some(tls_opt) => {
            // setup http server
            let listener =
                TcpListener::bind((settings.server.host.to_owned(), settings.server.port)).await?;
            info!("HTTP server listening on {}", listener.local_addr()?);

            // setup https server
            let rustls_config = tls::rustls_server_config(&tls_opt.key, &tls_opt.cert);
            let tls_acceptor = TlsAcceptor::from(rustls_config);
            let bind = (settings.server.host.to_owned(), tls_opt.port);
            let tcp_listener = TcpListener::bind(bind).await.unwrap();
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
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(rx))
        .await?;
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

    // let listener =
    //     TcpListener::bind((settings.server.host.to_owned(), settings.server.port)).await?;
    // info!("HTTP server listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(rx))
        .await?;
    Ok(())
}

fn app_router(state: AppState) -> Router {
    let app = spa::spa(
        Router::new()
            .nest("/api", api::api_routes(state.clone()))
            .with_state(state),
    )
    .layer(TraceLayer::new_for_http());

    if let Some(cors) = settings::get().cors.clone() {
        let origins = cors
            .allow_origin
            .iter()
            .map(|s| s.parse::<HeaderValue>().unwrap())
            .collect::<Vec<HeaderValue>>();

        app.layer(CorsLayer::new().allow_origin(origins))
    } else {
        app
    }
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
        let mut parts = uri.into_parts();
        parts.scheme = Some(Scheme::HTTPS);
        parts.authority = Some(a);
        let r_url = Uri::from_parts(parts).unwrap();
        Redirect::temporary(r_url.to_string().as_str())
    })
}

async fn serve_app<L>(listener: L, state: AppState, rx: Receiver<()>) -> Result<()>
where
    L: Listener,
    L::Addr: std::fmt::Display + std::fmt::Debug,
{
    // let (tx, rx) = mpsc::channel::<()>(1);

    let app = app_router(state);

    // let listener =
    //     TcpListener::bind((settings.server.host.to_owned(), settings.server.port)).await?;
    // info!("HTTP server listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(rx))
        .await?;
    Ok(())
}

pub fn init_logger() {
    let dirs = settings::get()
        .log
        .level
        .iter()
        .map(|(k, v)| {
            if k == "root" {
                v.to_string()
            } else {
                let mut s = String::from(k);
                s.push('=');
                s.push_str(&v.to_string());
                k.to_owned()
            }
        })
        .fold("".to_owned(), |mut acc, x| {
            acc.push(',');
            acc.push_str(&x);
            acc
        });
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .parse_lossy(dirs);
    use tracing_subscriber::{EnvFilter, fmt::time::LocalTime};
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

    let fmt = fmt::layer()
        .with_ansi(true)
        // .with_file(true)
        .with_line_number(true)
        .with_timer(LocalTime::rfc_3339());
    // let filter = EnvFilter::try_from_default_env().unwrap_or(f);
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(fmt)
        .try_init();
}
