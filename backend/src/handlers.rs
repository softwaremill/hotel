use crate::app_state::AppState;
use crate::db::{get_bookings_by_hotel_id, get_hotel_by_id, get_next_booking_id};
use crate::models::{Booking, BookingCreatedEvent, CreateBookingRequest, Event, Hotel};
use axum::{
    Json,
    extract::{Path, State},
    response::Json as ResponseJson,
};
use serde_json::{Value, json};

pub async fn health_check() -> ResponseJson<Value> {
    ResponseJson(json!({
        "status": "healthy",
        "service": "hotel-backend"
    }))
}

pub async fn get_bookings(
    State(app_state): State<AppState>,
    Path(hotel_id): Path<i64>,
) -> Result<ResponseJson<Vec<Booking>>, ResponseJson<Value>> {
    match get_bookings_by_hotel_id(&app_state.db_pool, hotel_id).await {
        Ok(bookings) => Ok(ResponseJson(bookings)),
        Err(err) => Err(ResponseJson(json!({
            "error": "Database error",
            "message": err.to_string()
        }))),
    }
}

pub async fn create_booking(
    State(app_state): State<AppState>,
    Path(hotel_id): Path<i64>,
    Json(request): Json<CreateBookingRequest>,
) -> Result<ResponseJson<Value>, ResponseJson<Value>> {
    // Generate booking ID
    let booking_id = match get_next_booking_id(&app_state.db_pool).await {
        Ok(id) => id,
        Err(err) => {
            return Err(ResponseJson(json!({
                "error": "Failed to generate booking ID",
                "message": err.to_string()
            })));
        }
    };

    // Create the booking event
    let event = Event::BookingCreated(BookingCreatedEvent {
        booking_id,
        hotel_id,
        guest_name: request.guest_name,
        start_time: request.start_time,
        end_time: request.end_time,
    });

    // Process the event (will insert into events table and update projections)
    let stream_id = booking_id; // Use booking_id as stream_id
    match app_state
        .event_processor
        .process_event(stream_id, event)
        .await
    {
        Ok(_) => Ok(ResponseJson(json!({
            "booking_id": booking_id,
            "message": "Booking created successfully"
        }))),
        Err(err) => Err(ResponseJson(json!({
            "error": "Failed to create booking",
            "message": err.to_string()
        }))),
    }
}

pub async fn get_hotel(
    State(app_state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<ResponseJson<Hotel>, ResponseJson<Value>> {
    match get_hotel_by_id(&app_state.db_pool, id).await {
        Ok(Some(hotel)) => Ok(ResponseJson(hotel)),
        Ok(None) => Err(ResponseJson(json!({
            "error": "Hotel not found",
            "id": id
        }))),
        Err(err) => Err(ResponseJson(json!({
            "error": "Database error",
            "message": err.to_string()
        }))),
    }
}
