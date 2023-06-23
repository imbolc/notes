//! Restricting number of simultaneously running futures in Rust
//!
//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! futures = "0.3"
//! rand = "0.8"
//! ```

use futures::{stream, StreamExt};
use tokio::time::{sleep, Duration};

const CONCURRENCY_LIMIT: usize = 2;

#[tokio::main]
async fn main() {
    let data = { 1..6 }
        .map(|n| (n, Duration::from_micros(rand::random::<u16>() as u64 * 10)))
        .collect::<Vec<_>>();

    println!("=== Limited concurrency");
    let jobs = data.clone().into_iter().map(|(n, d)| job(n, d));
    stream::iter(jobs)
        .for_each_concurrent(CONCURRENCY_LIMIT, |job| async move {
            println!("{}", job.await);
        })
        .await;

    println!("=== Iterating through results as they're ready");
    let jobs = data.clone().into_iter().map(|(n, d)| job(n, d));
    let mut buffered = stream::iter(jobs).buffer_unordered(CONCURRENCY_LIMIT);
    while let Some(result) = buffered.next().await {
        println!("{}", result);
    }

    println!("=== Collecting results");
    let jobs = data.clone().into_iter().map(|(n, d)| job(n, d));
    let results = stream::iter(jobs)
        .buffer_unordered(CONCURRENCY_LIMIT)
        .collect::<Vec<_>>()
        .await;
    dbg!(results);
}

async fn job(n: usize, delay: Duration) -> String {
    println!("Job {} fall asleep for {:?}", n, delay);
    sleep(delay).await;
    format!("Job {} finished", n)
}
