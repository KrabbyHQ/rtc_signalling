//! # RTC Signalling Server Binary

use rtc_signalling::utils::load_config::load_config;
use rtc_signalling::utils::load_env::load_env;
use rtc_signalling::{AppState, create_app};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error};
use tracing_subscriber::fmt::time::SystemTime;

fn initialize_logging() {
    tracing_subscriber::fmt()
        .json()
        .with_timer(SystemTime)
        .with_level(true)
        .init();
}

#[tokio::main]
async fn main() {
    load_env();
    initialize_logging();

    let app_config = match load_config() {
        Ok(config) => {
            if let Err(e) = config.validate() {
                error!("SERVER START-UP ERROR: FAILED TO VALIDATE CONFIG, {}", e);
                std::process::exit(1);
            }
            config
        }
        Err(e) => {
            error!("SERVER START-UP ERROR: FAILED TO LOAD CONFIG, {}", e);
            std::process::exit(1);
        }
    };

    let state = AppState {
        config: Arc::new(app_config),
    };

    let host = state.config.server.as_ref().map(|s| s.host.as_str()).unwrap_or("127.0.0.1");
    let port = state.config.server.as_ref().map(|s| s.port).unwrap_or(8081);

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid server address");

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");

    print!(
        "
        .................................................
        Environment: {}
        Status: Online
        .................................................

        Server running on http://{}
        ",
        state.config.app.environment.as_deref().unwrap_or("unknown"),
        addr
    );

    let app = create_app(state);
    axum::serve(listener, app).await.expect("Server error");
}
