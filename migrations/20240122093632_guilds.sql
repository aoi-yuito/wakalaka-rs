CREATE TABLE IF NOT EXISTS guilds (
    guild_id BIGINT PRIMARY KEY,
    owner_id BIGINT NOT NULL,
    logs_channel_id BIGINT,
    suggestions_channel_id BIGINT,
    usage_channel_id BIGINT,
    welcome_channel_id BIGINT,
    FOREIGN KEY (owner_id) REFERENCES users(user_id)
);
PRAGMA foreign_keys = ON;