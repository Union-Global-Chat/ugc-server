use serde::{Deserialize, Serialize};

use crate::data::Data;

#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum GatewayEvent {
    Hello,
    IdentifyEvent(IdentifyEvent),
    SendDataEvent(SendDataEvent),
}

#[derive(Serialize, Deserialize)]
pub struct IdentifyEvent {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendDataEvent {
    pub from_bot_id: i64,
    pub data: Data,
}