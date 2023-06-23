use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    println!("{:?}", map.get(&field_name));

    let nums = vec![1, 2, 3, 1, 2, 3, 1];
    println!("mode of {:?} is {}", nums, mode(&nums));
}

fn mode(nums: &Vec<i32>) -> i32 {
    // Returns mode (the value that occurs most often)
    let mut counter = HashMap::new();
    for num in nums {
        *counter.entry(num).or_insert(0) += 1;
    }
    println!("{:?}", counter);
    let mut mode = 0;
    let mut mode_freq = 0;
    for (k, v) in counter {
        if v > mode_freq {
            mode = *k;
            mode_freq = v;
        }
    }
    mode
}
