//! Using `serde_qs` to parse multi-dimensional form with Axum
//!
//! ```cargo
//! edition = "2024"
//! [dependencies]
//! axum = { version = "0.8" }
//! serde = { version = "1", features = ["derive"] }
//! serde_qs = { version = "1.0.0-rc.3", features = ["axum"] }
//! tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
//! ```
#![allow(dead_code)]
use axum::{response::Html, routing::{get, post}, serve, Router};
use serde_qs::web::QsForm;

#[derive(Debug, serde::Deserialize)]
struct Item {
    name: String,
    quantity: u32,
}

#[derive(Debug, serde::Deserialize)]
struct Order {
    items: Vec<Item>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_form))
        .route("/submit", post(post_form));

    serve(tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap(), app).await.unwrap();
}

async fn get_form() -> Html<&'static str> {
    "
    <form action='/submit' method='post'>
        <input type='text' name='items[0][name]' value='Apple'>
        <input type='number' name='items[0][quantity]' value='1'>
        <br>
        <input type='text' name='items[1][name]' value='Orange'>
        <input type='number' name='items[1][quantity]' value='2'>
        <br>
        <button type='submit'>Submit</button>
    </form>
    ".into()
}

async fn post_form(QsForm(order): QsForm<Order>) -> String {
    format!("{order:#?}")
}
