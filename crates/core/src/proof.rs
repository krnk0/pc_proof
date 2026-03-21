use std::collections::HashMap;
use serde::Serialize;

pub type Model = HashMap<String, bool>;

/// `check_tautology` の結果
#[derive(Debug, Serialize)]
pub enum TautologyOutcome {
    /// 証明成功
    Proved { tree: ProofNode },
    /// 反例あり（model が反例を与える変数割り当て）
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
    /// 充足可能（model が充足する変数割り当て）
    Satisfiable { tree: ProofNode, model: Model },
    /// 充足不能
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
    /// 表示用ラベル
    pub label: String,
    /// このノード（枝）が閉じているか
    pub closed: bool,
    pub children: Vec<ProofNode>,
}

impl ProofNode {
    pub fn leaf(label: impl Into<String>, closed: bool) -> Self {
        Self { label: label.into(), closed, children: vec![] }
    }

    pub fn node(label: impl Into<String>, closed: bool, children: Vec<ProofNode>) -> Self {
        Self { label: label.into(), closed, children }
    }
}
