use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BookingStatus {
    Confirmed,
    CheckedIn,
    CheckedOut,
    Cancelled,
}

impl std::fmt::Display for BookingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            BookingStatus::Confirmed => "confirmed",
            BookingStatus::CheckedIn => "checked_in",
            BookingStatus::CheckedOut => "checked_out",
            BookingStatus::Cancelled => "cancelled",
        };
        write!(f, "{}", status_str)
    }
}

impl std::str::FromStr for BookingStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "confirmed" => Ok(BookingStatus::Confirmed),
            "checked_in" => Ok(BookingStatus::CheckedIn),
            "checked_out" => Ok(BookingStatus::CheckedOut),
            "cancelled" => Ok(BookingStatus::Cancelled),
            _ => Err(format!("Invalid booking status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotel {
    pub id: i64,
    pub name: String,
    pub room_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booking {
    pub id: i64,
    pub hotel_id: i64,
    pub room_number: Option<i32>,
    pub guest_name: String,
    pub start_time: NaiveDate,
    pub end_time: NaiveDate,
    pub status: BookingStatus,
}
