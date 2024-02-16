CREATE TABLE IF NOT EXISTS restricted_users (
    user_id BIGINT,
    reason VARCHAR(120) NOT NULL,
    PRIMARY KEY (user_id)
);