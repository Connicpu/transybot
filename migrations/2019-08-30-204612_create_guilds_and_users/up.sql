PRAGMA foreign_keys = ON;

CREATE TABLE guilds (
    id BIGINT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,

    rejection_message TEXT
) WITHOUT ROWID;

CREATE TABLE rules (
    guild_id BIGINT NOT NULL,
    rule_number INT NOT NULL,
    rule_text TEXT NOT NULL,
    rule_description TEXT,

    -- Response types:
    -- 0: "Rule" type. User must respond with checkmark emoji.
    -- 1: True or False.
    -- 2: Multiple choice. Responses will be 1 through 9 emoji.
    -- 3: Open ended question.
    -- 4: Pronoun selection.
    -- 5: General role selection.
    response_type INT NOT NULL DEFAULT 0,
    multiple_choice_count INT,

    FOREIGN KEY (guild_id)
        REFERENCES guilds (guild_id)
        ON DELETE CASCADE,

    PRIMARY KEY (guild_id, rule_number ASC)
) WITHOUT ROWID;

CREATE TABLE role_categories (
    id INTEGER PRIMARY KEY NOT NULL,
    guild_id BIGINT NOT NULL,
    name TEXT,
    list_order INTEGER,
    has_divider BOOLEAN NOT NULL DEFAULT false,
    divider_name TEXT,

    FOREIGN KEY (guild_id)
        REFERENCES guilds (id)
        ON DELETE CASCADE
);

CREATE TABLE roles (
    guild_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,
    name TEXT NOT NULL,
    category_id INTEGER,
    list_order INTEGER,
    color TEXT,

    -- Role Types:
    -- 0: tag role
    -- 1: silence role
    -- 2: user role
    -- 3: moderator role
    -- 4: admin role
    role_type INT NOT NULL DEFAULT 0,
    -- Whether this is the default role for this role type
    is_default BOOLEAN NOT NULL DEFAULT false,
    is_pronoun BOOLEAN NOT NULL DEFAULT false,
    is_18plus BOOLEAN NOT NULL DEFAULT false,

    self_assignable BOOLEAN NOT NULL DEFAULT false,
    mod_assignable BOOLEAN NOT NULL DEFAULT false,
    times_joined INT NOT NULL DEFAULT 1,
    last_left TIMESTAMP,

    FOREIGN KEY (guild_id)
        REFERENCES guilds (id)
        ON DELETE CASCADE,

    FOREIGN KEY (category_id)
        REFERENCES role_categories (id)
        ON DELETE SET NULL,

    PRIMARY KEY (guild_id, role_id)
) WITHOUT ROWID;

CREATE TABLE role_aliases (
    guild_id BIGINT NOT NULL,
    alias_name TEXT NOT NULL,
    role_id BIGINT NOT NULL,

    FOREIGN KEY (guild_id, role_id)
        REFERENCES roles (guild_id, role_id)
        ON DELETE CASCADE,

    PRIMARY KEY (guild_id, alias_name)
) WITHOUT ROWID;

CREATE TABLE users (
    user_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    username TEXT NOT NULL,

    -- 0: Fresh user
    -- 1: Answering Questions
    -- 2: Finished Answering
    -- 3: Accepted
    join_status INT NOT NULL DEFAULT 0,

    -- 0: Unjoined
    -- 1: User
    -- 2: Moderator
    -- 3: Admin
    server_rank INT NOT NULL DEFAULT 0,
    rank_role BIGINT,

    silenced_until TIMESTAMP,

    first_joined TIMESTAMP NOT NULL,
    last_left TIMESTAMP,
    times_left INT NOT NULL DEFAULT 0,

    confirmed_18 BOOLEAN NOT NULL DEFAULT false,

    FOREIGN KEY (guild_id)
        REFERENCES guilds (id)
        ON DELETE CASCADE,

    FOREIGN KEY (rank_role)
        REFERENCES roles (role_id)
        ON DELETE SET NULL,

    PRIMARY KEY (guild_id, user_id)
) WITHOUT ROWID;

CREATE TABLE user_roles (
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,

    FOREIGN KEY (guild_id, user_id)
        REFERENCES users (guild_id, id)
        ON DELETE CASCADE,
        
    FOREIGN KEY (role_id)
        REFERENCES roles (role_id)
        ON DELETE CASCADE,

    PRIMARY KEY (guild_id, user_id, role_id)
) WITHOUT ROWID;

CREATE TABLE user_responses (
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    rule_number INT NOT NULL,

    response TEXT NOT NULL,

    FOREIGN KEY (guild_id)
        REFERENCES guilds (id)
        ON DELETE CASCADE,

    FOREIGN KEY (guild_id, user_id)
        REFERENCES users (guild_id, user_id)
        ON DELETE CASCADE,

    FOREIGN KEY (guild_id, rule_number)
        REFERENCES rules (guild_id, rule_number)
        ON DELETE CASCADE,

    PRIMARY KEY (guild_id, user_id, rule_number)
) WITHOUT ROWID;

CREATE TABLE permission_rules (
    rule_id INTEGER PRIMARY KEY,

    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    is_category BOOLEAN NOT NULL,
    role_id BIGINT,

    -- Permission types:
    -- 0: Grant read
    -- 1: Grant write
    -- 2: Deny read
    -- 3: Deny write
    permission_type INT NOT NULL,

    FOREIGN KEY (guild_id)
        REFERENCES guilds (id)
        ON DELETE CASCADE,

    FOREIGN KEY (guild_id, role_id)
        REFERENCES roles (guild_id, role_id)
        ON DELETE SET NULL,

    UNIQUE (guild_id, channel_id, role_id, permission_type)
);

CREATE TABLE special_channels (
    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,

    -- Special Types:
    -- 0: Log channel. Bot errors and warnings will appear here.
    -- 1: Join log, to keep track of joins, leaves, kicks, and bans.
    -- 2: Join approval channel, where moderators approve or reject new users.
    -- 3: Rules channel. The bot keeps a post with the up to date rules here.
    -- 4: Roles channel. The bot keeps the role list up to date here.
    -- 5: Insta-deletion channel. Sends to `other_channel` if not null.
    special_type INT NOT NULL,

    other_channel BIGINT,

    FOREIGN KEY (guild_id)
        REFERENCES guilds (id)
        ON DELETE CASCADE,

    PRIMARY KEY (guild_id, channel_id)
) WITHOUT ROWID;

CREATE UNIQUE INDEX default_roles
    ON roles (guild_id, role_type)
    WHERE is_default = true;

CREATE INDEX role_names
    ON roles (guild_id, name);

CREATE INDEX pronoun_roles
    ON roles (guild_id, name)
    WHERE is_pronoun = true;

CREATE INDEX special_roles
    ON roles (guild_id, role_type)
    WHERE role_type > 0;

CREATE INDEX silenced_users
    ON users (silenced_until)
    WHERE silenced_until IS NOT NULL;

CREATE INDEX guild_categories
    ON role_categories (guild_id);

CREATE INDEX guild_permissions
    ON permission_rules (guild_id);

CREATE INDEX guild_rules
    ON rules (guild_id);

CREATE INDEX channel_perms
    ON permission_rules (guild_id, channel_id);
