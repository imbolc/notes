//! Serde
//!
//! ```cargo
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```

#[derive(serde::Serialize)]
struct Foo {
    // Don't serialize field if it's None
    #[serde(skip_serializing_if = "Option::is_none")]
    opt: Option<String>,
}

#[derive(serde::Serialize)]
#[serde(untagged)]
enum UntaggedEnum {
    Empty,
    Full { i: i32 },
}

fn main() {
    assert_eq!(serde_json::to_string(&Foo { opt: None }).unwrap(), "{}");

    assert_eq!(serde_json::to_string(&UntaggedEnum::Empty).unwrap(), "null");
    assert_eq!(
        serde_json::to_string(&UntaggedEnum::Full { i: 1 }).unwrap(),
        r#"{"i":1}"#
    );
}
