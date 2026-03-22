use crate::{
    ast::Expr,
    parser::parse,
    proof::{Model, TautologyOutcome, SatOutcome},
    sequent, tableau,
};

// ---------- テスト用評価器 ----------

fn eval(expr: &Expr, model: &Model) -> bool {
    match expr {
        Expr::Var(name)       => *model.get(name).unwrap_or(&false),
        Expr::Const(b)        => *b,
        Expr::Not(e)          => !eval(e, model),
        Expr::Bin(op, l, r)   => {
            use crate::ast::BinOp;
            match op {
                BinOp::And  => eval(l, model) && eval(r, model),
                BinOp::Or   => eval(l, model) || eval(r, model),
                BinOp::Impl => !eval(l, model) || eval(r, model),
            }
        }
    }
}

// ---------- パーサー ----------

#[test]
fn parse_var() {
    assert_eq!(parse("p").unwrap().to_string(), "p");
}

#[test]
fn parse_not() {
    assert_eq!(parse("not p").unwrap().to_string(), "¬p");
    assert_eq!(parse("¬p").unwrap().to_string(),   "¬p");
    assert_eq!(parse("!p").unwrap().to_string(),    "¬p");
    assert_eq!(parse("NOT p").unwrap().to_string(), "¬p");  // 大文字
}

#[test]
fn parse_and() {
    assert_eq!(parse("p and q").unwrap().to_string(), "(p ∧ q)");
    assert_eq!(parse("p AND q").unwrap().to_string(), "(p ∧ q)");  // 大文字
    assert_eq!(parse("p & q").unwrap().to_string(),   "(p ∧ q)");
    assert_eq!(parse("p ∧ q").unwrap().to_string(),   "(p ∧ q)");
    assert_eq!(parse("p /\\ q").unwrap().to_string(), "(p ∧ q)");
}

#[test]
fn parse_or() {
    assert_eq!(parse("p or q").unwrap().to_string(),  "(p ∨ q)");
    assert_eq!(parse("p OR q").unwrap().to_string(),  "(p ∨ q)");  // 大文字
    assert_eq!(parse("p | q").unwrap().to_string(),   "(p ∨ q)");
    assert_eq!(parse("p ∨ q").unwrap().to_string(),   "(p ∨ q)");
    assert_eq!(parse("p \\/ q").unwrap().to_string(), "(p ∨ q)");
}

#[test]
fn parse_impl() {
    assert_eq!(parse("p -> q").unwrap().to_string(), "(p → q)");
    assert_eq!(parse("p => q").unwrap().to_string(), "(p → q)");
    assert_eq!(parse("p → q").unwrap().to_string(),  "(p → q)");
}

#[test]
fn parse_impl_right_assoc() {
    // p -> q -> r  =  p -> (q -> r)
    assert_eq!(parse("p -> q -> r").unwrap().to_string(), "(p → (q → r))");
}

#[test]
fn parse_and_left_assoc() {
    // p and q and r  =  (p and q) and r
    assert_eq!(parse("p and q and r").unwrap().to_string(), "((p ∧ q) ∧ r)");
}

#[test]
fn parse_or_left_assoc() {
    // p or q or r  =  (p or q) or r
    assert_eq!(parse("p or q or r").unwrap().to_string(), "((p ∨ q) ∨ r)");
}

#[test]
fn parse_precedence() {
    // not p or q and r  =  (¬p) ∨ (q ∧ r)
    assert_eq!(parse("not p or q and r").unwrap().to_string(), "(¬p ∨ (q ∧ r))");
}

#[test]
fn parse_consts() {
    assert_eq!(parse("⊤").unwrap().to_string(),     "⊤");
    assert_eq!(parse("⊥").unwrap().to_string(),     "⊥");
    assert_eq!(parse("true").unwrap().to_string(),  "⊤");
    assert_eq!(parse("false").unwrap().to_string(), "⊥");
    assert_eq!(parse("True").unwrap().to_string(),  "⊤");
    assert_eq!(parse("FALSE").unwrap().to_string(), "⊥");
}

#[test]
fn parse_paren() {
    assert_eq!(parse("(p or q) and r").unwrap().to_string(), "((p ∨ q) ∧ r)");
}

#[test]
fn parse_keywords_not_variables() {
    // キーワードは変数名として使えない
    assert!(parse("and").is_err());
    assert!(parse("or").is_err());
    assert!(parse("not").is_err());
}

#[test]
fn parse_empty() {
    assert!(parse("").is_err());
    assert!(parse("   ").is_err());
}

#[test]
fn parse_error() {
    assert!(parse("p and").is_err());
    assert!(parse("or p q").is_err());
    assert!(parse("p -> -> q").is_err());
    assert!(parse("(p or q").is_err());
}

// ---------- Tableau：恒真 / 非恒真 ----------

fn is_tautology_tableau(s: &str) -> bool {
    tableau::check_tautology(&parse(s).unwrap()).is_tautology()
}

fn is_satisfiable_tableau(s: &str) -> bool {
    tableau::check_satisfiable(&parse(s).unwrap()).is_satisfiable()
}

#[test]
fn tableau_tautologies() {
    assert!(is_tautology_tableau("p or not p"));
    assert!(is_tautology_tableau("not not p -> p"));
    assert!(is_tautology_tableau("⊤"));
    assert!(is_tautology_tableau("not ⊥"));
    assert!(is_tautology_tableau("not (p and q) -> (not p or not q)"));
    assert!(is_tautology_tableau("(p -> q) -> (not q -> not p)"));
    assert!(is_tautology_tableau("(p -> q) and (q -> r) -> (p -> r)"));
    assert!(is_tautology_tableau("(p and q) -> p"));
    assert!(is_tautology_tableau("p -> (p or q)"));
}

#[test]
fn tableau_non_tautologies() {
    assert!(!is_tautology_tableau("p"));
    assert!(!is_tautology_tableau("p and not p"));
    assert!(!is_tautology_tableau("p -> q"));
    assert!(!is_tautology_tableau("p or q"));
    assert!(!is_tautology_tableau("not p"));
}

// ---------- Tableau：反例の正当性（プロパティ検証）----------

fn assert_counterexample_valid(s: &str) {
    let expr = parse(s).unwrap();
    match tableau::check_tautology(&expr) {
        TautologyOutcome::Refuted { model, .. } => {
            assert!(
                !eval(&expr, &model),
                "{s:?} の反例 {model:?} が式を偽にしていない"
            );
        }
        TautologyOutcome::Proved { .. } => panic!("{s:?} は恒真式なので反例は存在しない"),
    }
}

#[test]
fn tableau_counterexample_valid() {
    for s in [
        "p",
        "p -> q",
        "p or q",
        "not p",
        "p and q -> r",
        "(p or q) -> p",
        "p -> q or r",
    ] {
        assert_counterexample_valid(s);
    }
}

// ---------- Tableau：充足可能性 / 充足不能性 ----------

#[test]
fn tableau_satisfiable() {
    assert!(is_satisfiable_tableau("p and q"));
    assert!(is_satisfiable_tableau("p -> q"));
    assert!(is_satisfiable_tableau("p or q"));
    assert!(!is_satisfiable_tableau("p and not p"));
    assert!(!is_satisfiable_tableau("⊥"));
    assert!(!is_satisfiable_tableau("(p -> q) and p and not q"));
}

// ---------- Tableau：充足モデルの正当性（プロパティ検証）----------

fn assert_model_valid(s: &str) {
    let expr = parse(s).unwrap();
    match tableau::check_satisfiable(&expr) {
        SatOutcome::Satisfiable { model, .. } => {
            assert!(
                eval(&expr, &model),
                "{s:?} のモデル {model:?} が式を真にしていない"
            );
        }
        SatOutcome::Unsatisfiable { .. } => panic!("{s:?} は充足可能なはず"),
    }
}

#[test]
fn tableau_model_valid() {
    for s in [
        "p",
        "p and q",
        "p or q",
        "p -> q",
        "not p or not q",
        "p and (q or r)",
        "(p -> q) and (q -> r)",
    ] {
        assert_model_valid(s);
    }
}

// ---------- Sequent ----------

fn is_tautology_sequent(s: &str) -> bool {
    sequent::check_tautology(&parse(s).unwrap()).is_tautology()
}

#[test]
fn sequent_tautologies() {
    assert!(is_tautology_sequent("p or not p"));
    assert!(is_tautology_sequent("not not p -> p"));
    assert!(is_tautology_sequent("⊤"));
    assert!(is_tautology_sequent("not ⊥"));
    assert!(is_tautology_sequent("not (p and q) -> (not p or not q)"));
    assert!(is_tautology_sequent("(p -> q) -> (not q -> not p)"));
    assert!(is_tautology_sequent("(p -> q) and (q -> r) -> (p -> r)"));
    assert!(is_tautology_sequent("(p and q) -> p"));
    assert!(is_tautology_sequent("p -> (p or q)"));
}

#[test]
fn sequent_non_tautologies() {
    assert!(!is_tautology_sequent("p"));
    assert!(!is_tautology_sequent("p and not p"));
    assert!(!is_tautology_sequent("p -> q"));
    assert!(!is_tautology_sequent("p or q"));
    assert!(!is_tautology_sequent("not p"));
}

// ---------- 反駁木の構造 ----------

fn count_open_leaves(node: &crate::proof::ProofNode) -> usize {
    if node.children.is_empty() {
        if !node.closed { 1 } else { 0 }
    } else {
        node.children.iter().map(count_open_leaves).sum()
    }
}

#[test]
fn tableau_refuted_tree_not_closed() {
    for s in ["p", "p -> q", "p or q", "p and q -> r"] {
        let expr = parse(s).unwrap();
        match tableau::check_tautology(&expr) {
            TautologyOutcome::Refuted { tree, .. } => {
                assert!(!tree.closed, "{s:?} の反駁木の根が closed になっている");
                assert!(
                    count_open_leaves(&tree) >= 1,
                    "{s:?} の反駁木に開いた葉がない"
                );
            }
            TautologyOutcome::Proved { .. } => panic!("{s:?} は非恒真のはず"),
        }
    }
}

#[test]
fn tableau_refuted_tree_root_label() {
    // tableau は ¬φ を根にして展開する
    let expr = parse("p -> q").unwrap();
    match tableau::check_tautology(&expr) {
        TautologyOutcome::Refuted { tree, .. } => {
            assert!(
                tree.label.contains('→') || tree.label.contains("->"),
                "根ラベルに元の式が含まれていない: {:?}", tree.label
            );
        }
        _ => panic!(),
    }
}

#[test]
fn sequent_refuted_tree_not_closed() {
    for s in ["p", "p -> q", "p or q"] {
        let expr = parse(s).unwrap();
        match sequent::check_tautology(&expr) {
            TautologyOutcome::Refuted { tree, .. } => {
                assert!(!tree.closed, "{s:?} の sequent 反駁木の根が closed になっている");
                assert!(
                    count_open_leaves(&tree) >= 1,
                    "{s:?} の sequent 反駁木に開いた葉がない"
                );
            }
            TautologyOutcome::Proved { .. } => panic!("{s:?} は非恒真のはず"),
        }
    }
}

#[test]
fn sequent_refuted_tree_root_label() {
    // sequent は ⊢ φ を根にする
    let expr = parse("p -> q").unwrap();
    match sequent::check_tautology(&expr) {
        TautologyOutcome::Refuted { tree, .. } => {
            assert!(
                tree.label.contains('⊢'),
                "根ラベルに ⊢ が含まれていない: {:?}", tree.label
            );
        }
        _ => panic!(),
    }
}

// ---------- Tableau と Sequent の一致 ----------

#[test]
fn tableau_sequent_agree() {
    for s in [
        "p or not p",
        "p and not p",
        "p -> q",
        "not (p and q) -> (not p or not q)",
        "(p -> q) and p -> q",
        "(p -> q) -> (q -> r) -> (p -> r)",
        "p or q",
        "(p and q) -> p",
        "p -> (p or q)",
        "not p or not q",
        "(p or q) -> p",
    ] {
        let e = parse(s).unwrap();
        assert_eq!(
            tableau::check_tautology(&e).is_tautology(),
            sequent::check_tautology(&e).is_tautology(),
            "tableau/sequent が一致しない: {s}"
        );
    }
}
