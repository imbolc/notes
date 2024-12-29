//! Processing errors with `anyhow`
//!
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! ```
use std::io::Read;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path = "/tmp/foo";
    let mut data = Vec::new();

    let mut file = std::fs::File::open(path).context("open")?;
    let _content = file.read_to_end(&mut data).context("read")?;
    Ok(())
}
