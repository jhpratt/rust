use rustc_hir::def_id::LocalDefId;
use rustc_hir::intravisit::Visitor;
use rustc_hir::{Item, ItemKind, Node, intravisit};
use rustc_middle::query::Providers;
use rustc_middle::span_bug;
use rustc_middle::ty::{Restriction, TyCtxt};

use crate::errors;

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers { impl_restriction, check_impl_restriction, ..*providers };
}

struct ImplOfRestrictedTraitVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl<'v> Visitor<'v> for ImplOfRestrictedTraitVisitor<'v> {
    type NestedFilter = rustc_middle::hir::nested_filter::All;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_item(&mut self, item: &'v Item<'v>) {
        if let ItemKind::Trait(..) = item.kind {
            let restriction = self.tcx.impl_restriction(item.owner_id.def_id);

            for impl_ in self.tcx.local_trait_impls(item.owner_id.to_def_id()) {
                if !restriction.is_allowed_in(impl_.to_def_id(), self.tcx) {
                    self.tcx.dcx().emit_err(errors::ImplOfRestrictedTrait {
                        impl_span: self
                            .tcx
                            .span_of_impl(impl_.to_def_id())
                            .expect("impl should be local"),
                        restriction_span: restriction.span(),
                    });
                }
            }
        };

        intravisit::walk_item(self, item)
    }
}

pub(crate) fn impl_restriction(tcx: TyCtxt<'_>, def_id: LocalDefId) -> Restriction {
    match tcx.resolutions(()).impl_restrictions.get(&def_id) {
        Some(restriction) => *restriction,
        None => match tcx.hir_node_by_def_id(def_id) {
            Node::Item(Item { kind: ItemKind::Trait(..), .. }) => span_bug!(
                tcx.def_span(def_id),
                "impl restriction table unexpectedly missing a def-id: {def_id:?}",
            ),
            _ => span_bug!(
                tcx.def_span(def_id),
                "called `impl_restriction` on non-trait: {def_id:?}",
            ),
        },
    }
}

pub(crate) fn check_impl_restriction(tcx: TyCtxt<'_>, _: ()) {
    tcx.hir_walk_toplevel_module(&mut ImplOfRestrictedTraitVisitor { tcx });
}
