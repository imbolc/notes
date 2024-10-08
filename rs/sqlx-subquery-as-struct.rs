//! SQLx: subquery as a stcuct
//! Add `~/.env`: `DATABASE_URL=postgres:///` for `sqlx` macros to work
//!
//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }
//! ```

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    ocupation: Option<Ocupation>,
}

#[derive(Debug, sqlx::Type)]
struct Ocupation {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;

    let people = sqlx::query_as!(
        Person,
        r#"
        WITH
            people ("id", "name", "ocupation_id") AS (VALUES
                (1, 'Alice', 1),
                (2, 'Bob', null)
            ),
            ocupation ("id", "name") AS (VALUES
                (1, 'Queen')
            )
        SELECT
            p.id as "id!",
            p.name as "name!",
            (
                SELECT (o.id, o.name)
                FROM ocupation o
                WHERE o.id = p.ocupation_id
            ) as "ocupation: Ocupation"
        FROM people p
        "#,
    )
    .fetch_all(&db)
    .await?;

    dbg!(people);
    Ok(())
}
