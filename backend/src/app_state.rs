use crate::db::DbPool;
use crate::event_processor::EventProcessor;
use reqwest::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub event_processor: Arc<EventProcessor>,
    pub http_client: Client,
}