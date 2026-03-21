use std::fmt;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum BinOp {
    And,
    Or,
    Impl,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Expr {
    Var(String),
    Const(bool),
    Not(Box<Expr>),
    Bin(BinOp, Box<Expr>, Box<Expr>),
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::And  => write!(f, "∧"),
            BinOp::Or   => write!(f, "∨"),
            BinOp::Impl => write!(f, "→"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Var(name)        => write!(f, "{name}"),
            Expr::Const(true)      => write!(f, "⊤"),
            Expr::Const(false)     => write!(f, "⊥"),
            Expr::Not(e)           => write!(f, "¬{e}"),
            Expr::Bin(op, l, r)    => write!(f, "({l} {op} {r})"),
        }
    }
}
