use sqlx::{PgPool, Pool, Postgres};
use anyhow::Result;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}