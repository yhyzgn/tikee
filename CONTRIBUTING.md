# Contributing to Tikeo

Thanks for helping improve Tikeo. This project aims to become a reliable open-source orchestration control plane for scheduled jobs, workflows, scripts, and multi-language workers.

## Good first contributions

Start with changes that are easy to verify:

- Documentation fixes and clearer examples.
- Reproducible bug reports with logs or failing tests.
- Worker SDK demo improvements for Java, Rust, Go, Python, or Node.js.
- Web console copy, accessibility, and i18n improvements.
- Deployment notes for Docker Compose, Kubernetes, Helm, or Terraform.

## Development setup

Required tools vary by surface, but the common local stack uses Rust, Bun, Docker, Go, Java, Python, and Node.js.

```bash
cargo test --workspace
(cd web && bun run typecheck && bun run build)
```

For SDK-specific changes, also run the matching checks:

```bash
(cd sdks/java && ./gradlew test --no-daemon)
(cd sdks/rust/tikeo && cargo test --all-features)
(cd sdks/go/tikeo && go test ./...)
(cd sdks/python/tikeo && python -m pytest)
(cd sdks/nodejs/tikeo && bun test && bun run build)
```

If a command cannot run in your environment, mention the exact reason in the pull request.

## Pull request expectations

- Keep changes focused and reviewable.
- Add or update tests for behavior changes.
- Update docs when changing user-facing workflows, APIs, SDK behavior, or deployment paths.
- Do not claim production readiness for a feature unless the verification evidence supports it.
- Do not add new package dependencies without explaining why existing tools are insufficient.

## Architecture guardrails

- Workers should initiate outbound connections; do not design features that require business workers to expose inbound task-execution ports by default.
- Server-side code should schedule, govern, persist, and audit; user code execution belongs on controlled workers.
- Prefer structured fields, capabilities, labels, and permissions over string naming conventions.
- Keep source files modular and avoid growing large entrypoint files into implementation dumps.

## Reporting issues

Use the GitHub issue templates when possible. Include:

- Tikeo version or commit SHA.
- Deployment mode: local, Docker Compose, Kubernetes, or other.
- Database: SQLite, PostgreSQL, MySQL, or other.
- Relevant server, worker, browser, or SDK logs.
- Minimal reproduction steps.
