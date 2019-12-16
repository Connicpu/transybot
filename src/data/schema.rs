table! {
    use diesel::sql_types::*;

    guilds (id) {
        id -> BigInt,
        name -> Text,
        rejection_message -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;

    permission_rules (rule_id) {
        rule_id -> Nullable<Integer>,
        guild_id -> BigInt,
        channel_id -> BigInt,
        is_category -> Bool,
        role_id -> Nullable<BigInt>,
        permission_type -> Integer,
    }
}

table! {
    use diesel::sql_types::*;

    role_aliases (guild_id, alias_name) {
        guild_id -> BigInt,
        alias_name -> Text,
        role_id -> BigInt,
    }
}

table! {
    use diesel::sql_types::*;

    role_categories (id) {
        id -> Integer,
        guild_id -> BigInt,
        name -> Nullable<Text>,
        list_order -> Nullable<Integer>,
        has_divider -> Bool,
        divider_name -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;

    roles (guild_id, role_id) {
        guild_id -> BigInt,
        role_id -> BigInt,
        name -> Text,
        category_id -> Nullable<Integer>,
        list_order -> Nullable<Integer>,
        color -> Nullable<Text>,
        role_type -> Integer,
        is_default -> Bool,
        is_pronoun -> Bool,
        is_18plus -> Bool,
        self_assignable -> Bool,
        mod_assignable -> Bool,
        times_joined -> Integer,
        last_left -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;

    rules (guild_id, rule_number) {
        guild_id -> BigInt,
        rule_number -> Integer,
        rule_text -> Text,
        rule_description -> Nullable<Text>,
        response_type -> Integer,
        multiple_choice_count -> Nullable<Integer>,
    }
}

table! {
    use diesel::sql_types::*;

    special_channels (guild_id, channel_id) {
        guild_id -> BigInt,
        channel_id -> BigInt,
        special_type -> Integer,
        other_channel -> Nullable<BigInt>,
    }
}

table! {
    use diesel::sql_types::*;

    user_responses (guild_id, user_id, rule_number) {
        guild_id -> BigInt,
        user_id -> BigInt,
        rule_number -> Integer,
        response -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    user_roles (guild_id, user_id, role_id) {
        guild_id -> BigInt,
        user_id -> BigInt,
        role_id -> BigInt,
    }
}

table! {
    use diesel::sql_types::*;

    users (user_id, guild_id) {
        user_id -> BigInt,
        guild_id -> BigInt,
        username -> Text,
        join_status -> Integer,
        server_rank -> Integer,
        rank_role -> Nullable<BigInt>,
        silenced_until -> Nullable<Timestamp>,
        first_joined -> Timestamp,
        last_left -> Nullable<Timestamp>,
        times_left -> Integer,
        confirmed_18 -> Bool,
    }
}

joinable!(permission_rules -> guilds (guild_id));
joinable!(role_categories -> guilds (guild_id));
joinable!(roles -> guilds (guild_id));
joinable!(roles -> role_categories (category_id));
joinable!(special_channels -> guilds (guild_id));
joinable!(user_responses -> guilds (guild_id));
joinable!(users -> guilds (guild_id));

allow_tables_to_appear_in_same_query!(
    guilds,
    permission_rules,
    role_aliases,
    role_categories,
    roles,
    rules,
    special_channels,
    user_responses,
    user_roles,
    users,
);
