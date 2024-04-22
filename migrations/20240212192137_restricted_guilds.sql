CREATE TABLE IF NOT EXISTS restricted_guilds (
    guild_id BIGINT,
    reason VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    PRIMARY KEY (guild_id)
);