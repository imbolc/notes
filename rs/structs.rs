#[derive(Debug)]
struct Person<'a> {
    // 'a means that lifetime of reference at least as long as the struct itself
    first_name: &'a str,
    second_name: &'a str,
}

impl<'a> Person<'a> {
    // associated funciton
    fn new(first_name: &'a str, second_name: &'a str) -> Self {
        Self {
            first_name,
            second_name,
        }
    }

    // `&self` indecates it's a method
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.second_name)
    }
}

fn main() {
    let person = Person::new("Sherlock", "Holmes");
    println!("{}", person.full_name());
}
