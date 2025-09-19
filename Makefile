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

.PHONY: exp-run exp-collect exp-validate exp-plot-times exp-smoke
exp-run:
	python spec/experiments/round_runner.py $(CFG)
exp-collect:
	python spec/experiments/collect_metrics.py
exp-validate:
	python spec/experiments/validate_transcripts.py $$(ls -d artifacts/20* | tail -n1)
exp-plot-times:
	python spec/experiments/plot_times.py $$(ls -d artifacts/20* | tail -n1)
exp-smoke:
	$(MAKE) exp-run CFG=spec/experiments/baselines/adult-lr-plain.yaml
	$(MAKE) exp-run CFG=spec/experiments/baselines/adult-lr-fedzk.yaml
	$(MAKE) exp-collect
	$(MAKE) exp-validate
	$(MAKE) exp-plot-times
