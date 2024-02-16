CREATE TABLE IF NOT EXISTS violations (
    uuid UUID,
    kind VARCHAR,
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    moderator_id BIGINT NOT NULL,
    reason VARCHAR(120),
    created_at TIMESTAMP NOT NULL,
    PRIMARY KEY (uuid, kind),
    FOREIGN KEY (guild_id) REFERENCES guilds (guild_id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (user_id),
    FOREIGN KEY (moderator_id) REFERENCES users (user_id)
);