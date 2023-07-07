use super::*;

use crate::syntax_tree::ast::*;

impl Display for PrettyPrinted<'_, Expr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_expr(self.0, f, 0)
    }
}

fn fmt_expr(expr: &Expr, f: &mut Formatter<'_>, soft_tab_count: usize) -> FmtResult {
    match expr {
        Expr::Ind(e) => fmt_ind(e, f, soft_tab_count),
        Expr::Vcon(e) => fmt_vcon(e, f, soft_tab_count),
        Expr::Match(e) => fmt_match(e, f, soft_tab_count),
        Expr::Fun(e) => fmt_fun(e, f, soft_tab_count),
        Expr::App(e) => fmt_app(e, f, soft_tab_count),
        Expr::For(e) => fmt_for(e, f, soft_tab_count),
        Expr::Deb(e) => fmt_deb(e, f, soft_tab_count),
        Expr::Universe(e) => fmt_universe(e, f, soft_tab_count),
    }
}

fn fmt_ind(ind: &RcSemHashed<Ind>, f: &mut Formatter<'_>, soft_tab_count: usize) -> FmtResult {
    todo!()
}

fn fmt_vcon(vcon: &RcSemHashed<Vcon>, f: &mut Formatter<'_>, soft_tab_count: usize) -> FmtResult {
    todo!()
}

fn fmt_match(m: &RcSemHashed<Match>, f: &mut Formatter<'_>, soft_tab_count: usize) -> FmtResult {
    todo!()
}

fn fmt_fun(fun: &RcSemHashed<Fun>, f: &mut Formatter<'_>, soft_tab_count: usize) -> FmtResult {
    todo!()
}

fn fmt_app(app: &RcSemHashed<App>, f: &mut Formatter<'_>, soft_tab_count: usize) -> FmtResult {
    todo!()
}

fn fmt_for(for_: &RcSemHashed<For>, f: &mut Formatter<'_>, soft_tab_count: usize) -> FmtResult {
    todo!()
}

fn fmt_deb(deb: &RcSemHashed<DebNode>, f: &mut Formatter<'_>, soft_tab_count: usize) -> FmtResult {
    todo!()
}

fn fmt_universe(
    universe: &RcSemHashed<UniverseNode>,
    f: &mut Formatter<'_>,
    soft_tab_count: usize,
) -> FmtResult {
    todo!()
}
