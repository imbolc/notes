fn main() {
    let list: Vec<_> = (0..10).collect();

    for (index, item) in list.iter().enumerate() {
        println!("{}: {}", index, item);
    }

    for i in Counter::new().skip(1).filter(|x| x % 2 == 0) {
        println!("{}", i);
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.count > 5 {
            None
        } else {
            self.count += 1;
            Some(self.count)
        }
    }
}
