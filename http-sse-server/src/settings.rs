use std::env;
use config::{ConfigError, Config, File, Environment};
use serde::{Deserialize};
use crate::app_ops::RuntimeInfo;

const APP_NAME: &str="http-sse-server";
const APP_ENV_PREFIX: &str="SSE_";

#[derive(Debug, Deserialize, Clone)]
pub struct AppSettings {
    pub app_name: String,
    pub settings: Settings,
    pub runtime_info : RuntimeInfo,
}

impl AppSettings {
    pub fn load() -> AppSettings {
        AppSettings {
            app_name: String::from(APP_NAME),
            settings: Settings::new().expect("fail to load settings"),
            runtime_info: RuntimeInfo::new()
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub environment: String,
    pub debug: bool,
    pub port: u16,
    pub url_prefix: String,

    pub log_level: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        // Start off by merging in the "default.toml" configuration file
        s.merge(File::with_name("config/default.toml"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env_name = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env_name)).required(false))?;
        s.set("environment", env_name)?;
        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix(APP_ENV_PREFIX))?;
        // Now that we're done, let's access our configuration

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}