//! Builder pattern

#[derive(Default)]
struct Person {
    given_name: String,
    family_name: Option<String>,
}

struct PersonBuilder(Person);

fn main() {
    // `Default` trait - if there's reasonable defaults for every field we can reduce boilerplate
    // without creating a builder
    let _alice = Person {
        given_name: "Alice".into(),
        ..Default::default()
    };

    let _bond = PersonBuilder::new("James").family_name("Bond").build();
}

impl PersonBuilder {
    /// Constructor takes required fields
    fn new(given_name: impl Into<String>) -> Self {
        Self(Person {
            given_name: given_name.into(),
            family_name: None,
        })
    }

    /// And all the optional fields have setters like this
    fn family_name(mut self, s: impl Into<String>) -> Self {
        self.0.family_name = Some(s.into());
        self
    }

    fn build(self) -> Person {
        self.0
    }
}
