//@ revisions: missing_assoc private_bounds

// FIXME: Synthetic dyn aliases should print as their underlying dyn types.

#[cfg(missing_assoc)]
mod missing_assoc {
    trait Foo<T: ?Sized> {
        type Out;
    }

    trait Bar: Foo<dyn Bar> {}
    //[missing_assoc]~^ ERROR the value of the associated type `Out`

    trait Baz: Foo<dyn Send> {}
    fn f(_: &dyn Baz) {}
    //[missing_assoc]~^ ERROR the value of the associated type `Out`
}

#[cfg(private_bounds)]
mod private_bounds {
    #![deny(private_bounds)]

    trait Foo<T: ?Sized> {}

    pub trait Bar: Foo<dyn Bar> {}
    //[private_bounds]~^ ERROR more private than the item

    pub trait Baz: Foo<dyn Send> {}
    //[private_bounds]~^ ERROR more private than the item
}

fn main() {}
