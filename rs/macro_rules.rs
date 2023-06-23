//! An macro_rules example

macro_rules! foo {
    // "Literal" could be a character, string, number, etc
    (greet $name:literal) => {
        println!("Hello, {}!", $name)
    };
    // expr: an "expression" is anything that ends with `=>`, `,` or `;`
    (square $a:expr) => {
        println!("square: {}", $a * $a)
    };
    // ident: an "identifier", like a variable name. ident metavariables Can be followed by anything
    (inc $a:ident) => {
        $a += 1
    };
    // block: a "block expression" (anything inside curly braces). Can be followed by anything.
    (twice $a:block) => {
        for _ in 0..2 {
            $a
        }
    };
    // ty: a type. Can only be followed by =>, ,, =, |, ;, :, >, >>, [, {, as, where, or a block metavariable
    (var $k:ident: $t:ty = $v:expr) => {
        let $k: $t = $v;
    };
    // repetition
    (assign $($i:ident is $e:expr),+) => {
        $(let $i = $e;)+
    }
}

fn main() {
    foo!(greet "World");
    foo!(square(1 + 1) * 2);

    let mut x = 1;
    foo!(inc x);
    dbg!(x);

    foo!(twice { println!("twice me") });

    foo!(var y: i32 = 5 * 5);
    dbg!(y);

    foo!(assign pi is 3.14, tau is 6.28);
    dbg!(pi, tau);
}
