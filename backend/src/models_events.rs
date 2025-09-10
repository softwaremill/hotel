use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// Event types for event sourcing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", content = "data")]
pub enum Event {
    BookingCreated(BookingCreatedEvent),
    BookingCheckedIn(BookingCheckedInEvent),
    BookingCheckedOut(BookingCheckedOutEvent),
    BookingCancelled(BookingCancelledEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingCreatedEvent {
    pub booking_id: i64,
    pub hotel_id: i64,
    pub guest_name: String,
    pub start_time: NaiveDate,
    pub end_time: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingCheckedInEvent {
    pub booking_id: i64,
    pub assigned_room: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingCheckedOutEvent {
    pub booking_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingCancelledEvent {
    pub booking_id: i64,
}