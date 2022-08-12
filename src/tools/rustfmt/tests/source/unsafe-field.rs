struct Foo {
    unsafe
    field: (),
}

struct FooTuple(
    unsafe
    (),
);

enum Bar {
    Variant {
        unsafe
        field: (),
    },
    VariantTuple(
        unsafe
        (),
    ),
}

union Baz {
    unsafe
    field: (),
}
