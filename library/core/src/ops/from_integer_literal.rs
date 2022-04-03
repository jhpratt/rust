#![allow(missing_docs)] // FIXME(jhpratt)

mod sealed {
    #[unstable(feature = "from_integer_literal", issue = "none")]
    pub trait Integer {}
    impl Integer for u8 {}
    impl Integer for u16 {}
    impl Integer for u32 {}
    impl Integer for u64 {}
    impl Integer for u128 {}
    impl Integer for usize {}
    impl Integer for i8 {}
    impl Integer for i16 {}
    impl Integer for i32 {}
    impl Integer for i64 {}
    impl Integer for i128 {}
    impl Integer for isize {}
}

// FIXME(jhpratt) Enforce all implementations are `const`.
#[cfg_attr(not(bootstrap), lang = "from_integer_literal")]
#[unstable(feature = "from_integer_literal", issue = "none")]
pub trait FromIntegerLiteral: Sized {
    // FIXME(jhpratt) Enforce the bound in the compiler?
    type Input: sealed::Integer;

    fn from_integer_literal(i: Self::Input) -> Self;
}
