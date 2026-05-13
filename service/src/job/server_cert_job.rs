use std::fs;
use std::time::Duration;

use acme::AcmeClient;
use acme::gen_p256_key;
use acme::key_to_pkcs8_pem;
use acme::make_csr;
use async_trait::async_trait;

use tokio::time::sleep;
use tracing::debug;

use super::Job;
use super::JobType;
use crate::AppState;
use crate::api::BizError;
use crate::api::BizResult;

pub struct ServerCertJob {
    state: AppState,
}

#[async_trait]
impl Job for ServerCertJob {
    fn name(&self) -> &'static str {
        "server_cert"
    }

    fn job_type(&self) -> JobType {
        JobType::Periodic
    }

    async fn bootstrap_run_duration(&self) -> Duration {
        Duration::ZERO
    }

    async fn job_body(&self) -> BizResult<Option<Duration>> {
        let g_settings = settings::get();
        if let Some(tls) = &g_settings.server.tls
            && let Some(acme) = &tls.acme
        {
            let mut acme_client = AcmeClient::new(&acme.server, &acme.email).await?;
            let domain = g_settings.server.host.clone();

            let account = acme_client.load_or_create_account(&acme.email).await?;

            let mut order = acme_client.new_order(&account, &[&domain]).await?;
            for auth_url in order.order.authorizations.iter() {
                let auth = acme_client.get_authorization(&account, auth_url).await?;
                if let Some(c) = auth.get_http01_challenge() {
                    let value = format!("{}.{}", c.token, account.jwk_thumbprint()?);
                    self.state.set_acme_token(c.token.clone(), value).await;
                    let _c_res = acme_client.challenge(&account, &c.url).await?;

                    debug!("challenge res: {:?}", _c_res);

                    sleep(Duration::from_secs(5)).await;

                    let c = auth.get_http01_challenge().unwrap();
                    if !c.status.is_valid() {
                        self.state.clear_acme_token().await;
                        return Err(BizError::custom("acme valid challenge error!"));
                    }
                }
            }

            self.state.clear_acme_token().await;

            let key = gen_p256_key();
            let domain_refs = [domain.as_str()];
            let csr = make_csr(&key, &domain_refs)?;
            acme_client
                .finalize_order(&mut order, &account, &csr)
                .await?;

            if order.order.status == acme::OrderStatus::Valid {
                debug!("order is valid");
                let cert = acme_client.download_cert(&account, &order).await?;
                let key_pem = key_to_pkcs8_pem(&key)?;
                fs::write(&tls.key, key_pem)?;
                fs::write(&tls.cert, cert.to_pem())?;
                return Ok(None);
            } else {
                return Err(BizError::custom("acme sign cert error!"));
            }
        }
        Ok(None)
    }
}

impl ServerCertJob {
    pub fn new(state: AppState) -> Self {
        ServerCertJob { state }
    }

    pub(super) fn boxed(self) -> Box<dyn Job + Send + Sync> {
        Box::new(self)
    }
}
