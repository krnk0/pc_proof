# pc_proof

A minimal toolkit for propositional logic.

## Features
- formula parser
- sequent calculus prover
- tableau solver with countermodels
- SVG proof diagrams via Graphviz using `prove_svg`

## Install
Requires Python 3.10+ and Graphviz.
```bash
pip install -e .
```

## Example
```python
from sequent import prove
from tableau import solve
from viz import prove_svg

prove("p or not p")
taut, model = solve("p and not p", tautology=True)
print(taut, model)
prove_svg("p or not p", "proof")
```

## License
MIT. See [LICENSE](LICENSE).
