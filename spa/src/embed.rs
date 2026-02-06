use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../ui/dist"]
struct Asset;

pub(crate) use impl2::spa_handler;

#[allow(unused)]
mod impl1 {
    use crate::INDEX_HTML;
    use crate::embed::Asset;
    use axum::http::StatusCode;
    use axum::http::Uri;
    use axum::http::header;
    use axum::response::Html;
    use axum::response::IntoResponse;
    use axum::response::Response;
    use mime_guess::mime;
    use time::UtcDateTime;
    use time::format_description::well_known::Rfc2822;
    use tracing::info;

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

                        if let Some(lmt_in_sec) = content.metadata.last_modified()
                            && let Ok(date) = UtcDateTime::from_unix_timestamp(lmt_in_sec as i64)
                            && let Ok(date_formated) = date.format(&Rfc2822)
                            && let Ok(header_value) = date_formated.parse::<header::HeaderValue>()
                        {
                            let headers = resp.headers_mut();
                            headers.append(header::LAST_MODIFIED, header_value);
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
    use super::Asset;
    use crate::INDEX_HTML;
    use axum::http::StatusCode;
    use axum::http::Uri;
    use axum::http::header;
    use axum::response::IntoResponse;
    use axum::response::Response;
    use hex::ToHex;
    use rust_embed::EmbeddedFile;
    use time::UtcDateTime;
    use time::format_description::StaticFormatDescription;
    use time::macros::format_description;
    use tracing::debug;

    struct StaticFile(EmbeddedFile);

    pub(super) const DATETIME_FORMAT: StaticFormatDescription = format_description!(
        "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT"
    );

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
            if let Some(lmt_in_sec) = self.0.metadata.last_modified()
                && let Ok(d) = UtcDateTime::from_unix_timestamp(lmt_in_sec as i64)
                && let Ok(formated) = d.format(&DATETIME_FORMAT)
                && let Ok(header_value) = formated.parse::<header::HeaderValue>()
            {
                let headers = resp.headers_mut();
                headers.append(header::LAST_MODIFIED, header_value);
            }
            resp
        }
    }

    pub(crate) async fn spa_handler(uri: Uri) -> impl IntoResponse {
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

    use super::impl2::DATETIME_FORMAT;
    use crate::INDEX_HTML;
    use crate::embed::Asset;
    use time::UtcDateTime;
    use time::format_description::well_known::Rfc2822;

    #[test]
    fn test_last_mod_time() {
        let index = Asset::get(INDEX_HTML).unwrap();
        let lmt_in_sec = index.metadata.last_modified().unwrap();

        dbg!(lmt_in_sec);

        let dt = UtcDateTime::from_unix_timestamp(lmt_in_sec as i64).unwrap();
        let _ = dbg!(dt.format(&DATETIME_FORMAT));
    }
}
