//! Processing errors with `anyhow`
//!
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! ```
use anyhow::{Context, Result};
use std::io::Read;

fn main() {
    run().map_err(|e| eprintln!("{:?}", e)).ok();
}

fn run() -> Result<()> {
    let path = "/tmp/foo";
    let mut data = Vec::new();

    let mut file = std::fs::File::open(path).with_context(|| format!("open `{path}`"))?;
    file.read_to_end(&mut data)
        .with_context(|| format!("read `{path}`"))?;
    Ok(())
}
