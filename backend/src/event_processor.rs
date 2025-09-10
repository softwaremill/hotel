use crate::db::DbPool;
use crate::models_events::Event;
use anyhow::Result;
use sqlx::{Postgres, Row, Transaction};

pub struct EventProcessor;

impl EventProcessor {
    pub fn new(_pool: DbPool) -> Self {
        Self
    }

    pub async fn process_event_with_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        stream_id: i64,
        event: Event,
    ) -> Result<()> {
        // Get next version for this stream
        let version = self.get_next_version(tx, stream_id).await?;

        // Insert event into events table
        let event_data = serde_json::to_value(&event)?;
        sqlx::query("INSERT INTO events (stream_id, version, data) VALUES ($1, $2, $3)")
            .bind(stream_id)
            .bind(version)
            .bind(event_data)
            .execute(&mut **tx)
            .await?;

        // Apply projection updates for all events
        crate::projections::handle_booking_event(tx, &event).await?;

        Ok(())
    }

    async fn get_next_version(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        stream_id: i64,
    ) -> Result<i32> {
        let row = sqlx::query(
            "SELECT COALESCE(MAX(version), 0) + 1 as next_version FROM events WHERE stream_id = $1",
        )
        .bind(stream_id)
        .fetch_one(&mut **tx)
        .await?;

        Ok(row.get("next_version"))
    }
}
