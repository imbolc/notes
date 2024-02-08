//! Scoped threads allow borrowing of local variables
use std::thread;

fn main() {
    let data: Vec<usize> = (1..=5).collect();

    thread::scope(|s| {
        for _ in 0..5 {
            s.spawn(|| {
                for n in &data {
                    println!("{:?}: {}", thread::current().id(), n);
                }
            });
        }
    });

    println!("All scoped threads are joined");
}
