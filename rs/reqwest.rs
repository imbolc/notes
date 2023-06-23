//! Rust - http calls with `reqwest`
//!
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! reqwest = { version = "0.11", features = ["gzip", "brotli", "json"] }
//! serde = "1"
//! serde_json = "1"
//! tokio = { version = "1", features = ["full"] }
//! ```
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let json: serde_json::Value = get_json(
        "https://httpbin.org/anything",
        [("foo", "bar"), ("baz", "ы")],
    )
    .await?;
    dbg!(&json);
    assert_eq!(json.pointer("/args/foo").unwrap().as_str().unwrap(), "bar");

    let json: serde_json::Value = get_json("https://httpbin.org/anything", []).await?;
    assert!(json
        .pointer("/args")
        .unwrap()
        .as_object()
        .unwrap()
        .is_empty(),);
    Ok(())
}

/// Fetches json from the url with given params, pass `[]` if no params required
pub async fn get_json<'a, U, P, R>(url: U, query: P) -> Result<R>
where
    U: AsRef<str>,
    P: AsRef<[(&'a str, &'a str)]>,
    R: serde::de::DeserializeOwned,
{
    let url = url.as_ref();
    let query = query.as_ref();
    let resp = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::ACCEPT, "application/json")
        .query(query)
        .send()
        .await?
        .error_for_status()?;
    let url = resp.url().to_string();
    Ok(resp.json().await.context(url)?)
}
