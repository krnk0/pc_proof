# pc_proof

A minimal toolkit for propositional logic.

## Features
- formula parser
- sequent calculus prover
- tableau solver with countermodels
- upcoming SVG proof diagrams via Graphviz

## Install
Requires Python 3.10+ and Graphviz.
```bash
pip install -e .
```

## Example
```python
from pc_proof.sequent import prove
from pc_proof.tableau import solve

prove("p or not p")
taut, model = solve("p and not p", tautology=True)
print(taut, model)
```

## License
MIT. See [LICENSE](LICENSE).
