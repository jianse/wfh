use rust_embed::Embed;

#[derive(Embed)]
#[folder = "ui/dist"]
struct Asset;

static INDEX_HTML: &str = "index.html";
static DATETIME_FORMAT: &str = "%a, %d %b %G %H:%M:%S GMT";

pub(crate) use impl2::spa_handler;

#[allow(unused)]
mod impl1 {
    use axum::{
        http::{StatusCode, Uri, header},
        response::{Html, IntoResponse, Response},
    };
    use mime_guess::mime;
    use tracing::info;

    use crate::embed::{Asset, INDEX_HTML};

    pub(crate) async fn spa_handler(uri: Uri) -> impl IntoResponse {
        let path = uri.path().trim_start_matches('/');
        info!("reading: {path}");
        if path.is_empty() || path == INDEX_HTML {
            return index_html().await;
        }

        match Asset::get(path) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                match mime.type_() {
                    mime::IMAGE => {
                        let mut resp = (
                            [
                                (header::CONTENT_TYPE, mime.as_ref()),
                                (
                                    header::ETAG,
                                    hex::encode(content.metadata.sha256_hash()).as_ref(),
                                ),
                            ],
                            content.data,
                        )
                            .into_response();

                        if let Some(lmt_in_sec) = content.metadata.last_modified() {
                            let last_mod_time =
                                chrono::DateTime::from_timestamp(lmt_in_sec as i64, 0);
                            if let Some(d) = last_mod_time {
                                // headers.append((header::LAST_MODIFIED, d.form));
                                let headers = resp.headers_mut();
                                headers
                                    .append(header::LAST_MODIFIED, d.to_rfc2822().parse().unwrap());
                                // todo!()
                            }
                        }
                        resp
                    }
                    _ => ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response(),
                }
            }
            None => index_html().await,
        }
    }

    async fn index_html() -> Response {
        match Asset::get(INDEX_HTML) {
            Some(content) => Html(content.data).into_response(),
            None => not_found().await,
        }
    }

    async fn not_found() -> Response {
        (StatusCode::NOT_FOUND, "404").into_response()
    }
}

mod impl2 {
    use axum::{
        http::{StatusCode, Uri, header},
        response::{IntoResponse, Response},
    };
    use hex::ToHex;
    use rust_embed::EmbeddedFile;
    use tracing::debug;

    use crate::embed::{Asset, INDEX_HTML};

    use super::DATETIME_FORMAT;

    struct StaticFile(EmbeddedFile);

    impl IntoResponse for StaticFile {
        fn into_response(self) -> Response {
            let mut resp = (
                [
                    (header::CONTENT_TYPE, self.0.metadata.mimetype()),
                    (
                        header::ETAG,
                        self.0
                            .metadata
                            .sha256_hash()
                            .encode_hex::<String>()
                            .as_ref(),
                    ),
                ],
                self.0.data,
            )
                .into_response();
            if let Some(lmt_in_sec) = self.0.metadata.last_modified() {
                let last_mod_time = chrono::DateTime::from_timestamp(lmt_in_sec as i64, 0);
                if let Some(d) = last_mod_time {
                    let headers = resp.headers_mut();
                    let mut buf = String::new();
                    let _ = d.format(DATETIME_FORMAT).write_to(&mut buf);

                    headers.append(header::LAST_MODIFIED, buf.parse().unwrap());
                }
            }
            resp
        }
    }

    pub(crate) async fn spa_handler(uri: Uri) -> impl IntoResponse {
        // todo!()
        let path = uri.path().trim_start_matches('/');
        debug!("reading: {path}");
        if path.is_empty() || path == INDEX_HTML {
            return index_html().await;
        }
        match Asset::get(path) {
            Some(f) => StaticFile(f).into_response(),
            None => index_html().await,
        }
    }
    async fn index_html() -> Response {
        match Asset::get(INDEX_HTML) {
            Some(f) => StaticFile(f).into_response(),
            None => not_found().await,
        }
    }
    async fn not_found() -> Response {
        StatusCode::NOT_FOUND.into_response()
    }
}

#[cfg(test)]
mod test {
    // use crate::{Asset, INDEX_HTML};

    use crate::embed::{Asset, INDEX_HTML};

    #[test]
    fn test_last_mod_time() {
        let index = Asset::get(INDEX_HTML).unwrap();
        let lmt_in_sec = index.metadata.last_modified().unwrap();

        dbg!(lmt_in_sec);

        let dt = chrono::DateTime::from_timestamp(lmt_in_sec as i64, 0).unwrap();
        dbg!(dt.to_rfc2822());
    }
}
