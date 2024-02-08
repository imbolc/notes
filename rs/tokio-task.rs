//! Restricting number of simultaneously running futures in Rust
//!
//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! futures = "0.3"
//! rand = "0.8"
//! ```

#[tokio::main]
async fn main() {
    let (_first, _second) = tokio::join! {
        tokio::task::spawn(sleep(2)),
        tokio::task::spawn(sleep(1)),
    };
}

async fn sleep(secs: u64) {
    println!("Sleep for {} seconds", secs);
    tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
    println!("Woke up after {} seconds", secs);
}
