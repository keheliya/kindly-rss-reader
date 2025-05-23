mod config;
mod controllers;
mod middlewares;
mod models;
pub mod providers;
mod repositories;
mod router;
pub mod services;
mod state;
mod tracing;
mod view_models;

use crate::repositories::init_database;
use crate::tracing::init_tracing;
use config::Config;
use state::State;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Init tracing
    init_tracing();

    // Configuration
    let config = Arc::new(Config::load());

    // Init database
    let connection = init_database(&config);

    // Create state
    let state = State::new(connection, config.clone()).await;

    // Initialize App
    let app = router::build(state, &config);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.ip, config.port))
        .await
        .expect("unable to bind tcp listener");

    config.print_information();
    axum::serve(listener, app).await.unwrap();
}
