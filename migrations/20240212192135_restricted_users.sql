CREATE TABLE IF NOT EXISTS restricted_users (
    user_id BIGINT,
    reason VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    PRIMARY KEY (user_id)
);