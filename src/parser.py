from dataclasses import dataclass
from pathlib import Path
from typing import cast
from lark import Lark, Transformer, v_args

@dataclass(frozen=True)
class Var:
    name: str


@dataclass(frozen=True)
class Const:
    value: bool

@dataclass(frozen=True)
class Not:
    expr: "Expr"


@dataclass(frozen=True)
class Bin:
    op: str
    left: "Expr"
    right: "Expr"

Expr = Var | Const | Not | Bin


@v_args(inline=True)
class BuildAST(Transformer):
    # トークン
    def const_true(self, _):              return Const(True)
    def const_false(self, _):             return Const(False)

    # 一項 / 二項
    def not_single(self,_tok, expr):
        if isinstance(expr, Const):
            return Const(not expr.value)
        return Not(expr)
    def and_chain(self, l, _tok, r):      return Bin("AND", l, r)
    def or_chain(self,  l, _tok, r):      return Bin("OR",  l, r)
    def impl_chain(self, l, _tok, r):     return Bin("IMPL", l, r)

    def var(self, tok):            return Var(tok.value)

    # 折り畳みルール
    def impl_single(self, expr):    return expr
    def paren(self, expr):          return expr

_GRAMMER_PATH = Path(__file__).with_name("logic.lark")
_parser = Lark.open(
    str(_GRAMMER_PATH),
    parser="lalr",
    transformer=BuildAST(),
    cache=True
)

def parse(text: str) -> Expr:
    """
    文字列 `text` を解析して AST (Var / Const / Not / Bin) を返す
    """
    return cast(Expr, _parser.parse(text))

