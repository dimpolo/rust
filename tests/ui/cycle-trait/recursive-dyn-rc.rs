//@ check-pass
//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

use std::marker::PhantomData;

pub struct Rc<Dyn: ?Sized + Trace<Dyn>> {
    ptr_and_stuff: PhantomData<Dyn>,
}

pub trait Trace<Dyn: Trace<Dyn> + ?Sized> {
    fn trace(&self, tracer: fn(&Rc<Dyn>));
}

trait MyTrait: Trace<dyn MyTrait> {}

struct MyStruct {
    field: Rc<dyn MyTrait>,
}

impl Trace<dyn MyTrait> for MyStruct {
    fn trace(&self, tracer: fn(&Rc<dyn MyTrait>)) {
        tracer(&self.field);
    }
}

impl MyTrait for MyStruct {}

fn main() {
    let value = MyStruct { field: Rc { ptr_and_stuff: PhantomData } };
    let object: &dyn MyTrait = &value;
    object.trace(|_: &Rc<dyn MyTrait>| {});
    let trace: &dyn Trace<dyn MyTrait> = object;
    trace.trace(|_: &Rc<dyn MyTrait>| {});
}
