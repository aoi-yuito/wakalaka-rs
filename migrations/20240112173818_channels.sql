CREATE TABLE IF NOT EXISTS channels (
    id BIGINT NOT NULL,
    type INTEGER NOT NULL,
    guild_id BIGINT NOT NULL,
    rate_limit_per_user INTEGER NOT NULL,
    FOREIGN KEY (guild_id) REFERENCES guilds(id),
    PRIMARY KEY (id)
)