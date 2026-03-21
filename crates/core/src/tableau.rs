use crate::ast::{BinOp, Expr};
use crate::proof::{Model, Outcome, ProofNode};

fn is_literal(e: &Expr) -> bool {
    matches!(e, Expr::Var(_) | Expr::Const(_))
        || matches!(e, Expr::Not(inner) if matches!(inner.as_ref(), Expr::Var(_)))
}

fn negate(e: &Expr) -> Expr {
    match e {
        Expr::Const(b)                                => Expr::Const(!b),
        Expr::Not(inner) if matches!(inner.as_ref(), Expr::Var(_)) => *inner.clone(),
        _ => Expr::Not(Box::new(e.clone())),
    }
}

#[derive(Debug)]
enum Rule {
    Alpha(Vec<Expr>),        // adds all to the same branch
    Beta(Vec<Vec<Expr>>),    // creates one child branch per element
}

fn decompose(e: &Expr) -> Option<Rule> {
    match e {
        Expr::Bin(BinOp::And, l, r) =>
            Some(Rule::Alpha(vec![*l.clone(), *r.clone()])),
        Expr::Bin(BinOp::Or, l, r) =>
            Some(Rule::Beta(vec![vec![*l.clone()], vec![*r.clone()]])),
        Expr::Bin(BinOp::Impl, l, r) =>
            Some(Rule::Beta(vec![vec![Expr::Not(l.clone())], vec![*r.clone()]])),
        Expr::Not(inner) => match inner.as_ref() {
            Expr::Not(e2) =>
                Some(Rule::Alpha(vec![*e2.clone()])),
            Expr::Bin(BinOp::And, l, r) =>
                Some(Rule::Beta(vec![vec![Expr::Not(l.clone())], vec![Expr::Not(r.clone())]])),
            Expr::Bin(BinOp::Or, l, r) =>
                Some(Rule::Alpha(vec![Expr::Not(l.clone()), Expr::Not(r.clone())])),
            Expr::Bin(BinOp::Impl, l, r) =>
                Some(Rule::Alpha(vec![*l.clone(), Expr::Not(r.clone())])),
            _ => None,
        },
        _ => None,
    }
}

fn fmt_branch(formulas: &[Expr]) -> String {
    formulas.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")
}

/// `branch` は現在の枝にある式のリスト
fn tableau_rec(mut branch: Vec<Expr>) -> ProofNode {
    // Not(Const) を正規化
    branch = branch
        .into_iter()
        .map(|e| match e {
            Expr::Not(inner) if matches!(inner.as_ref(), Expr::Const(_)) => {
                if let Expr::Const(b) = *inner { Expr::Const(!b) } else { unreachable!() }
            }
            other => other,
        })
        .collect();

    let label = fmt_branch(&branch);

    // ⊥ → closed
    if branch.iter().any(|e| matches!(e, Expr::Const(false))) {
        return ProofNode::leaf(label, true);
    }
    // ⊤ → remove and continue
    branch.retain(|e| !matches!(e, Expr::Const(true)));

    let literals: Vec<&Expr> = branch.iter().filter(|e| is_literal(e)).collect();

    // Contradiction among literals → closed
    if literals.iter().any(|l| branch.contains(&negate(l))) {
        return ProofNode::leaf(label, true);
    }

    // All literals → open; extract model
    if branch.iter().all(is_literal) {
        let model: Model = branch
            .iter()
            .filter_map(|e| match e {
                Expr::Var(name) => Some((name.clone(), true)),
                Expr::Not(inner) => {
                    if let Expr::Var(name) = inner.as_ref() {
                        Some((name.clone(), false))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();
        // Store model in a global-like via a special leaf representation;
        // we bubble it up via the return value separately.
        let _ = model; // handled by the caller via collect_model
        return ProofNode::leaf(label, false);
    }

    // Find an alpha rule first, then beta
    let pos = branch.iter().position(|e| matches!(decompose(e), Some(Rule::Alpha(_))))
        .or_else(|| branch.iter().position(|e| matches!(decompose(e), Some(Rule::Beta(_)))));

    let pos = pos.expect("non-literal with no decomposition rule");
    let target = branch.remove(pos);
    let rest = branch;

    match decompose(&target).unwrap() {
        Rule::Alpha(additions) => {
            let mut new_branch = rest;
            new_branch.extend(additions);
            let child = tableau_rec(new_branch);
            let closed = child.closed;
            ProofNode::node(label, closed, vec![child])
        }
        Rule::Beta(branches) => {
            let children: Vec<ProofNode> = branches
                .into_iter()
                .map(|additions| {
                    let mut new_branch = rest.clone();
                    new_branch.extend(additions);
                    tableau_rec(new_branch)
                })
                .collect();
            let closed = children.iter().all(|c| c.closed);
            ProofNode::node(label, closed, children)
        }
    }
}

/// 開いた葉からモデルを収集する
fn collect_model(node: &ProofNode, _branch_formulas: Option<&str>) -> Option<Model> {
    if node.children.is_empty() && !node.closed {
        // open leaf: parse label back into model
        let model = node.label.split(", ").filter_map(|s| {
            let s = s.trim();
            if s.starts_with('¬') {
                let name = s.trim_start_matches('¬');
                Some((name.to_string(), false))
            } else if !s.is_empty() && !matches!(s, "⊤" | "⊥") {
                Some((s.to_string(), true))
            } else {
                None
            }
        }).collect();
        return Some(model);
    }
    for child in &node.children {
        if let Some(m) = collect_model(child, None) {
            return Some(m);
        }
    }
    None
}

pub fn check_tautology(expr: &Expr) -> Outcome {
    // Negate the formula; if the negation is unsatisfiable → tautology
    let negated = Expr::Not(Box::new(expr.clone()));
    let tree = tableau_rec(vec![negated]);
    if tree.closed {
        Outcome::Tautology { tree }
    } else {
        let model = collect_model(&tree, None).unwrap_or_default();
        Outcome::Refuted { tree, model }
    }
}

pub fn check_satisfiable(expr: &Expr) -> Outcome {
    let tree = tableau_rec(vec![expr.clone()]);
    if tree.closed {
        // unsatisfiable → treat as "refuted" (no model)
        Outcome::Refuted { tree, model: Default::default() }
    } else {
        let _model = collect_model(&tree, None).unwrap_or_default();
        // "Tautology" slot reused to mean "satisfiable witness found"
        Outcome::Tautology { tree }
    }
}
