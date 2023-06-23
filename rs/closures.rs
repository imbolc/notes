use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    // simulated_expensive_calculation(1);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    // value: Option<u32>,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    // TODO: Try modifying Cacher to hold a hash map rather than a single value.
    // The keys of the hash map will be the arg values that are passed in,
    // and the values of the hash map will be the result of calling the closure on that key.
    fn value(&mut self, arg: u32) -> u32 {
        if self.values.contains_key(&arg) {
            self.values.get(&arg).unwrap().clone()
        } else {
            let v = (self.calculation)(arg);
            self.values.insert(arg, v);
            v
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {:?} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
