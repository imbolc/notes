//! HTMx loading-states extension
//!
//! The example of using https://github.com/bigskysoftware/htmx-extensions/tree/main/src/loading-states
//!
//! ```cargo
//! [dependencies]
//! axum = { version = "0.7" }
//! axum-htmx = "0.5"
//! maud = { version = "0.26", features = ["axum"] }
//! tokio = { version = "1", features = ["macros"] }
//! ```
use axum::{routing::get, Router};
use maud::{html, Markup, PreEscaped};
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

const ORIGIN: &str = "localhost:3000";
static RESPONSE_ID: AtomicUsize = AtomicUsize::new(1);

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind(ORIGIN).await.unwrap();
    println!("Listening on http://{ORIGIN}");
    axum::serve(listener, app).await.unwrap();
}

async fn handler(hx_request: axum_htmx::HxRequest) -> Markup {
    if hx_request.0 {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let resp_id = RESPONSE_ID.fetch_add(1, Ordering::SeqCst);
        return html! { "hx-response-" (resp_id) };
    }
    html! {
        body hx-ext="loading-states" {
            p data-loading-states {
                button
                    hx-get=""
                    hx-target="#target-1"
                    hx-disabled-elt="this"
                    data-loading-aria-busy
                    data-loading-target="[data-loading-on-1]"
                    data-loading-on-1
                    { "get-1" }
            }

            p data-loading-states {
                button
                    hx-get=""
                    hx-target="#target-2"
                    hx-disabled-elt="this"
                    data-loading-aria-busy
                    data-loading-target="[data-loading-on-2]"
                    data-loading-on-2
                    { "get-2" }
            }

            fieldset data-loading-on-1 {
                legend { "target-1"}
                span id="target-1" { "not loaded" }
            }

            fieldset data-loading-on-2 {
                legend { "target-2"}
                span id="target-2" { "not loaded" }
            }
        }

        style { (PreEscaped("[aria-busy='true'] { background: lightgreen }")) }
        script src="https://unpkg.com/htmx.org@2.0.0-beta4/dist/htmx.min.js" {}
        script src="https://unpkg.com/htmx-ext-loading-states@2.0.0/loading-states.js" {}

    }
}
