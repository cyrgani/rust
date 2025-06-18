struct Foo;

trait First {
    type Assoc;
}

trait Second {
    type Assoc;
}

trait Final {
    fn call();
}

impl First for Foo {
    #[cfg(not(a))]
    type Assoc = ();
}

impl<T> Second for T
where
    T: First,
    T::Assoc: Into<Box<()>>,
{
    type Assoc = ();
}

impl<T> Final for T
where
    T: Second,
    T::Assoc: Clone,
{
    fn call() {}
}

fn main() {
    Foo::call();
}
