CREATE TABLE IF NOT EXISTS Channels (
    id BIGINT NOT NULL,
    parentId BIGINT NOT NULL,
    guildId BIGINT NOT NULL,
    position INTEGER NOT NULL,
    FOREIGN KEY (guildId) REFERENCES Guilds(id),
    FOREIGN KEY (parentId) REFERENCES Channels(id),
    PRIMARY KEY (id)
);