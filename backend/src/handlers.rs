use axum::{Json, response::Json as ResponseJson, extract::{Path, State}};
use serde_json::{json, Value};
use crate::models::{Room, Booking, Hotel};
use crate::db::{DbPool, get_hotel_by_id, get_bookings_by_hotel_id};

pub async fn health_check() -> ResponseJson<Value> {
    ResponseJson(json!({
        "status": "healthy",
        "service": "hotel-backend"
    }))
}

pub async fn get_rooms(
    State(_pool): State<DbPool>,
    Path(_hotel_id): Path<i32>
) -> ResponseJson<Vec<Room>> {
    // TODO: Implement room fetching for specific hotel
    ResponseJson(vec![])
}

pub async fn get_bookings(
    State(pool): State<DbPool>,
    Path(hotel_id): Path<i32>
) -> Result<ResponseJson<Vec<Booking>>, ResponseJson<Value>> {
    match get_bookings_by_hotel_id(&pool, hotel_id).await {
        Ok(bookings) => Ok(ResponseJson(bookings)),
        Err(err) => Err(ResponseJson(json!({
            "error": "Database error",
            "message": err.to_string()
        })))
    }
}

pub async fn create_booking(Json(_booking): Json<Booking>) -> ResponseJson<Value> {
    ResponseJson(json!({
        "message": "Booking creation not implemented yet"
    }))
}

pub async fn get_hotel(
    State(pool): State<DbPool>,
    Path(id): Path<i32>
) -> Result<ResponseJson<Hotel>, ResponseJson<Value>> {
    match get_hotel_by_id(&pool, id).await {
        Ok(Some(hotel)) => Ok(ResponseJson(hotel)),
        Ok(None) => Err(ResponseJson(json!({
            "error": "Hotel not found",
            "id": id
        }))),
        Err(err) => Err(ResponseJson(json!({
            "error": "Database error",
            "message": err.to_string()
        })))
    }
}