CREATE TABLE IF NOT EXISTS restricted_users (
    user_id BIGINT PRIMARY KEY,
    reason VARCHAR(120) NOT NULL
);