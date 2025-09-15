use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

/// Client-side events that can be generated when offline and synced later
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientEvent {
    #[serde(rename = "offline_checkin")]
    OfflineCheckin(OfflineCheckinEvent),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfflineCheckinEvent {
    pub booking_id: String, // Accept as string to handle large integers safely
    pub room_number: i32,
    pub today: NaiveDate,
}