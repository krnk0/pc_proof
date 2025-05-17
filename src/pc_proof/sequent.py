from __future__ import annotations
from typing import List
from .parser import parse, Var, Const, Not, Bin, Expr


def is_literal(node: Expr) -> bool:
    """Return True for atomic formulas.

    For the sequent calculus implementation we only treat variables and
    constants as literals so that negated variables are expanded using the
    logical rules.
    """
    return isinstance(node, (Var, Const))


def _prove(left: List[Expr], right: List[Expr]) -> bool:
    left = list(left)
    right = list(right)

    # simplify constants
    if any(isinstance(e, Const) and e.value is False for e in left):
        return True
    if any(isinstance(e, Const) and e.value is True for e in right):
        return True

    left = [e for e in left if not (isinstance(e, Const) and e.value is True)]
    right = [e for e in right if not (isinstance(e, Const) and e.value is False)]

    # identity
    for e in left:
        if e in right:
            return True

    # atomic failure
    if all(is_literal(e) for e in left + right):
        return False

    # expand left formulas
    for i, e in enumerate(left):
        if is_literal(e):
            continue
        rest = left[:i] + left[i + 1 :]
        if isinstance(e, Not):
            return _prove(rest, right + [e.expr])
        if isinstance(e, Bin):
            if e.op == "AND":
                return _prove(rest + [e.left, e.right], right)
            if e.op == "OR":
                return _prove(rest + [e.left], right) and _prove(rest + [e.right], right)
            if e.op == "IMPL":
                return _prove(rest, right + [e.left]) and _prove(rest + [e.right], right)

    # expand right formulas
    for i, e in enumerate(right):
        if is_literal(e):
            continue
        rest = right[:i] + right[i + 1 :]
        if isinstance(e, Not):
            return _prove(left + [e.expr], rest)
        if isinstance(e, Bin):
            if e.op == "AND":
                return _prove(left, rest + [e.left]) and _prove(left, rest + [e.right])
            if e.op == "OR":
                return _prove(left, rest + [e.left, e.right])
            if e.op == "IMPL":
                return _prove(left + [e.left], rest + [e.right])

    return False


def prove(text: str) -> bool:
    ast = parse(text)
    return _prove([], [ast])


if __name__ == "__main__":
    import sys

    src = sys.argv[1] if len(sys.argv) > 1 else "(p -> q) and p -> q"
    if prove(src):
        print("Provable")
    else:
        print("Not provable")
