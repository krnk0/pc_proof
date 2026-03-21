use crate::ast::{BinOp, Expr};
use crate::proof::{TautologyOutcome, ProofNode};

fn fmt_sequent(left: &[Expr], right: &[Expr]) -> String {
    let l = left.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ");
    let r = right.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ");
    format!("{l} ⊢ {r}")
}

fn is_literal(e: &Expr) -> bool {
    matches!(e, Expr::Var(_) | Expr::Const(_))
}

fn prove_rec(left: Vec<Expr>, right: Vec<Expr>) -> ProofNode {
    let label = fmt_sequent(&left, &right);

    // ⊥ on left → closed
    if left.iter().any(|e| matches!(e, Expr::Const(false))) {
        return ProofNode::leaf(label, true);
    }
    // ⊤ on right → closed
    if right.iter().any(|e| matches!(e, Expr::Const(true))) {
        return ProofNode::leaf(label, true);
    }

    let left: Vec<Expr> = left
        .into_iter()
        .filter(|e| !matches!(e, Expr::Const(true)))
        .collect();
    let right: Vec<Expr> = right
        .into_iter()
        .filter(|e| !matches!(e, Expr::Const(false)))
        .collect();

    // Identity axiom
    if left.iter().any(|e| right.contains(e)) {
        return ProofNode::leaf(label, true);
    }

    // All literals → open (not provable)
    if left.iter().chain(right.iter()).all(is_literal) {
        return ProofNode::leaf(label, false);
    }

    // Expand left
    for i in 0..left.len() {
        if is_literal(&left[i]) {
            continue;
        }
        let e = left[i].clone();
        let rest: Vec<Expr> = left[..i].iter().chain(left[i + 1..].iter()).cloned().collect();

        match &e {
            Expr::Not(inner) => {
                let child = prove_rec(rest, [right.clone(), vec![*inner.clone()]].concat());
                let closed = child.closed;
                return ProofNode::node(label, closed, vec![child]);
            }
            Expr::Bin(BinOp::And, l, r) => {
                let child = prove_rec(
                    [rest.clone(), vec![*l.clone(), *r.clone()]].concat(),
                    right.clone(),
                );
                let closed = child.closed;
                return ProofNode::node(label, closed, vec![child]);
            }
            Expr::Bin(BinOp::Or, l, r) => {
                let c1 = prove_rec([rest.clone(), vec![*l.clone()]].concat(), right.clone());
                let c2 = prove_rec([rest.clone(), vec![*r.clone()]].concat(), right.clone());
                let closed = c1.closed && c2.closed;
                return ProofNode::node(label, closed, vec![c1, c2]);
            }
            Expr::Bin(BinOp::Impl, l, r) => {
                let c1 = prove_rec(rest.clone(), [right.clone(), vec![*l.clone()]].concat());
                let c2 = prove_rec([rest.clone(), vec![*r.clone()]].concat(), right.clone());
                let closed = c1.closed && c2.closed;
                return ProofNode::node(label, closed, vec![c1, c2]);
            }
            _ => unreachable!(),
        }
    }

    // Expand right
    for i in 0..right.len() {
        if is_literal(&right[i]) {
            continue;
        }
        let e = right[i].clone();
        let rest: Vec<Expr> = right[..i].iter().chain(right[i + 1..].iter()).cloned().collect();

        match &e {
            Expr::Not(inner) => {
                let child = prove_rec([left.clone(), vec![*inner.clone()]].concat(), rest);
                let closed = child.closed;
                return ProofNode::node(label, closed, vec![child]);
            }
            Expr::Bin(BinOp::And, l, r) => {
                let c1 = prove_rec(left.clone(), [rest.clone(), vec![*l.clone()]].concat());
                let c2 = prove_rec(left.clone(), [rest.clone(), vec![*r.clone()]].concat());
                let closed = c1.closed && c2.closed;
                return ProofNode::node(label, closed, vec![c1, c2]);
            }
            Expr::Bin(BinOp::Or, l, r) => {
                let child = prove_rec(
                    left.clone(),
                    [rest.clone(), vec![*l.clone(), *r.clone()]].concat(),
                );
                let closed = child.closed;
                return ProofNode::node(label, closed, vec![child]);
            }
            Expr::Bin(BinOp::Impl, l, r) => {
                let child = prove_rec(
                    [left.clone(), vec![*l.clone()]].concat(),
                    [rest.clone(), vec![*r.clone()]].concat(),
                );
                let closed = child.closed;
                return ProofNode::node(label, closed, vec![child]);
            }
            _ => unreachable!(),
        }
    }

    ProofNode::leaf(label, false)
}

pub fn check_tautology(expr: &Expr) -> TautologyOutcome {
    let tree = prove_rec(vec![], vec![expr.clone()]);
    if tree.closed {
        TautologyOutcome::Proved { tree }
    } else {
        // sequent は反例モデルを生成しない（反例が必要なら tableau を使う）
        TautologyOutcome::Refuted { tree, model: Default::default() }
    }
}
