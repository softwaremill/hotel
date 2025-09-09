use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Event types for event sourcing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", content = "data")]
pub enum Event {
    BookingCreated(BookingCreatedEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingCreatedEvent {
    pub booking_id: i64,
    pub hotel_id: i64,
    pub guest_name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}