use super::*;

use crate::syntax_tree::ast::*;

impl Display for PrettyPrinted<'_, Expr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_expr(self.0.clone(), f, Indentation { soft_tab_count: 0 })
    }
}

// Every `fmt_{node}` function writes an indent
// at the beginning.
// It does _not_ write a newline at the end.

fn fmt_expr(expr: Expr, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
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

fn fmt_ind(ind: RcSemHashed<Ind>, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    let i1 = indent.incremented();
    let universe_level = ind.value.universe_level.0;
    write!(f, "{indent}(\n{i1}ind\n{i1}Type{universe_level}\n")?;
    fmt_str_literal(ind.value.name.clone(), f, i1)?;
    write!(f, "\n")?;

    fmt_parenthesized_expressions(ind.value.index_types.clone(), f, i1)?;
    write!(f, "\n")?;

    fmt_parenthesized_vcon_defs(ind.value.vcon_defs.clone(), f, i1)?;
    write!(f, "\n{indent})")?;

    Ok(())
}

fn fmt_str_literal(
    str_literal: Rc<StringValue>,
    f: &mut Formatter<'_>,
    indent: Indentation,
) -> FmtResult {
    write!(f, "{indent}\"")?;

    for c in str_literal.0.chars() {
        if c.is_ascii_alphanumeric() || " _`~!@#$%^&*()-_=+[]|;:',<.>/?".contains(c) {
            write!(f, "{c}")?;
        } else {
            write!(f, "{{0x{:X}}}", u32::from(c))?;
        }
    }

    write!(f, "\"")?;
    Ok(())
}

fn fmt_parenthesized_vcon_defs(
    defs: RcSemHashed<Vec<VconDef>>,
    f: &mut Formatter<'_>,
    indent: Indentation,
) -> FmtResult {
    if defs.value.is_empty() {
        return write!(f, "{indent}()");
    }

    write!(f, "{indent}(")?;
    let i1 = indent.incremented();

    for def in defs.value.iter() {
        write!(f, "\n")?;
        fmt_vcon_def(def, f, i1)?;
    }

    write!(f, "\n{indent})")?;
    Ok(())
}

fn fmt_vcon_def(def: &VconDef, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    write!(f, "{indent}(\n")?;

    let i1 = indent.incremented();
    fmt_parenthesized_expressions(def.param_types.clone(), f, i1)?;
    write!(f, "\n")?;

    fmt_parenthesized_expressions(def.index_args.clone(), f, i1)?;

    write!(f, "\n{indent})")?;
    Ok(())
}

fn fmt_vcon(vcon: RcSemHashed<Vcon>, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    let i1 = indent.incremented();
    write!(f, "{indent}(\n{i1}vcon\n")?;

    fmt_ind(vcon.value.ind.clone(), f, i1)?;

    let vcon_index = vcon.value.vcon_index;
    write!(f, "\n{i1}{vcon_index}\n{indent})")?;
    Ok(())
}

fn fmt_match(m: RcSemHashed<Match>, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    let i1 = indent.incremented();
    write!(f, "{indent}(\n{i1}match\n")?;

    fmt_expr(m.value.matchee.clone(), f, i1)?;
    write!(f, "\n")?;

    fmt_expr(m.value.return_type.clone(), f, i1)?;
    write!(f, "\n")?;

    fmt_parenthesized_match_cases(m.value.cases.clone(), f, i1)?;
    write!(f, "\n{indent})")?;
    Ok(())
}

fn fmt_parenthesized_match_cases(
    cases: RcSemHashed<Vec<MatchCase>>,
    f: &mut Formatter<'_>,
    indent: Indentation,
) -> FmtResult {
    if cases.value.is_empty() {
        return write!(f, "{indent}()");
    }

    write!(f, "{indent}(")?;
    let i1 = indent.incremented();

    for case in cases.value.iter() {
        write!(f, "\n")?;
        fmt_match_case(case, f, i1)?;
    }

    write!(f, "\n{indent})")?;
    Ok(())
}

fn fmt_match_case(case: &MatchCase, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    let i1 = indent.incremented();
    let case_arity = case.arity;
    write!(f, "{indent}(\n{i1}{case_arity}\n")?;

    fmt_expr(case.return_val.clone(), f, i1)?;
    write!(f, "\n{indent})")?;
    Ok(())
}

fn fmt_fun(fun: RcSemHashed<Fun>, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    let i1 = indent.incremented();
    write!(f, "{indent}(\n{i1}fun\n")?;

    fmt_decreasing_index(fun.value.decreasing_index, f, i1)?;
    write!(f, "\n")?;

    fmt_parenthesized_expressions(fun.value.param_types.clone(), f, i1)?;
    write!(f, "\n")?;

    fmt_expr(fun.value.return_type.clone(), f, i1)?;
    write!(f, "\n")?;

    fmt_expr(fun.value.return_val.clone(), f, i1)?;
    write!(f, "\n{indent})")?;
    Ok(())
}

fn fmt_decreasing_index(
    index: Option<usize>,
    f: &mut Formatter<'_>,
    indent: Indentation,
) -> FmtResult {
    match index {
        Some(index) => write!(f, "{indent}{index}"),
        None => write!(f, "{indent}nonrec"),
    }
}

fn fmt_app(app: RcSemHashed<App>, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    if app.value.args.value.is_empty() {
        write!(f, "{indent}(")?;
        fmt_expr(app.value.callee.clone(), f, indent)?;
        write!(f, ")")?;
        return Ok(());
    }

    write!(f, "{indent}(\n")?;

    let i1 = indent.incremented();
    fmt_expr(app.value.callee.clone(), f, i1)?;

    for arg in app.value.args.value.iter() {
        write!(f, "\n")?;
        fmt_expr(arg.clone(), f, i1)?;
    }

    write!(f, "\n{indent})")?;
    Ok(())
}

fn fmt_for(for_: RcSemHashed<For>, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    let i1 = indent.incremented();
    write!(f, "{indent}(\n{i1}for\n")?;

    fmt_parenthesized_expressions(for_.value.param_types.clone(), f, i1)?;
    write!(f, "\n")?;

    fmt_expr(for_.value.return_type.clone(), f, i1)?;
    write!(f, "\n{indent})")?;
    Ok(())
}

fn fmt_deb(deb: RcSemHashed<DebNode>, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    let index = deb.value.deb.0;
    write!(f, "{indent}{index}")
}

fn fmt_universe(
    universe: RcSemHashed<UniverseNode>,
    f: &mut Formatter<'_>,
    indent: Indentation,
) -> FmtResult {
    let level = universe.value.level.0;
    write!(f, "{indent}Type{level}")
}

fn fmt_parenthesized_expressions(
    parenthesized_expressions: RcSemHashed<Vec<Expr>>,
    f: &mut Formatter<'_>,
    indent: Indentation,
) -> FmtResult {
    if parenthesized_expressions.value.is_empty() {
        return write!(f, "{indent}()");
    }

    write!(f, "{indent}(")?;
    let i1 = indent.incremented();

    for expr in parenthesized_expressions.value.iter() {
        write!(f, "\n")?;
        fmt_expr(expr.clone(), f, i1)?;
    }

    write!(f, "\n{indent})")?;
    Ok(())
}
