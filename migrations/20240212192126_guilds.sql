CREATE TABLE IF NOT EXISTS guilds (
    guild_id BIGINT,
    owner_id BIGINT REFERENCES users (user_id) ON DELETE CASCADE,
    PRIMARY KEY (guild_id, owner_id)
);