use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotel {
    pub id: i32,
    pub name: String,
    pub room_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: Uuid,
    pub number: String,
    pub room_type: String,
    pub price_per_night: f64,
    pub is_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booking {
    pub id: i32,
    pub hotel_id: i32,
    pub room_number: Option<i32>,
    pub guest_name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: String,
}