use std::collections::HashMap;
use serde::Serialize;

pub type Model = HashMap<String, bool>;

/// `check_tautology` の結果
#[derive(Debug, Serialize)]
pub enum TautologyOutcome {
    Proved { tree: ProofNode },
    Refuted { tree: ProofNode, model: Model },
}

impl TautologyOutcome {
    pub fn is_tautology(&self) -> bool {
        matches!(self, TautologyOutcome::Proved { .. })
    }
}

/// `check_satisfiable` の結果
#[derive(Debug, Serialize)]
pub enum SatOutcome {
    Satisfiable { tree: ProofNode, model: Model },
    Unsatisfiable { tree: ProofNode },
}

impl SatOutcome {
    pub fn is_satisfiable(&self) -> bool {
        matches!(self, SatOutcome::Satisfiable { .. })
    }
}

/// 証明木の単一ノード（sequent / tableau 共通）
#[derive(Debug, Serialize)]
pub struct ProofNode {
    pub label: String,
    pub closed: bool,
    pub children: Vec<ProofNode>,
    /// このノードで適用した規則名
    pub rule: Option<String>,
}

impl ProofNode {
    /// 開いた葉（証明失敗）または正規化後の中間状態
    pub fn leaf(label: impl Into<String>, closed: bool) -> Self {
        Self { label: label.into(), closed, children: vec![], rule: None }
    }

    /// 公理・矛盾など規則名付きの閉じた葉
    pub fn axiom(label: impl Into<String>, rule: impl Into<String>) -> Self {
        Self { label: label.into(), closed: true, children: vec![], rule: Some(rule.into()) }
    }

    /// 規則を適用した内部ノード
    pub fn node(
        label: impl Into<String>,
        closed: bool,
        children: Vec<ProofNode>,
        rule: impl Into<String>,
    ) -> Self {
        Self { label: label.into(), closed, children, rule: Some(rule.into()) }
    }
}
