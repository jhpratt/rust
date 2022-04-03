// compile-flags:--crate-type=lib

use std::num::{NonZeroI8, NonZeroU8};

struct Alpha {
    value: NonZeroU8,
}
struct Beta {
    value: NonZeroI8,
}

const fn foo(_: NonZeroU8) {}
const fn bar(_: NonZeroI8) {}

const _: NonZeroU8 = 0; //~ ERROR expected non-zero value
const _: NonZeroI8 = 0; //~ ERROR expected non-zero value
const _: () = foo(0); //~ ERROR expected non-zero value
const _: () = bar(0); //~ ERROR expected non-zero value
const _: Alpha = Alpha {
    value: 0, //~ ERROR expected non-zero value
};
const _: Beta = Beta {
    value: 0, //~ ERROR expected non-zero value
};

const _: NonZeroU8 = 300; // FIXME(jhpratt) hard error if value is out of range
