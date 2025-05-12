from parser import parse, Var, Not, Bin
import sys, pprint

def is_literal(node):
    return isinstance(node, Var) or (isinstance(node, Not) and isinstance(node.expr, Var))

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
