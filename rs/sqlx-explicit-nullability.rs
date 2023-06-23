//! SQLx: explicit nullability
//! Add `~/.env`: `DATABASE_URL=postgres:///` for `sqlx` macros to work
//!
//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! dotenv = "0.15"
//! sqlx = { version = "0.6", features = ["runtime-tokio-native-tls", "postgres"] }
//! ```
//!
#[tokio::main]
async fn main() -> sqlx::Result<()> {
    dotenv::dotenv().ok();
    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;

    // Sometimes SQLx can't determine nullaility, so we have to be explicit

    // Force `NOT NULL` with `foo as "foo!"`
    assert_eq!(
        sqlx::query!(r#"SELECT 1 as "id!""#)
            .fetch_one(&db)
            .await?
            .id,
        1
    );

    // Force nullable with `foo as "foo?"`
    assert_eq!(
        sqlx::query!(r#"SELECT 1 as "id?""#)
            .fetch_one(&db)
            .await?
            .id,
        Some(1)
    );

    Ok(())
}
