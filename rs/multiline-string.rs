fn main() {
    // By default we get line breaks with indents
    let s = "1
    2";
    assert_eq!(s, "1\n    2");

    // We can remove indents
    let s = "1\n\
    2";
    assert_eq!(s, "1\n2");

    // Or we can remove both line breaks indents
    let s = "1 \
    2";
    assert_eq!(s, "1 2");
}
