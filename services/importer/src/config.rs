use serde::{self, Deserialize};
use ucb_mscs_courses_core::config::{NewConfig, Sentry};

#[derive(Deserialize)]
pub struct AppConfig {
    pub sentry: Sentry,
}

impl NewConfig for AppConfig {}

pub fn init() -> AppConfig {
    AppConfig::new().expect("Service configuration could not be loaded.")
}
