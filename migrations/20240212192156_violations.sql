CREATE TABLE IF NOT EXISTS violations (
    uuid VARCHAR(32) PRIMARY KEY,
    kind TEXT,
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    moderator_id BIGINT NOT NULL,
    reason VARCHAR(120),
    FOREIGN KEY (guild_id) REFERENCES guilds (guild_id),
    FOREIGN KEY (user_id) REFERENCES users (user_id),
    FOREIGN KEY (moderator_id) REFERENCES users (user_id)
);