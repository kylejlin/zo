use super::*;

use crate::syntax_tree::minimal_ast::*;

impl Display for PrettyPrint<'_, Expr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_expr(self.0.clone(), f, Indentation { soft_tab_count: 0 })
    }
}

impl Display for PrettyPrint<'_, Ind> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_ind(&self.0, f, Indentation { soft_tab_count: 0 })
    }
}

impl Display for PrettyPrint<'_, [VconDef]> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_parenthesized_vcon_defs(self.0.clone(), f, Indentation { soft_tab_count: 0 })
    }
}

impl Display for PrettyPrint<'_, VconDef> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_vcon_def(self.0, f, Indentation { soft_tab_count: 0 })
    }
}

impl Display for PrettyPrint<'_, Vcon> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_vcon(&self.0, f, Indentation { soft_tab_count: 0 })
    }
}

impl Display for PrettyPrint<'_, Match> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_match(&self.0, f, Indentation { soft_tab_count: 0 })
    }
}

impl Display for PrettyPrint<'_, [MatchCase]> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_parenthesized_match_cases(self.0.clone(), f, Indentation { soft_tab_count: 0 })
    }
}

impl Display for PrettyPrint<'_, MatchCase> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_match_case(self.0, f, Indentation { soft_tab_count: 0 })
    }
}

impl Display for PrettyPrint<'_, Fun> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_fun(&self.0, f, Indentation { soft_tab_count: 0 })
    }
}

impl Display for PrettyPrint<'_, App> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_app(&self.0, f, Indentation { soft_tab_count: 0 })
    }
}

impl Display for PrettyPrint<'_, For> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_for(&self.0, f, Indentation { soft_tab_count: 0 })
    }
}

impl Display for PrettyPrint<'_, DebNode> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_deb_node(&self.0, f, Indentation { soft_tab_count: 0 })
    }
}

impl Display for PrettyPrint<'_, UniverseNode> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_universe_node(&self.0, f, Indentation { soft_tab_count: 0 })
    }
}

// Every `fmt_{node}` function writes an indent
// at the beginning.
// It does _not_ write a newline at the end.

fn fmt_expr(expr: Expr, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    match expr {
        Expr::Ind(e) => fmt_ind(&e.hashee, f, indent),
        Expr::Vcon(e) => fmt_vcon(&e.hashee, f, indent),
        Expr::Match(e) => fmt_match(&e.hashee, f, indent),
        Expr::Fun(e) => fmt_fun(&e.hashee, f, indent),
        Expr::App(e) => fmt_app(&e.hashee, f, indent),
        Expr::For(e) => fmt_for(&e.hashee, f, indent),
        Expr::Deb(e) => fmt_deb_node(&e.hashee, f, indent),
        Expr::Universe(e) => fmt_universe_node(&e.hashee, f, indent),
    }
}

fn fmt_ind(ind: &Ind, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    let i1 = indent.incremented();
    write!(f, "{indent}(\n{i1}ind\n")?;
    fmt_universe(ind.universe, f, i1)?;
    write!(f, "\n")?;

    fmt_str_literal(&ind.name, f, i1)?;
    write!(f, "\n")?;

    fmt_parenthesized_expressions(ind.index_types.clone(), f, i1)?;
    write!(f, "\n")?;

    fmt_parenthesized_vcon_defs(&ind.vcon_defs.hashee, f, i1)?;
    write!(f, "\n{indent})")?;

    Ok(())
}

fn fmt_universe(universe: Universe, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    let level = universe.level.0;
    let set_or_prop = if universe.erasable { "Prop" } else { "Set" };
    write!(f, "{indent}{set_or_prop}{level}")
}

fn fmt_str_literal(
    str_literal: &StringValue,
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
    defs: &[VconDef],
    f: &mut Formatter<'_>,
    indent: Indentation,
) -> FmtResult {
    if defs.is_empty() {
        return write!(f, "{indent}()");
    }

    write!(f, "{indent}(")?;
    let i1 = indent.incremented();

    for def in defs.iter() {
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

fn fmt_vcon(vcon: &Vcon, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    let i1 = indent.incremented();
    write!(f, "{indent}(\n{i1}vcon\n")?;

    fmt_ind(&vcon.ind.hashee, f, i1)?;

    let vcon_index = vcon.vcon_index;
    write!(f, "\n{i1}{vcon_index}\n{indent})")?;
    Ok(())
}

fn fmt_match(m: &Match, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    let i1 = indent.incremented();
    write!(f, "{indent}(\n{i1}match\n")?;

    fmt_expr(m.matchee.clone(), f, i1)?;
    write!(f, "\n")?;

    let return_type_arity = m.return_type_arity;
    write!(f, "{i1}{return_type_arity}\n")?;

    fmt_expr(m.return_type.clone(), f, i1)?;
    write!(f, "\n")?;

    fmt_parenthesized_match_cases(&m.cases.hashee, f, i1)?;
    write!(f, "\n{indent})")?;
    Ok(())
}

fn fmt_parenthesized_match_cases(
    cases: &[MatchCase],
    f: &mut Formatter<'_>,
    indent: Indentation,
) -> FmtResult {
    if cases.is_empty() {
        return write!(f, "{indent}()");
    }

    write!(f, "{indent}(")?;
    let i1 = indent.incremented();

    for case in cases.iter() {
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

fn fmt_fun(fun: &Fun, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    let i1 = indent.incremented();
    write!(f, "{indent}(\n{i1}fun\n")?;

    fmt_decreasing_index(fun.decreasing_index, f, i1)?;
    write!(f, "\n")?;

    fmt_parenthesized_expressions(fun.param_types.clone(), f, i1)?;
    write!(f, "\n")?;

    fmt_expr(fun.return_type.clone(), f, i1)?;
    write!(f, "\n")?;

    fmt_expr(fun.return_val.clone(), f, i1)?;
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

fn fmt_app(app: &App, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    if app.args.hashee.is_empty() {
        write!(f, "{indent}(")?;
        fmt_expr(app.callee.clone(), f, indent)?;
        write!(f, ")")?;
        return Ok(());
    }

    write!(f, "{indent}(\n")?;

    let i1 = indent.incremented();
    fmt_expr(app.callee.clone(), f, i1)?;

    for arg in app.args.hashee.iter() {
        write!(f, "\n")?;
        fmt_expr(arg.clone(), f, i1)?;
    }

    write!(f, "\n{indent})")?;
    Ok(())
}

fn fmt_for(for_: &For, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    let i1 = indent.incremented();
    write!(f, "{indent}(\n{i1}for\n")?;

    fmt_parenthesized_expressions(for_.param_types.clone(), f, i1)?;
    write!(f, "\n")?;

    fmt_expr(for_.return_type.clone(), f, i1)?;
    write!(f, "\n{indent})")?;
    Ok(())
}

fn fmt_deb_node(node: &DebNode, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    let index = node.deb.0;
    write!(f, "{indent}{index}")
}

fn fmt_universe_node(node: &UniverseNode, f: &mut Formatter<'_>, indent: Indentation) -> FmtResult {
    fmt_universe(node.universe, f, indent)
}

fn fmt_parenthesized_expressions(
    parenthesized_expressions: RcHashedVec<Expr>,
    f: &mut Formatter<'_>,
    indent: Indentation,
) -> FmtResult {
    if parenthesized_expressions.hashee.is_empty() {
        return write!(f, "{indent}()");
    }

    write!(f, "{indent}(")?;
    let i1 = indent.incremented();

    for expr in parenthesized_expressions.hashee.iter() {
        write!(f, "\n")?;
        fmt_expr(expr.clone(), f, i1)?;
    }

    write!(f, "\n{indent})")?;
    Ok(())
}
