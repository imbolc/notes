//! Builder pattern

#[derive(Default)]
struct Person {
    given_name: String,
    family_name: Option<String>,
}

struct PersonBuilder(Person);

fn main() {
    // If there's reasonable defaults for every optional field, we can reduce
    // boilerplate without a builder using `Default` trait
    let _alice = Person {
        given_name: "Alice".into(),
        ..Default::default()
    };

    let _bond = Person::builder("James".to_string())
        .family_name("Bond".to_string())
        .build();
}

impl Person {
    /// A convenience method to start building without a need to import the builder
    fn builder(given_name: String) -> PersonBuilder {
        PersonBuilder::new(given_name)
    }
}

impl PersonBuilder {
    /// Constructor takes required fields
    fn new(given_name: String) -> Self {
        Self(Person {
            given_name: given_name.into(),
            ..Default::default()
        })
    }

    /// And all the optional fields have setters like this
    fn family_name(mut self, s: impl Into<Option<String>>) -> Self {
        self.0.family_name = s.into();
        self
    }

    fn build(self) -> Person {
        self.0
    }
}
