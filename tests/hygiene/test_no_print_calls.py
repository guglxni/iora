import pathlib
import re

SRC = pathlib.Path("src")
PAT = re.compile(r"^\s*print\(", re.M)


def test_no_print_calls_in_src() -> None:
    offenders = []
    for p in SRC.rglob("*.py"):
        if "logging_config.py" in str(p):
            continue
        txt = p.read_text(encoding="utf-8", errors="ignore")
        if PAT.search(txt):
            offenders.append(str(p))
    assert not offenders, f"`print(` found in: {offenders}"
