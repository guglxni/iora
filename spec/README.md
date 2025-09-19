# Spec-Kit in This Repo

We keep a light Spec-Kit flow: **/specify → /plan → /tasks**.
- **PRD**: `spec/prd/fedzk-prd.md`
- **Plan**: `spec/plans/development-plan.md`
- **Tasks**: `spec/tasks/*.md`

**Workflow**
1) Create a feature branch
2) Run a phase prompt in Cursor (e.g., "/tasks: Phase A A1–A5")
3) Generate diffs → review → commit → PR

Acceptance checks live in each task file. We evolve specs alongside code (no external PM tool).
