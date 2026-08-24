trait Foo<T: ?Sized> {}
trait Bar<'a>: Foo<dyn Bar<'a> + 'a> {}
//~^ ERROR cycle detected

fn main() {}
