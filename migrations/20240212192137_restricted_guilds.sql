CREATE TABLE IF NOT EXISTS restricted_guilds (
    guild_id BIGINT,
    reason VARCHAR(255) NOT NULL,
    PRIMARY KEY (guild_id)
);
CREATE TRIGGER IF NOT EXISTS restricted_guilds_delete
AFTER DELETE ON restricted_guilds BEGIN
DELETE FROM restricted_users
WHERE guild_id = OLD.guild_id;
END;