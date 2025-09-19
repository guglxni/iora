import ast
import pathlib
from typing import Iterator

SRC = pathlib.Path("src")


def iter_python_files(root: pathlib.Path) -> Iterator[pathlib.Path]:
    for p in root.rglob("*.py"):
        if ".venv" in p.parts or "site-packages" in p.parts:
            continue
        yield p


def has_bare_except(code: str) -> bool:
    try:
        tree = ast.parse(code)
    except SyntaxError:
        return False
    for n in ast.walk(tree):
        if isinstance(n, ast.ExceptHandler) and n.type is None:
            return True
    return False


def test_no_bare_except() -> None:
    offenders = []
    for p in iter_python_files(SRC):
        if has_bare_except(p.read_text(encoding="utf-8", errors="ignore")):
            offenders.append(str(p))
    assert not offenders, f"Bare 'except:' found in: {offenders}"
