use axum::{Router, ServiceExt};
use axum_server::tls_rustls::RustlsConfig;
use eyre::Result;
use service::init_logger;
use service::tls::rustls_server_config;
use std::path::PathBuf;
use std::sync::Arc;
use tokio_rustls::rustls::crypto::ring::default_provider;

#[tokio::test]
async fn serve() -> Result<()> {
    let p = default_provider().install_default();
    init_logger(1);
    let app = Router::new().fallback(async || "hello world!");
    let addr = "0.0.0.0:8080".parse()?;
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await?;
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
