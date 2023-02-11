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

ALTER TABLE starboards ADD COLUMN filters INT[] NOT NULL DEFAULT '{}';
ALTER TABLE autostar_channels ADD COLUMN filters INT[] NOT NULL DEFAULT '{}';
