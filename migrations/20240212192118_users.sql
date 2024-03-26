CREATE TABLE IF NOT EXISTS users (
    user_id BIGINT,
    violations INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id)
);