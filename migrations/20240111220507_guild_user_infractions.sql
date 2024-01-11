CREATE TABLE IF NOT EXISTS Infractions (
    id BIGINT NOT NULL,
    userId BIGINT NOT NULL,
    moderatorId BIGINT NOT NULL,
    guildId BIGINT NOT NULL,
    infractionType INTEGER,
    createdAt TIMESTAMP,
    deletedAt TIMESTAMP,
    expiresIn TIMESTAMP,
    reason TEXT,
    FOREIGN KEY (guildId) REFERENCES Guilds(id),
    FOREIGN KEY (moderatorId) REFERENCES Users(id),
    FOREIGN KEY (userId) REFERENCES Users(id),
    PRIMARY KEY (id)
);