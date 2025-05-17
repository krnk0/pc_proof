from __future__ import annotations
from itertools import count
from typing import List, Tuple

from graphviz import Digraph

from parser import parse, Var, Const, Not, Bin, Expr
from sequent import is_literal


_counter = count()


def _fmt(expr: Expr) -> str:
    if isinstance(expr, Var):
        return expr.name
    if isinstance(expr, Const):
        return "⊤" if expr.value else "⊥"
    if isinstance(expr, Not):
        return f"¬{_fmt(expr.expr)}"
    if isinstance(expr, Bin):
        op = {"AND": "∧", "OR": "∨", "IMPL": "→"}[expr.op]
        return f"({_fmt(expr.left)} {op} {_fmt(expr.right)})"
    raise ValueError(f"unknown expr: {expr}")


def _fmt_list(lst: List[Expr]) -> str:
    if not lst:
        return ""
    return ", ".join(_fmt(e) for e in lst)


def _prove_graph(left: List[Expr], right: List[Expr], g: Digraph) -> Tuple[bool, str]:
    node_id = str(next(_counter))
    label = f"{_fmt_list(left)} |- {_fmt_list(right)}"
    g.node(node_id, label=label)

    # simplify constants
    if any(isinstance(e, Const) and e.value is False for e in left):
        g.node(node_id, label=label, color="green")
        return True, node_id
    if any(isinstance(e, Const) and e.value is True for e in right):
        g.node(node_id, label=label, color="green")
        return True, node_id

    left = [e for e in left if not (isinstance(e, Const) and e.value is True)]
    right = [e for e in right if not (isinstance(e, Const) and e.value is False)]

    # identity
    for e in left:
        if e in right:
            g.node(node_id, label=label, color="green")
            return True, node_id

    # atomic failure
    if all(is_literal(e) for e in left + right):
        g.node(node_id, label=label, color="red")
        return False, node_id

    # expand left formulas
    for i, e in enumerate(left):
        if is_literal(e):
            continue
        rest = left[:i] + left[i + 1 :]
        if isinstance(e, Not):
            res, child = _prove_graph(rest, right + [e.expr], g)
            g.edge(node_id, child)
            g.node(node_id, label=label, color="green" if res else "red")
            return res, node_id
        if isinstance(e, Bin):
            if e.op == "AND":
                res, child = _prove_graph(rest + [e.left, e.right], right, g)
                g.edge(node_id, child)
                g.node(node_id, label=label, color="green" if res else "red")
                return res, node_id
            if e.op == "OR":
                r1, c1 = _prove_graph(rest + [e.left], right, g)
                r2, c2 = _prove_graph(rest + [e.right], right, g)
                g.edge(node_id, c1)
                g.edge(node_id, c2)
                res = r1 and r2
                g.node(node_id, label=label, color="green" if res else "red")
                return res, node_id
            if e.op == "IMPL":
                r1, c1 = _prove_graph(rest, right + [e.left], g)
                r2, c2 = _prove_graph(rest + [e.right], right, g)
                g.edge(node_id, c1)
                g.edge(node_id, c2)
                res = r1 and r2
                g.node(node_id, label=label, color="green" if res else "red")
                return res, node_id

    # expand right formulas
    for i, e in enumerate(right):
        if is_literal(e):
            continue
        rest = right[:i] + right[i + 1 :]
        if isinstance(e, Not):
            res, child = _prove_graph(left + [e.expr], rest, g)
            g.edge(node_id, child)
            g.node(node_id, label=label, color="green" if res else "red")
            return res, node_id
        if isinstance(e, Bin):
            if e.op == "AND":
                r1, c1 = _prove_graph(left, rest + [e.left], g)
                r2, c2 = _prove_graph(left, rest + [e.right], g)
                g.edge(node_id, c1)
                g.edge(node_id, c2)
                res = r1 and r2
                g.node(node_id, label=label, color="green" if res else "red")
                return res, node_id
            if e.op == "OR":
                res, child = _prove_graph(left, rest + [e.left, e.right], g)
                g.edge(node_id, child)
                g.node(node_id, label=label, color="green" if res else "red")
                return res, node_id
            if e.op == "IMPL":
                res, child = _prove_graph(left + [e.left], rest + [e.right], g)
                g.edge(node_id, child)
                g.node(node_id, label=label, color="green" if res else "red")
                return res, node_id

    g.node(node_id, label=label, color="red")
    return False, node_id


def prove_svg(text: str, outfile: str) -> bool:
    """Prove ``text`` and save the proof tree as SVG to ``outfile``."""
    ast = parse(text)
    g = Digraph(format="svg")
    res, root = _prove_graph([], [ast], g)
    g.render(outfile, format="svg", cleanup=True)
    return res

