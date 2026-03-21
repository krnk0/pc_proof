use crate::ast::{BinOp, Expr};
use crate::proof::{Model, TautologyOutcome, SatOutcome, ProofNode};

fn is_literal(e: &Expr) -> bool {
    matches!(e, Expr::Var(_) | Expr::Const(_))
        || matches!(e, Expr::Not(inner) if matches!(inner.as_ref(), Expr::Var(_)))
}

fn negate(e: &Expr) -> Expr {
    match e {
        Expr::Const(b) => Expr::Const(!b),
        Expr::Not(inner) if matches!(inner.as_ref(), Expr::Var(_)) => *inner.clone(),
        _ => Expr::Not(Box::new(e.clone())),
    }
}

#[derive(Debug)]
enum Rule {
    Alpha(Vec<Expr>, &'static str),
    Beta(Vec<Vec<Expr>>, &'static str),
}

fn decompose(e: &Expr) -> Option<Rule> {
    match e {
        Expr::Bin(BinOp::And, l, r) =>
            Some(Rule::Alpha(vec![*l.clone(), *r.clone()], "∧")),
        Expr::Bin(BinOp::Or, l, r) =>
            Some(Rule::Beta(vec![vec![*l.clone()], vec![*r.clone()]], "∨")),
        Expr::Bin(BinOp::Impl, l, r) =>
            Some(Rule::Beta(vec![vec![Expr::Not(l.clone())], vec![*r.clone()]], "→")),
        Expr::Not(inner) => match inner.as_ref() {
            Expr::Not(e2) =>
                Some(Rule::Alpha(vec![*e2.clone()], "¬¬")),
            Expr::Bin(BinOp::And, l, r) =>
                Some(Rule::Beta(vec![vec![Expr::Not(l.clone())], vec![Expr::Not(r.clone())]], "¬∧")),
            Expr::Bin(BinOp::Or, l, r) =>
                Some(Rule::Alpha(vec![Expr::Not(l.clone()), Expr::Not(r.clone())], "¬∨")),
            Expr::Bin(BinOp::Impl, l, r) =>
                Some(Rule::Alpha(vec![*l.clone(), Expr::Not(r.clone())], "¬→")),
            _ => None,
        },
        _ => None,
    }
}

fn fmt_branch(formulas: &[Expr]) -> String {
    if formulas.is_empty() {
        return "∅".to_string();
    }
    formulas.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")
}

fn literals_to_model(literals: &[Expr]) -> Model {
    literals.iter().filter_map(|e| match e {
        Expr::Var(name) => Some((name.clone(), true)),
        Expr::Not(inner) => match inner.as_ref() {
            Expr::Var(name) => Some((name.clone(), false)),
            _ => None,
        },
        _ => None,
    }).collect()
}

fn tableau_rec(mut branch: Vec<Expr>) -> (ProofNode, Option<Vec<Expr>>) {
    // Not(Const) を正規化
    branch = branch.into_iter().map(|e| match e {
        Expr::Not(inner) if matches!(inner.as_ref(), Expr::Const(_)) => {
            if let Expr::Const(b) = *inner { Expr::Const(!b) } else { unreachable!() }
        }
        other => other,
    }).collect();

    let label = fmt_branch(&branch);

    if branch.iter().any(|e| matches!(e, Expr::Const(false))) {
        return (ProofNode::axiom(label, "⊥"), None);
    }
    branch.retain(|e| !matches!(e, Expr::Const(true)));

    let has_contradiction = branch.iter()
        .filter(|e| is_literal(e))
        .any(|l| branch.contains(&negate(l)));
    if has_contradiction {
        return (ProofNode::axiom(label, "Ax"), None);
    }

    if branch.iter().all(is_literal) {
        return (ProofNode::leaf(label, false), Some(branch));
    }

    let pos = branch.iter().position(|e| matches!(decompose(e), Some(Rule::Alpha(..))))
        .or_else(|| branch.iter().position(|e| matches!(decompose(e), Some(Rule::Beta(..)))));
    let pos = pos.expect("non-literal with no decomposition rule");

    let target = branch.remove(pos);
    let rest = branch;

    match decompose(&target).unwrap() {
        Rule::Alpha(additions, rule) => {
            let mut new_branch = rest;
            new_branch.extend(additions);
            let (child, open) = tableau_rec(new_branch);
            let closed = child.closed;
            (ProofNode::node(label, closed, vec![child], rule), open)
        }
        Rule::Beta(branches, rule) => {
            let results: Vec<(ProofNode, Option<Vec<Expr>>)> = branches
                .into_iter()
                .map(|additions| {
                    let mut new_branch = rest.clone();
                    new_branch.extend(additions);
                    tableau_rec(new_branch)
                })
                .collect();

            let open = results.iter().find_map(|(_, o)| o.clone());
            let closed = results.iter().all(|(n, _)| n.closed);
            let children = results.into_iter().map(|(n, _)| n).collect();
            (ProofNode::node(label, closed, children, rule), open)
        }
    }
}

pub fn check_tautology(expr: &Expr) -> TautologyOutcome {
    let negated = Expr::Not(Box::new(expr.clone()));
    let (tree, open) = tableau_rec(vec![negated]);
    if tree.closed {
        TautologyOutcome::Proved { tree }
    } else {
        let model = open.as_deref().map(literals_to_model).unwrap_or_default();
        TautologyOutcome::Refuted { tree, model }
    }
}

pub fn check_satisfiable(expr: &Expr) -> SatOutcome {
    let (tree, open) = tableau_rec(vec![expr.clone()]);
    if tree.closed {
        SatOutcome::Unsatisfiable { tree }
    } else {
        let model = open.as_deref().map(literals_to_model).unwrap_or_default();
        SatOutcome::Satisfiable { tree, model }
    }
}
