//! Function pointer type

fn main() {
    // Coersing function into function pointer
    let mut f: fn(_) -> _ = inc;
    assert_eq!(f(1), 2);

    // Closures with the same parameters, which don't capture environment variables,
    // can be coerced into the same type
    f = |n: i32| n - 1;
    assert_eq!(f(1), 0);

    // Using coercion to function pointers is useful for passing both functions and closures
    // as parameters.
    assert_eq!(calc(&[inc, inc, |n: i32| n * 2], 0), 4);
}

fn inc(n: i32) -> i32 {
    n + 1
}

/// Applies a list of functions in order to the input `n`
fn calc(fns: &[fn(i32) -> i32], n: i32) -> i32 {
    let mut r = n;
    for f in fns {
        r = f(r);
    }
    r
}
