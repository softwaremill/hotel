use crate::app_state::AppState;
use crate::db::{
    get_and_lock_overlapping_bookings, get_bookings_by_hotel_id, get_hotel_by_id,
    get_next_booking_id,
};
use crate::error::{AppError, AppResult};
use crate::models::Hotel;
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

async fn get_hotel_or_not_found<'a, E>(executor: E, hotel_id: i64) -> AppResult<Hotel>
where
    E: Executor<'a, Database = Postgres>,
{
    match get_hotel_by_id(executor, hotel_id).await? {
        Some(hotel) => Ok(hotel),
        None => Err(AppError::not_found("Hotel not found")),
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
) -> AppResult<Response> {
    let bookings = get_bookings_by_hotel_id(&app_state.db_pool, hotel_id).await?;
    Ok((StatusCode::OK, ResponseJson(bookings)).into_response())
}

pub async fn create_booking(
    State(app_state): State<AppState>,
    Path(hotel_id): Path<i64>,
    Json(request): Json<CreateBookingRequest>,
) -> AppResult<Response> {
    // Start a single database transaction for the entire operation
    let mut tx = app_state.db_pool.begin().await?;

    // First, get hotel info to check room count within the transaction
    let hotel = get_hotel_or_not_found(&mut *tx, hotel_id).await?;

    // Check room availability within the transaction
    // Using SELECT ... FOR UPDATE so that it's not possible to concurrently add overlapping bookings,
    // which might use stale data to be used to verify booking possibility (write skew).
    let overlapping_bookings =
        get_and_lock_overlapping_bookings(&mut tx, hotel_id, request.start_time, request.end_time)
            .await?;

    if !can_accommodate_booking(
        hotel.room_count,
        overlapping_bookings,
        request.start_time,
        request.end_time,
    ) {
        return Err(AppError::bad_request(
            "No rooms available for the requested dates",
            "NO_ROOMS_AVAILABLE",
        ));
    }

    // Generate booking ID within the transaction
    let booking_id = get_next_booking_id(&mut tx).await?;

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
    app_state
        .event_processor
        .process_event_with_tx(&mut tx, stream_id, event)
        .await?;

    // Commit the transaction
    tx.commit().await?;

    Ok((
        StatusCode::CREATED,
        ResponseJson(json!({
            "booking_id": booking_id,
            "message": "Booking created successfully"
        })),
    )
        .into_response())
}

pub async fn get_hotel(
    State(app_state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Response> {
    let hotel = get_hotel_or_not_found(&app_state.db_pool, id).await?;
    Ok((StatusCode::OK, ResponseJson(hotel)).into_response())
}
