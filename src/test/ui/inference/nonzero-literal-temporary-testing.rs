// rustc-env:RUSTC_LOG=rustc_typeck::check::coercion=debug,rustc_mir_build::thir::cx::expr=debug
// compile-flags:--crate-type=lib
// check-pass

const _: std::num::NonZeroU8 = 1;
