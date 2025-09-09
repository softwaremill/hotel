use axum::{
    Router,
    routing::{get, post},
};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod app_state;
mod db;
mod event_processor;
mod handlers;
mod models;
mod projections;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get database URL from environment or use default for development
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/hotel".to_string());

    println!("Connecting to database...");
    let pool = db::create_pool(&database_url).await?;

    println!("Running database migrations...");
    db::run_migrations(&pool).await?;
    println!("Migrations completed successfully");

    // Set up event processor
    let event_processor = Arc::new(event_processor::EventProcessor::new(pool.clone()));

    // Create app state
    let app_state = app_state::AppState {
        db_pool: pool,
        event_processor,
    };

    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/hotels/:id", get(handlers::get_hotel))
        .route("/hotels/:id/bookings", get(handlers::get_bookings))
        .route("/hotels/:id/bookings", post(handlers::create_booking))
        .with_state(app_state)
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Hotel backend server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
