//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! dotenv = "0.15"
//! sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }
//! ```
use sqlx::Row;

#[derive(Debug, PartialEq, sqlx::Type)]
#[sqlx(type_name = "color")]
#[sqlx(rename_all = "lowercase")]
enum Color {
    Red,
    Green,
    Blue,
}

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    dotenv::dotenv().ok();
    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;
    let mut tx = db.begin().await?;

    sqlx::query("CREATE TYPE color AS ENUM ('red', 'green', 'blue')")
        .execute(&mut *tx)
        .await?;
    sqlx::query("CREATE TABLE colors (color color)")
        .execute(&mut *tx)
        .await?;
    sqlx::query("INSERT INTO colors VALUES ($1), ($2)")
        .bind(Color::Red)
        .bind(Color::Green)
        .execute(&mut *tx)
        .await?;

    // Quering by a single enum works
    assert_eq!(
        sqlx::query("SELECT color FROM colors WHERE color = $1")
            .bind(Color::Red)
            .fetch_one(&mut *tx)
            .await?
            .get::<Color, &str>("color"),
        Color::Red
    );

    assert_eq!(
        sqlx::query("SELECT color FROM colors WHERE color = ANY($1::color[])")
            // Error: the trait `PgHasArrayType` is not implemented for `Color`
            // .bind(&[Color::Blue, Color::Red])
            .bind(&["blue", "red"])
            .fetch_one(&mut *tx)
            .await?
            .get::<Color, &str>("color"),
        Color::Red
    );

    tx.rollback().await?;
    Ok(())
}
