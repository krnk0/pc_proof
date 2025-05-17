import os
import shutil
import pytest
from viz import prove_svg

pytestmark = pytest.mark.skipif(shutil.which("dot") is None, reason="graphviz not installed")


def test_svg_generation(tmp_path):
    out = tmp_path / "proof"
    res = prove_svg("p or not p", str(out))
    assert res is True
    svg_file = str(out) + ".svg"
    assert os.path.exists(svg_file)
    assert os.path.getsize(svg_file) > 0


def test_svg_failure(tmp_path):
    out = tmp_path / "proof2"
    res = prove_svg("p and not p", str(out))
    assert res is False
    svg_file = str(out) + ".svg"
    assert os.path.exists(svg_file)
    assert os.path.getsize(svg_file) > 0
