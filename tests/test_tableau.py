import pytest
from tableau import solve

@pytest.mark.pametarize("expr",[
    "p or not p",
    "not not p",
    "not (p and q) -> (not p or not q)",
])
def test_tautology(expr):
    taut, _ = solve(expr)
    assert taut, f"{expr!r} should be tautology"

# ---------- 反例モデルを取れる式 ----------
@pytest.mark.parametrize("expr, expected", [
    ("p and not p",                 {"p": False}),          # 矛盾式
    ("p or q and not p",            {"p": False, "q": True}),
    ("not (p or q)",                {"p": False, "q": False}),
])
def test_counter_example(expr, expected):
    taut, model = solve(expr)
    assert not taut, "should be falsifiable"
    # 期待値のキーだけ比較（余計な変数は don't-care で OK）
    for k, v in expected.items():
        assert model.get(k) == v
