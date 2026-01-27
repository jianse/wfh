use std::net::SocketAddr;
use std::task::Context;
use std::task::Poll;

use axum::Router;
use axum::extract::ConnectInfo;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::http::Uri;
use axum::response::IntoResponse;
use settings::UiSettings;
use tower::Layer;
use tower::Service;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;
use tracing::warn;

#[cfg(feature = "embed")]
mod embed;

static INDEX_HTML: &str = "index.html";

pub fn spa(router: Router) -> Router {
    match &settings::get().ui {
        UiSettings::Local { path } => {
            let fb = ServiceBuilder::new()
                .layer(LogNotFoundLayer)
                .service(ServeFile::new(path.join(INDEX_HTML)));
            router.fallback_service(ServeDir::new(path).fallback(fb))
        }
        UiSettings::None => router.fallback(not_found),
        UiSettings::Embed => {
            #[cfg(feature = "embed")]
            {
                router.fallback(embed::spa_handler)
            }

            #[cfg(not(feature = "embed"))]
            {
                tracing::warn!(
                    "this build is build without the `ui` feature enabled. \"embed\" is just the same as \"none\""
                );
                router.fallback(not_found)
            }
        }
    }
}

pub(crate) struct LogNotFoundLayer;

impl<S> Layer<S> for LogNotFoundLayer {
    type Service = LogNotFoundService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LogNotFoundService { inner }
    }
}

#[derive(Clone)]
pub(crate) struct LogNotFoundService<S> {
    inner: S,
}

impl<S> Service<Request> for LogNotFoundService<S>
where
    S: Service<Request>,
    // Request: fmt::Debug,
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let uri = req.uri();
        // println!("uri = {:?}", uri);
        let client_ip = req
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|ci| ci.0.to_string())
            .unwrap_or("unknown".to_owned());
        warn!("client ip: {} are trying to access uri: {} fallback to index", client_ip, uri);
        self.inner.call(req)
    }
}

pub(crate) async fn not_found(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    uri: Uri,
) -> impl IntoResponse {
    warn!(
        "client ip: {} are trying to access not exist uri: {}",
        ip, uri
    );
    StatusCode::NOT_FOUND
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    #[test]
    fn test_parse_ip() {
        let ip = "unknown";
        let ip = ip.parse::<std::net::SocketAddr>();
        // assert!(ip.is_err());
        let _ = dbg!(ip);
    }

    #[test]
    fn test_ip_to_string() {
        let ip = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3210));
        dbg!(ip.to_string());
    }
}
