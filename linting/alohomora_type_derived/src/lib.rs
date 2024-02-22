#![feature(rustc_private)]
#![warn(unused_extern_crates)]

//extern crate rustc_hir;
extern crate rustc_span;
extern crate rustc_middle;

use rustc_lint::{LateContext, LateLintPass};
use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::get_trait_def_id;
//use clippy_utils::ty::implements_trait; 
//use rustc_hir::Expr;

use rustc_middle::ty::fast_reject::SimplifiedType;

use std::vec::Vec;


dylint_linting::declare_late_lint! {
    /// ### What it does
    /// Denies manual implementations of AlohomoraType. 
    /// ### Why is this bad?
    /// Developers must derive impls of AlohomoraType to ensure integrity of BBoxes.
    /// ### Known problems
    /// Remove if none.
    ///
    /// ### Example
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub ALOHOMORA_TY_DERIVED,
    Warn,
    "AlohomoraType must always be DERIVED, not user-implemented"
}

impl<'tcx> LateLintPass<'tcx> for AlohomoraTyDerived {
    // A list of things you might check can be found here:
    // https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html
    //fn check_expr(&mut self, cx: &LateContext<'_>, expr: &Expr<'_>) {
    fn check_crate(&mut self, cx: &LateContext<'tcx>) {

        // this returns None -> panics: 
        let parts = vec!["trait_def", "AlohomoraType"]; 
        let path: &[&str] = &parts;
        let aloh_ty_did = get_trait_def_id(cx, path).unwrap(); 
                                          
        let secret = "ALOHOMORA"; 
        let contains_secret = |(_, v): (&SimplifiedType, &Vec<rustc_span::def_id::DefId>)| 
                                v.iter()
                                .any(|&def_id|
                                    cx.tcx
                                    .get_attr(def_id, rustc_span::symbol::Symbol::intern("doc"))
                                    .and_then(|attr| Some(attr.doc_str().unwrap().to_ident_string()))
                                    .and_then(|doc| Some(doc.contains(secret)))
                                    .unwrap_or(false)); 
          
        // attempt at checking crate, but only works for library impls
        // pretty sure crate shows up as user-side when from an expanded macro
        /* let contains_secret = |(k, v): (&SimplifiedType, &Vec<rustc_span::def_id::DefId>)| 
                                v.iter()
                                .any(|&def_id|
                                    def_id.krate == aloh_ty_did.krate); */

        let bad_impls: Vec<(&SimplifiedType, &Vec<rustc_span::def_id::DefId>)> = cx.tcx
                                .trait_impls_of(aloh_ty_did)
                                .non_blanket_impls()
                                .iter()
                                .filter(|&(k, v)| !contains_secret((k, v)))
                                .collect();
        
        let map : rustc_middle::hir::map::Map = cx.tcx.hir(); 
        bad_impls.iter()
            .for_each(|&(_, v)| {
                 v.iter()
                    .for_each(|def_id| {
                        let span = map.span_if_local(def_id.clone()).unwrap(); 
                        //TODO if span.cxt() and use rustc_middle::lint::in_external_macro;

                        span_lint_and_help (
                            cx,
                            ALOHOMORA_TY_DERIVED,
                            span,
                            "manual implementation of AlohomoraType trait", 
                            None, "use `#[derive(AlohomoraType)]` instead"
                        );
                    });
                }
            );
    }

}

#[test]
fn ui() {
    dylint_testing::ui_test(
        env!("CARGO_PKG_NAME"),
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui"),
    );
}
