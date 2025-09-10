use crate::models::BookingStatus;
use crate::models_events::Event;
use anyhow::Result;
use sqlx::{Postgres, Transaction};

pub async fn handle_booking_event(tx: &mut Transaction<'_, Postgres>, event: &Event) -> Result<()> {
    match event {
        Event::BookingCreated(booking_event) => {
            // Insert booking into projections table
            sqlx::query(
                "INSERT INTO bookings (id, hotel_id, room_number, guest_name, start_time, end_time, status) 
                 VALUES ($1, $2, $3, $4, $5, $6, $7)"
            )
            .bind(booking_event.booking_id)
            .bind(booking_event.hotel_id)
            .bind(None::<i32>) // room_number is null until check-in
            .bind(&booking_event.guest_name)
            .bind(booking_event.start_time)
            .bind(booking_event.end_time)
            .bind(BookingStatus::Confirmed.to_string())
            .execute(&mut **tx)
            .await?;
            
            Ok(())
        }
        Event::BookingCheckedIn(checkin_event) => {
            // Update booking status to checked_in
            sqlx::query(
                "UPDATE bookings SET status = $1 WHERE id = $2"
            )
            .bind(BookingStatus::CheckedIn.to_string())
            .bind(checkin_event.booking_id)
            .execute(&mut **tx)
            .await?;
            
            Ok(())
        }
    }
}