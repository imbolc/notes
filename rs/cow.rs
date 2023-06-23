use std::borrow::Cow;

fn main() {
    let borrowed = cut("foo", None);
    let owned = cut("foobar", Some("bar"));

    assert_eq!(borrowed, "foo");
    assert_eq!(owned, "foo");

    // Accessing inner value by reference
    assert_eq!(borrowed.as_ref(), owned.as_ref());
}

/// Removes substrings
fn cut<'a>(s: &'a str, to_cut: Option<&str>) -> Cow<'a, str> {
    if let Some(to_cut) = to_cut {
        Cow::from(s.replace(to_cut, ""))
    } else {
        Cow::from(s)
    }
}
