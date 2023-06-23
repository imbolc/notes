fn main() {
    let items = vec!["foo", "bar"];
    passing_by_slice(&items);
    into_iterator(items.iter());
    dbg!(items);
}

fn as_slice(items: &[&str]) {
    dbg!(items);
}

fn as_iterator<'a, I>(it: I)
where
    I: Iterator<Item = &'a &'a str>,
{
    for s in it {
        dbg!(s);
    }
}
