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

## CLI
After installation the command `pc-proof` is available.

```bash
pc-proof "p or not p"
pc-proof -m tableau "p and not p"
pc-proof -o proof "p or not p"  # SVG for sequent proof
```

The `-o` option is only valid when using the sequent method.

## Testing
Run tests inside the provided virtual environment:
```bash
.venv/bin/python -m pytest -q
```

## License
MIT. See [LICENSE](LICENSE).
