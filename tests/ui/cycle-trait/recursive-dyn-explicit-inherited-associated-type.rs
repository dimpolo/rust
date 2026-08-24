trait Foo<T: ?Sized> {}
trait Bar: Foo<dyn Bar<Item = ()>> + Iterator<Item = ()> {}
//~^ ERROR cycle detected

fn main() {}
