use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Data {
    pub channel: Channel,
    pub author: Author,
    pub guild: Guild,
    pub message: Message,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Channel {
    pub name: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Author {
    pub username: String,
    pub discriminator: String,
    pub id: String,
    #[serde(rename = "avatarURL")]
    pub avatar_url: String,
    pub bot: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Guild {
    pub name: String,
    pub id: String,
    #[serde(rename = "iconURL")]
    pub icon_url: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Message {
    pub content: String,
    pub id: String,
    pub clean_content: String,
    pub reference: Option<MessageReference>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MessageReference {
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub message_id: Option<String>,
}
