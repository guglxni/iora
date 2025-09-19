.PHONY: format check test hooks
format:
	pre-commit run black -a || true
	pre-commit run isort -a || true
check:
	pre-commit run -a
test:
	pytest -q
hooks:
	pre-commit install
