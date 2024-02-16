CREATE TABLE IF NOT EXISTS violations (
    uuid VARCHAR(32),
    kind VARCHAR,
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL REFERENCES users (user_id) ON DELETE CASCADE,
    moderator_id BIGINT NOT NULL REFERENCES users (user_id) ON DELETE CASCADE,
    reason VARCHAR(255),
    created_at TIMESTAMP NOT NULL,
    PRIMARY KEY (uuid, kind),
    FOREIGN KEY (guild_id) REFERENCES guilds (guild_id) ON DELETE CASCADE
);