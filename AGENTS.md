# Agent / Automation Guidelines

## Approved Tasks
- Run Rust lint/test/build commands inside the devcontainer environment.
- Update documentation, workflows, and code within this repository.

## Restricted Tasks
- Do **not** modify dependency repositories (`~/.cargo/git` is mounted read-only).
- Do **not** push tags or publish releases without human approval.

## Environment
- Use `.devcontainer/devcontainer.json` when working via Copilot, Codespaces, or VS Code containers.
- GitHub Actions run in `mcr.microsoft.com/devcontainers/rust:1-bullseye`.
## Nix Environment

If this repo has `flake.nix`, `shell.nix`, or `default.nix`, run project
commands inside the Nix environment. At task start, check `IN_NIX_SHELL`.
`IN_NIX_SHELL=impure` from direnv is valid. If it is empty, use
`direnv exec . <command>` or `nix develop -c <command>`; do not silently fall
back to host tools. Scripts, Make targets, and agent entrypoints that require
repo tooling should fail loudly when they are not running under Nix.
