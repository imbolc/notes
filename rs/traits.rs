trait Greetable {
    // Defining a trait method with a default implementation
    fn hi(&self) -> &str {
        "Hi"
    }
}

struct S;
impl Greetable for S {}

// rust will preform monomorphization for each particular type of `s`
// `impl` is a sugar for `<T: Greetable>(s: &T)`
fn static_dispatch(s: &impl Greetable) -> &str {
    s.hi()
}

// trait objects: heap-allocation instead of monomorphization
fn dynamic_dispatch(s: &dyn Greetable) -> &str {
    s.hi()
}

fn main() {
    let s = S;
    assert_eq!(s.hi(), "Hi"); // Calling trait methods on a struct
    assert_eq!(Greetable::hi(&s), "Hi"); // Another way of doing the same

    assert_eq!(static_dispatch(&s), "Hi");
    assert_eq!(dynamic_dispatch(&s), "Hi");
}
