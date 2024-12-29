//! Postgres text <=> Rust enum
//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! sqlx = { version = "0.8", features = ["runtime-tokio-native-tls", "postgres"] }
//! ```

#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum Answer {
    #[sqlx(rename = "Y")]
    Yes,
    #[sqlx(rename = "N")]
    No,
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;

    let serialized = sqlx::query_scalar!("SELECT $1", Answer::Yes as _)
        .fetch_one(&db)
        .await?
        .unwrap();
    assert_eq!(serialized, "Y");

    let deserialized = sqlx::query_scalar!(
        r#"
        SELECT 'N' as "a: Answer"
        "#
    )
    .fetch_one(&db)
    .await?
    .unwrap();
    assert_eq!(deserialized, Answer::No);

    Ok(())
}
