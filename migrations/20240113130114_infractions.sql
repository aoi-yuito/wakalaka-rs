CREATE TABLE IF NOT EXISTS infractions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type VARCHAR,
    user_id BIGINT NOT NULL,
    moderator_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    reason VARCHAR(120) NOT NULL,
    created_at TIMESTAMP,
    FOREIGN KEY (moderator_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
PRAGMA foreign_keys = ON;