use crate::app_state::AppState;
use crate::db::{
    get_bookings_by_hotel_id, get_hotel_by_id, get_next_booking_id, get_overlapping_bookings,
};
use crate::models_events::{BookingCreatedEvent, Event};
use crate::models_request::CreateBookingRequest;
use crate::room_assignment::can_accommodate_booking;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json as ResponseJson, Response},
};
use serde_json::{Value, json};
use sqlx::{Executor, Postgres};
use tracing::error;

fn internal_server_error() -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        ResponseJson(json!({
            "error": "Internal server error"
        })),
    )
        .into_response()
}

fn not_found_error(message: &str) -> Response {
    (
        StatusCode::NOT_FOUND,
        ResponseJson(json!({
            "error": message
        })),
    )
        .into_response()
}

fn bad_request_error_with_code(message: &str, code: &str) -> Response {
    (
        StatusCode::BAD_REQUEST,
        ResponseJson(json!({
            "error": message,
            "code": code,
        })),
    )
        .into_response()
}

async fn get_hotel_or_error<'a, E>(
    executor: E,
    hotel_id: i64,
) -> Result<crate::models::Hotel, Response>
where
    E: Executor<'a, Database = Postgres>,
{
    match get_hotel_by_id(executor, hotel_id).await {
        Ok(Some(hotel)) => Ok(hotel),
        Ok(None) => Err(not_found_error("Hotel not found")),
        Err(err) => {
            error!("Failed to get hotel {}: {}", hotel_id, err);
            Err(internal_server_error())
        }
    }
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
    // Start a single database transaction for the entire operation
    let mut tx = match app_state.db_pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            error!("Failed to start transaction: {}", err);
            return internal_server_error();
        }
    };

    // First, get hotel info to check room count within the transaction
    let hotel = match get_hotel_or_error(&mut *tx, hotel_id).await {
        Ok(hotel) => hotel,
        Err(response) => return response,
    };

    // Check room availability within the transaction
    let overlapping_bookings =
        match get_overlapping_bookings(&mut tx, hotel_id, request.start_time, request.end_time)
            .await
        {
            Ok(bookings) => bookings,
            Err(err) => {
                error!(
                    "Failed to check overlapping bookings for hotel {}: {}",
                    hotel_id, err
                );
                return internal_server_error();
            }
        };

    if !can_accommodate_booking(
        hotel.room_count,
        overlapping_bookings,
        request.start_time,
        request.end_time,
    ) {
        return bad_request_error_with_code(
            "No rooms available for the requested dates",
            "NO_ROOMS_AVAILABLE",
        );
    }

    // Generate booking ID within the transaction
    let booking_id = match get_next_booking_id(&mut tx).await {
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

    // Process the event within the existing transaction
    let stream_id = booking_id; // Use booking_id as stream_id
    if let Err(err) = app_state
        .event_processor
        .process_event_with_tx(&mut tx, stream_id, event)
        .await
    {
        error!("Failed to process booking event: {}", err);
        return bad_request_error_with_code(
            &format!("Failed to create booking: {}", err),
            "BOOKING_CREATION_FAILED",
        );
    }

    // Commit the transaction
    if let Err(err) = tx.commit().await {
        error!("Failed to commit booking transaction: {}", err);
        return internal_server_error();
    }

    (
        StatusCode::CREATED,
        ResponseJson(json!({
            "booking_id": booking_id,
            "message": "Booking created successfully"
        })),
    )
        .into_response()
}

pub async fn get_hotel(State(app_state): State<AppState>, Path(id): Path<i64>) -> Response {
    match get_hotel_or_error(&app_state.db_pool, id).await {
        Ok(hotel) => (StatusCode::OK, ResponseJson(hotel)).into_response(),
        Err(response) => response,
    }
}
