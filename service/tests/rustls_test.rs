use axum::Router;
use axum::routing::get;
use service::tls::TlsListener;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::rustls::pki_types::pem::PemObject;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::test]
#[ignore]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let rustls_config = rustls_server_config(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
    );
    dbg!(&rustls_config);

    let tls_acceptor = TlsAcceptor::from(rustls_config);
    let bind = "[::1]:3000";
    let tcp_listener = TcpListener::bind(bind).await.unwrap();
    // tcp_listener
    info!("HTTPS server listening on {bind}. To contact curl -k https://localhost:3000");
    let listener = TlsListener::new(tls_acceptor, tcp_listener);
    let app = Router::new().route("/", get(handler));

    axum::serve(listener, app)
        .await
        .expect("TODO: panic message");
}

async fn handler() -> &'static str {
    "Hello, World!"
}

fn rustls_server_config(key: impl AsRef<Path>, cert: impl AsRef<Path>) -> Arc<ServerConfig> {
    // CryptoProvider::install_default(/* CryptoProvider */);
    let key = PrivateKeyDer::from_pem_file(key).unwrap();

    let certs = CertificateDer::pem_file_iter(cert)
        .unwrap()
        .map(|cert| cert.unwrap())
        .collect();

    let mut config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("bad certificate/key");

    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    Arc::new(config)
}
