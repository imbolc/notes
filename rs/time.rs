// Elapsed time

fn main() {
    let start = std::time::Instant::now();
    std::thread::sleep(std::time::Duration::from_nanos(1));
    println!("Took: {} secs", start.elapsed().as_secs_f64());

    let (timer, times, start) = ("my_timer", 1000, std::time::Instant::now());
    for _ in 0..times {
        std::thread::sleep(std::time::Duration::from_nanos(1));
    }
    let took = start.elapsed().as_secs_f64();
    let per_sec = times as f64 / took;
    println!("{times}x {timer} took: {took} secs, {per_sec} / sec ");
}
