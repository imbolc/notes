fn main() {
    let lst = vec![2, 3, 1];
    println!("The largest of {:?} is {}", lst, largest(&lst));

    let lst = vec!['b', 'c', 'a'];
    println!("The largest of {:?} is {}", lst, largest(&lst));

    let a = Point { x: "hello ", y: 10 };
    let b = Point {
        x: true,
        y: "world",
    };
    print!("{:?}", a.mixup(b));

    dbg!(add(1, 2), add(1., 1.5));
}

// `<T>` after function definition declares generic type, so we can use it as a parameter
// we should declare `PartialOrd` trate as we're comparing `T`
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// `<T>` after `impl` telling that type `T` is generic parameters used in struct
impl<T, U> Point<T, U> {
    // `V` and `W` some other generic parameters different from `T` and `U`
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T> + std::fmt::Debug,
{
    a + b
}
