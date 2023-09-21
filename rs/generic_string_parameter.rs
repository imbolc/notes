//! Rust - allow a parameter to be generic over anything stringy
use std::borrow::Cow;

type CowStr = Cow<'static, str>;

fn main() {
    let cow_str: CowStr = "cow".into();

    into_string("foo");
    into_string(String::from("foo"));
    into_string(CowStr::from("cow"));

    as_ref("foo");
    as_ref(String::from("foo"));
    as_ref(&CowStr::from("cow"));

    cow("foo");
    cow(String::from("foo"));
    cow(cow_str);
}

/// Should be used when we need an owned string inside
fn into_string(s: impl Into<String>) {
    let _my_string = s.into();
}

/// Should be used when we only need a `&str` inside
fn as_ref(s: impl AsRef<str>) {
    let _my_str = s.as_ref();
}

/// Used mostly in structs to avoid converting `&'static str` int Strings
fn cow(s: impl Into<CowStr>) {
    let _s = s.into();
}
