//! SQLx: typed json
//! Add `~/.env`: `DATABASE_URL=postgres:///` for `sqlx` macros to work
//!
//! ```cargo
//! [dependencies]
//! dotenv = "0.15"
//! serde = { version = "1", features = ["derive"] }
//! sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres", "json"] }
//! tokio = { version = "1", features = ["full"] }
//! ```

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
struct MyJson {
    id: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct MyRow {
    my_json: sqlx::types::Json<MyJson>,
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    dotenv::dotenv().ok();
    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;

    let rows = sqlx::query_as!(
        MyRow,
        r#"
        WITH
            my_row ("my_json") AS (VALUES
                ('{"id": 1}'::jsonb)
            )
        SELECT my_json as "my_json!: sqlx::types::Json<MyJson>"
        FROM my_row
        "#,
    )
    .fetch_all(&db)
    .await?;

    dbg!(rows);
    Ok(())
}
