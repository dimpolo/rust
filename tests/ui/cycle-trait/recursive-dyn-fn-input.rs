trait Foo<T> {}
trait Bar: Foo<fn(&dyn Bar)> {}
//~^ ERROR cycle detected

fn main() {}
