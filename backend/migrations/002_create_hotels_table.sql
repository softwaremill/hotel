-- Create hotels configuration table
-- Dictionary mapping hotel ID to number of rooms
-- The rooms are numbered from 1 to room_count (inclusive)

CREATE TABLE hotels (
    id          BIGSERIAL PRIMARY KEY,
    name        TEXT NOT NULL,
    room_count  INTEGER NOT NULL CHECK (room_count > 0)
);