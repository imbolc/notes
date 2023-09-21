// Working with files in Rust
use std::{
    fs,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

fn main() -> io::Result<()> {
    let input = "foo\nbar";
    let filename = "/tmp/foo.txt";

    // write into a file
    fs::write(filename, input)?;

    // check if the file exists
    let exists = Path::new(filename).exists();
    assert!(exists);

    // read the whole file to string
    let output = fs::read_to_string(filename)?;
    assert_eq!(input, output);

    // read bytes
    let bytes = fs::read(filename)?;
    dbg!(bytes);

    // read by line
    let f = fs::File::open(filename)?;
    let f = BufReader::new(f);
    for line in f.lines() {
        if let Ok(line) = line {
            println!("line by line: {}", line);
        }
    }

    dbg!(rel_path("foo/foo.log")?);
    dbg!(exe_name());
    Ok(())
}

/// A path relative to current excecutable
fn rel_path(path: &str) -> io::Result<PathBuf> {
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
