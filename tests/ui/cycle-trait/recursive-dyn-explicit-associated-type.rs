trait Foo<T: ?Sized> {}
trait Bar: Foo<dyn Bar<Item = ()>> {
    //~^ ERROR cycle detected
    type Item;
}

fn main() {}
