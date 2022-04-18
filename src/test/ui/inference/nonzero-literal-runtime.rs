// run-pass

#![feature(from_integer_literal)]
#![allow(dead_code)]

use std::num::{NonZeroI8, NonZeroU8};

struct Alpha {
    value: NonZeroU8,
}
struct Beta {
    value: NonZeroI8,
}

const fn foo(_: NonZeroU8) {}
const fn bar(_: NonZeroI8) {}

const _: NonZeroU8 = 1;
const _: NonZeroI8 = -1;
const _: Alpha = Alpha { value: 1 };
const _: () = foo(1);
const _: () = bar(-1);
const _: Beta = Beta { value: -1 };

trait Foo {
    fn foo(self) -> &'static str;
}

impl Foo for u8 {
    fn foo(self) -> &'static str {
        "zeroable"
    }
}

impl Foo for NonZeroU8 {
    fn foo(self) -> &'static str {
        "nonzero"
    }
}

trait Bar {
    // Make sure autoref works when called as a method.
    fn bar(&self) -> &'static str;
}

impl Bar for u8 {
    fn bar(&self) -> &'static str {
        "zeroable"
    }
}

impl Bar for NonZeroI8 {
    fn bar(&self) -> &'static str {
        "nonzero"
    }
}

fn main() {
    // Anything other than "zeroable" would _technically_ be permitted, but
    // would cause enormous compatibility issues.
    assert_eq!(Foo::foo(1), "zeroable");
    assert_eq!(1.foo(), "zeroable");
    assert_eq!(Bar::bar(&1), "zeroable");
    assert_eq!(1.bar(), "zeroable");
}
