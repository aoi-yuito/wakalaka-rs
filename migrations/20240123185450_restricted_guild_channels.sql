CREATE TABLE IF NOT EXISTS restricted_guild_channels (
    channel_id BIGINT PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    FOREIGN KEY (guild_id) REFERENCES guilds(guild_id) ON DELETE CASCADE
);
PRAGMA foreign_keys = ON;