//! SQLx row streaming. Data is read asynchronously from the database and
//! decoded on demand.
//!
//! Add `~/.env`: `DATABASE_URL=postgres:///` for `sqlx` macros to work
//!
//! ```cargo
//! [dependencies]
//! futures = "0.3"
//! tokio = { version = "1", features = ["full"] }
//! sqlx = { version = "0.8", features = ["runtime-tokio-native-tls", "postgres"] }
//! ```
use futures::TryStreamExt;

#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;

    let mut stream = sqlx::query_as!(
        Person,
        r#"
        WITH people ("id", "name") AS (VALUES
            (1, 'Alice'),
            (2, 'Bob'),
            (3, 'Carol'),
            (4, 'Dave')
        )
        SELECT
            p.id as "id!",
            p.name as "name!"
        FROM people p
        "#,
    )
    .fetch(&db);

    while let Some(person) = stream.try_next().await? {
        dbg!(person);
    }

    Ok(())
}
