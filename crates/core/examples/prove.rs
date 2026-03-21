use pc_proof_core::{
    parser::parse,
    proof::{ProofNode, TautologyOutcome},
    tableau, sequent,
};

// ---------- タブロー：上から下のツリー + 規則名 ----------

fn print_tableau(node: &ProofNode, prefix: &str, is_last: bool) {
    let connector = if is_last { "└─" } else { "├─" };
    let status = if node.closed { "✓" } else { "✗" };
    let rule = node.rule.as_ref()
        .map(|r| format!("  [{r}]"))
        .unwrap_or_default();
    println!("{prefix}{connector} [{status}] {}{rule}", node.label);

    let child_prefix = format!("{}{}", prefix, if is_last { "   " } else { "│  " });
    for (i, child) in node.children.iter().enumerate() {
        print_tableau(child, &child_prefix, i == node.children.len() - 1);
    }
}

// ---------- シーケント：番号付き bottom-up リスト ----------

fn collect_steps<'a>(
    node: &'a ProofNode,
    steps: &mut Vec<(&'a ProofNode, Vec<usize>)>,
) -> usize {
    let child_indices: Vec<usize> = node.children.iter()
        .map(|child| collect_steps(child, steps))
        .collect();
    let idx = steps.len() + 1;
    steps.push((node, child_indices));
    idx
}

fn print_sequent(node: &ProofNode) {
    let mut steps = vec![];
    collect_steps(node, &mut steps);

    for (i, (n, refs)) in steps.iter().enumerate() {
        let step = i + 1;
        let rule = n.rule.as_deref().unwrap_or("—");
        let refs_str = if refs.is_empty() {
            String::new()
        } else {
            format!(
                "  ← ({})",
                refs.iter().map(|r| r.to_string()).collect::<Vec<_>>().join(", ")
            )
        };
        let status = if n.closed { "✓" } else { "✗" };
        println!("  {:>2}. [{}] {:<48} {:<4}{}", step, status, n.label, rule, refs_str);
    }
}

// ---------- メイン ----------

fn run(formula: &str) {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Formula: {formula}");

    let expr = match parse(formula) {
        Ok(e) => e,
        Err(e) => { println!("  Parse error: {e}"); return; }
    };

    // Tableau
    println!("\n[Tableau]");
    match tableau::check_tautology(&expr) {
        TautologyOutcome::Proved { tree } => {
            println!("✓  Tautology");
            print_tableau(&tree, "   ", true);
        }
        TautologyOutcome::Refuted { tree, model } => {
            let ce: Vec<String> = {
                let mut v: Vec<_> = model.iter().collect();
                v.sort_by_key(|(k, _)| k.as_str());
                v.iter().map(|(k, v)| format!("{k}={v}")).collect()
            };
            println!("✗  Not a tautology  (counterexample: {})", ce.join(", "));
            print_tableau(&tree, "   ", true);
        }
    }

    // Sequent
    println!("\n[Sequent]");
    match sequent::check_tautology(&expr) {
        TautologyOutcome::Proved { tree } => {
            println!("✓  Provable");
            print_sequent(&tree);
        }
        TautologyOutcome::Refuted { tree, .. } => {
            println!("✗  Not provable");
            print_sequent(&tree);
        }
    }
    println!();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        run(&args[1..].join(" "));
    } else {
        run("p or not p");
        run("not (p and q) -> (not p or not q)");
        run("p -> q");
    }
}
