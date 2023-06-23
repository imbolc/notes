//! Rust - allow a parameter to be generic over anything stringy

fn main() {
    into_string("foo");
    into_string(String::from("foo"));

    as_ref("foo");
    as_ref(String::from("foo"));
}

/// Should be used when we need an owned string inside
fn into_string(s: impl Into<String>) {
    let _my_string = s.into();
}

/// Should be used when we only need a `&str` inside
fn as_ref(s: impl AsRef<str>) {
    let _my_str = s.as_ref();
}
