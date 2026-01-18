//! Polymorphysm in Rust

fn main() {
    // it accepts anythins printable
    print_items(&[&"foo", &1, &true]);

    // this function can accept arguments of different types
    into_enum(1);
    into_enum(true);

    // this one can accept tuples of different types
    print(('a',));
    print((1, true));
}

// === Traits
// We can accept anything as a trait object
fn print_items(items: &[&dyn std::fmt::Display]) {
    for item in items {
        println!("{item}");
    }
}

// === Enums
// Assume we'd like a function to take different types

// First let's put them into an enum
enum E {
    Int(i32),
    Bool(bool),
}

// Then we make converters from inner types to the enum
impl From<i32> for E {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}
impl From<bool> for E {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

// And now we can define our function like this
fn into_enum(x: impl Into<E>) {
    let x = x.into();
    match x {
        E::Int(i) => println!("Got integer: {i}"),
        E::Bool(b) => println!("Got boolean: {b}"),
    }
}

// === Tuples
// Now assume we'd like a function to take sequence of different types

// Fist we define a trait with the function logic
trait Printable {
    fn print(&self);
}

// Now we implement the trait to tuples of different sizes
impl<T1> Printable for (T1,)
where
    T1: std::fmt::Display,
{
    fn print(&self) {
        println!("{}", self.0);
    }
}
impl<T1, T2> Printable for (T1, T2)
where
    T1: std::fmt::Display,
    T2: std::fmt::Display,
{
    fn print(&self) {
        println!("{} and {}", self.0, self.1);
    }
}

// And now we define the function itself
fn print(x: impl Printable) {
    x.print()
}
