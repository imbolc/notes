//! Scoped threads allow borrowing of local variables
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    thread::scope(|s| {
        s.spawn(|| {
            for n in &data {
                println!("{:?}: {}", thread::current().id(), n);
            }
        });
        s.spawn(|| {
            for n in &data {
                println!("{:?}: {}", thread::current().id(), n);
            }
        });
    });

    println!("All scoped threads are joined");
}
