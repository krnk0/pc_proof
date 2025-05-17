import pytest
from pc_proof.sequent import prove

@pytest.mark.parametrize("expr", [
    "p or not p",
    "not not p -> p",
    "⊤",
    "not ⊥",
])
def test_provable(expr):
    assert prove(expr), f"{expr!r} should be provable"


@pytest.mark.parametrize("expr", [
    "p and not p",
    "p or q and not p",
    "not (p or q)",
    "⊥",
])
def test_not_provable(expr):
    assert not prove(expr)
