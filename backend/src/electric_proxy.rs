use crate::app_state::AppState;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::StatusCode,
    response::Response,
};
use futures::StreamExt;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct BookingShapeParams {
    date: String,
    offset: Option<String>,
    handle: Option<String>,
    live: Option<bool>,
}

pub async fn get_hotel_bookings_shape(
    Path(hotel_id): Path<i64>,
    Query(params): Query<BookingShapeParams>,
    State(app_state): State<AppState>,
) -> Result<Response, StatusCode> {
    // Get Electric URL from environment or use default
    let electric_url =
        env::var("ELECTRIC_URL").unwrap_or_else(|_| "http://localhost:3001".to_string());

    // Build Electric API URL for bookings table
    let url = format!("{}/v1/shape", electric_url);

    // Build query parameters - fix table to bookings and hotel_id with mandatory date filtering
    let where_clause = format!(
        "hotel_id = {} AND start_time <= '{}' AND end_time >= '{}'",
        hotel_id, params.date, params.date
    );

    let mut query_params = vec![("table", "bookings".to_string()), ("where", where_clause)];

    if let Some(offset) = params.offset {
        query_params.push(("offset", offset));
    }

    if let Some(handle) = params.handle {
        query_params.push(("handle", handle));
    }

    if let Some(live) = params.live {
        query_params.push(("live", live.to_string()));
    }

    // Forward request to Electric using shared HTTP client
    let response = app_state
        .http_client
        .get(&url)
        .query(&query_params)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to connect to Electric at {}: {}", electric_url, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Preserve Electric's status code and headers
    let status = response.status();
    let headers = response.headers().clone();

    // If Electric response is not successful, log the error with response body
    if !status.is_success() {
        let error_body = response
            .text()
            .await
            .unwrap_or_else(|_| "Unable to read response body".to_string());
        
        tracing::warn!(
            "Electric returned error status: {} for hotel_id: {}, body: {}",
            status,
            hotel_id,
            error_body
        );
        
        let status_code = match status.as_u16() {
            400..=499 => StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_REQUEST),
            500..=599 => StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
            _ => StatusCode::BAD_GATEWAY,
        };
        return Err(status_code);
    }

    // Convert response to streaming body
    let stream = response.bytes_stream().map(|result| {
        result.map_err(|e| {
            tracing::error!("Error streaming from Electric: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })
    });

    let body = Body::from_stream(stream);

    // Build response with streaming body and forward Electric headers
    let mut response_builder = Response::builder().status(StatusCode::OK);

    // Forward headers from Electric
    if let Some(response_headers) = response_builder.headers_mut() {
        for (key, value) in headers.iter() {
            // Convert header names and values to strings for logging and comparison
            let key_str = key.as_str();

            // Forward relevant headers, skip problematic ones
            if key_str == "content-type"
                || key_str == "cache-control"
                || key_str == "etag"
                || key_str.starts_with("electric-")
            {
                // Convert reqwest headers to axum headers by creating new ones
                if let Ok(header_name) = axum::http::HeaderName::from_bytes(key.as_str().as_bytes())
                {
                    if let Ok(header_value) = axum::http::HeaderValue::from_bytes(value.as_bytes())
                    {
                        response_headers.insert(header_name, header_value);
                    }
                }
            }
        }
    }

    let response = response_builder.body(body).map_err(|e| {
        tracing::error!("Failed to build response: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(response)
}
