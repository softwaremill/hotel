use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBookingRequest {
    pub guest_name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}