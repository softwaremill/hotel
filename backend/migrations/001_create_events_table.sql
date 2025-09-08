-- Create events table for event sourcing
-- Based on: https://softwaremill.com/implementing-event-sourcing-using-a-relational-database/

CREATE TABLE events (
    id        SERIAL PRIMARY KEY,
    stream_id BIGINT NOT NULL,
    version   BIGINT NOT NULL,
    data      JSONB  NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_stream_version UNIQUE (stream_id, version)
);