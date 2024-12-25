use crate::error::APIResult;
use crate::state::AppState;
use crate::token::Token;
use axum::{extract::State, Json};
use db::message::add_message;
use model::data::*;
use model::gateway::{GatewayEvent, SendDataEvent};

pub async fn create_message(
    State(state): State<AppState>,
    token: Token,
    Json(data): Json<Data>,
) -> APIResult<()> {
    add_message(&state.pool, data.clone()).await?;
    state
        .tx
        .send(serde_json::to_string(&GatewayEvent::SendData(Box::new(
            SendDataEvent {
                from_bot_id: token.bot_id,
                data,
            },
        )))?)?;
    Ok(())
}
