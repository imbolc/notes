//! Restricting number of simultaneously running futures in Rust
//!
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! tokio = { version = "1", features = ["full"] }
//! tracing = { version = "0.1", features = ["attributes"] }
//! tracing-subscriber = "0.2"
//! futures = "0.3"
//! log = "0.4"
//! ```

use tokio::time::{sleep, Duration};
use tracing::{debug, info, trace_span, Instrument};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env().add_directive("trace".parse()?),
        )
        .init();

    // creates a span manually
    // let span = trace_span!("root span");
    // enters into the span, returns the guard with exits the span when dropped
    // it won't work with `await` though
    // let _enter = span.enter();

    log::debug!("log records converted into events");
    debug!("event");

    futures::join!(instrumented_by_macro(2), instrumented_by_macro(1));

    foo(1)
        .instrument(trace_span!("instrumented by trait", i = 1))
        .await;
    Ok(())
}

#[tracing::instrument]
async fn instrumented_by_macro(i: usize) {
    debug!("fall asleep");
    sleep(Duration::from_secs(i as u64)).await;
    info!("woke up");
}

async fn foo(i: usize) {
    debug!("foo fall asleep");
    sleep(Duration::from_secs(i as u64)).await;
    info!("foo woke up");
}
