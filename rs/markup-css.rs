//! A component way of managing CSS with `markup` in Rust
//!
//! It's possible to keep [`makrup`][] styles bound to templates and render the styles only once on the first
//! component rendering. Here's some benefits of this approach include:
//!
//! - a page would contain only styles it needs
//! - a browser could have enough information to correctly render even a half-loaded page
//! - multiple usage of a component won't lead to its styles repetition on the page
//!
//! [markup]: https://github.com/utkarshkukreti/markup.rs
//!
//! ```cargo
//! [dependencies]
//! markup = "0.13"
//! ```

markup::define! {
    #[derive(Default)]
    HelloStyles(shown: std::cell::Cell<bool>) {
        @if !shown.replace(true) {
            style {
                "p.hello { color: green }"
            }
        }
    }

    Hello<'a>(
        styles: &'a HelloStyles,
        name: &'a str,
    ) {
        @styles
        p.hello { @format!("Hello, {name}") }
    }
}

fn main() {
    let styles = HelloStyles::default();
    println!(
        "{}",
        Hello {
            styles: &styles,
            name: "World",
        }
    );
    println!(
        "{}",
        Hello {
            styles: &styles,
            name: "Self",
        }
    );
}
