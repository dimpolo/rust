use rustc_ast::TraitObjectSyntax;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::LocalDefId;
use rustc_hir::definitions::{DefPathData, PerParentDisambiguatorState};
use rustc_hir::intravisit::{self, Visitor};
use rustc_hir::{self as hir, HirId};
use rustc_middle::middle::resolve_bound_vars::ResolvedArg;
use rustc_middle::ty::{self, TyCtxt};

pub(super) fn dyn_trait_aliases(tcx: TyCtxt<'_>, def_id: LocalDefId) -> &[(HirId, LocalDefId)] {
    let hir::Node::Item(item) = tcx.hir_node_by_def_id(def_id) else {
        return &[];
    };
    let hir::ItemKind::Trait { generics, .. } = item.kind else {
        return &[];
    };
    if !generics.params.is_empty() {
        return &[];
    }

    let mut collector = Collector {
        tcx,
        owner: def_id,
        aliases: Vec::new(),
        disambiguator: PerParentDisambiguatorState::new(def_id),
    };
    collector.visit_item(item);
    tcx.arena.alloc_from_iter(collector.aliases)
}

struct Collector<'tcx> {
    tcx: TyCtxt<'tcx>,
    owner: LocalDefId,
    aliases: Vec<(HirId, LocalDefId)>,
    disambiguator: PerParentDisambiguatorState,
}

impl<'tcx> Visitor<'tcx> for Collector<'tcx> {
    fn visit_ty(&mut self, ty: &'tcx hir::Ty<'tcx, hir::AmbigArg>) {
        if let hir::TyKind::TraitObject(bounds, lifetime) = ty.kind
            && lifetime.tag() == TraitObjectSyntax::Dyn
            && let [bound] = bounds
            && bound.bound_generic_params.is_empty()
            && bound.trait_ref.path.res == Res::Def(DefKind::Trait, self.owner.to_def_id())
            && bound.trait_ref.path.segments.iter().all(|segment| segment.args.is_none())
            && matches!(
                self.tcx.named_bound_var(lifetime.pointer().hir_id),
                Some(ResolvedArg::StaticLifetime)
            )
        {
            let alias = self.tcx.at(ty.span).create_def(
                self.owner,
                None,
                DefKind::TyAlias,
                Some(DefPathData::AnonDynTy),
                &mut self.disambiguator,
            );
            alias.feed_hir();
            alias.def_ident_span(Some(ty.span));
            alias.visibility(ty::Visibility::Public);
            alias.defaultness(hir::Defaultness::Final);
            alias.generics_of(ty::Generics {
                parent: None,
                parent_count: 0,
                own_params: Vec::new(),
                param_def_id_to_index: Default::default(),
                has_self: false,
                has_late_bound_regions: None,
            });
            alias.explicit_clauses_of(ty::GenericClauses { parent: None, clauses: &[] });
            alias.inferred_outlives_of(&[]);
            self.aliases.push((ty.hir_id, alias.def_id()));
        }
        intravisit::walk_ty(self, ty);
    }
}
