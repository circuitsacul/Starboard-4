CREATE TABLE filter_groups (
    id SERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    name TEXT NOT NULL,

    UNIQUE (guild_id, name),
    FOREIGN KEY (guild_id) REFERENCES guilds (guild_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE filters (
    id SERIAL PRIMARY KEY,

    position SMALLINT NOT NULL,
    filter_group_id INT NOT NULL,

    instant_pass BOOLEAN NOT NULL DEFAULT false,
    instant_fail BOOLEAN NOT NULL DEFAULT true,

    -- user context
    user_has_all_of BIGINT[],
    user_missing_all_of BIGINT[],
    user_has_some_of BIGINT[],
    user_missing_some_of BIGINT[],
    user_is_bot BOOLEAN,

    -- message context
    not_in_channel BIGINT[],
    in_channel BIGINT[],
    not_in_channel_or_sub_channels BIGINT[],
    in_channel_or_sub_channels BIGINT[],
    min_length INT,
    max_length INT,
    min_attachments SMALLINT,
    max_attachments SMALLINT,
    matches TEXT,
    not_matches TEXT,

    -- vote context
    voter_has_all_of BIGINT[],
    voter_missing_all_of BIGINT[],
    voter_has_some_of BIGINT[],
    voter_missing_some_of BIGINT[],
    older_than BIGINT,
    newer_than BIGINT,

    FOREIGN KEY (filter_group_id) REFERENCES filter_groups (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE starboard_filter_groups (
    filter_group_id INT NOT NULL,
    starboard_id INT NOT NULL,

    PRIMARY KEY (filter_group_id, starboard_id),

    FOREIGN KEY (filter_group_id) REFERENCES filter_groups (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (starboard_id) REFERENCES starboards (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE autostar_channel_filter_groups (
    filter_group_id INT NOT NULL,
    autostar_channel_id INT NOT NULL,

    PRIMARY KEY (filter_group_id, autostar_channel_id),

    FOREIGN KEY (filter_group_id) REFERENCES filter_groups (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (autostar_channel_id) REFERENCES autostar_channels (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

-- migrate settings from starboards
DO $$
DECLARE
    starboard record;
    filter_group record;
BEGIN
    FOR starboard IN SELECT * FROM starboards WHERE
        matches IS NOT NULL OR
        not_matches IS NOT NULL
    LOOP
        -- create the filter group
        INSERT INTO filter_groups (guild_id, name) VALUES
            (starboard.guild_id, starboard.name);
        -- get the created filter group
        SELECT * FROM filter_groups INTO filter_group WHERE
            guild_id=starboard.guild_id AND
            name=starboard.name;
        -- create the filter
        INSERT INTO filters (position, filter_group_id, matches, not_matches) VALUES
            (1, filter_group.id, starboard.matches, starboard.not_matches);
    END LOOP;
END $$;
