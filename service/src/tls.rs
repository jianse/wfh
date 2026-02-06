use axum::serve::Listener;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::rustls::pki_types::CertificateDer;
use tokio_rustls::rustls::pki_types::PrivateKeyDer;
use tokio_rustls::rustls::pki_types::pem::PemObject;
use tokio_rustls::server::TlsStream;
use tracing::debug;
use tracing::error;

pub struct TlsListener<U: Listener> {
    acceptor: TlsAcceptor,
    under_layer: U,
}

impl<U: Listener> TlsListener<U> {
    pub fn new(acceptor: TlsAcceptor, under_layer: U) -> Self {
        Self {
            acceptor,
            under_layer,
        }
    }
}

impl<U: Listener<Addr = SocketAddr>> Listener for TlsListener<U>
where
    U::Addr: std::fmt::Display,
{
    type Io = TlsStream<U::Io>;
    type Addr = SocketAddr;

    async fn accept(&mut self) -> (Self::Io, Self::Addr) {
        loop {
            let (io, addr) = self.under_layer.accept().await;
            match timeout(Duration::from_secs(3), self.acceptor.accept(io)).await {
                Ok(r) => {
                    match r {
                        Ok(cnx) => {
                            return (cnx, addr);
                        }
                        Err(e) => {
                            error!(
                                "error during tls handshake connection from {}. error: {}",
                                addr, e
                            );
                            // let _ = e.source().
                        }
                    }
                }
                Err(_) => {
                    debug!("handshake timed out");
                }
            };
            // io.poll_shutdown(self).await;
        }
    }

    fn local_addr(&self) -> std::io::Result<Self::Addr> {
        self.under_layer.local_addr()
    }
}

pub fn rustls_server_config(key: impl AsRef<Path>, cert: impl AsRef<Path>) -> Arc<ServerConfig> {
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
