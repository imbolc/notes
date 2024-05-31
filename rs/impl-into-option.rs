//! `impl Into<Option<T>>` - a convenient trick to accept `T` or `None` on the caller side

#[derive(Debug, Default)]
struct Input {
    placeholder: Option<String>,
}

impl Input {
    // Acepting `Into Option` instead of `Option`
    fn with_placeholder(&mut self, s: impl Into<Option<String>>) {
        self.placeholder = s.into();
    }
}

fn main() {
    let mut input = Input::default();

    // We can pass strings or `None` without wrapping them in `Option`
    input.with_placeholder("foo".to_string());
    input.with_placeholder(None);
}
