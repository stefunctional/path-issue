pub struct Foo;

#[cfg(test)]
mod tests {
    use bar::foo::Foo;

    #[test]
    fn it_works() {
        bar::bar(Foo);
    }
}
