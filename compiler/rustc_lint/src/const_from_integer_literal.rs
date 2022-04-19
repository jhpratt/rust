use rustc_hir::{Constness, Impl, Item, ItemKind};

use crate::{LateContext, LateLintPass, LintContext};

declare_lint! {
    /// The `const_from_integer_literal` lint detects implementations of
    /// `FromIntegerLiteral` that are not `const`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use std::ops::FromIntegerLiteral;
    ///
    /// struct Foo(u8);
    ///
    /// impl FromIntegerLiteral for Foo {
    ///     type Input = u8;
    ///
    ///     fn from_integer_literal(n: Self::Input) -> Self {
    ///         Foo(n)
    ///     }
    /// }
    /// ```
    ///
    /// ### Explanation
    ///
    /// The coercion from an integer literal to the desired type happens at
    /// compile-time.
    pub CONST_FROM_INTEGER_LITERAL,
    Deny,
    "detects `FromIntegerLiteral` implementations that are not `const`"
}

#[derive(Copy, Clone, Default)]
pub struct ConstFromIntegerLiteral;

impl_lint_pass!(ConstFromIntegerLiteral => [CONST_FROM_INTEGER_LITERAL]);

impl<'tcx> LateLintPass<'tcx> for ConstFromIntegerLiteral {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        let ItemKind::Impl(Impl {
            constness: Constness::NotConst,
            of_trait: Some(trait_ref),
            ..
        }) = &item.kind else { return };
        let Some(did) = trait_ref.trait_def_id() else { return };

        if Some(did) == cx.tcx.lang_items().from_integer_literal() {
            cx.struct_span_lint(CONST_FROM_INTEGER_LITERAL, item.span, |lint| {
                lint.build("`FromIntegerLiteral` implementations must be `const`")
                    .note("this ensures that the value can be used in a const context")
                    .emit()
            });
        }
    }
}
