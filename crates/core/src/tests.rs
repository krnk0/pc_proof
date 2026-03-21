use crate::{parser::parse, proof::Outcome, sequent, tableau};

// ---------- Parser ----------

#[test]
fn parse_var() {
    let e = parse("p").unwrap();
    assert_eq!(e.to_string(), "p");
}

#[test]
fn parse_not() {
    assert_eq!(parse("not p").unwrap().to_string(), "¬p");
    assert_eq!(parse("¬p").unwrap().to_string(), "¬p");
    assert_eq!(parse("!p").unwrap().to_string(), "¬p");
}

#[test]
fn parse_and() {
    assert_eq!(parse("p and q").unwrap().to_string(), "(p ∧ q)");
    assert_eq!(parse("p & q").unwrap().to_string(), "(p ∧ q)");
    assert_eq!(parse("p ∧ q").unwrap().to_string(), "(p ∧ q)");
    assert_eq!(parse("p /\\ q").unwrap().to_string(), "(p ∧ q)");
}

#[test]
fn parse_or() {
    assert_eq!(parse("p or q").unwrap().to_string(), "(p ∨ q)");
    assert_eq!(parse("p | q").unwrap().to_string(), "(p ∨ q)");
    assert_eq!(parse("p ∨ q").unwrap().to_string(), "(p ∨ q)");
    assert_eq!(parse("p \\/ q").unwrap().to_string(), "(p ∨ q)");
}

#[test]
fn parse_impl() {
    assert_eq!(parse("p -> q").unwrap().to_string(), "(p → q)");
    assert_eq!(parse("p => q").unwrap().to_string(), "(p → q)");
    assert_eq!(parse("p → q").unwrap().to_string(), "(p → q)");
}

#[test]
fn parse_impl_right_assoc() {
    // p -> q -> r  should parse as  p -> (q -> r)
    assert_eq!(parse("p -> q -> r").unwrap().to_string(), "(p → (q → r))");
}

#[test]
fn parse_precedence() {
    // not p or q and r  =  (¬p) ∨ (q ∧ r)
    assert_eq!(
        parse("not p or q and r").unwrap().to_string(),
        "(¬p ∨ (q ∧ r))"
    );
}

#[test]
fn parse_consts() {
    assert_eq!(parse("⊤").unwrap().to_string(), "⊤");
    assert_eq!(parse("⊥").unwrap().to_string(), "⊥");
    assert_eq!(parse("true").unwrap().to_string(), "⊤");
    assert_eq!(parse("false").unwrap().to_string(), "⊥");
    assert_eq!(parse("True").unwrap().to_string(), "⊤");
    assert_eq!(parse("FALSE").unwrap().to_string(), "⊥");
}

#[test]
fn parse_paren() {
    assert_eq!(parse("(p or q) and r").unwrap().to_string(), "((p ∨ q) ∧ r)");
}

#[test]
fn parse_error() {
    assert!(parse("p and").is_err());
    assert!(parse("or p q").is_err());
    assert!(parse("p -> -> q").is_err());
}

// ---------- Tableau ----------

fn is_tautology_tableau(s: &str) -> bool {
    let e = parse(s).unwrap();
    tableau::check_tautology(&e).is_tautology()
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
}

#[test]
fn tableau_non_tautologies() {
    assert!(!is_tautology_tableau("p"));
    assert!(!is_tautology_tableau("p and not p"));
    assert!(!is_tautology_tableau("p -> q"));
}

#[test]
fn tableau_counterexample() {
    let e = parse("p -> q").unwrap();
    match tableau::check_tautology(&e) {
        Outcome::Refuted { model, .. } => {
            // model must falsify p -> q  i.e.  p=true, q=false
            assert_eq!(model.get("p"), Some(&true));
            assert_eq!(model.get("q"), Some(&false));
        }
        _ => panic!("expected refuted"),
    }
}

// ---------- Sequent ----------

fn is_tautology_sequent(s: &str) -> bool {
    let e = parse(s).unwrap();
    sequent::check_tautology(&e).is_tautology()
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
}

#[test]
fn sequent_non_tautologies() {
    assert!(!is_tautology_sequent("p"));
    assert!(!is_tautology_sequent("p and not p"));
    assert!(!is_tautology_sequent("p -> q"));
}

// ---------- Tableau と Sequent の一致 ----------

#[test]
fn tableau_sequent_agree() {
    let cases = [
        "p or not p",
        "p and not p",
        "p -> q",
        "not (p and q) -> (not p or not q)",
        "(p -> q) and p -> q",
        "(p -> q) -> (q -> r) -> (p -> r)",
        "p or q",
    ];
    for s in cases {
        let e = parse(s).unwrap();
        assert_eq!(
            tableau::check_tautology(&e).is_tautology(),
            sequent::check_tautology(&e).is_tautology(),
            "tableau/sequent disagree on: {s}"
        );
    }
}
