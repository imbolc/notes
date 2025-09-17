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
use axum::{response::Html, routing::{get, post}, serve, Router};
use tokio::net::TcpListener;
use std::collections::BTreeMap;
use serde_qs::web::QsForm;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Order {
    /// Order items <id, quantity>
    items: BTreeMap<i32, u32>,
}

#[tokio::main]
async fn main() {
    let app = app();

    serve(TcpListener::bind("127.0.0.1:3000").await.unwrap(), app).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(get_form))
        .route("/submit", post(post_form))
}

async fn get_form() -> Html<&'static str> {
    "
    <form action='/submit' method='post'>
        Item id=1:
        <input type='number' name='items[1]' value='11'>
        <br>
        Item id=2:
        <input type='number' name='items[2]' value='22'>
        <br>
        <button type='submit'>Submit</button>
    </form>
    ".into()
}

async fn post_form(QsForm(order): QsForm<Order>) -> axum::Json<Order> {
    axum::Json(order)
}

#[cfg(test)]
#[tokio::test]
async fn works() {
    let server = axum_test::TestServer::new(app()).unwrap();

    let response = server
        .post("/submit")
        .form(&[("items[1]", "3"), ("items[2]", "5")])
        .await;

    response.assert_status_success();
    let order: Order = response.json();
    let mut expected = BTreeMap::new();
    expected.insert(1, 3);
    expected.insert(2, 5);
    assert_eq!(order.items, expected);
}
