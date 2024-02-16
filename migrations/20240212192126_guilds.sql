CREATE TABLE IF NOT EXISTS guilds (
    guild_id BIGINT,
    owner_id BIGINT NOT NULL,
    PRIMARY KEY (guild_id),
    FOREIGN KEY (owner_id) REFERENCES users (user_id) ON DELETE CASCADE
);