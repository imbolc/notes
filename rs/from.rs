// Implementing From trait

struct Foo<'a> {
    name: &'a str,
}

impl<'a> From<&'a str> for Foo<'a> {
    fn from(s: &'a str) -> Self {
        Foo { name: s.into() }
    }
}

fn main() {
    let foo: Foo = "foo".into();
    assert_eq!(foo.name, "foo");
}
