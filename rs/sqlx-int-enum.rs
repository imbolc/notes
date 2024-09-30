//! Postgres integer <=> Rust enum
//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }
//! ```

#[derive(Debug, PartialEq, sqlx::Type)]
#[repr(i32)]
pub enum Number {
    Zero = 0,
    One = 1,
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;

    let serialized = sqlx::query_scalar!("SELECT $1::int", Number::Zero as _)
        .fetch_one(&db)
        .await?
        .unwrap();
    assert_eq!(serialized, 0);

    let deserialized = sqlx::query_scalar!(r#"SELECT 1 as "n: Number""#)
        .fetch_one(&db)
        .await?
        .unwrap();
    assert_eq!(deserialized, Number::One);

    Ok(())
}
