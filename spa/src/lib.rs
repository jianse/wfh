use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use settings;
use settings::UiSettings;
use tower_http::services::{ServeDir, ServeFile};

#[cfg(feature = "embed")]
mod embed;

static INDEX_HTML: &str = "index.html";

pub fn spa(router: Router) -> Router {
    // router.fa
    match &settings::get().ui {
        UiSettings::Local { path } => router.fallback_service(
            ServeDir::new(path).not_found_service(ServeFile::new(path.join(INDEX_HTML))),
        ),
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

pub(crate) async fn not_found() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}
