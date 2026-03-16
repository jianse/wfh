mod defaults;
mod helpers;

use std::collections::HashMap;
use std::fmt::Display;
use std::path::Path;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;

use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use config::Config;
use config::ConfigError;
use helpers::DurationString;
use rand::RngExt;
use rand::distr::Alphanumeric;
use rand::rng;
use serde::Deserialize;
use serde::Serialize;
use serde_with::TryFromInto;
use serde_with::serde_as;
use thiserror::Error;
use tracing::error;
use tracing::info;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub struct Settings {
    /// server settings
    pub server: ServerSettings,

    /// database settings
    pub db: DatabaseSettings,

    /// auth settings
    pub auth: AuthenticateSettings,

    /// ui settings
    pub ui: UiSettings,

    /// log settings
    pub log: LogSettings,

    /// cors settings
    pub cors: Option<CorsSettings>,

    /// job settings
    pub job: JobSettings,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy)]
pub enum JobRetryPolicy {
    #[serde(alias = "fixed")]
    Fixed,

    #[serde(alias = "linear")]
    Linear,

    #[default]
    #[serde(alias = "exponential")]
    Exponential,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct JobSettingOverrides {
    pub interval_secs: Option<u64>,
    pub retry_policy: Option<JobRetryPolicy>,
    pub max_retries: Option<u32>,
    pub timeout_secs: Option<u64>,
    #[serde(default = "defaults::bool::_true")]
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JobSettings {
    pub interval_secs: u64,
    pub retry_policy: JobRetryPolicy,
    pub max_retries: u32,
    pub timeout_secs: u64,
    #[serde(default = "Default::default")]
    pub overrides: HashMap<String, JobSettingOverrides>,
}

impl Default for JobSettings {
    fn default() -> Self {
        Self {
            interval_secs: 10 * 60,
            retry_policy: Default::default(),
            max_retries: 5,
            timeout_secs: 30,
            overrides: Default::default(),
        }
    }
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct CorsSettings {
    pub allow_origin: Option<AllowOrigin>,

    pub allow_methods: Option<AllowMethods>,

    pub allow_headers: Option<AllowHeaders>,

    pub allow_credentials: Option<bool>,

    #[serde_as(as = "Option<TryFromInto<DurationString>>")]
    pub max_age: Option<Duration>,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum AllowHeaders {
    #[serde(rename = "*")]
    Any,
    Exact(#[serde_as(as = "serde_with::DisplayFromStr")] http::HeaderName),
    List(#[serde_as(as = "Vec<serde_with::DisplayFromStr>")] Vec<http::HeaderName>),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum AllowOrigin {
    #[serde(rename = "*")]
    Any,
    Exact(String),
    List(Vec<String>),
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum AllowMethods {
    #[serde(rename = "*")]
    Any,
    Exact(#[serde_as(as = "serde_with::DisplayFromStr")] http::Method),
    List(#[serde_as(as = "Vec<serde_with::DisplayFromStr>")] Vec<http::Method>),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(default)]
pub struct LogSettings {
    #[serde(default = "default_log_level")]
    pub level: HashMap<String, LogLevel>,

    pub file: Option<LogFileSettings>,
}

impl Default for LogSettings {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            file: Default::default(),
        }
    }
}

fn default_log_level() -> HashMap<String, LogLevel> {
    let mut map = HashMap::new();
    map.insert("root".to_string(), LogLevel::Info);
    map
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct LogFileSettings {
    #[serde(default = "default_log_file_root")]
    pub root: PathBuf,

    pub prefix: String,

    #[serde(default = "defaults::bool::_true")]
    pub non_blocking: bool,

    #[serde(default = "Default::default")]
    pub rotate: LogRotate,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub enum LogRotate {
    #[serde(alias = "never")]
    Never,

    #[default]
    #[serde(alias = "daily")]
    Daily,

    #[serde(alias = "hourly")]
    Hourly,
}

fn default_log_file_root() -> PathBuf {
    ".".into()
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub enum LogLevel {
    #[serde(alias = "trace")]
    Trace,
    #[serde(alias = "debug")]
    Debug,
    #[default]
    #[serde(alias = "info")]
    Info,
    #[serde(alias = "warn")]
    Warn,
    #[serde(alias = "error")]
    Error,
}
impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => f.write_str("trace"),
            LogLevel::Debug => f.write_str("debug"),
            LogLevel::Info => f.write_str("info"),
            LogLevel::Warn => f.write_str("warn"),
            LogLevel::Error => f.write_str("error"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(tag = "source")]
pub enum UiSettings {
    /// embed in binary
    #[default]
    #[serde(alias = "embed")]
    Embed,
    /// local dir
    #[serde(alias = "local")]
    Local { path: PathBuf },
    /// no ui just api
    // #[default]
    #[serde(alias = "none")]
    None,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerSettings {
    /// server host default is `localhost`
    pub host: String,

    /// server address default is `127.0.0.1`
    pub addr: String,

    /// server port default is `80`
    pub port: u16,

    /// settings about tls default is `None`
    /// witch mean do not use tls
    pub tls: Option<TlsSettings>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TlsSettings {
    #[serde(default = "default_tls_port")]
    pub port: u16,
    pub cert: PathBuf,
    pub key: PathBuf,
    #[serde(default = "defaults::bool::_false")]
    pub redirect_to_https: bool,
}

fn default_tls_port() -> u16 {
    443
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            host: "localhost".to_owned(),
            addr: "127.0.0.1".to_owned(),
            port: 80,
            tls: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DatabaseSettings {
    pub database_url: String,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            database_url: "sqlite://wf_api.sqlite?mode=rwc".to_owned(),
        }
    }
}

/// configuration for jwt
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AuthenticateSettings {
    pub token: TokenSettings,
    pub pwd_hasher: PwdHasher,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TokenSettings {
    pub secret_key: String,
    #[serde_as(as = "TryFromInto<DurationString>")]
    pub refresh_valid: Duration,

    #[serde_as(as = "TryFromInto<DurationString>")]
    pub token_valid: Duration,
    pub issuer: String,
}

impl Default for TokenSettings {
    fn default() -> Self {
        let rand_string: String = rng()
            .sample_iter(&Alphanumeric)
            .take(18)
            .map(char::from)
            .collect();

        Self {
            issuer: "wf_api".to_owned(),
            secret_key: rand_string,
            // 30 days
            refresh_valid: Duration::from_hours(24 * 30),

            // 5 minutes
            token_valid: Duration::from_mins(5),
        }
    }
}

fn init_settings() -> Result<Settings, ConfigError> {
    let path = CONFIG_PATH.get_or_init(|| PathBuf::from("config.toml"));
    let defaults = Settings::default();

    let default_settings = Config::try_from(&defaults)?;
    let l_settings = Config::builder()
        .add_source(default_settings)
        .add_source(config::File::from(path.as_path()).required(false))
        .build()?;
    let mut result = l_settings.try_deserialize::<Settings>()?;

    if result.auth.token.secret_key == defaults.auth.token.secret_key {
        info!(
            "generated JWT secret  key: \"{}\" . If you want use fixed JWT secret key please setup \"auth.token.secret_key\"",
            defaults.auth.token.secret_key
        );
    }

    result.auth.pwd_hasher.try_init().unwrap();

    result
        .log
        .level
        .entry("root".to_owned())
        .or_insert(LogLevel::Info);

    Ok(result)
}

pub fn set_default_config_path<P: AsRef<Path>>(path: P) {
    CONFIG_PATH.set(PathBuf::from(path.as_ref())).unwrap();
}

static CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();
static SETTINGS: OnceLock<Settings> = OnceLock::new();

pub fn get() -> &'static Settings {
    SETTINGS.get_or_init(|| init_settings().unwrap())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "name", content = "options")]
pub enum PwdHasher {
    #[serde(alias = "bcrypt")]
    Bcrypt(Option<BcryptOptions>),

    #[serde(alias = "argon2")]
    Argon2(Option<Argon2Options>),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct BcryptOptions {
    pub cost: u32,
}
impl Default for BcryptOptions {
    fn default() -> Self {
        Self { cost: 10 }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Copy)]
pub struct Argon2Options {
    algorithm: Argon2Algorithm,
    version: Argon2Version,
    params: Argon2Params,
}

impl Default for PwdHasher {
    fn default() -> Self {
        PwdHasher::Argon2(None)
    }
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, Copy)]
pub enum Argon2Algorithm {
    Argon2d,
    Argon2i,
    #[default]
    Argon2id,
}

impl From<Argon2Algorithm> for argon2::Algorithm {
    fn from(value: Argon2Algorithm) -> Self {
        match value {
            Argon2Algorithm::Argon2d => argon2::Algorithm::Argon2d,
            Argon2Algorithm::Argon2i => argon2::Algorithm::Argon2i,
            Argon2Algorithm::Argon2id => argon2::Algorithm::Argon2id,
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, Copy)]
pub enum Argon2Version {
    V0x10,
    #[default]
    V0x13,
}

impl From<Argon2Version> for argon2::Version {
    fn from(value: Argon2Version) -> Self {
        match value {
            Argon2Version::V0x10 => argon2::Version::V0x10,
            Argon2Version::V0x13 => argon2::Version::V0x13,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Argon2Params {
    m_cost: u32,
    t_cost: u32,
    p_cost: u32,
    output_len: Option<usize>,
}

impl TryInto<argon2::Params> for Argon2Params {
    type Error = PwdHasherError;

    fn try_into(self) -> Result<argon2::Params, Self::Error> {
        argon2::Params::new(self.m_cost, self.t_cost, self.p_cost, self.output_len)
            .map_err(PwdHasherError::ParseArgon2Params)
    }
}

impl Default for Argon2Params {
    fn default() -> Self {
        Self {
            m_cost: argon2::Params::DEFAULT_M_COST,
            t_cost: argon2::Params::DEFAULT_T_COST,
            p_cost: argon2::Params::DEFAULT_P_COST,
            output_len: None,
        }
    }
}

// pub struct PwdHasher {}
#[derive(Debug, Error)]
pub enum PwdHasherError {
    #[error("parse argon2 params error. {0}")]
    ParseArgon2Params(argon2::Error),

    #[error(transparent)]
    BcryptHash(bcrypt::BcryptError),

    #[error("password hash error, {0}")]
    Argon2Hash(argon2::password_hash::Error),

    #[error(transparent)]
    BcryptVerify(bcrypt::BcryptError),

    #[error(transparent)]
    Argon2Verify(argon2::password_hash::Error),

    #[error("unknown hash format")]
    UnknownHashFormat,
}

impl PwdHasher {
    pub fn try_init(&self) -> Result<(), PwdHasherError> {
        match self {
            PwdHasher::Bcrypt(..) => Ok(()),
            PwdHasher::Argon2(opt) => {
                if let Some(opt) = opt {
                    let _: argon2::Params = opt.params.try_into()?;
                }
                Ok(())
            }
        }
    }

    pub fn hash<P: AsRef<[u8]>>(&self, password: P) -> Result<String, PwdHasherError> {
        match self {
            PwdHasher::Bcrypt(opt) => {
                let opt = opt.unwrap_or(BcryptOptions::default());
                Ok(bcrypt::hash(password, opt.cost).map_err(PwdHasherError::BcryptHash)?)
            }
            PwdHasher::Argon2(opt) => {
                let hasher = if let Some(opt) = opt {
                    Argon2::new(
                        opt.algorithm.into(),
                        opt.version.into(),
                        opt.params.try_into()?,
                    )
                } else {
                    Argon2::default()
                };
                let salt = SaltString::generate(&mut OsRng);
                Ok(hasher
                    .hash_password(password.as_ref(), &salt)
                    .map_err(PwdHasherError::Argon2Hash)?
                    .to_string())
            }
        }
    }

    pub fn verify<P: AsRef<[u8]>>(&self, password: P, hash: &str) -> Result<bool, PwdHasherError> {
        // hash.starts_with("pat")
        match hash {
            hash if hash.starts_with("$2b") => {
                Ok(bcrypt::verify(password, hash).map_err(PwdHasherError::BcryptVerify)?)
            }
            hash if hash.starts_with("$argon2") => {
                let parsed_hash = PasswordHash::new(hash).map_err(PwdHasherError::Argon2Verify)?;
                Ok(Argon2::default()
                    .verify_password(password.as_ref(), &parsed_hash)
                    .is_ok())
            }
            hash => {
                error!("unknown pwd hash format! \"{hash}\"");
                Err(PwdHasherError::UnknownHashFormat)
            }
        }
    }
}
