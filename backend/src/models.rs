use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;

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
    pub id: Uuid,
    pub room_id: Uuid,
    pub guest_name: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub total_price: f64,
}