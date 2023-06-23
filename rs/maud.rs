//! Maud template engine
//!
//! ```cargo
//! [dependencies]
//! maud = "*"
//! ```

use maud::{html, Markup, PreEscaped};

fn main() {
    // Boolean attributes and hidden classes
    assert_eq!(html! { option checked[false]; }.into_string(), "<option>");
    assert_eq!(html! { p.foo[false]; }.into_string(), r#"<p class="">"#);

    // Raw html
    assert_eq!(html! { (PreEscaped("<br>")) }.into_string(), "<br>");

    // Base template
    assert_eq!(base(html! { "foo" }).into_string(), "<body>foo</body>");
}

fn base(content: Markup) -> Markup {
    html! { body { (content) } }
}
