/// Default trait provides default values for a struct

#[derive(Debug, PartialEq)]
enum Foo {
    Foo,
}

// You can implement the trait manually defining default values
impl Default for Foo {
    fn default() -> Self {
        Foo::Foo
    }
}

// Or you can derive the trait for a struct if all its fields implement the trait
#[derive(Debug, Default, PartialEq)]
struct Bar {
    foo: Foo,
    bar: bool,
    baz: Option<f32>,
}

fn main() {
    assert_eq!(
        Bar {
            bar: true,
            ..Default::default()
        },
        Bar {
            foo: Foo::Foo,
            bar: true,
            baz: None
        }
    );
}
