use serde::{Deserialize, Serialize};

use crate::data::Data;

#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum GatewayEvent {
    Hello,
    Identify(IdentifyEvent),
    Ready(String),
    SendData(Box<SendDataEvent>),
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
