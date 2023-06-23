fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = Excerpt::from_text(&novel);
    println!("The excerpt: {}", excerpt.part)
}

// `<'a> means an instance of `Exerpt` can't outlive its `part` field
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn from_text(text: &str) -> Excerpt {
        let first_sentence = text.split('.').next().unwrap();
        return Excerpt {
            part: first_sentence,
        };
    }
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    match a.len() > b.len() {
        true => a,
        false => b,
    }
}

struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}
