use super::*;

use crate::syntax_tree::ast::*;

impl Display for PrettyPrinted<'_, Expr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_expr(self.0, f, Indentation { soft_tab_count: 0 })
    }
}

// Every `fmt_{node}` function writes an indent
// at the beginning.
// It does _not_ write a newline at the end.

fn fmt_expr(expr: &Expr, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    match expr {
        Expr::Ind(e) => fmt_ind(e, f, indent),
        Expr::Vcon(e) => fmt_vcon(e, f, indent),
        Expr::Match(e) => fmt_match(e, f, indent),
        Expr::Fun(e) => fmt_fun(e, f, indent),
        Expr::App(e) => fmt_app(e, f, indent),
        Expr::For(e) => fmt_for(e, f, indent),
        Expr::Deb(e) => fmt_deb(e, f, indent),
        Expr::Universe(e) => fmt_universe(e, f, indent),
    }
}

fn fmt_ind(ind: &RcSemHashed<Ind>, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    todo!()
}

fn fmt_vcon(vcon: &RcSemHashed<Vcon>, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    todo!()
}

fn fmt_match(m: &RcSemHashed<Match>, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    todo!()
}

fn fmt_fun(fun: &RcSemHashed<Fun>, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    todo!()
}

fn fmt_app(app: &RcSemHashed<App>, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    todo!()
}

fn fmt_for(for_: &RcSemHashed<For>, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    todo!()
}

fn fmt_deb(deb: &RcSemHashed<DebNode>, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    let index = deb.value.deb.0;
    write!(f, "{indent}Type{index}")
}

fn fmt_universe(
    universe: &RcSemHashed<UniverseNode>,
    f: &mut Formatter<'_>,
    indent: Indentation,
) -> FmtResult {
    let level = universe.value.level.0;
    write!(f, "{indent}Type{level}")
}
