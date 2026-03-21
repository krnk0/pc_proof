use std::collections::HashMap;
use serde::Serialize;

pub type Model = HashMap<String, bool>;

/// 証明器が返す結果
#[derive(Debug, Serialize)]
pub enum Outcome {
    /// 恒真式（証明成功）
    Tautology { tree: ProofNode },
    /// 反例あり
    Refuted { tree: ProofNode, model: Model },
}

impl Outcome {
    pub fn is_tautology(&self) -> bool {
        matches!(self, Outcome::Tautology { .. })
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
