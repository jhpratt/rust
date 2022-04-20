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

/// Construct a value of this type from an unsuffixed integer literal. When
/// `FromIntegerLiteral` is implemented for a type, a value can be transparently
/// constructed from an integer literal.
///
/// ```rust
/// #![feature(from_integer_literal)]
/// #![feature(const_trait_impl)]
/// 
/// # use core::ops::FromIntegerLiteral;
/// struct Even(u32);
///
/// impl const FromIntegerLiteral for Even {
///     type Input = u32;
///
///     fn from_integer_literal(value: Self::Input) -> Self {
///         if value % 2 == 0 {
///             Even(value)
///         } else {
///             panic!("value must be even");
///         }
///     }
/// }
///
/// const ZERO: Even = 0;
/// ```
///
/// All implementations of `FromIntegerLiteral` **must** be `const`. This is
/// enforced by a deny-by-default lint. This restriction is in place to ensure
/// that the construction of the value is pure, has no side effects, and is
/// usable in `const` contexts.
#[cfg_attr(not(bootstrap), lang = "from_integer_literal")]
#[unstable(feature = "from_integer_literal", issue = "none")]
pub trait FromIntegerLiteral: Sized {
    /// The type passed as a parameter to `from_integer_literal`. This is always
    /// a primitive integer.
    type Input: sealed::Integer;

    /// Construct an instance of this type from a primitive integer. This
    /// method must panic if the passed value cannot be represented by this
    /// type.
    fn from_integer_literal(i: Self::Input) -> Self;
}
