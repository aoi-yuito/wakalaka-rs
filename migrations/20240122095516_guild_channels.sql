CREATE TABLE IF NOT EXISTS guild_channels (
    channel_id BIGINT PRIMARY KEY,
    type VARCHAR NOT NULL,
    guild_id BIGINT NOT NULL,
    nsfw BOOLEAN NOT NULL DEFAULT FALSE,
    rate_limit_per_user INTEGER NOT NULL,
    restrict BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (guild_id) REFERENCES guilds(guild_id) ON DELETE CASCADE
);