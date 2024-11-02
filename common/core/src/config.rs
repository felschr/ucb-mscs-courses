//! Common structs & traits for use with `config` crate.

use config::{Config, ConfigError, Environment};
use sentry::types::Dsn;
use serde::{de::DeserializeOwned, Deserialize};

/// Provides default implementation for creating new service configuration.
pub trait NewConfig: Sized + DeserializeOwned {
    fn new() -> Result<Self, ConfigError> {
        let c = Config::builder()
            // @TODO SENTRY_DSN fails with "_" as separator, thus we use "__"
            .add_source(Environment::default().try_parsing(false).separator("__"))
            .build()?;
        c.try_deserialize()
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Sentry {
    pub enable: bool,
    #[serde(default)]
    pub debug: bool,
    pub dsn: Option<Dsn>,
    pub release: String,
    pub environment: String,
}

// A common configuration that most services will use.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct CommonConfig {
    pub sentry: Sentry,
}

impl NewConfig for CommonConfig {}
