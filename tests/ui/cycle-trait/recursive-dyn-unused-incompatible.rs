//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

trait Foo<T: ?Sized> {}
trait Bar: Foo<dyn Bar> {
    //~^ ERROR not dyn compatible
    fn generic<T>(&self);
}

fn main() {}
