use axum::{Json, response::Json as ResponseJson};
use serde_json::{json, Value};
use crate::models::{Room, Booking};

pub async fn health_check() -> ResponseJson<Value> {
    ResponseJson(json!({
        "status": "healthy",
        "service": "hotel-backend"
    }))
}

pub async fn get_rooms() -> ResponseJson<Vec<Room>> {
    ResponseJson(vec![])
}

pub async fn get_bookings() -> ResponseJson<Vec<Booking>> {
    ResponseJson(vec![])
}

pub async fn create_booking(Json(_booking): Json<Booking>) -> ResponseJson<Value> {
    ResponseJson(json!({
        "message": "Booking creation not implemented yet"
    }))
}