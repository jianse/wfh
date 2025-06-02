use std::sync::OnceLock;

use config::{Config, ConfigError};
use rand::distr::Alphanumeric;
use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub(crate) struct Settings {
    /// server settings
    pub(crate) server: ServerSettings,

    /// database settings
    pub(crate) db: DatabaseSettings,

    /// auth settings
    pub(crate) auth: AuthenticateSettings,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct ServerSettings {
    pub(crate) addr: String,
    pub(crate) port: u16,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            addr: "localhost".to_owned(),
            port: 3000,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct DatabaseSettings {
    pub(crate) database_url: String,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            database_url: "sqlite://wf_api.sqlite?mode=rwc".to_owned(),
        }
    }
}

/// configuration for jwt
#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct AuthenticateSettings {
    pub(crate) jwt_secret_key: String,
    pub(crate) jwt_valid_time_secs: u64,
}

impl Default for AuthenticateSettings {
    fn default() -> Self {
        let rand_string: String = rng()
            .sample_iter(&Alphanumeric)
            .take(18)
            .map(char::from)
            .collect();

        Self {
            jwt_secret_key: rand_string,
            jwt_valid_time_secs: 8 * 60 * 60,
        }
    }
}

fn init_settings() -> Result<Settings, ConfigError> {
    let defaults = Settings::default();

    let cd = Config::try_from(&defaults)?;
    let l_settings = Config::builder()
        .add_source(cd)
        .add_source(config::File::with_name("config.toml").required(false))
        .build()?;
    let result = l_settings.try_deserialize::<Settings>()?;

    if result.auth.jwt_secret_key == defaults.auth.jwt_secret_key {
        info!(
            "generated JWT secret  key: \"{}\" . If you want use fixed JWT secret key please setup \"auth.jwt_secret_key\"",
            defaults.auth.jwt_secret_key
        );
    }
    Ok(result)
}

static SETTINGS: OnceLock<Settings> = OnceLock::new();

pub(crate) fn get() -> &'static Settings {
    SETTINGS.get_or_init(|| init_settings().unwrap())
}
