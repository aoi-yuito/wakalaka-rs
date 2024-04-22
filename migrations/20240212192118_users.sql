CREATE TABLE IF NOT EXISTS users (
    user_id BIGINT,
    created_at TIMESTAMP NOT NULL,
    warnings INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id)
);