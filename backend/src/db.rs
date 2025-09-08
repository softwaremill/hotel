use anyhow::Result;
use sqlx::{PgPool, Pool, Postgres, migrate::MigrateError};

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
