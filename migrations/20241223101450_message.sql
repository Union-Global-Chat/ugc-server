-- Add migration script here
CREATE TABLE message (
    channel_name TEXT NOT NULL,
    channel_id TEXT NOT NULL,
    author_name TEXT NOT NULL,
    author_discriminator TEXT NOT NULL,
    author_id TEXT NOT NULL,
    author_avatar_url TEXT NOT NULL,
    author_bot BOOLEAN NOT NULL,
    guild_name TEXT NOT NULL,
    guild_id TEXT NOT NULL,
    guild_icon_url TEXT NOT NULL,
    message_content TEXT NOT NULL,
    message_id VARCHAR(255) NOT NULL PRIMARY KEY,
    message_clean_content TEXT NOT NULL,
    message_reference_channel_id TEXT,
    message_reference_guild_id TEXT,
    message_reference_message_id TEXT
);

CREATE TABLE message_attachments (
    message_id VARCHAR(255) NOT NULL,
    url TEXT NOT NULL,
    height INTEGER NOT NULL,
    width INTEGER NOT NULL,
    content_type TEXT NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY base_massage_id(message_id) REFERENCES message(message_id) ON DELETE CASCADE
)
