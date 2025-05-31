//@ compile-flags: --crate-type=lib

#![feature(impl_restriction)]

pub mod foo {
    pub mod bar {
        pub(crate) impl(super) trait Foo {}
    }

    impl bar::Foo for i8 {}
}

impl foo::bar::Foo for u8 {} //~ ERROR implementation of restricted trait
