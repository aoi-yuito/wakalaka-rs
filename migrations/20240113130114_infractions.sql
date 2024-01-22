CREATE TABLE IF NOT EXISTS infractions (
    uuid VARCHAR(32) PRIMARY KEY,
    type VARCHAR,
    user_id BIGINT NOT NULL,
    moderator_id BIGINT NOT NULL,
    reason VARCHAR(120) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    guild_id BIGINT NOT NULL,
    FOREIGN KEY (guild_id) REFERENCES guilds(guild_id) ON DELETE CASCADE,
    FOREIGN KEY (moderator_id) REFERENCES users(user_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);