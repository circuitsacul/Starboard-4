CREATE TABLE guilds (
    guild_id BIGINT NOT NULL,
    premium_end TIMESTAMPTZ,

    PRIMARY KEY (guild_id)
);

CREATE TABLE users (
    user_id BIGINT NOT NULL,
    is_bot BOOLEAN NOT NULL,
    credits INTEGER NOT NULL DEFAULT 0,
    -- total cents donated excluding patreon
    donated_cents BIGINT NOT NULL DEFAULT 0,
    patreon_status SMALLINT NOT NULL DEFAULT 0,

    PRIMARY KEY (user_id)
);

CREATE TABLE patrons (
    patreon_id VARCHAR(64) NOT NULL,
    discord_id BIGINT,
    last_patreon_total_cents BIGINT NOT NULL DEFAULT 0,

    FOREIGN KEY (discord_id) REFERENCES users (user_id)
        MATCH SIMPLE
        ON DELETE RESTRICT
        ON UPDATE CASCADE,

    PRIMARY KEY (patreon_id)
);

CREATE TABLE members (
    user_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    xp REAL NOT NULL DEFAULT 0,
    autoredeem_enabled BOOLEAN NOT NULL DEFAULT false,

    FOREIGN KEY (user_id) REFERENCES users (user_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (guild_id) REFERENCES guilds (guild_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    PRIMARY KEY (user_id, guild_id)
);

CREATE TABLE starboards (
    id SERIAL NOT NULL,
    name TEXT NOT NULL,
    channel_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    webhook_id BIGINT,
    premium_locked BOOLEAN NOT NULL DEFAULT false,

    -- style
    display_emoji TEXT DEFAULT '⭐',
    ping_author BOOLEAN NOT NULL DEFAULT false,
    use_server_profile BOOLEAN NOT NULL DEFAULT true,
    extra_embeds BOOLEAN NOT NULL DEFAULT true,
    use_webhook BOOLEAN NOT NULL DEFAULT false,

    -- embed style
    color INTEGER,
    jump_to_message BOOLEAN NOT NULL DEFAULT true,
    attachments_list BOOLEAN NOT NULL DEFAULT true,
    replied_to BOOLEAN NOT NULL DEFAULT true,

    -- requirements
    required SMALLINT NOT NULL DEFAULT 3,
    required_remove SMALLINT NOT NULL DEFAULT 0,
    upvote_emojis TEXT[] NOT NULL DEFAULT '{"⭐"}',
    downvote_emojis TEXT[] NOT NULL DEFAULT '{}',
    self_vote BOOLEAN NOT NULL DEFAULT false,
    allow_bots BOOLEAN NOT NULL DEFAULT true,
    require_image BOOLEAN NOT NULL DEFAULT false,
    older_than BIGINT NOT NULL DEFAULT 0,
    newer_than BIGINT NOT NULL DEFAULT 0,

    -- behaviour
    enabled BOOLEAN NOT NULL DEFAULT true,
    autoreact_upvote BOOLEAN NOT NULL DEFAULT true,
    autoreact_downvote BOOLEAN NOT NULL DEFAULT true,
    remove_invalid_reactions BOOLEAN NOT NULL DEFAULT true,
    link_deletes BOOLEAN NOT NULL DEFAULT false,
    link_edits BOOLEAN NOT NULL DEFAULT true,
    private BOOLEAN NOT NULL DEFAULT false,
    xp_multiplier REAL NOT NULL DEFAULT 1.0,
    cooldown_enabled BOOLEAN NOT NULL DEFAULT false,
    cooldown_count SMALLINT NOT NULL DEFAULT 5,
    cooldown_period SMALLINT NOT NULL DEFAULT 5,

    UNIQUE (guild_id, name),

    FOREIGN KEY (guild_id) REFERENCES guilds (guild_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    PRIMARY KEY (id)
);

CREATE TABLE overrides (
    id SERIAL NOT NULL,
    guild_id BIGINT NOT NULL,
    name TEXT NOT NULL,
    starboard_id INTEGER NOT NULL,
    channel_ids BIGINT[] NOT NULL DEFAULT '{}',
    overrides JSON NOT NULL DEFAULT '{}',

    FOREIGN KEY (guild_id) REFERENCES guilds (guild_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (starboard_id) REFERENCES starboards (id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    PRIMARY KEY (id)
);

CREATE TABLE permroles (
    role_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    obtain_xproles BOOLEAN,
    give_votes BOOLEAN,
    receive_votes BOOLEAN,

    FOREIGN KEY (guild_id) REFERENCES guilds (guild_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    PRIMARY KEY (role_id)
);

CREATE TABLE permrole_starboards (
    permrole_id BIGINT NOT NULL,
    starboard_id INTEGER NOT NULL,
    give_votes BOOLEAN,
    receive_votes BOOLEAN,

    FOREIGN KEY (starboard_id) REFERENCES starboards (id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (permrole_id) REFERENCES permroles (role_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    PRIMARY KEY (permrole_id, starboard_id)
);

CREATE TABLE aschannels (
    id SERIAL NOT NULL,
    name TEXT NOT NULL,
    channel_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    premium_locked BOOLEAN NOT NULL DEFAULT false,
    emojis TEXT[] NOT NULL DEFAULT '{}',
    min_chars SMALLINT NOT NULL DEFAULT 0,
    max_chars SMALLINT,
    require_image BOOLEAN NOT NULL DEFAULT false,
    delete_invalid BOOLEAN NOT NULL DEFAULT false,

    UNIQUE (guild_id, name),

    FOREIGN KEY (guild_id) REFERENCES guilds (guild_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    PRIMARY KEY (id)
);

CREATE TABLE xproles (
    role_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    required SMALLINT NOT NULL,

    FOREIGN KEY (guild_id) REFERENCES guilds (guild_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    PRIMARY KEY (role_id)
);

CREATE TABLE posroles (
    role_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    max_members INTEGER NOT NULL,

    FOREIGN KEY (guild_id) REFERENCES guilds (guild_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    PRIMARY KEY (role_id)
);

CREATE TABLE posrole_members (
    role_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,

    FOREIGN KEY (role_id) REFERENCES posroles (role_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (user_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    PRIMARY KEY (role_id, user_id)
);

CREATE TABLE messages (
    message_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    author_id BIGINT NOT NULL,
    is_nsfw BOOLEAN NOT NULL,
    forced_to INTEGER[] NOT NULL DEFAULT '{}',
    trashed BOOLEAN NOT NULL DEFAULT false,
    trash_reason VARCHAR,
    frozen BOOLEAN NOT NULL DEFAULT false,

    FOREIGN KEY (guild_id) REFERENCES guilds (guild_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (author_id) REFERENCES users (user_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    PRIMARY KEY (message_id)
);

CREATE TABLE starboard_messages (
    message_id BIGINT NOT NULL,
    starboard_id INTEGER NOT NULL,
    starboard_message_id BIGINT,
    last_known_point_count SMALLINT NOT NULL DEFAULT 0,

    FOREIGN KEY (message_id) REFERENCES messages (message_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (starboard_id) REFERENCES starboards (id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    PRIMARY KEY (message_id, starboard_id)
);

CREATE TABLE votes (
    message_id BIGINT NOT NULL,
    starboard_id INTEGER NOT NULL,
    user_id BIGINT NOT NULL,
    target_author_id BIGINT NOT NULL,
    is_downvote BOOLEAN NOT NULL,

    FOREIGN KEY (message_id) REFERENCES messages (message_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (starboard_id) REFERENCES starboards (id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (user_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (target_author_id) REFERENCES users (user_id)
        MATCH SIMPLE
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    PRIMARY KEY (message_id, starboard_id)
);

-- indexes
CREATE INDEX _btree_index_patrons__discord_id ON patrons USING BTREE ((discord_id));
CREATE INDEX _btree_index_aschannels__guild_id_name ON aschannels USING BTREE ((guild_id), (name));
CREATE INDEX _btree_index_aschannels__channel_id ON aschannels USING BTREE ((channel_id));
CREATE INDEX _btree_index_guilds__premium_end ON guilds USING BTREE ((premium_end));
CREATE INDEX _btree_index_members__guild_id ON members USING BTREE ((guild_id));
CREATE INDEX _btree_index_members__autoredeem_enabled ON members USING BTREE ((autoredeem_enabled));
CREATE INDEX _btree_index_members__xp ON members USING BTREE ((xp));
CREATE UNIQUE INDEX _btree_index_overrides__guild_id_name ON overrides USING BTREE ((guild_id), (name));
CREATE INDEX _btree_index_overrides__starboard_id ON overrides USING BTREE ((starboard_id));
CREATE INDEX _gin_index_overrides__channel_ids ON overrides USING GIN ((channel_ids));
CREATE UNIQUE INDEX _btree_index_sb_messages__sb_message_id ON starboard_messages USING BTREE ((starboard_message_id));
CREATE INDEX _btree_index_sb_messages__last_known_point_count ON starboard_messages USING BTREE ((last_known_point_count));
CREATE INDEX _btree_index_sb_messages__starboard_id ON starboard_messages USING BTREE ((starboard_id));
CREATE INDEX _btree_index_permroles__guild_id ON permroles USING BTREE ((guild_id));
CREATE UNIQUE INDEX _btree_index_posroles__guild_id_max_members ON posroles USING BTREE ((guild_id), (max_members));
CREATE INDEX _btree_index_starboards__guild_id_name ON starboards USING BTREE ((guild_id), (name));
CREATE INDEX _btree_index_starboards__channel_id ON starboards USING BTREE ((channel_id));
CREATE INDEX _btree_index_xproles__guild_id ON xproles USING BTREE ((guild_id));
CREATE INDEX _btree_index_votes__starboard_id ON votes USING BTREE ((starboard_id));
CREATE INDEX _btree_index_votes__user_id ON votes USING BTREE ((user_id));
CREATE INDEX _btree_index_votes__message_id ON votes USING BTREE ((message_id));
CREATE INDEX _btree_index_votes__target_author_id ON votes USING BTREE ((target_author_id));
CREATE INDEX _btree_index_votes__is_downvote ON votes USING BTREE ((is_downvote));
