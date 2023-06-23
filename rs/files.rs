// Working with files in Rust
use std::fs;
use std::io::prelude::*; // for `f.lines()`
use std::io::BufReader;
use std::path::Path;

fn main() {
    let input = "foo\nbar";
    let filename = "/tmp/foo.txt";

    // write into a file
    fs::write(filename, input).expect("Unable to write file");

    // check if the file exists
    let exists = Path::new(filename).exists();
    assert!(exists, true);

    // read the whole file to string
    let output = fs::read_to_string(filename).expect("Unable to read file");
    assert_eq!(input, output);

    // read bytes
    let bytes = fs::read(filename).unwrap();

    // read by line
    let f = fs::File::open(filename).unwrap();
    let f = BufReader::new(f);
    for line in f.lines() {
        if let Ok(line) = line {
            println!("{}", line);
        }
    }

    println!("{:?}", rel_path("foo/foo.log").unwrap());
    println!("{}", exe_name().unwrap());
}

/// A path relative to current excecutable
fn rel_path(path: &str) -> std::io::Result<std::path::PathBuf> {
    let mut dir = std::env::current_exe()?;
    dir.pop();
    dir.push(path);
    Ok(dir)
}

/// Returns executable name without it's path and `.exe` suffix
fn exe_name() -> Option<String> {
    std::env::current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .trim_end_matches(".exe")
        .to_owned()
        .into()
}
