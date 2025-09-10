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
            // Update booking status to checked_in and assign room
            sqlx::query(
                "UPDATE bookings SET status = $1, room_number = $2 WHERE id = $3"
            )
            .bind(BookingStatus::CheckedIn.to_string())
            .bind(checkin_event.assigned_room)
            .bind(checkin_event.booking_id)
            .execute(&mut **tx)
            .await?;
            
            Ok(())
        }
        Event::BookingCheckedOut(checkout_event) => {
            // Update booking status to checked_out and free up the room
            sqlx::query(
                "UPDATE bookings SET status = $1, room_number = $2 WHERE id = $3"
            )
            .bind(BookingStatus::CheckedOut.to_string())
            .bind(None::<i32>) // Clear room assignment
            .bind(checkout_event.booking_id)
            .execute(&mut **tx)
            .await?;
            
            Ok(())
        }
        Event::BookingCancelled(cancel_event) => {
            // Update booking status to cancelled
            sqlx::query(
                "UPDATE bookings SET status = $1 WHERE id = $2"
            )
            .bind(BookingStatus::Cancelled.to_string())
            .bind(cancel_event.booking_id)
            .execute(&mut **tx)
            .await?;
            
            Ok(())
        }
    }
}