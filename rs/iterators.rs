use std::fmt::Display;

fn main() {
    print_iterator(['a', 'b']);
    print_iterator(&['c', 'd']);
    print_iterator(vec!['e', 'f']);

    print_iterator(powers_of_two());
    print_iterator(PowerOfTwo::default());
}

/// Accepting iterator - `IntoIterator` let's you pass anything convertable into iterator
/// including arrays, slices and vectors
fn print_iterator(items: impl IntoIterator<Item = impl Display>) {
    for item in items.into_iter() {
        println!("{item}");
    }
}

/// Returning an iterator
fn powers_of_two() -> impl Iterator<Item = usize> {
    (0..3).map(|i| 2_usize.pow(i))
}

/// Implementing iterator
#[derive(Default)]
struct PowerOfTwo {
    exp: u32,
}

impl Iterator for PowerOfTwo {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exp >= 3 {
            None
        } else {
            let n = 2_usize.pow(self.exp);
            self.exp += 1;
            Some(n)
        }
    }
}
