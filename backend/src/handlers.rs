use crate::app_state::AppState;
use crate::db::{get_bookings_by_hotel_id, get_hotel_by_id, get_next_booking_id};
use crate::models_events::{BookingCreatedEvent, Event};
use crate::models_request::CreateBookingRequest;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json as ResponseJson, Response},
};
use serde_json::{Value, json};
use tracing::error;

fn internal_server_error() -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        ResponseJson(json!({
            "error": "Internal server error"
        })),
    ).into_response()
}

fn not_found_error(message: &str) -> Response {
    (
        StatusCode::NOT_FOUND,
        ResponseJson(json!({
            "error": message
        })),
    ).into_response()
}

fn bad_request_error(message: &str) -> Response {
    (
        StatusCode::BAD_REQUEST,
        ResponseJson(json!({
            "error": message,
        })),
    ).into_response()
}

pub async fn health_check() -> ResponseJson<Value> {
    ResponseJson(json!({
        "status": "healthy",
        "service": "hotel-backend"
    }))
}

pub async fn get_bookings(
    State(app_state): State<AppState>,
    Path(hotel_id): Path<i64>,
) -> Response {
    match get_bookings_by_hotel_id(&app_state.db_pool, hotel_id).await {
        Ok(bookings) => (StatusCode::OK, ResponseJson(bookings)).into_response(),
        Err(err) => {
            error!("Failed to get bookings for hotel {}: {}", hotel_id, err);
            internal_server_error()
        }
    }
}

pub async fn create_booking(
    State(app_state): State<AppState>,
    Path(hotel_id): Path<i64>,
    Json(request): Json<CreateBookingRequest>,
) -> Response {
    // Generate booking ID - failure here is a server error (500)
    let booking_id = match get_next_booking_id(&app_state.db_pool).await {
        Ok(id) => id,
        Err(err) => {
            error!("Failed to generate booking ID: {}", err);
            return internal_server_error();
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

    // Process the event - failure here might be user error (400)
    let stream_id = booking_id; // Use booking_id as stream_id
    match app_state
        .event_processor
        .process_event(stream_id, event)
        .await
    {
        Ok(_) => (
            StatusCode::CREATED,
            ResponseJson(json!({
                "booking_id": booking_id,
                "message": "Booking created successfully"
            })),
        )
            .into_response(),
        Err(err) => bad_request_error(&format!("Failed to create booking: {}", err)),
    }
}

pub async fn get_hotel(State(app_state): State<AppState>, Path(id): Path<i64>) -> Response {
    match get_hotel_by_id(&app_state.db_pool, id).await {
        Ok(Some(hotel)) => (StatusCode::OK, ResponseJson(hotel)).into_response(),
        Ok(None) => not_found_error("Hotel not found"),
        Err(err) => {
            error!("Failed to get hotel {}: {}", id, err);
            internal_server_error()
        }
    }
}
