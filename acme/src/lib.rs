use std::collections::HashMap;
use std::fmt::Display;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use backoff::ExponentialBackoff;
use backoff::backoff::Backoff;
use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use chrono::DateTime;
use chrono::TimeDelta;
use chrono::Utc;
use jsonwebtoken::Algorithm;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::jwk::Jwk;
use jsonwebtoken::jwk::ThumbprintHash;
use p256::ecdsa::DerSignature;
use p256::elliptic_curve::rand_core::OsRng;
use p256::elliptic_curve::zeroize::Zeroizing;
use p256::pkcs8::DecodePrivateKey;
use p256::pkcs8::EncodePrivateKey;
use p256::pkcs8::LineEnding;
use reqwest::Client;
use reqwest::Response;
use reqwest::StatusCode;
use reqwest::header::ACCEPT_LANGUAGE;
use reqwest::header::AsHeaderName;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::LOCATION;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use tokio::time::sleep;
use tracing::debug;
use tracing::info;
use tracing::instrument::WithSubscriber;
use tracing::warn;
use x509_cert::Certificate;
use x509_cert::builder::Builder;
use x509_cert::builder::RequestBuilder;
use x509_cert::certificate::CertificateInner;
use x509_cert::der::Encode;
use x509_cert::der::EncodePem;
use x509_cert::der::asn1::Ia5String;
use x509_cert::ext::pkix::SubjectAltName;
use x509_cert::ext::pkix::name::GeneralName;
use x509_cert::name::Name;
use x509_cert::request::CertReq;

pub use crate::authz::Authz;
pub use crate::authz::Challenge;
pub use crate::authz::ChallengeStatus;
pub use crate::authz::ChallengeType;
pub use crate::error::AcmeError;
pub use crate::error::AcmeErrorResponse;
pub use crate::error::AcmeResult;
pub use crate::jws::compute_dns01_challenge;
pub use crate::jws::compute_http01_challenge;
pub use crate::order::FinalizeOrderPayload;
pub use crate::order::Identifier;
pub use crate::order::NewOrderPayload;
pub use crate::order::NewOrderRes;
pub use crate::order::OrderRes;
pub use crate::order::OrderStatus;

mod account;
mod authz;
mod error;
mod jws;
mod order;

#[derive(Debug, Deserialize)]
pub struct Directory {
    #[serde(rename = "newNonce")]
    pub new_nonce: String,

    #[serde(rename = "newAccount")]
    pub new_account: String,
    #[serde(rename = "newOrder")]
    pub new_order: String,
    #[serde(rename = "newAuthz")]
    pub new_authz: Option<String>,

    #[serde(rename = "revokeCert")]
    pub revoke_cert: String,
    #[serde(rename = "keyChange")]
    pub key_change: String,
}

impl Directory {
    pub async fn fetch(client: &Client, url: &str) -> AcmeResult<Self> {
        let text = client.get(url).send().await?.text().await?;
        debug!("{}", &text);
        let dir = serde_json::from_str::<Directory>(&text)?;
        Ok(dir)
    }

    pub async fn get_nonce(&self, client: &Client) -> AcmeResult<HeaderValue> {
        client
            .head(&self.new_nonce)
            .send()
            .await?
            .headers()
            .get(REPLAY_NONCE)
            .cloned()
            .ok_or_else(|| AcmeError::MissingHeader(REPLAY_NONCE.to_string()))
    }
}

const REPLAY_NONCE: &str = "Replay-Nonce";

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LocalConfig {
    #[serde(default)]
    accounts: HashMap<String, LocalAccount>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocalAccount {
    pub email: String,
    key: String,
    pub kid: String,
}

impl LocalAccount {
    pub fn jwk_thumbprint(&self) -> AcmeResult<String> {
        let ek = EncodingKey::from_ec_pem(self.key.as_bytes())?;
        let jwk = Jwk::from_encoding_key(&ek, Algorithm::ES256)?;
        let thumbprint = jwk.thumbprint(ThumbprintHash::SHA256);
        Ok(thumbprint)
    }

    pub fn key_pair(&self) -> AcmeResult<p256::SecretKey> {
        let pem = self.key.as_str();
        Ok(p256::SecretKey::from_pkcs8_pem(pem)?)
    }
}

pub struct AcmeClient {
    client: Client,
    dir_url: String,
    directory: Directory,
    nonce: String,
    config: LocalConfig,
    cache_dir: PathBuf,
}

/// Builder for creating an [`AcmeClient`] with custom configuration.
///
/// # Example
///
/// ```no_run
/// use acme::AcmeClientBuilder;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = AcmeClientBuilder::new()
///     .directory_url("https://acme-staging-v02.api.letsencrypt.org/directory")
///     .cache_dir("./acme-cache")
///     .user_agent("MyApp/1.0")
///     .accept_invalid_certs(false)
///     .build()
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct AcmeClientBuilder {
    directory_url: Option<String>,
    cache_dir: Option<PathBuf>,
    #[deprecated(note = "Exponential backoff is now used instead of fixed retry count")]
    max_retry_count: Option<usize>,
    accept_invalid_certs: bool,
    user_agent: Option<String>,
    language: Option<String>,
}

impl AcmeClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn directory_url(mut self, url: &str) -> Self {
        self.directory_url = Some(url.to_string());
        self
    }

    pub fn cache_dir<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.cache_dir = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn max_retry_count(mut self, count: usize) -> Self {
        self.max_retry_count = Some(count);
        self
    }

    /// Sets whether to accept invalid TLS certificates.
    ///
    /// # ⚠️ Security Warning
    ///
    /// Setting this to `true` disables TLS certificate validation, which makes
    /// the connection vulnerable to man-in-the-middle attacks. This should only
    /// be used for:
    /// - Testing against local ACME servers (e.g., Pebble)
    /// - Development environments with self-signed certificates
    ///
    /// **NEVER** use this in production with real ACME servers like Let's Encrypt.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use acme::AcmeClientBuilder;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // For testing with Pebble (local test server)
    /// let client = AcmeClientBuilder::new()
    ///     .directory_url("https://localhost:14000/dir")
    ///     .cache_dir("./acme-cache")
    ///     .accept_invalid_certs(true)  // Only for local testing!
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn accept_invalid_certs(mut self, accept: bool) -> Self {
        self.accept_invalid_certs = accept;
        self
    }

    pub fn user_agent(mut self, ua: &str) -> Self {
        self.user_agent = Some(ua.to_string());
        self
    }

    pub fn language(mut self, lang: &str) -> Self {
        self.language = Some(lang.to_string());
        self
    }

    pub async fn build(self) -> AcmeResult<AcmeClient> {
        let dir_url = self
            .directory_url
            .ok_or_else(|| AcmeError::InvalidConfig("directory_url is required".to_string()))?;

        let cache_dir = self
            .cache_dir
            .ok_or_else(|| AcmeError::InvalidConfig("cache_dir is required".to_string()))?;

        let mut headers = HeaderMap::new();
        if let Some(lang) = self.language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(&lang)?);
        } else {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en"));
        }

        let mut builder = Client::builder()
            .user_agent(
                self.user_agent
                    .unwrap_or_else(|| "Rust ACME Client/1.0".to_string()),
            )
            .default_headers(headers);

        if self.accept_invalid_certs {
            builder = builder.danger_accept_invalid_certs(true);
        }

        let client = builder.build()?;

        if !cache_dir.exists() {
            std::fs::create_dir_all(&cache_dir)?;
        }

        let config_file = cache_dir.join("config.json");
        if !config_file.exists() {
            std::fs::write(&config_file, "{}")?;
        }
        let file = std::fs::File::open(config_file)?;
        let rdr = std::io::BufReader::new(file);
        let config = serde_json::from_reader(rdr)?;

        let directory = Directory::fetch(&client, &dir_url).await?;
        let nonce = Self::fetch_nonce(&client, &directory.new_nonce).await?;

        Ok(AcmeClient {
            client,
            dir_url,
            directory,
            nonce,
            config,
            cache_dir,
        })
    }

    async fn fetch_nonce(client: &Client, url: &str) -> AcmeResult<String> {
        let resp = client.head(url).send().await?;
        let nonce_header = resp
            .headers()
            .get(REPLAY_NONCE)
            .ok_or_else(|| AcmeError::MissingHeader(REPLAY_NONCE.to_string()))?;
        Ok(nonce_header.to_str()?.to_owned())
    }
}

impl AcmeClient {
    /// Creates an exponential backoff strategy for retrying failed operations.
    fn create_backoff(&self) -> ExponentialBackoff {
        ExponentialBackoff {
            initial_interval: Duration::from_secs(1),
            max_interval: Duration::from_secs(30),
            multiplier: 2.0,
            max_elapsed_time: Some(Duration::from_secs(300)), // 5 minutes total
            ..Default::default()
        }
    }

    /// Returns a new [`AcmeClientBuilder`] for configuring the client.
    ///
    /// This is a convenience method that is equivalent to calling
    /// [`AcmeClientBuilder::new()`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use acme::AcmeClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AcmeClient::builder()
    ///     .directory_url("https://acme-staging-v02.api.letsencrypt.org/directory")
    ///     .cache_dir("./acme-cache")
    ///     .accept_invalid_certs(false)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder() -> AcmeClientBuilder {
        AcmeClientBuilder::new()
    }

    pub async fn new<P: AsRef<Path>>(dir_url: &str, cache_dir: P) -> AcmeResult<Self> {
        // For production use, always use AcmeClientBuilder with accept_invalid_certs(false).
        AcmeClientBuilder::new()
            .directory_url(dir_url)
            .cache_dir(cache_dir)
            .accept_invalid_certs(false)
            .build()
            .await
    }

    pub async fn reload_directory(&mut self) -> AcmeResult<()> {
        self.directory = Directory::fetch(&self.client, &self.dir_url).await?;
        Ok(())
    }

    pub async fn refresh_nonce(&mut self) -> AcmeResult<()> {
        let nonce = self.directory.get_nonce(&self.client).await?;
        self.nonce = nonce.to_str()?.to_owned();
        Ok(())
    }

    async fn create_account(
        &mut self,
        email: &str,
        key: &EncodingKey,
    ) -> AcmeResult<account::NewAccountRes> {
        let payload = account::NewAccountPayload {
            contact: Some(vec![format!("mailto:{}", email)]),
            terms_of_service_agreed: Some(true),
            ..Default::default()
        };

        let res = account::new_account(
            &self.client,
            &self.directory.new_account,
            &self.nonce,
            &payload,
            key,
        )
        .await?;
        debug!("{:#?}", &res);

        self.nonce = res.nonce.clone();
        Ok(res)
    }

    async fn check_account_exist(&mut self, acc: &LocalAccount) -> AcmeResult<bool> {
        let payload = account::NewAccountPayload {
            only_return_existing: Some(true),
            ..Default::default()
        };
        let sk = EncodingKey::from_ec_pem(acc.key.as_bytes())?;

        let mut header = Header::new(Algorithm::ES256);
        header.url = Some(self.directory.new_account.clone());
        let jwk = Jwk::from_encoding_key(&sk, Algorithm::ES256)?;
        header.jwk = Some(jwk);

        let mut backoff = self.create_backoff();

        loop {
            let jws = crate::jws::create_jwk_jws(
                &sk,
                Some(&payload),
                &self.directory.new_account,
                &self.nonce,
            )?;
            let body = serde_json::to_string(&jws)?;

            let resp = self
                .client
                .post(&self.directory.new_account)
                .body(body)
                .header(CONTENT_TYPE, "application/jose+json")
                .send()
                .await?;

            let status = resp.status();
            debug!("check account exist status: {}", status);

            match status {
                StatusCode::OK => {
                    self.nonce = get_header_str(&resp, REPLAY_NONCE)?;
                    return Ok(true);
                }
                StatusCode::BAD_REQUEST => {
                    self.nonce = get_header_str(&resp, REPLAY_NONCE)?;
                    let text = resp.text().await?;
                    if let Some(err) = AcmeError::from_response(status, &text)
                        && matches!(err, AcmeError::AccountNotExist)
                    {
                        return Ok(false);
                    }
                    warn!("{}", text);
                }
                _ => {
                    return Err(AcmeError::InvalidConfig(format!(
                        "Unexpected status: {}",
                        status
                    )));
                }
            }

            // Apply exponential backoff
            if let Some(next) = backoff.next_backoff() {
                debug!("Retrying check_account_exist in {:?}", next);
                tokio::time::sleep(next).await;
            } else {
                return Err(AcmeError::MaxRetryReached);
            }
        }
    }

    pub async fn load_or_create_account(&mut self, email: &str) -> AcmeResult<LocalAccount> {
        let acc = if let Some(mut acc) = self.config.accounts.remove(email) {
            if !self.check_account_exist(&acc).await? {
                let sk = EncodingKey::from_ec_pem(acc.key.as_bytes())?;
                let res = self.create_account(email, &sk).await?;
                acc.kid = res.kid;
            }
            acc
        } else {
            let key = Self::generate_key()?;
            let sk = EncodingKey::from_ec_pem(key.as_bytes())?;
            let res = self.create_account(email, &sk).await?;
            LocalAccount {
                email: email.to_string(),
                key: key.to_string(),
                kid: res.kid,
            }
        };

        self.config.accounts.insert(email.to_string(), acc.clone());
        self.save_config()?;
        Ok(acc)
    }

    fn save_config(&self) -> AcmeResult<()> {
        let config_file = self.cache_dir.join("config.json");
        let file = std::fs::File::create(config_file)?;
        let wtr = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(wtr, &self.config)?;
        Ok(())
    }

    pub fn generate_key() -> AcmeResult<Zeroizing<String>> {
        let mut rng = OsRng;
        let key = p256::SecretKey::random(&mut rng);
        Ok(key.to_pkcs8_pem(LineEnding::default())?)
    }

    pub async fn get_authorization(&mut self, acc: &LocalAccount, url: &str) -> AcmeResult<Authz> {
        let jws = crate::jws::create_kid_jws::<String>(acc, None, url, &self.nonce)?;
        let body = serde_json::to_string(&jws)?;
        debug!("{:#?}", &body);

        let resp = self
            .client
            .post(url)
            .body(body)
            .header(CONTENT_TYPE, "application/jose+json")
            .send()
            .await?;

        self.nonce = get_header_str(&resp, REPLAY_NONCE)?;

        let text = resp.text().await?;
        debug!("authz body:{}", &text);

        Ok(serde_json::from_str::<Authz>(&text)?)
    }

    pub async fn new_order(
        &mut self,
        acc: &LocalAccount,
        domains: &[&str],
    ) -> AcmeResult<NewOrderRes> {
        let identifiers: Vec<Identifier> = domains
            .iter()
            .map(|domain| Identifier {
                typ: "dns".to_string(),
                value: domain.to_string(),
            })
            .collect();

        let payload = NewOrderPayload {
            identifiers,
            not_before: None,
            not_after: None,
        };

        let mut backoff = self.create_backoff();

        loop {
            let jws = crate::jws::create_kid_jws(
                acc,
                Some(&payload),
                &self.directory.new_order,
                &self.nonce,
            )?;
            let body = serde_json::to_vec(&jws)?;

            let resp = self
                .client
                .post(&self.directory.new_order)
                .body(body)
                .header(CONTENT_TYPE, "application/jose+json")
                .send()
                .await?;

            self.nonce = get_header_str(&resp, REPLAY_NONCE)?;

            let status = resp.status();
            match status {
                StatusCode::CREATED => {
                    let location = get_header_str(&resp, LOCATION)?;
                    let text = resp.text().await?;
                    debug!("{}", text);

                    let order_res: OrderRes = serde_json::from_str(&text)?;
                    debug!("{:#?}", &order_res);

                    return Ok(NewOrderRes {
                        order: order_res,
                        location,
                    });
                }
                StatusCode::BAD_REQUEST => {
                    let text = resp.text().await?;
                    if let Some(err) = AcmeError::from_response(status, &text) {
                        return Err(err);
                    }
                }
                _ => {
                    info!(op = "new order", ?status, "doing retry");
                }
            }

            // Apply exponential backoff
            if let Some(next) = backoff.next_backoff() {
                debug!("Retrying new_order in {:?}", next);
                tokio::time::sleep(next).await;
            } else {
                return Err(AcmeError::MaxRetryReached);
            }
        }
    }

    pub async fn challenge(
        &mut self,
        acc: &LocalAccount,
        challenge: &Challenge,
    ) -> AcmeResult<Challenge> {
        let mut backoff = self.create_backoff();

        loop {
            let payload = json!({});
            let jws = crate::jws::create_kid_jws(acc, Some(&payload), &challenge.url, &self.nonce)?;
            let body = serde_json::to_vec(&jws)?;

            let resp = self
                .client
                .post(&challenge.url)
                .body(body)
                .header(CONTENT_TYPE, "application/jose+json")
                .send()
                .await?;

            self.nonce = get_header_str(&resp, REPLAY_NONCE)?;

            let status = resp.status();
            if status.is_success() {
                let text = resp.text().await?;
                debug!("challenge resp body: {:#?}", &text);
                return Ok(serde_json::from_str(&text)?);
            }

            // Check for non-retryable errors
            if status == StatusCode::BAD_REQUEST {
                let text = resp.text().await?;
                if let Some(err) = AcmeError::from_response(status, &text) {
                    return Err(err);
                }
            }

            // Apply exponential backoff
            if let Some(next) = backoff.next_backoff() {
                debug!("Retrying challenge in {:?}", next);
                tokio::time::sleep(next).await;
            } else {
                return Err(AcmeError::MaxRetryReached);
            }
        }
    }

    async fn post_as_get(&mut self, acc: &LocalAccount, url: &str) -> AcmeResult<String> {
        let mut backoff = self.create_backoff();

        loop {
            let jws = crate::jws::create_kid_jws::<String>(acc, None, url, &self.nonce)?;
            let body = serde_json::to_string(&jws)?;

            let resp = self
                .client
                .post(url)
                .body(body)
                .header(CONTENT_TYPE, "application/jose+json")
                .send()
                .await?;

            let status_code = resp.status();
            if status_code == StatusCode::OK {
                self.nonce = get_header_str(&resp, REPLAY_NONCE)?;
                let text = resp.text().await?;
                debug!("{}", &text);
                return Ok(text);
            }

            debug!("error when post_as_get. status = {}, retrying", status_code);

            // Apply exponential backoff
            if let Some(next) = backoff.next_backoff() {
                tokio::time::sleep(next).await;
            } else {
                return Err(AcmeError::MaxRetryReached);
            }
        }
    }

    pub async fn finalize_order(
        &mut self,
        order: &mut NewOrderRes,
        acc: &LocalAccount,
        csr: &CertReq,
    ) -> AcmeResult<()> {
        let der = csr.to_der()?;
        let csr_b64 = BASE64_URL_SAFE_NO_PAD.encode(der);
        let payload = FinalizeOrderPayload { csr: csr_b64 };

        let mut backoff = self.create_backoff();

        loop {
            let jws = crate::jws::create_kid_jws(
                acc,
                Some(&payload),
                &order.order.finalize,
                &self.nonce,
            )?;
            let body = serde_json::to_vec(&jws)?;

            let resp = self
                .client
                .post(&order.order.finalize)
                .body(body)
                .header(CONTENT_TYPE, "application/jose+json")
                .send()
                .await?;

            debug!("finalize order resp: {:#?}", &resp);
            let headers = resp.headers().clone();
            self.nonce = get_header_str(&resp, REPLAY_NONCE)?;

            let status_code = resp.status();
            match status_code {
                StatusCode::OK => {
                    let text = resp.text().await?;
                    debug!("finalize order resp body: {}", text);
                    let order_res: OrderRes = serde_json::from_str(&text)?;
                    debug!("{:#?}", &order_res);

                    match order_res.status {
                        OrderStatus::Processing => {
                            let retry_after = headers
                                .get("Retry-After")
                                .ok_or_else(|| AcmeError::MissingHeader("Retry-After".to_string()))?
                                .to_str()?;

                            self.wait_for_order_processing(retry_after).await?;
                            let text = self.post_as_get(acc, &order.location).await?;
                            let order_res = serde_json::from_str::<OrderRes>(&text)?;
                            debug!("order after waiting is :{:#?}", &order_res);
                            order.order = order_res;
                            return Ok(());
                        }
                        OrderStatus::Valid => {
                            order.order = order_res;
                            return Ok(());
                        }
                        s => {
                            return Err(AcmeError::OrderNotValid(s.to_string()));
                        }
                    }
                }
                _ => {
                    debug!(
                        "finalize order failed! status code: {}, retrying",
                        status_code
                    );
                }
            }

            // Apply exponential backoff
            if let Some(next) = backoff.next_backoff() {
                tokio::time::sleep(next).await;
            } else {
                return Err(AcmeError::MaxRetryReached);
            }
        }
    }

    async fn wait_for_order_processing(&self, retry_after: &str) -> AcmeResult<()> {
        if let Ok(t) = retry_after.parse::<u64>() {
            sleep(Duration::from_secs(t)).await;
        } else if let Ok(t) = DateTime::parse_from_rfc2822(retry_after) {
            let d = t.signed_duration_since(Utc::now());
            if d > TimeDelta::zero() && d < TimeDelta::seconds(20) {
                sleep(d.to_std()?).await;
            } else {
                return Err(AcmeError::InvalidConfig(
                    "invalid Retry-After header".to_string(),
                ));
            }
        } else {
            return Err(AcmeError::InvalidConfig(
                "invalid Retry-After header".to_string(),
            ));
        }
        Ok(())
    }

    pub async fn download_cert(
        &mut self,
        acc: &LocalAccount,
        order: &NewOrderRes,
    ) -> AcmeResult<CertificateChain> {
        if order.order.status != OrderStatus::Valid {
            return Err(AcmeError::OrderNotValid(order.order.status.to_string()));
        }

        let url = order
            .order
            .certificate
            .as_ref()
            .ok_or_else(|| AcmeError::OrderNotValid("no certificate URL".to_string()))?;

        // post_as_get already has retry logic, so we just call it once
        let text = self.post_as_get(acc, url).await?;
        CertificateChain::from_pem(&text)
    }

    pub async fn revoke_certificate(
        &mut self,
        acc: &LocalAccount,
        cert_der: &[u8],
    ) -> AcmeResult<()> {
        let mut backoff = self.create_backoff();

        loop {
            let cert_b64 = BASE64_URL_SAFE_NO_PAD.encode(cert_der);
            let payload = json!({ "certificate": cert_b64 });

            let jws = crate::jws::create_kid_jws(
                acc,
                Some(&payload),
                &self.directory.revoke_cert,
                &self.nonce,
            )?;
            let body = serde_json::to_vec(&jws)?;

            let resp = self
                .client
                .post(&self.directory.revoke_cert)
                .body(body)
                .header(CONTENT_TYPE, "application/jose+json")
                .send()
                .await?;

            let status = resp.status();
            self.nonce = get_header_str(&resp, REPLAY_NONCE)?;

            match status {
                StatusCode::OK => return Ok(()),
                StatusCode::BAD_REQUEST => {
                    let text = resp.text().await?;
                    if let Some(err) = AcmeError::from_response(status, &text) {
                        return Err(err);
                    }
                }
                _ => {
                    debug!(
                        "revoke certificate failed! status code: {}, retrying",
                        status
                    );
                }
            }

            // Apply exponential backoff
            if let Some(next) = backoff.next_backoff() {
                tokio::time::sleep(next).await;
            } else {
                return Err(AcmeError::MaxRetryReached);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CertificateChain {
    pub certs: Vec<CertificateInner>,
}

impl CertificateChain {
    pub fn from_pem(pem_str: &str) -> AcmeResult<Self> {
        let certs = Certificate::load_pem_chain(pem_str.as_bytes())?;

        Ok(Self { certs })
    }

    pub fn to_pem(&self) -> String {
        self.certs
            .iter()
            .map(|c| c.to_pem(LineEnding::default()))
            .collect::<std::result::Result<Vec<_>, _>>()
            .unwrap()
            .join("")
    }

    /// Returns the expiry date of the leaf certificate.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use acme::{AcmeClient, CertificateChain};
    /// # async fn example(cert: &CertificateChain) -> Result<(), Box<dyn std::error::Error>> {
    /// let expiry = cert.certificate_expiry()?;
    /// println!("Certificate expires at: {}", expiry);
    /// # Ok(())
    /// # }
    /// ```
    pub fn certificate_expiry(&self) -> AcmeResult<DateTime<Utc>> {
        let cert = &self.certs[0];

        let not_after = cert.tbs_certificate.validity.not_after;
        let asn1_time = not_after.to_system_time();
        Ok(DateTime::from(asn1_time))
    }

    /// Returns the number of days until the certificate expires.
    ///
    /// Returns a negative value if the certificate has already expired.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use acme::CertificateChain;
    /// # fn example(cert: &CertificateChain) -> Result<(), Box<dyn std::error::Error>> {
    /// let days = cert.days_until_expiry()?;
    /// if days < 30 {
    ///     println!("Certificate will expire in {} days, renewal needed!", days);
    /// } else {
    ///     println!("Certificate valid for {} more days", days);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn days_until_expiry(&self) -> AcmeResult<i64> {
        let expiry = self.certificate_expiry()?;
        let now = Utc::now();
        let duration = expiry.signed_duration_since(now);
        Ok(duration.num_days())
    }

    /// Checks if the certificate should be renewed based on the given threshold.
    ///
    /// Returns `true` if the certificate expires within the specified number of days,
    /// or if it has already expired.
    ///
    /// # Arguments
    ///
    /// * `days_before_expiry` - Number of days before expiry to trigger renewal
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use acme::CertificateChain;
    /// # fn example(cert: &CertificateChain) -> Result<(), Box<dyn std::error::Error>> {
    /// // Renew if expires within 30 days
    /// if cert.should_renew(30)? {
    ///     println!("Time to renew the certificate!");
    /// } else {
    ///     println!("Certificate is still valid");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn should_renew(&self, days_before_expiry: u32) -> AcmeResult<bool> {
        let days_left = self.days_until_expiry()?;
        Ok(days_left < days_before_expiry as i64)
    }

    /// Checks if the certificate has already expired.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use acme::CertificateChain;
    /// # fn example(cert: &CertificateChain) -> Result<(), Box<dyn std::error::Error>> {
    /// if cert.is_expired()? {
    ///     eprintln!("Certificate has expired!");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_expired(&self) -> AcmeResult<bool> {
        let days_left = self.days_until_expiry()?;
        Ok(days_left <= 0)
    }
}

fn get_header_str<K: AsHeaderName + Display + Clone>(
    resp: &Response,
    header_name: K,
) -> AcmeResult<String> {
    Ok(resp
        .headers()
        .get(header_name.clone())
        .cloned()
        .ok_or_else(|| AcmeError::MissingHeader(header_name.to_string()))?
        .to_str()?
        .to_string())
}

pub fn gen_p256_key() -> p256::SecretKey {
    let mut rng = OsRng;
    p256::SecretKey::random(&mut rng)
}

/// Converts a P-256 secret key to PKCS#8 PEM format
pub fn key_to_pkcs8_pem(key: &p256::SecretKey) -> AcmeResult<String> {
    Ok(key.to_pkcs8_pem(LineEnding::default())?.to_string())
}

pub fn make_csr(key: &p256::SecretKey, domains: &[&str]) -> AcmeResult<CertReq> {
    let primary_domain = domains.first().ok_or(AcmeError::AccountNotExist)?;

    let subject = Name::from_str(&format!("CN={}", primary_domain))?;

    let req_signer = p256::ecdsa::SigningKey::from(key);
    let mut builder = RequestBuilder::new(subject, &req_signer)?;

    let sans: AcmeResult<Vec<GeneralName>> = domains
        .iter()
        .map(|domain| Ok(GeneralName::DnsName(Ia5String::new(domain)?)))
        .collect();

    builder.add_extension(&SubjectAltName(sans?))?;

    let cert_req = builder.build::<DerSignature>()?;
    Ok(cert_req)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::sync::Arc;
    use std::time::Duration;

    use axum::Router;
    use axum::extract::Path;
    use axum::extract::State;
    use axum::http::Uri;
    use axum::response::IntoResponse;
    use axum::routing::get;
    use base64::Engine;
    use base64::prelude::BASE64_URL_SAFE_NO_PAD;
    use eyre::OptionExt;
    use tokio::net::TcpListener;
    use tokio::sync::RwLock;
    use tokio::sync::oneshot::Receiver;
    use tokio::sync::oneshot::Sender;
    use tokio::time::sleep;
    use tracing::debug;
    use tracing::info;
    use x509_cert::der::EncodePem;

    use super::*;

    pub fn init_logger() -> AcmeResult<()> {
        let filter =
            tracing_subscriber::EnvFilter::from_str("debug,hyper_util::client=info,h2=info")
                .map_err(|e| AcmeError::InvalidConfig(format!("Failed to init logger: {}", e)))?;
        let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
        Ok(())
    }

    async fn method_router(
        Path(token): Path<String>,
        State(state): State<SharedState>,
    ) -> impl IntoResponse {
        let state = state.read().await;
        info!(token, "acme endpoint hit");

        match state.get(&token) {
            Some(v) => (StatusCode::OK, v.clone()),
            None => (StatusCode::NOT_FOUND, "Not Found".to_owned()),
        }
    }

    async fn fallback_router(uri: Uri) -> impl IntoResponse {
        let uri = uri.to_string();
        info!(uri, "fallback router hit");
        (StatusCode::NOT_FOUND, "Not Found".to_owned())
    }

    type SharedState = Arc<RwLock<HashMap<String, String>>>;

    #[tokio::test]
    #[ignore = "Requires external Pebble ACME server"]
    async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let _ = init_logger();

        let dir_url = "https://localhost:14000/dir";
        let email = "test@example.com";
        let domain = "123.example";

        let mut c = AcmeClient::builder()
            .directory_url(dir_url)
            .cache_dir("./tmp")
            .accept_invalid_certs(true)
            .build()
            .await?;

        let acc = c.load_or_create_account(email).await?;

        let domains = vec![domain];
        let mut new_order_res = c.new_order(&acc, &domains).await?;

        debug!("{:#?}", &new_order_res);

        let authz_url = new_order_res.order.authorizations[0].clone();

        let authz = c.get_authorization(&acc, &authz_url).await?;

        debug!("{:#?}", &authz);

        let http_challenge = authz
            .get_challenge(ChallengeType::Http01)
            .ok_or_else(|| "http-01 is not supported")?;

        if http_challenge.is_pending() {
            let thumbprint = acc.jwk_thumbprint()?;
            let key_auth = format!("{}.{}", http_challenge.token, thumbprint);

            let mut map = HashMap::new();
            map.insert(http_challenge.token.clone(), key_auth);

            let state = Arc::new(RwLock::new(map));

            let (tx, rx) = tokio::sync::oneshot::channel::<()>();
            let (ready_tx, ready_rx) = tokio::sync::oneshot::channel::<()>();
            tokio::spawn(run_server(state, rx, ready_tx));

            ready_rx.await?;

            sleep(Duration::from_secs(1)).await;
            let challenge_res = c.challenge(&acc, http_challenge).await?;

            debug!("{:#?}", &challenge_res);

            sleep(Duration::from_secs(5)).await;

            let authz = c.get_authorization(&acc, &authz_url).await?;

            let http_challenge = authz
                .get_challenge(ChallengeType::Http01)
                .ok_or_else(|| "http-01 is not supported")?;

            if http_challenge.is_valid() {
                info!("challenge now valid");
            } else {
                return Err(
                    format!("Unexpected challenge status: {:?}", http_challenge.status).into(),
                );
            }

            tx.send(()).ok();
        } else if http_challenge.is_valid() {
            info!("challenge already valid");
        } else {
            return Err(format!("Unexpected challenge status: {:?}", http_challenge.status).into());
        }

        let key = gen_p256_key();
        let domain_refs: Vec<&str> = domains.iter().map(|s| s.as_ref()).collect();
        let csr = make_csr(&key, &domain_refs)?;
        c.finalize_order(&mut new_order_res, &acc, &csr).await?;

        if new_order_res.order.status == OrderStatus::Valid {
            info!("order valid, download certificate");
            let cert = c.download_cert(&acc, &new_order_res).await?;
            info!("cert: {}", &cert.to_pem());
        } else {
            return Err(format!("Order not valid: {:?}", new_order_res.order.status).into());
        }

        Ok(())
    }

    async fn run_server(
        state: SharedState,
        rx: Receiver<()>,
        ready_tx: Sender<()>,
    ) -> std::io::Result<()> {
        let app = Router::new()
            .route("/.well-known/acme-challenge/{token}", get(method_router))
            .with_state(state)
            .fallback(fallback_router);

        let listener = TcpListener::bind("127.0.0.1:5002").await?;
        ready_tx
            .send(())
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "send ready failed"))?;
        axum::serve(listener, app)
            .with_graceful_shutdown(sig(rx))
            .await?;
        Ok(())
    }

    async fn sig(rx: Receiver<()>) {
        let _ = rx.await;
    }

    #[tokio::test]
    #[ignore = "Integration test"]
    async fn test_run_server() -> Result<(), Box<dyn std::error::Error>> {
        let _ = init_logger();
        let state = Arc::new(RwLock::new(HashMap::new()));
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let (ready_tx, _ready_rx) = tokio::sync::oneshot::channel::<()>();
        tokio::spawn(run_server(state, rx, ready_tx));

        sleep(Duration::from_secs(5)).await;
        let _ = tx.send(());
        Ok(())
    }

    #[test]
    fn test_gen_csr_der() -> AcmeResult<()> {
        let _ = init_logger();
        let key = gen_p256_key();
        let domains = vec!["test.example.com"];
        let csr = make_csr(&key, &domains)?;
        debug!("csr : \n{}", &csr.to_pem(LineEnding::default())?);
        let der = csr.to_der()?;
        debug!("csr : {:?}", BASE64_URL_SAFE_NO_PAD.encode(&der));
        Ok(())
    }

    #[tokio::test]
    #[ignore = "Requires external Pebble ACME server"]
    async fn test_account_not_exist() -> Result<(), Box<dyn std::error::Error>> {
        let _ = init_logger();
        let dir_url = "https://localhost:14000/dir";
        let email = "test1@example.com";
        let mut acme_client = AcmeClient::builder()
            .directory_url(dir_url)
            .cache_dir("tmp/")
            .accept_invalid_certs(true) // For testing with Pebble
            .build()
            .await?;

        let acc = LocalAccount {
            email: email.to_string(),
            key: gen_p256_key()
                .to_pkcs8_pem(LineEnding::default())?
                .to_string(),
            kid: "".to_owned(),
        };

        let exist = acme_client.check_account_exist(&acc).await?;
        debug!("account exist : {}", exist);
        Ok(())
    }
}
