//@ check-pass
//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver
//@ aux-build: recursive_dyn.rs

extern crate recursive_dyn;

use std::marker::PhantomData;

use recursive_dyn::{Object, Rc, Trace};

struct Value(Rc<dyn Object>);

impl Trace<dyn Object> for Value {
    fn trace(&self, tracer: fn(&Rc<dyn Object>)) {
        tracer(&self.0);
    }
}

impl Object for Value {}

fn main() {
    let value = Value(Rc(PhantomData));
    let object: &dyn Object = &value;
    object.trace(|_: &Rc<dyn Object>| {});
    let _: &dyn Trace<dyn Object> = object;
}
