//! SQLx: typed json
//!
//! ```cargo
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres", "json"] }
//! tokio = { version = "1", features = ["full"] }
//! ```
use sqlx::types::Json;

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
struct Person {
    name: String,
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;

    let deserialized = sqlx::query_scalar!(
        r#"
        SELECT '{"name": "Alice"}'::jsonb as "person!: Json<Person>"
        "#,
    )
    .fetch_one(&db)
    .await?;
    assert_eq!(
        deserialized.0,
        Person {
            name: "Alice".into()
        }
    );

    let serialized = sqlx::query_scalar!(
        r#"
        SELECT $1::jsonb::text
        "#,
        Json(Person { name: "Bob".into() }) as _
    )
    .fetch_one(&db)
    .await?
    .unwrap();
    assert_eq!(serialized, r#"{"name": "Bob"}"#);

    Ok(())
}
