import pytest
from lark.exceptions import LarkError
from parser import parse, Var, Const, Not, Bin

def test_basic():
   ast = parse("not p or q") 
   assert ast == Bin("OR", Not(Var("p")), Var("q"))

@pytest.mark.parametrize("src, expect", [
    # 単項
    ("not p",                 Not(Var("p"))),

    # 優先順位: NOT > AND > OR
    ("not p or q and r",
        Bin("OR",
            Not(Var("p")),
            Bin("AND", Var("q"), Var("r")))),

    # 含意は右結合
    ("p -> q -> r",
        Bin("IMPL", Var("p"),
            Bin("IMPL", Var("q"), Var("r")))),

    # 括弧で上書き
    ("(p or q) and r",
        Bin("AND",
            Bin("OR", Var("p"), Var("q")),
            Var("r"))),

    # ユニコード演算子
    ("¬p ∨ q",
        Bin("OR", Not(Var("p")), Var("q"))),

    # TRUE / FALSE
    ("⊤ -> ⊥",
        Bin("IMPL", Const(True), Const(False))),

    # Deeply nested parentheses
    ("((p)) -> ((q or r))",
        Bin("IMPL", Var("p"), Bin("OR", Var("q"), Var("r")))),

    # Mixed Unicode and ASCII operators
    ("p ∧ not q",
        Bin("AND", Var("p"), Not(Var("q")))),

    # Negated constants folded to Const
    ("not ⊤", Const(False)),
    ("not ⊥", Const(True)),
])
def test_ok(src, expect):
    assert parse(src) == expect


@pytest.mark.parametrize("bad_src", [
    "p and",            # 不完全
    "or p q",           # 演算子先頭
    "p (and q)",        # 演算子が外
    "p -> -> q",        # 連続含意
])
def test_error(bad_src):
    with pytest.raises(LarkError):
        parse(bad_src)
