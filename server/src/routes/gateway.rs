use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use model::gateway::GatewayEvent;
use tokio::time::Duration;

use crate::{error::APIResult, state::AppState};
use db::token::get_token;

pub async fn gateway_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> APIResult<impl IntoResponse> {
    Ok(ws.on_upgrade(|socket| websocket(socket, state)))
}

async fn websocket(stream: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = stream.split();

    sender
        .send(Message::Text(
            serde_json::to_string(&GatewayEvent::Hello).unwrap(),
        ))
        .await
        .unwrap();
    let mut bot_id = None;
    tokio::select! {
        Some(Ok(Message::Text(msg))) = &mut receiver.next() => {
            let event: GatewayEvent = serde_json::from_str(&msg).unwrap();
            if let GatewayEvent::IdentifyEvent(identify) = event {
                bot_id = get_token(&state.pool, identify.token).await.unwrap();
            }
        },
        _ = tokio::time::sleep(Duration::from_secs(20)) => {}
    };
    let bot_id = bot_id.unwrap();
    let mut rx = state.tx.subscribe();

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
}
