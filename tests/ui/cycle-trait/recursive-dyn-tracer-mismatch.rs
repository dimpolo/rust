//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

use std::marker::PhantomData;

pub struct Rc<Dyn: ?Sized + Trace<Dyn>>(PhantomData<Dyn>);

pub trait Trace<Dyn: Trace<Dyn> + ?Sized> {
    fn trace(&self, tracer: fn(&Rc<Dyn>));
}

trait MyTrait: Trace<dyn MyTrait> {}
trait OtherTrait: Trace<dyn OtherTrait> {}

fn trace(object: &dyn MyTrait, tracer: fn(&Rc<dyn OtherTrait>)) {
    object.trace(tracer);
    //~^ ERROR mismatched types
}

fn main() {}
