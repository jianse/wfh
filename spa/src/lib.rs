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
        println!("uri = {:?}", uri);
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
