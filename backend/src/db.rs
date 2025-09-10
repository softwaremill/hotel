use crate::models::{Booking, BookingStatus, Hotel};
use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use sqlx::{Executor, PgPool, Pool, Postgres, Row, Transaction, migrate::MigrateError};
use std::str::FromStr;

const SELECT_HOTEL_QUERY: &str = "SELECT id, name, room_count FROM hotels WHERE id = $1";
const SELECT_NEXT_BOOKING_ID_QUERY: &str = "SELECT nextval('booking_id_seq') as next_id";
const SELECT_OVERLAPPING_BOOKINGS_QUERY: &str =
    "SELECT FOR UPDATE id, hotel_id, room_number, guest_name, start_time, end_time, status 
     FROM bookings 
     WHERE hotel_id = $1 
     AND status IN ('confirmed', 'checked_in')
     AND start_time < $3 
     AND end_time > $2
     ORDER BY start_time"; // ordering also ensures the locks are acquired always in the same order
const SELECT_BOOKINGS_BY_HOTEL_QUERY: &str =
    "SELECT id, hotel_id, room_number, guest_name, start_time, end_time, status 
     FROM bookings 
     WHERE hotel_id = $1
     ORDER BY start_time DESC";

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    Ok(PgPool::connect(database_url).await?)
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

fn row_to_hotel(row: &sqlx::postgres::PgRow) -> Hotel {
    Hotel {
        id: row.get("id"),
        name: row.get("name"),
        room_count: row.get("room_count"),
    }
}

fn row_to_booking(row: &sqlx::postgres::PgRow) -> Result<Booking> {
    let status_str: String = row.get("status");
    let status = BookingStatus::from_str(&status_str).map_err(|e| anyhow!(e))?;

    Ok(Booking {
        id: row.get("id"),
        hotel_id: row.get("hotel_id"),
        room_number: row.get("room_number"),
        guest_name: row.get("guest_name"),
        start_time: row.get("start_time"),
        end_time: row.get("end_time"),
        status,
    })
}

/// Gets hotel information from the database pool.
pub async fn get_hotel_by_id<'a, E>(executor: E, id: i64) -> Result<Option<Hotel>>
where
    E: Executor<'a, Database = Postgres>,
{
    let row = sqlx::query(SELECT_HOTEL_QUERY)
        .bind(id)
        .fetch_optional(executor)
        .await
        .with_context(|| format!("Failed to fetch hotel with ID {}", id))?;

    Ok(row.map(|row| row_to_hotel(&row)))
}

/// Generates the next booking ID using an existing database transaction.
pub async fn get_next_booking_id(tx: &mut Transaction<'_, Postgres>) -> Result<i64> {
    let row = sqlx::query(SELECT_NEXT_BOOKING_ID_QUERY)
        .fetch_one(&mut **tx)
        .await
        .context("Failed to generate next booking ID")?;

    Ok(row.get("next_id"))
}

/// Gets overlapping bookings using an existing database transaction.
/// This ensures consistent reads within a transaction context.
pub async fn get_and_lock_overlapping_bookings(
    tx: &mut Transaction<'_, Postgres>,
    hotel_id: i64,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Result<Vec<Booking>> {
    let rows = sqlx::query(SELECT_OVERLAPPING_BOOKINGS_QUERY)
        .bind(hotel_id)
        .bind(start_time)
        .bind(end_time)
        .fetch_all(&mut **tx)
        .await
        .with_context(|| {
            format!(
                "Failed to fetch overlapping bookings for hotel {}",
                hotel_id
            )
        })?;

    rows.into_iter().map(|row| row_to_booking(&row)).collect()
}

/// Gets all bookings for a specific hotel from the database pool.
pub async fn get_bookings_by_hotel_id(pool: &DbPool, hotel_id: i64) -> Result<Vec<Booking>> {
    let rows = sqlx::query(SELECT_BOOKINGS_BY_HOTEL_QUERY)
        .bind(hotel_id)
        .fetch_all(pool)
        .await
        .with_context(|| format!("Failed to fetch bookings for hotel {}", hotel_id))?;

    rows.into_iter().map(|row| row_to_booking(&row)).collect()
}
