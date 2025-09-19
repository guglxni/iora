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

.PHONY: exp-batch-curve exp-attacks exp-plot-batch exp-plot-accept bundle-artifacts
exp-batch-curve:
	python spec/experiments/run_batch_curve.py spec/experiments/baselines/adult-lr-fedzk-batch.yaml
	python spec/experiments/collect_metrics.py
	python spec/experiments/plot_batch_throughput.py
exp-attacks:
	python spec/experiments/run_attacks.py spec/experiments/baselines/adult-lr-fedzk.yaml
	python spec/experiments/collect_metrics.py
	python spec/experiments/plot_acceptance_rates.py
exp-plot-batch:
	python spec/experiments/plot_batch_throughput.py
exp-plot-accept:
	python spec/experiments/plot_acceptance_rates.py

.PHONY: exp-grid paper-figs
exp-grid:
	python spec/experiments/run_grid.py spec/experiments/baselines/adult-lr-fedzk.yaml
paper-figs:
	python spec/experiments/plot_accuracy.py
	python spec/experiments/plot_throughput_vs_batch.py
	python spec/experiments/table_circuits.py
	python spec/experiments/table_timings.py

bundle-artifacts:
	python scripts/bundle_artifacts.py
