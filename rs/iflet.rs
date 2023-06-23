fn main() {
    let five = Some(5);
    let none: Option<i32> = None;

    // it's Some
    if let Some(num) = five {
        println!("It's {}", num);
    }

    // it's none
    if let Some(_) = none {
        println!("This arm won't work");
    } else {
        println!("It's none there");
    }
}
