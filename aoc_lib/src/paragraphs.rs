use std::str::Lines;

pub trait Paragraphs {
    fn paragraphs(&self) -> impl Iterator<Item = Lines<'_>>;
}

impl<T> Paragraphs for T where T: AsRef<str> {
    fn paragraphs(&self) -> impl Iterator<Item = Lines<'_>> {
        self.as_ref().split("\n\n").into_iter().map(|s| s.lines())
    }
}
