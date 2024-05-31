//! Newtype and "parse, don't validate" idioms
use std::{fmt, str::FromStr};

struct Name(String);
struct Age(usize);

fn main() -> Result<(), String> {
    // Let's assume input data came from a web form as strings
    let name_input = "Alice".to_string();
    let age_input = "5".to_string();

    // After constructing our newtypes, we can be sure the data is valid and
    // use these types afterwards without validation
    let name: Name = name_input.try_into()?;
    let age: Age = age_input.parse()?;

    println!("name: {name}");
    println!("age: {age}");
    Ok(())
}

/// We implement `FromStr` which also can be called using `.parse` method
impl FromStr for Age {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<usize>() {
            Ok(n) => Ok(Self(n)),
            Err(_) => Err("invalid balance amount".to_string()),
        }
    }
}

/// We implement `TryFrom` instead of `FromStr` to avoid an unnecessary allocation
/// during string copying
impl TryFrom<String> for Name {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() < 5 {
            return Err("name is too short".into());
        }
        Ok(Name(s))
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Age> for usize {
    fn from(value: Age) -> usize {
        value.0
    }
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
