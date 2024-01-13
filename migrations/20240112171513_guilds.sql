CREATE TABLE IF NOT EXISTS guilds (
    id BIGINT NOT NULL,
    owner_id BIGINT NOT NULL,
    preferred_locale VARCHAR NOT NULL DEFAULT 'en-US',
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (id)
)