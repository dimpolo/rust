trait Foo<T: ?Sized> { type Out; }
trait Bar: Foo<dyn Bar, Out = ()> {}
//~^ ERROR cycle detected

fn main() {}
