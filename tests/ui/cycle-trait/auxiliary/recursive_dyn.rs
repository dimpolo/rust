use std::marker::PhantomData;

pub struct Rc<Dyn: ?Sized + Trace<Dyn>>(pub PhantomData<Dyn>);

pub trait Trace<Dyn: Trace<Dyn> + ?Sized> {
    fn trace(&self, tracer: fn(&Rc<Dyn>));
}

pub trait Object: Trace<dyn Object> {}
