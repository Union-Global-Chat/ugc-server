-- Add migration script here
CREATE TABLE token (
    bot_id BIGINT NOT NULL PRIMARY KEY,
    token TEXT NOT NULL
)