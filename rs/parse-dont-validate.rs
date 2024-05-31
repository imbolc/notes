//! Parse don't validate
//!
//! The [idea][origin] is to parse input into a more structured form early on,
//! ensuring that invalid data is never processed by the following logic of the program.
//! This approach enhances reliability and maintainability by leveraging the static
//! type system to enforce constraints and eliminate the need for redundant checks
//! throughout the code.
//!
//! [origin]: https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/
//!
//! ```cargo
//! [dependencies]
//! thiserror = "1"
//! ```

/// A newtype wrapper for a valid email address
#[derive(Debug)]
struct Email(String);

/// Email parsing error
#[derive(Debug, thiserror::Error)]
#[error("can't parse email: {0}")]
struct EmailParseError(String);

/// Usually you'd implement `FromStr` for from-string conversion, but in this case
/// implementing `TryFrom<String>` has the advantage of moving the decision about
/// the string allocation to the client
impl TryFrom<String> for Email {
    type Error = EmailParseError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.contains('@') {
            Ok(Self(s))
        } else {
            Err(EmailParseError(s))
        }
    }
}

/// Let's also implement `FromStr` as a convention
impl std::str::FromStr for Email {
    type Err = EmailParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.to_string().try_into()
    }
}

/// Now all email-related functionality must accept the newtype
fn send_email(to: &Email) {
    println!("Sending email to: {}", to.0);
}

fn main() {
    let input = "foo@bar.com"; // An unvalidated input
    let email: Email = input.parse().unwrap(); // Parse the input as we get it

    // All the following program doesn't need to perform any email validation
    send_email(&email)
}
