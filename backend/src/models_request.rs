use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBookingRequest {
    pub guest_name: String,
    pub start_time: NaiveDate,
    pub end_time: NaiveDate,
}