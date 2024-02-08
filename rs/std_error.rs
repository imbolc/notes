//! Implementing a custom error type
//!
//! This example using `thiserror` crate is in `this_error.rs`

use std::{
    error, fmt,
    io::{self, Read},
};

#[derive(Debug)]
pub enum Error {
    OpenFile(io::Error, &'static str),
    ReadConfig(io::Error),
}

fn main() {
    println!("{}", error_chain("string error"));
    run().map_err(|e| eprintln!("{}", error_chain(e))).ok();
}

fn run() -> Result<(), Error> {
    let path = "/tmp/foo";
    let mut data = Vec::new();

    let mut file = std::fs::File::open(path).map_err(|e| Error::OpenFile(e, path))?;
    file.read_to_end(&mut data).map_err(Error::ReadConfig)?;
    Ok(())
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::OpenFile(_, path) => write!(f, "can't open file: {}", path),
            Error::ReadConfig(_) => f.write_str("can't read config"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            OpenFile(ref e, _) => Some(e),
            ReadConfig(ref e) => Some(e),
        }
    }
}

/// A helper to format error with its source chain
/// Boxing allows accepting strings as errors
fn error_chain(e: impl Into<Box<dyn std::error::Error + 'static>>) -> String {
    let e = e.into();
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
