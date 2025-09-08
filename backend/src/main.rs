use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;
use std::env;

mod models;
mod handlers;
mod db;

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

    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/hotels/:id", get(handlers::get_hotel))
        .route("/hotels/:id/rooms", get(handlers::get_rooms))
        .route("/hotels/:id/bookings", get(handlers::get_bookings))
        .route("/hotels/:id/bookings", post(handlers::create_booking))
        .with_state(pool)
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Hotel backend server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}