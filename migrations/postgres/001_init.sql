-- Initial schema for matrix-bridge-zulip

-- Organizations table
CREATE TABLE IF NOT EXISTS organizations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    site TEXT NOT NULL,
    email TEXT NOT NULL,
    api_key TEXT NOT NULL,
    connected BOOLEAN NOT NULL DEFAULT FALSE,
    max_backfill_amount INTEGER NOT NULL DEFAULT 100,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Room mappings table
CREATE TABLE IF NOT EXISTS room_mappings (
    id BIGSERIAL PRIMARY KEY,
    matrix_room_id TEXT NOT NULL UNIQUE,
    zulip_stream_id BIGINT NOT NULL,
    zulip_stream_name TEXT NOT NULL,
    zulip_topic TEXT,
    organization_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    room_type TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- User mappings table
CREATE TABLE IF NOT EXISTS user_mappings (
    id BIGSERIAL PRIMARY KEY,
    matrix_user_id TEXT NOT NULL UNIQUE,
    zulip_user_id BIGINT NOT NULL UNIQUE,
    zulip_email TEXT,
    display_name TEXT,
    avatar_url TEXT,
    is_bot BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Message mappings table
CREATE TABLE IF NOT EXISTS message_mappings (
    id BIGSERIAL PRIMARY KEY,
    matrix_event_id TEXT NOT NULL UNIQUE,
    matrix_room_id TEXT NOT NULL,
    zulip_message_id BIGINT NOT NULL UNIQUE,
    zulip_sender_id BIGINT NOT NULL,
    message_type TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Processed events table (for deduplication)
CREATE TABLE IF NOT EXISTS processed_events (
    id BIGSERIAL PRIMARY KEY,
    event_id TEXT NOT NULL UNIQUE,
    event_type TEXT NOT NULL,
    source TEXT NOT NULL,
    processed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Reaction mappings table
CREATE TABLE IF NOT EXISTS reaction_mappings (
    id BIGSERIAL PRIMARY KEY,
    matrix_event_id TEXT NOT NULL,
    zulip_message_id BIGINT NOT NULL,
    zulip_reaction_id BIGINT NOT NULL,
    emoji TEXT NOT NULL,
    matrix_reaction_event_id TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(zulip_reaction_id, emoji)
);

-- Indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_room_mappings_organization ON room_mappings(organization_id);
CREATE INDEX IF NOT EXISTS idx_room_mappings_zulip_stream ON room_mappings(zulip_stream_id);
CREATE INDEX IF NOT EXISTS idx_room_mappings_type ON room_mappings(room_type);

CREATE INDEX IF NOT EXISTS idx_user_mappings_zulip ON user_mappings(zulip_user_id);

CREATE INDEX IF NOT EXISTS idx_message_mappings_matrix_room ON message_mappings(matrix_room_id);
CREATE INDEX IF NOT EXISTS idx_message_mappings_zulip_message ON message_mappings(zulip_message_id);

CREATE INDEX IF NOT EXISTS idx_processed_events_event_id ON processed_events(event_id);
CREATE INDEX IF NOT EXISTS idx_processed_events_processed_at ON processed_events(processed_at);

CREATE INDEX IF NOT EXISTS idx_reaction_mappings_zulip_message ON reaction_mappings(zulip_message_id);
CREATE INDEX IF NOT EXISTS idx_reaction_mappings_matrix_event ON reaction_mappings(matrix_event_id);
