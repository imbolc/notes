//! Lock for count in Postgres
//!
//! Add `~/.env`: `DATABASE_URL=postgres:///` for `sqlx` macros to work
//!
//! ```cargo
//! [dependencies]
//! futures = "0.3"
//! sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }
//! tokio = { version = "1", features = ["full"] }
//! ```
use futures::future::join_all;
use sqlx::{
    postgres::{PgPool, PgRow},
    Row,
};

const MAX_INGREDIENTS_PER_RECIPE: i64 = 3;

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let db = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;
    create_tables(&db).await?;

    let num_recipes = 3;

    // create a few of recipes
    for recipe_id in 1..=num_recipes {
        add_recipe(&db, recipe_id).await?;
    }

    // try adding ingredients exciding per-recipe limit
    let mut handles = Vec::new();
    for recipe_id in 1..=num_recipes {
        for ingredient_id in 1..=5 {
            handles.push(tokio::task::spawn(add_ingredient(
                db.clone(),
                recipe_id,
                ingredient_id,
            )))
        }
    }
    join_all(handles).await;

    println!();
    for recipe_id in 1..=num_recipes {
        println!(
            "Ingredients in recipe #{recipe_id}: {}",
            ingredients_per_recipe(&db, recipe_id).await?
        );
    }

    drop_tables(&db).await?;
    Ok(())
}

async fn add_ingredient(db: PgPool, recipe_id: i32, ingredient_id: i32) -> sqlx::Result<()> {
    let mut tx = db.begin().await?;

    let _ = sqlx::query("SELECT id FROM recipe WHERE id = $1 FOR UPDATE")
        .bind(recipe_id)
        .fetch_optional(&mut *tx)
        .await?;

    let limit_exceeded = sqlx::query("SELECT count(*) >= $2 FROM ingredient WHERE recipe_id = $1")
        .bind(recipe_id)
        .bind(MAX_INGREDIENTS_PER_RECIPE)
        .map(|r: PgRow| r.get(0))
        .fetch_one(&mut *tx)
        .await?;

    if limit_exceeded {
        println!("recipe #{recipe_id}, ingredient #{ingredient_id}: limit exceeded");
        return Ok(());
    }

    sqlx::query("INSERT INTO ingredient (recipe_id, ingredient_id) VALUES ($1, $2)")
        .bind(recipe_id)
        .bind(ingredient_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}

async fn ingredients_per_recipe(db: &PgPool, recipe_id: i32) -> sqlx::Result<i64> {
    sqlx::query("SELECT count(*) FROM ingredient WHERE recipe_id = $1")
        .bind(recipe_id)
        .map(|r: PgRow| r.get(0))
        .fetch_one(db)
        .await
}

async fn add_recipe(db: &PgPool, id: i32) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO recipe (id) VALUES ($1)")
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}

async fn create_tables(db: &PgPool) -> sqlx::Result<()> {
    sqlx::query(
        "
        CREATE TABLE recipe (
            id int PRIMARY KEY
        )
        ",
    )
    .execute(db)
    .await?;
    sqlx::query(
        "
        CREATE TABLE ingredient (
            recipe_id int NOT NULL REFERENCES recipe (id),
            ingredient_id int,
            PRIMARY KEY (recipe_id, ingredient_id)
        )
        ",
    )
    .execute(db)
    .await?;
    Ok(())
}

async fn drop_tables(db: &PgPool) -> sqlx::Result<()> {
    sqlx::query("DROP TABLE ingredient").execute(db).await?;
    sqlx::query("DROP TABLE recipe").execute(db).await?;
    Ok(())
}
