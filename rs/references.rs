use std::ops::Deref;

fn main() {
    let a = 5;

    let b = &a;
    assert_eq!(5, *b);

    let c = Box::new(a);
    assert_eq!(5, *c);

    let d = MyBox::new(a);
    assert_eq!(5, *d);
    // acutally runs behind the scene
    assert_eq!(5, *(d.deref()));
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
