//! # RTC Signalling Server Library

use crate::utils::load_config::AppConfig;
use axum::Router;
use std::sync::Arc;

pub mod utils;

/// Global application state.
#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Arc<AppConfig>,
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            axum::routing::get(|| async { "RTC Signalling Server" }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::load_config::{
        AppConfig, AppSection, ClientIntegrationsSection, ObservabilitySection, ServerSection,
    };
    use axum_test::TestServer;

    fn mock_config() -> AppConfig {
        AppConfig {
            app: AppSection {
                name: "test".to_string(),
                environment: Some("test".to_string()),
            },
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
                port: 8080,
                request_timeout_secs: 30,
            }),
        }
    }

    #[tokio::test]
    async fn test_root_route() {
        let state = AppState {
            config: Arc::new(mock_config()),
        };
        let app = create_app(state);
        let server = TestServer::new(app).unwrap();

        let response = server.get("/").await;
        response.assert_status_ok();
        response.assert_text("RTC Signalling Server");
    }
}
