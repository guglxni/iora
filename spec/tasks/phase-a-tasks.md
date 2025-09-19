# Phase A Tasks — Hygiene & Baseline

**A1. Exceptions**
- Replace all `except:` with specific exceptions.
- Use `logger.exception` on unexpected paths; re-raise where appropriate.
- Tests must assert original error types surface.

**A2. Logging**
- Add `src/fedzk/logging_config.py` with JSON logger for CLI, human formatter for dev.
- Replace all `print(` in `src/` with logger calls.
- Add a test that fails if `print(` remains under `src/`.

**A3. Secrets**
- Remove real secrets/archives from VCS; commit only fixtures.
- Add SOPS/Vault integration in Helm; document local decrypt instructions.

**A4. Circuits Metadata**
- On circuit compilation, write `artifacts/meta/*.json` with constraint counts, key sizes, sample proof sizes. 
- Include script or CLI subcommand and add to CI.

**A5. Pre-commit & CI**
- Add pre-commit hooks (black, isort, flake8, mypy). 
- CI fails on style/type errors; provide `make format` and `make check`.

Acceptance: zero bare `except:`, zero `print(` in src/, metadata emitted, hooks enforced in CI.
