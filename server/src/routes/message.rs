use model::data::*;
use crate::state::AppState;
use axum::{extract::State, Json};
use db::message::*;

pub async fn create_message(Json(message): Json<Message>) -> anyhow::Result<()> {
    Ok(())
}