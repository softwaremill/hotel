-- Change booking start_time and end_time from TIMESTAMPTZ to DATE
-- This represents check-in and check-out dates in the hotel's local timezone

-- Drop the existing index that references the old columns
DROP INDEX idx_bookings_dates;

-- Change the column types from TIMESTAMPTZ to DATE
ALTER TABLE bookings 
ALTER COLUMN start_time TYPE DATE,
ALTER COLUMN end_time TYPE DATE;

-- Recreate the index for date-based queries
CREATE INDEX idx_bookings_dates ON bookings (hotel_id, start_time, end_time);