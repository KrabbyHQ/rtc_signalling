//! # Configuration Management
//!
//! Handles loading and validating application configuration from multiple sources.

use anyhow::{Context, Result};
use config::{Config, Environment, File};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct AppSection {
    pub name: String,
    pub environment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ClientIntegrationsSection {
    #[serde(default)]
    pub allow_logging_middleware: bool,
    #[serde(default)]
    pub allow_request_timeout_middleware: bool,
}

#[derive(Debug, Deserialize)]
pub struct ObservabilitySection {
    pub enable_tracing: bool,
    pub enable_metrics: bool,
}

#[derive(Debug, Deserialize)]
pub struct ServerSection {
    pub host: String,
    pub port: u16,
    pub request_timeout_secs: u64,
}

/// Root configuration structure.
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app: AppSection,
    pub client_integrations: ClientIntegrationsSection,
    pub observability: ObservabilitySection,
    pub server: Option<ServerSection>,
}

pub fn load_config() -> Result<AppConfig> {
    let env = std::env::var("APP__ENV").unwrap_or_else(|_| "development".into());

    let builder = Config::builder()
        .add_source(File::with_name("config/base").required(true))
        .add_source(File::with_name(&format!("config/{}", env)).required(false))
        .add_source(File::with_name("config/local").required(false))
        .add_source(
            Environment::default()
                .separator("__")
                .prefix("APP")
                .try_parsing(true),
        );

    builder
        .build()
        .context("Failed to build config")?
        .try_deserialize()
        .context("Invalid config shape")
}

#[derive(Debug)]
pub enum ConfigError {
    MissingAppName,
    InvalidServerPort,
    MissingServerSection,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingAppName => write!(f, "app.name cannot be empty"),
            ConfigError::InvalidServerPort => write!(f, "server.port cannot be 0"),
            ConfigError::MissingServerSection => write!(f, "server section is missing"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl AppConfig {
    pub fn validate(&self) -> std::result::Result<(), ConfigError> {
        if self.app.name.trim().is_empty() {
            return Err(ConfigError::MissingAppName);
        }

        let server = self
            .server
            .as_ref()
            .ok_or(ConfigError::MissingServerSection)?;
        if server.port == 0 {
            return Err(ConfigError::InvalidServerPort);
        }

        Ok(())
    }
}
