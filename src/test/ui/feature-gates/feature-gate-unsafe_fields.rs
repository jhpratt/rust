// compile-flags: --crate-type=lib
// revisions: with_gate without_gate
//[with_gate] check-pass

#![cfg_attr(with_gate, feature(unsafe_fields))]

struct Foo {
    unsafe field: (), //[without_gate]~ ERROR
}

struct FooTuple(unsafe ()); //[without_gate]~ ERROR

enum Bar {
    Variant { unsafe field: () }, //[without_gate]~ ERROR
    VariantTuple(unsafe ()), //[without_gate]~ ERROR
}

union Baz {
    unsafe field: (), //[without_gate]~ ERROR
}
