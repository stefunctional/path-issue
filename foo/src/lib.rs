pub struct Foo;

#[cfg(test)]
mod tests {
    use crate::Foo;

    #[test]
    fn it_works() {
        bar::bar(Foo);
    }
}
