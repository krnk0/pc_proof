import os
import shutil
import pytest

import cli

pytestmark = pytest.mark.skipif(shutil.which("dot") is None, reason="graphviz not installed")


def test_cli_sequent(capsys):
    cli.main(["p or not p"])
    out = capsys.readouterr().out
    assert "Provable" in out


def test_cli_tableau(capsys):
    cli.main(["-m", "tableau", "p and not p"])
    out = capsys.readouterr().out
    assert "Not provable" in out


def test_cli_svg_output(tmp_path, capsys):
    out_file = tmp_path / "proof"
    cli.main(["-o", str(out_file), "p or not p"])
    svg_path = str(out_file) + ".svg"
    assert os.path.exists(svg_path)
    out = capsys.readouterr().out
    assert "Provable" in out


def test_cli_tableau_output_error():
    with pytest.raises(SystemExit):
        cli.main(["-m", "tableau", "-o", "out", "p or p"])
