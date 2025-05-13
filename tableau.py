from parser import parse, Var, Not, Bin
import sys, pprint

def is_literal(node):
    return isinstance(node, Var) or (isinstance(node, Not) and isinstance(node.expr, Var))

def negate(lit):
    return lit.expr if isinstance(lit, Not) else Not(lit)

def decompose(node):
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


def solve(text: str):
    ast = parse(text)
    status, data = tableau(Counter([ast]))
    if status == "closed"
        return True, None
    else
        return False, None

if __name__ == '__main__':
    src = sys.argv[1] if len(sys.argv) > 1 else "(p -> q) and p -> q"
    taut, info = solve(src)
