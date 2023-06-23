//! Implementing a custom error type
//!
//! The example follows reasonings of https://kazlauskas.me/entries/errors.html
//!
//! 1. To remove ambiguity each falliable expression should have a unique variant or parameter
//! 2. Don’t implement `From<OtherErrorType>` for `Error` as it leads to violation of 1
//!
//! ```cargo
//! [dependencies]
//! thiserror = "1"
//! ```
use std::io::{self, Read};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("can't open file: {1}")]
    OpenFile(#[source] io::Error, &'static str),
    #[error("can't read config")]
    ReadConfig(#[source] io::Error),
}

type Result<T> = std::result::Result<T, Error>;

fn main() {
    run().map_err(|e| eprintln!("{}", error_chain(&e))).ok();
}

fn run() -> Result<()> {
    let path = "/tmp/foo";
    let mut data = Vec::new();

    let mut file = std::fs::File::open(path).map_err(|e| Error::OpenFile(e, path))?;
    file.read_to_end(&mut data).map_err(Error::ReadConfig)?;
    Ok(())
}

/// A helper to format error with its source chain
fn error_chain(e: &impl std::error::Error) -> String {
    let mut s = e.to_string();
    let mut current = e.source();
    if current.is_some() {
        s.push_str("\nCaused by:");
    }
    while let Some(cause) = current {
        s.push_str(&format!("\n\t{}", cause));
        current = cause.source();
    }
    s
}
