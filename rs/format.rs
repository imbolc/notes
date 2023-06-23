fn main() {
    // Zero-padding
    assert_eq!(format!("{:02}", 5), "05");

    // Padding from a variable
    let pad = 3;
    assert_eq!(format!("{:pad$}", 5), "  5");
}
