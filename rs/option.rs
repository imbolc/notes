fn main() {
    let five = Some(5);
    let none: Option<i32> = None;
    check(five);
    check(none);
}

fn check(num: Option<i32>) {
    dbg!(num);
    match num {
        None => println!("{:?} is None", num),
        Some(n) => println!("{:?} is a number", n),
    };
}
