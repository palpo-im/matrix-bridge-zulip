diesel::table! {
    organizations (id) {
        id -> Text,
        name -> Text,
        site -> Text,
        email -> Text,
        api_key -> Text,
        connected -> Bool,
        max_backfill_amount -> Integer,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    room_mappings (id) {
        id -> BigInt,
        matrix_room_id -> Text,
        zulip_stream_id -> BigInt,
        zulip_stream_name -> Text,
        zulip_topic -> Nullable<Text>,
        organization_id -> Text,
        room_type -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_mappings (id) {
        id -> BigInt,
        matrix_user_id -> Text,
        zulip_user_id -> BigInt,
        zulip_email -> Nullable<Text>,
        display_name -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        is_bot -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    message_mappings (id) {
        id -> BigInt,
        matrix_event_id -> Text,
        matrix_room_id -> Text,
        zulip_message_id -> BigInt,
        zulip_sender_id -> BigInt,
        message_type -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    processed_events (id) {
        id -> BigInt,
        event_id -> Text,
        event_type -> Text,
        source -> Text,
        processed_at -> Timestamptz,
    }
}

diesel::table! {
    reaction_mappings (id) {
        id -> BigInt,
        matrix_event_id -> Text,
        zulip_message_id -> BigInt,
        zulip_reaction_id -> BigInt,
        emoji -> Text,
        matrix_reaction_event_id -> Text,
        created_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    organizations,
    room_mappings,
    user_mappings,
    message_mappings,
    processed_events,
    reaction_mappings,
);
