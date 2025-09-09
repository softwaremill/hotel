-- Create sequence for generating booking IDs
CREATE SEQUENCE booking_id_seq START 1;

-- Create bookings projection table
-- This table is a projection of events, updated when booking events are processed

CREATE TABLE bookings (
    id              BIGINT PRIMARY KEY,
    hotel_id        BIGINT NOT NULL REFERENCES hotels(id),
    room_number     INTEGER NULL,
    guest_name      TEXT NOT NULL,
    start_time      TIMESTAMPTZ NOT NULL,
    end_time        TIMESTAMPTZ NOT NULL,
    status          TEXT NOT NULL DEFAULT 'confirmed' CHECK (status IN ('confirmed', 'checked_in', 'checked_out', 'cancelled')),
    
    -- Ensure room numbers are valid for the hotel (when assigned)
    CHECK (room_number IS NULL OR room_number > 0),
    -- Ensure booking dates are logical
    CHECK (end_time > start_time)
);

-- Index for finding bookings by date range
CREATE INDEX idx_bookings_dates ON bookings (hotel_id, start_time, end_time);
