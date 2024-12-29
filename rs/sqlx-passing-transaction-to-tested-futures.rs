//! PgPool and PgConnection compatible futures
//!
//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres"] }
//! ```
use sqlx::{Acquire, PgExecutor, PgPool, Postgres};

/// A helper trait to shorten `Acquire<'_, Database = Postgres>`
pub trait PgAcquire<'c>: Acquire<'c, Database = Postgres> {}
impl<'a, T> PgAcquire<'a> for T where T: Acquire<'a, Database = Postgres> {}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPool::connect("postgres:///").await?;
    let mut tx = pool.begin().await?;
    sqlx::query!("CREATE TEMP TABLE _foo (n int)")
        .execute(&mut *tx)
        .await?;

    outer(&mut *tx).await?;

    tx.commit().await?;

    Ok(())
}

/// PgAcquire allows passing a connection to multiple nested futures
async fn outer(db: impl PgAcquire<'_>) -> sqlx::Result<()> {
    let mut conn = db.acquire().await?;
    inner(&mut *conn, 1).await?;
    inner(&mut *conn, 2).await
}

/// PgExecutor is good for simple cases
async fn inner(db: impl PgExecutor<'_>, n: i32) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO _foo VALUES ($1)")
        .bind(n)
        .execute(db)
        .await?;
    Ok(())
}
