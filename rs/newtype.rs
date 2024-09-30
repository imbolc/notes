//! Newtype and "parse, don't validate" idioms
//!
//! ```cargo
//! [dependencies]
//! derive_more = "*"
//! ```
use std::str::FromStr;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::AsRef,
    derive_more::Display,
    // is it make sense to provide access to the most of the underlying type api?
    // probably not as a public api of a crate
    derive_more::Deref,
)]
struct Name(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
struct Age(usize);

fn main() -> Result<(), String> {
    // Let's assume input data came from a web form as strings
    let name_input = "Alice".to_string();
    let age_input = "5".to_string();

    // After constructing our newtypes, we can be sure the data is valid and
    // use these types afterwards without validation
    let name = Name::try_from(name_input)?;
    let age: Age = age_input.parse()?;

    // Also notice, that `age_input` isn't consumed, while `name_input` is
    dbg!(age_input);
    // dbg!(name_input); // compile error: value used here after move

    dbg!(name, age);
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

/// We implement `TryFrom` instead of `FromStr` to avoid an unnecessary
/// allocation during string copying
impl TryFrom<String> for Name {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() < 5 {
            return Err("name is too short".into());
        }
        Ok(Name(s))
    }
}
