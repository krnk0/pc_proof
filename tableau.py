from collections import Counter
from typing import List, Tuple, Union, Optional, cast
from parser import parse, Var, Not, Bin, Expr
import sys, pprint

def is_literal(node):
    return isinstance(node, Var) or (isinstance(node, Not) and isinstance(node.expr, Var))

def negate(lit):
    return lit.expr if isinstance(lit, Not) else Not(lit)

ExprList = List["Expr"]
BranchParts = Union[ExprList, List[ExprList]]  #

def decompose(node: Expr) -> Tuple[Optional[str], Optional[BranchParts]]:
    if isinstance(node, Bin):
        if node.op == "AND":
            return "alpha", [node.left, node.right]
        if node.op == "OR":
            return "beta", [[node.left], [node.right]]
        if node.op == "IMPL":
            return "beta", [[Not(node.left)], [node.right]]
    if isinstance(node, Not):
        inner = node.expr
        if isinstance(inner, Not):
            return "alpha", [inner.expr]
        if isinstance(inner, Bin) and inner.op == "AND":
            return "beta", [[Not(inner.left)], [Not(inner.right)]]
        if isinstance(inner, Bin) and inner.op == "OR":
            return "alpha", [Not(inner.left), Not(inner.right)]
    return None, None   # literal

def tableau(branch):
    lits = [n for n in branch if is_literal(n)]
    # 矛盾してるかチェック, してれば閉じる
    if any(negate(l) in branch for l in lits):
        return "closed", None
    if all(is_literal(n) for n in branch):
        return "open", branch
    # \alpha から探索
    for n in branch:
        typ, _ = decompose(n)
        if typ == "alpha":
            node = n
            break
    else:
        node = next(n for n in branch if decompose(n)[0] == "beta")

    typ, parts = decompose(node)
    branch[node] -= 1
    if branch[node] == 0:
        del branch[node]

    assert parts is not None

    if typ == "alpha":
        for p in parts:
            branch[p] += 1
        return tableau(branch)
    else:
        # beta
        for part in cast(List[List[Expr]], parts):
            new = branch.copy()
            for p in part:
                new[p] += 1
            status, data = tableau(new)
            if status == "open":
                return "open", data
        return "closed", None
        


def solve(text: str):
    ast = parse(text)
    status, data = tableau(Counter([ast]))
    if status == "closed":
        return True, None
    else:
        assert data is not None
        # Counter → dict 形式の反例
        model = {v.name: not isinstance(v, Not) for v in data if is_literal(v)}
        return False, model

if __name__ == '__main__':
    src = sys.argv[1] if len(sys.argv) > 1 else "(p -> q) and p -> q"
    taut, info = solve(src)
    if taut:
        print("Yes – tautology")
    else:
        print("No  – counter-example:", info)
