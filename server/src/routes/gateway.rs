use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use model::gateway::GatewayEvent;
use std::sync::Arc;
use tokio::sync::Mutex;
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
    let (sender, mut receiver) = stream.split();

    let sender = Arc::new(Mutex::new(sender));

    {
        let mut sender_lock = sender.lock().await;
        sender_lock
            .send(Message::Text(
                serde_json::to_string(&GatewayEvent::Hello).unwrap(),
            ))
            .await
            .unwrap();
    }
    let mut bot_id = None;
    tokio::select! {
        Some(Ok(Message::Text(msg))) = &mut receiver.next() => {
            let event: GatewayEvent = serde_json::from_str(&msg).unwrap();
            if let GatewayEvent::Identify(identify) = event {
                bot_id = get_token(&state.pool, identify.token).await.unwrap();
            }
        },
        _ = tokio::time::sleep(Duration::from_secs(20)) => {}
    };
    let bot_id = bot_id.unwrap();
    {
        let mut sender_lock = sender.lock().await;
        sender_lock
            .send(Message::Text(
                serde_json::to_string(&GatewayEvent::Ready("Ok".to_string())).unwrap(),
            ))
            .await
            .unwrap();
    }
    let mut rx = state.tx.subscribe();

    let announce_sender = Arc::clone(&sender);
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let mut sender_lock = announce_sender.lock().await;
            if sender_lock.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
    tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Ping(data) => {
                    let mut sender_lock = sender.lock().await;
                    sender_lock.send(Message::Pong(data)).await.unwrap()
                }
                _ => {}
            }
        }
    });
}
