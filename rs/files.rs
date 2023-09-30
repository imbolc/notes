// Working with files in Rust

fn main() -> std::io::Result<()> {
    let filename = "/tmp/files-rs.txt";

    // check if the file exists
    let _exists = std::path::Path::new(filename).exists();

    // write into a file
    std::fs::write(filename, "foo\n")?;

    // Read the file into a string
    let _content = std::fs::read_to_string(filename)?;

    // Read the file as bytes
    let _bytes = std::fs::read(filename)?;

    // Append a few lines to the file
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new().append(true).open(filename)?;
    f.write_all(b"bar\n")?;
    f.write_all(b"baz\n")?;

    // Read the file line by line
    use std::io::BufRead;
    let f = std::fs::File::open(filename)?;
    let f = std::io::BufReader::new(f);
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
