//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }
//! ```
use sqlx::Row;

#[derive(Debug, PartialEq, sqlx::Type)]
#[sqlx(type_name = "color", rename_all = "lowercase")]
enum Color {
    Red,
    Green,
    Blue,
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;
    let mut tx = db.begin().await?;

    sqlx::query!("CREATE TYPE color AS ENUM ('red', 'green', 'blue')")
        .execute(&mut *tx)
        .await?;
    sqlx::query!("CREATE TABLE colors (color color)")
        .execute(&mut *tx)
        .await?;
    sqlx::query("INSERT INTO colors VALUES ($1), ($2)")
        .bind(Color::Red)
        .bind(Color::Green)
        .execute(&mut *tx)
        .await?;

    // The macro won't compile as we create table on the fly,
    // but in RL the explicit typing `Color::Red as _` is necessary
    // sqlx::query!("INSERT INTO colors VALUES ($1)", Color::Red as _)
    //     .execute(&mut *tx)
    //     .await?;

    // Querying by a single enum works
    assert_eq!(
        sqlx::query("SELECT color FROM colors WHERE color = $1")
            .bind(Color::Red)
            .fetch_one(&mut *tx)
            .await?
            .get::<Color, _>("color"),
        Color::Red
    );

    // But for querying by an array we have to implement this trait
    impl sqlx::postgres::PgHasArrayType for Color {
        fn array_type_info() -> sqlx::postgres::PgTypeInfo {
            sqlx::postgres::PgTypeInfo::with_name("_color")
        }
    }

    assert_eq!(
        sqlx::query("SELECT color FROM colors WHERE color = ANY($1::color[])")
            .bind(&[Color::Blue, Color::Red])
            .fetch_one(&mut *tx)
            .await?
            .get::<Color, _>("color"),
        Color::Red
    );

    tx.rollback().await?;
    Ok(())
}
