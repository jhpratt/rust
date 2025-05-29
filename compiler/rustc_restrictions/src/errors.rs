use rustc_macros::Diagnostic;
use rustc_span::Span;

#[derive(Diagnostic)]
#[diag(restrictions_impl_of_restricted_trait)]
pub(crate) struct ImplOfRestrictedTrait {
    #[primary_span]
    pub(crate) impl_span: Span,
    #[note]
    pub(crate) restriction_span: Span,
}
