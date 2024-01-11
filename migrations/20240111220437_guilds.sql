CREATE TABLE IF NOT EXISTS Guilds (
    id BIGINT NOT NULL,
    ownerId BIGINT NOT NULL,
    preferredLocale TEXT,
    isUnavailable BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (ownerId) REFERENCES Users (id),
    PRIMARY KEY (id)
);