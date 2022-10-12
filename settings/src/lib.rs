use std::{env, fmt};

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::ffi::OsStr;
use std::path::Path;

const CONFIG_FILE_PATH: &str = "./settings/src/Default.json";
const CONFIG_FILE_PREFIX: &str = "./settings/src/";

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Development,
    Testing,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "Development"),
            ENV::Testing => write!(f, "Testing"),
            ENV::Production => write!(f, "Production"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Testing" => ENV::Testing,
            "Production" => ENV::Production,
            _ => ENV::Development,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub host: String,
    pub login: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
    pub env: ENV,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Development".into());
        let bulder = Config::builder();

        let settigs = bulder
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .add_source(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)).required(false))
            .add_source(Environment::with_prefix("chat").separator("__"))
            .set_override("env", env.clone())?
            .build()?;

        settigs.try_deserialize()
    }
}
