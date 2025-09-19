.PHONY: format check test hooks circuits-meta
format:
	pre-commit run black -a || true
	pre-commit run isort -a || true
check:
	pre-commit run -a
test:
	pytest -q
hooks:
	pre-commit install
circuits-meta:
	python scripts/emit_circuit_meta.py
