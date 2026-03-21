use chumsky::prelude::*;
use chumsky::Stream;
use crate::ast::{BinOp, Expr};

// ---------- Tokens ----------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    True,
    False,
    And,
    Or,
    Not,
    Impl,
    Var(String),
    LParen,
    RParen,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::True     => write!(f, "⊤"),
            Token::False    => write!(f, "⊥"),
            Token::And      => write!(f, "∧"),
            Token::Or       => write!(f, "∨"),
            Token::Not      => write!(f, "¬"),
            Token::Impl     => write!(f, "→"),
            Token::Var(s)   => write!(f, "{s}"),
            Token::LParen   => write!(f, "("),
            Token::RParen   => write!(f, ")"),
        }
    }
}

// ---------- Lexer ----------

fn lexer() -> impl Parser<char, Vec<(Token, std::ops::Range<usize>)>, Error = Simple<char>> {
    // Keywords / identifiers (ASCII only, case-insensitive for keywords)
    let ident = text::ident().map(|s: String| match s.to_ascii_lowercase().as_str() {
        "true"  => Token::True,
        "false" => Token::False,
        "and"   => Token::And,
        "or"    => Token::Or,
        "not"   => Token::Not,
        _       => Token::Var(s),
    });

    // Symbol tokens (longer matches first to avoid prefix ambiguity)
    let symbol = choice((
        just("->").to(Token::Impl),
        just("=>").to(Token::Impl),
        just("/\\").to(Token::And),
        just("\\/").to(Token::Or),
        just('→').to(Token::Impl),
        just('∧').to(Token::And),
        just('∨').to(Token::Or),
        just('¬').to(Token::Not),
        just('!').to(Token::Not),
        just('&').to(Token::And),
        just('|').to(Token::Or),
        just('⊤').to(Token::True),
        just('⊥').to(Token::False),
        just('(').to(Token::LParen),
        just(')').to(Token::RParen),
    ));

    // ident before symbol so "not" is lexed as keyword, not as 'n','o','t'
    choice((ident, symbol))
        .map_with_span(|tok, span| (tok, span))
        .padded()
        .repeated()
}

// ---------- Expression parser (operates on token stream) ----------

fn expr_parser() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    recursive(|formula| {
        let atom = select! {
            Token::True      => Expr::Const(true),
            Token::False     => Expr::Const(false),
            Token::Var(name) => Expr::Var(name),
        }
        .or(formula.clone().delimited_by(just(Token::LParen), just(Token::RParen)));

        // ¬ is prefix, right-recursive: ¬¬p = Not(Not(p))
        let not_expr = just(Token::Not)
            .repeated()
            .then(atom)
            .map(|(nots, e)| nots.into_iter().fold(e, |acc, _| Expr::Not(Box::new(acc))));

        let and_expr = not_expr.clone()
            .then(just(Token::And).ignore_then(not_expr).repeated())
            .foldl(|l, r| Expr::Bin(BinOp::And, Box::new(l), Box::new(r)));

        let or_expr = and_expr.clone()
            .then(just(Token::Or).ignore_then(and_expr).repeated())
            .foldl(|l, r| Expr::Bin(BinOp::Or, Box::new(l), Box::new(r)));

        // → is right-associative: p → q → r = p → (q → r)
        or_expr
            .then(just(Token::Impl).ignore_then(formula).or_not())
            .map(|(l, r)| match r {
                None    => l,
                Some(r) => Expr::Bin(BinOp::Impl, Box::new(l), Box::new(r)),
            })
    })
}

// ---------- Public API ----------

#[derive(Debug)]
pub struct ParseError(pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn parse(input: &str) -> Result<Expr, ParseError> {
    // Stage 1: lex
    let (tokens, lex_errs) = lexer().parse_recovery(input);
    if !lex_errs.is_empty() {
        let msg = lex_errs
            .iter()
            .map(|e| format!("{e}"))
            .collect::<Vec<_>>()
            .join("; ");
        return Err(ParseError(format!("lex error: {msg}")));
    }
    let tokens = tokens.unwrap_or_default();

    // Stage 2: parse
    let len = input.chars().count();
    let stream = Stream::from_iter(len..len + 1, tokens.into_iter());
    let (expr, parse_errs) = expr_parser().then_ignore(end()).parse_recovery(stream);

    match expr {
        Some(e) if parse_errs.is_empty() => Ok(e),
        _ => {
            let msg = parse_errs
                .iter()
                .map(|e| format!("{e}"))
                .collect::<Vec<_>>()
                .join("; ");
            Err(ParseError(format!("parse error: {msg}")))
        }
    }
}
