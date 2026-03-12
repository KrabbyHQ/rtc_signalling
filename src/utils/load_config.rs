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

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    fn valid_app_section() -> AppSection {
        AppSection {
            name: "Test App".to_string(),
            environment: Some("development".to_string()),
        }
    }

    fn valid_server_section() -> ServerSection {
        ServerSection {
            host: "127.0.0.1".to_string(),
            port: 8080,
            request_timeout_secs: 30,
        }
    }

    #[test]
    fn test_validate_valid_config() {
        let config = AppConfig {
            app: valid_app_section(),
            client_integrations: ClientIntegrationsSection {
                allow_logging_middleware: true,
                allow_request_timeout_middleware: true,
            },
            observability: ObservabilitySection {
                enable_tracing: true,
                enable_metrics: true,
            },
            server: Some(valid_server_section()),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_missing_app_name() {
        let mut config = AppConfig {
            app: valid_app_section(),
            client_integrations: ClientIntegrationsSection {
                allow_logging_middleware: false,
                allow_request_timeout_middleware: false,
            },
            observability: ObservabilitySection {
                enable_tracing: false,
                enable_metrics: false,
            },
            server: Some(valid_server_section()),
        };
        config.app.name = "".to_string();

        let result = config.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "app.name cannot be empty");
    }

    #[test]
    fn test_validate_invalid_port() {
        let config = AppConfig {
            app: valid_app_section(),
            client_integrations: ClientIntegrationsSection {
                allow_logging_middleware: false,
                allow_request_timeout_middleware: false,
            },
            observability: ObservabilitySection {
                enable_tracing: false,
                enable_metrics: false,
            },
            server: Some(ServerSection {
                host: "127.0.0.1".to_string(),
                port: 0,
                request_timeout_secs: 60,
            }),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "server.port cannot be 0");
    }

    #[test]
    fn test_validate_missing_server_section() {
        let config = AppConfig {
            app: valid_app_section(),
            client_integrations: ClientIntegrationsSection {
                allow_logging_middleware: false,
                allow_request_timeout_middleware: false,
            },
            observability: ObservabilitySection {
                enable_tracing: false,
                enable_metrics: false,
            },
            server: None,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "server section is missing");
    }

    #[test]
    #[serial]
    fn test_load_config_functional() {
        // Ensure APP__ENV is set for the test
        // SAFETY: This is a test environment and we are setting environment variables
        // for the duration of the test. In a multi-threaded test environment,
        // this could still be racey, but it's required for the test logic.
        unsafe { std::env::set_var("APP__ENV", "development") };

        let config_result = load_config();
        assert!(
            config_result.is_ok(),
            "Failed to load config: {:?}",
            config_result.err()
        );

        let config = config_result.unwrap();
        assert_eq!(config.app.name, "rtc-signalling");
        assert_eq!(config.app.environment.as_deref(), Some("development"));
    }

    #[test]
    #[serial]
    fn test_load_config_env_overrides() {
        // SAFETY: Setting environment variables for test overrides.
        unsafe {
            std::env::set_var("APP__ENV", "development");
            std::env::set_var("APP__SERVER__PORT", "1234");
            std::env::set_var("APP__APP__NAME", "env-override-test");
        }

        let config = load_config().expect("Failed to load config");

        assert_eq!(config.server.unwrap().port, 1234);
        assert_eq!(config.app.name, "env-override-test");

        // Clean up
        // SAFETY: Removing environment variables used in the test.
        unsafe {
            std::env::remove_var("APP__SERVER__PORT");
            std::env::remove_var("APP__APP__NAME");
        }
    }
}
