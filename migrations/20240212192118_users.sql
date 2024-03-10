CREATE TABLE IF NOT EXISTS users (
    user_id BIGINT,
    lastfm_name VARCHAR(15),
    lastfm_key VARCHAR(32),
    violations INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id)
);