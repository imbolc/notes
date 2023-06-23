// # Typestate pattern
//
// Typestate is encoding a state machine states into the type system.
// A common way of implementing the pattern in Rust would be using type parameters for this
// purpose.
use std::marker::PhantomData;

// Let's say we have a read-only file
struct File<State = Closed> {
    name: String,
    _state: PhantomData<State>,
}

// The file can be in the next states
struct Opened;
struct Closed;

// Only a `Closed` file (which is the default state) file can be opened
impl File<Closed> {
    fn new(name: String) -> File<Closed> {
        return File {
            name,
            _state: PhantomData,
        };
    }

    fn open(self) -> File<Opened> {
        return File {
            name: self.name,
            _state: PhantomData,
        };
    }
}

// And only `Opened` file can be read and closed
impl File<Opened> {
    fn read(&self) {
        println!("reading from {}", self.name);
    }

    fn close(self) -> File<Closed> {
        File::new(self.name)
    }
}

// We can allow reading and closing methods only for opened files
//

fn main() {
    let f = File::new("foo.txt".into());
    // trying to call `read` or `close` at this point would lead to a compilation error
    let f = f.open();
    // now we can read
    f.read();
    let _f = f.close();
    // and again, reading a closed file would be a compilation error
}
