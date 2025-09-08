use anyhow::Result;
use sqlx::{PgPool, Pool, Postgres, migrate::MigrateError, Row};
use crate::models::{Hotel, Booking};

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

pub async fn get_hotel_by_id(pool: &DbPool, id: i32) -> Result<Option<Hotel>> {
    let row = sqlx::query(
        "SELECT id, name, room_count FROM hotels WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    
    match row {
        Some(row) => Ok(Some(Hotel {
            id: row.get("id"),
            name: row.get("name"),
            room_count: row.get("room_count"),
        })),
        None => Ok(None),
    }
}

pub async fn get_bookings_by_hotel_id(pool: &DbPool, hotel_id: i32) -> Result<Vec<Booking>> {
    let rows = sqlx::query(
        "SELECT id, hotel_id, room_number, guest_name, start_time, end_time, status 
         FROM bookings 
         WHERE hotel_id = $1
         ORDER BY start_time DESC"
    )
    .bind(hotel_id)
    .fetch_all(pool)
    .await?;
    
    let bookings = rows
        .into_iter()
        .map(|row| Booking {
            id: row.get("id"),
            hotel_id: row.get("hotel_id"),
            room_number: row.get("room_number"),
            guest_name: row.get("guest_name"),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
            status: row.get("status"),
        })
        .collect();
    
    Ok(bookings)
}
