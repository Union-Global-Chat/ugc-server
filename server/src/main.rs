use tokio::net::TcpListener;
use axum::{routing::get, Router};

use std::env;

mod state;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::init();

    let state = state::AppState::connect(&env::var("DATABASE_URL")?).await?;
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
