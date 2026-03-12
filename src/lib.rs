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
        .route("/", axum::routing::get(|| async { "RTC Signalling Server" }))
        .with_state(state)
}
