// compile-flags: --crate-type=lib

#![feature(from_integer_literal)]

use std::ops::FromIntegerLiteral;

struct Foo(u8);

impl FromIntegerLiteral for Foo {
    //~^ ERROR `FromIntegerLiteral` implementations must be `const`
    type Input = u8;

    fn from_integer_literal(n: Self::Input) -> Self {
        Foo(n)
    }
}
