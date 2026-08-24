//@ check-pass

trait Foo<T: ?Sized> {}

trait Bar: Foo<dyn Bar> {}

fn main() {}
