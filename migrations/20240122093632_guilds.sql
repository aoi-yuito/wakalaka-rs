CREATE TABLE IF NOT EXISTS guilds (
    guild_id BIGINT PRIMARY KEY,
    owner_id BIGINT NOT NULL,
    usage_channel_id BIGINT,
    suggestions_channel_id BIGINT,
    logs_channel_id BIGINT,
    FOREIGN KEY (owner_id) REFERENCES users(user_id)
);