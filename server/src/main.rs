use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

use std::env;

mod error;
mod routes;
mod state;
mod token;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::init();

    let state = state::AppState::connect(&env::var("DATABASE_URL")?).await?;
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/messages", post(routes::message::create_message))
        .route("/gateway", get(routes::gateway::gateway_handler))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
