//! Passing SQLx transaction between functions
//!
//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres"] }
//! ```
use sqlx::Acquire;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = sqlx::PgPool::connect("postgres:///").await?;

    let mut con = pool.acquire().await?;

    sqlx::query!("CREATE TEMP TABLE _foo (n int)")
        .execute(&mut *con)
        .await?;

    let mut tx = con.begin().await?;
    outer(&mut tx).await?;
    println!(
        "Fetched inside transaction: {:?}",
        fetch_inserted(&mut *tx).await?
    );
    tx.commit().await?;

    println!(
        "Fetched outside transaction: {:?}",
        fetch_inserted(&mut *con).await?
    );
    Ok(())
}

/// Functions meant to change data in a transaction explicitly accept
/// `PgTransaction`
async fn outer(tx: &mut sqlx::PgTransaction<'_>) -> sqlx::Result<()> {
    inner(tx, 1).await?;

    inner(tx, 2).await
}

async fn inner(tx: &mut sqlx::PgTransaction<'_>, n: i32) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO _foo VALUES ($1)")
        .bind(n)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

/// Functions that meant to be compatible with both transactional and atomic
/// requests can accept `PgExecutor`
async fn fetch_inserted(db: impl sqlx::PgExecutor<'_>) -> sqlx::Result<Vec<i32>> {
    sqlx::query_scalar::<_, i32>("SELECT n FROM _foo")
        .fetch_all(db)
        .await
}
