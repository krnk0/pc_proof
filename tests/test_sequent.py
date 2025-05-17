import pytest
from sequent import prove
from tableau import solve

PROVABLE = [
    "p or not p",
    "not not p -> p",
    "⊤",
    "not ⊥",
]

NOT_PROVABLE = [
    "p and not p",
    "p or q and not p",
    "not (p or q)",
    "⊥",
]

@pytest.mark.parametrize("expr", PROVABLE)
def test_provable(expr):
    assert prove(expr), f"{expr!r} should be provable"


@pytest.mark.parametrize("expr", NOT_PROVABLE)
def test_not_provable(expr):
    assert not prove(expr)


@pytest.mark.parametrize("expr", PROVABLE + NOT_PROVABLE)
def test_prove_matches_tableau(expr):
    res_sequent = prove(expr)
    res_tableau, _ = solve(expr, tautology=True)
    assert res_sequent == res_tableau
