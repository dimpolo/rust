use rustc_hir::HirId;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;

pub(super) fn dyn_trait_aliases(_tcx: TyCtxt<'_>, _def_id: LocalDefId) -> &[(HirId, LocalDefId)] {
    &[]
}
