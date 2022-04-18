// FIXME(jhpratt) these errors aren't caught for some reason

#![feature(from_integer_literal)]

use std::num::{NonZeroI8, NonZeroU8};

struct Alpha {
    value: NonZeroU8,
}
struct Beta {
    value: NonZeroI8,
}

const fn foo(_: NonZeroU8) {}
const fn bar(_: NonZeroI8) {}

const _: NonZeroU8 = 0; //~ ERROR[E0080] evaluation of constant value failed
const _: NonZeroI8 = 0; //~ ERROR[E0080] evaluation of constant value failed
const _: () = foo(0); //~ ERROR[E0080] evaluation of constant value failed
const _: () = bar(0); //~ ERROR[E0080] evaluation of constant value failed
const _: Alpha = Alpha {
    value: 0, //~ ERROR[E0080] evaluation of constant value failed
};
const _: Beta = Beta {
    value: 0, //~ ERROR[E0080] evaluation of constant value failed
};

const _: NonZeroU8 = 300; // FIXME(jhpratt) hard error if value is out of range

fn main() {
    // FIXME(jhpratt) the value is not guaranteed to go through const eval
    // This needs to fail even though the value is visibly part of const eval.
    let _: NonZeroU8 = 0; //~ ERROR[E0080] evaluation of constant value failed
}
