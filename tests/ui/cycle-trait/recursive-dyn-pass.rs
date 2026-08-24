//@ check-pass
//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

mod fn_output {
    trait Foo<T> {}
    trait Bar: Foo<fn() -> &'static dyn Bar> {}

    impl Foo<fn() -> &'static dyn Bar> for () {}
    impl Bar for () {}

    pub fn check() {
        let object: &dyn Bar = &();
        let _: &dyn Foo<fn() -> &'static dyn Bar> = object;
    }
}

mod repeated {
    trait Foo<T: ?Sized> {}
    trait Other<T: ?Sized> {}
    trait Bar: Foo<dyn Bar> + Other<dyn Bar + 'static> {}

    impl Foo<dyn Bar> for () {}
    impl Other<dyn Bar> for () {}
    impl Bar for () {}

    pub fn check() {
        let object: &dyn Bar = &();
        let _: &dyn Foo<dyn Bar> = object;
        let _: &dyn Other<dyn Bar> = object;
    }
}

mod independent_associated_type {
    trait Foo<T: ?Sized> {}
    trait Bar: Foo<dyn Bar> + Iterator<Item = ()> {}

    struct Value;

    impl Foo<dyn Bar> for Value {}
    impl Iterator for Value {
        type Item = ();
        fn next(&mut self) -> Option<()> {
            None
        }
    }
    impl Bar for Value {}

    pub fn check() {
        let object: &mut dyn Bar = &mut Value;
        let _: Option<()> = object.next();
        let _: &dyn Foo<dyn Bar> = object;
    }
}

fn main() {
    fn_output::check();
    repeated::check();
    independent_associated_type::check();
}
