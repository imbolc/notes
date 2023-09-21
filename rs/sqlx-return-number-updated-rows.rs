//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }
//! ```

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let db = sqlx::PgPool::connect("postgres:///").await?;
    let mut tnx = db.begin().await?;

    sqlx::query("CREATE TABLE foo (id int)")
        .execute(&mut *tnx)
        .await?;

    let affected = sqlx::query("UPDATE foo SET id = 2")
        .execute(&mut *tnx)
        .await?
        .rows_affected();
    assert_eq!(affected, 0);

    sqlx::query("INSERT INTO foo VALUES ('1')")
        .execute(&mut *tnx)
        .await?;

    let affected = sqlx::query("UPDATE foo SET id = 2")
        .execute(&mut *tnx)
        .await?
        .rows_affected();
    assert_eq!(affected, 1);

    tnx.rollback().await?;

    Ok(())
}
