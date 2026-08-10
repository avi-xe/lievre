use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

pub async fn create_pool(database_url: &str) -> anyhow::Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .busy_timeout(std::time::Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    // Enable WAL mode
    sqlx::query("PRAGMA journal_mode=WAL")
        .execute(&pool)
        .await?;

    // Enable foreign keys
    sqlx::query("PRAGMA foreign_keys=ON")
        .execute(&pool)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::migrate!("../../migrations")
        .run(pool)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_pool() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        
        // Verify pool is working
        sqlx::query("SELECT 1")
            .execute(&pool)
            .await
            .unwrap();
    }
}
