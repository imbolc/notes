//! SQLx: collecting a nested composite type into a vec
//! Add `~/.env`: `DATABASE_URL=postgres:///` for `sqlx` macros to work
//!
//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! dotenv = "0.15"
//! sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }
//! ```

#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    follows: Vec<Followee>,
}

#[derive(Debug, sqlx::Type)]
struct Followee {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    dotenv::dotenv().ok();
    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;

    let people = sqlx::query_as!(
        Person,
        r#"
        WITH people ("id", "name", "follows") AS (VALUES
            (1, 'Alice', '{2, 4}'::int[]),
            (2, 'Bob', '{1, 2, 3}'),
            (3, 'Carol', '{2}'),
            (4, 'Dave', '{1}')
        )

        SELECT
            p.id as "id!",
            p.name as "name!",
            (SELECT ARRAY (
                SELECT (f.id, f.name) as "Followee"
                FROM people f
                WHERE f.id = any(p.follows)
            )) as "follows!: Vec<Followee>"
        FROM people p
        "#,
    )
    .fetch_all(&db)
    .await?;

    dbg!(people);
    Ok(())
}
