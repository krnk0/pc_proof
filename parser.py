from dataclasses import dataclass
from pathlib import Path
from typing import cast
from lark import Lark, Transformer, v_args

@dataclass(frozen=True)
class Var:
    name: str

@dataclass(frozen=True)
class Not:
    expr: "Expr"


@dataclass(frozen=True)
class Bin:
    op: str
    left: "Expr"
    right: "Expr"

Expr = Var | Not | Bin


@v_args(inline=True)
class BuildAST(Transformer):
    # トークン
    def NAME(self, tok):            return Var(tok.value)
    def var(self, child):            return child
    def TRUE(self, _):              return Var("⊤")   # constをVarで代用
    def FALSE(self, _):             return Var("⊥")

    # 一項 / 二項
    def not_single(self,_tok, expr):     return Not(expr)
    def and_chain(self, l, _tok, r):      return Bin("AND", l, r)
    def or_chain(self,  l, _tok, r):      return Bin("OR",  l, r)
    def impl_chain(self, l, _tok, r):     return Bin("IMPL", l, r)

    def var(self, child):            return child

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
    文字列 `text` を解析して AST (Var / Not / Bin) を返す
    """
    return cast(Expr, _parser.parse(text))

# ---------------- 簡易 CLI ----------------
if __name__ == "__main__":
    import sys, pprint
    src = sys.argv[1] if len(sys.argv) > 1 else "not p or q -> q"
    pprint.pp(parse(src))
