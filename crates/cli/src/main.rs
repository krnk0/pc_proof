use clap::Parser;
use pc_proof_core::{
    parser::parse,
    proof::{ProofNode, TautologyOutcome},
    sequent, tableau,
};
use std::process;

#[derive(Parser)]
#[command(name = "pc-proof", about = "Propositional logic tautology checker")]
struct Cli {
    /// Formula to check (e.g. "p or not p", or pass as multiple args)
    #[arg(required = true)]
    formula: Vec<String>,

    /// Proof method
    #[arg(long, value_name = "METHOD", default_value = "tableau")]
    method: Method,

    /// Show proof tree (only when tautology is proved)
    #[arg(short, long)]
    tree: bool,
}

#[derive(clap::ValueEnum, Clone)]
enum Method {
    Tableau,
    Sequent,
}

// ---------- tableau: top-down tree ----------

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

// ---------- sequent: bottom-up numbered list ----------

fn collect_steps<'a>(node: &'a ProofNode, steps: &mut Vec<(&'a ProofNode, Vec<usize>)>) -> usize {
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
        let rule = n.rule.as_deref().unwrap_or("—");
        let refs_str = if refs.is_empty() {
            String::new()
        } else {
            format!("  ← ({})", refs.iter().map(|r| r.to_string()).collect::<Vec<_>>().join(", "))
        };
        let status = if n.closed { "✓" } else { "✗" };
        println!("  {:>2}. [{}] {:<48} {:<4}{}", i + 1, status, n.label, rule, refs_str);
    }
}

// ---------- main ----------

fn main() {
    let cli = Cli::parse();
    let formula = cli.formula.join(" ");

    let expr = match parse(&formula) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(2);
        }
    };

    let outcome = match cli.method {
        Method::Tableau => tableau::check_tautology(&expr),
        Method::Sequent => sequent::check_tautology(&expr),
    };

    match &outcome {
        TautologyOutcome::Proved { tree } => {
            println!("✓  Tautology");
            if cli.tree {
                match cli.method {
                    Method::Tableau => print_tableau(tree, "   ", true),
                    Method::Sequent => print_sequent(tree),
                }
            }
            process::exit(0);
        }
        TautologyOutcome::Refuted { model, .. } => {
            let ce: Vec<String> = {
                let mut v: Vec<_> = model.iter().collect();
                v.sort_by_key(|(k, _)| k.as_str());
                v.iter().map(|(k, v)| format!("{k}={v}")).collect()
            };
            if ce.is_empty() {
                println!("✗  Not a tautology");
            } else {
                println!("✗  Not a tautology  (counterexample: {})", ce.join(", "));
            }
            process::exit(1);
        }
    }
}
