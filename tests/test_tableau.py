import pytest
from tableau import solve
from parser import parse, Var, Not, Bin

def eval_ast(node, env):
    if isinstance(node, Var):
        return env.get(node.name, False)
    if isinstance(node, Not):
        return not eval_ast(node.expr, env)
    if isinstance(node, Bin):
        left  = eval_ast(node.left,  env)
        right = eval_ast(node.right, env)
        if node.op == "AND":  return left and right
        if node.op == "OR":   return left or  right
        if node.op == "IMPL": return (not left) or right
    raise ValueError("unknown node")

@pytest.mark.parametrize("expr",[
    "p or not p",
    "not not p -> p",
#    "not (p and q) -> (not p or not q)",
])
def test_tautology(expr):
    taut, _ = solve(expr, tautology=True)
    assert taut, f"{expr!r} should be tautology"

# ---------- 反例モデルを取れる式 ----------
@pytest.mark.parametrize("expr", [
    "p and not p",          # 矛盾式
    "p or q and not p",
    "not (p or q)",
])
def test_counter_example(expr):
    taut, model = solve(expr, tautology=True)
    assert not taut, "should be falsifiable"

    ast = parse(expr)
    assert eval_ast(ast, model) is False
