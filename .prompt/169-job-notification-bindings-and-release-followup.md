# 169 — Job notification bindings and release follow-up

## Scope

Implement and release production-grade Job Notification Binding support:

- Job-facing CRUD/validate/preview over Notification Center policies.
- Runtime Job instance message payload context for templates.
- Notification message trace page/API with delivery attempts and redacted execution logs.
- Web Jobs notification drawer and Notification Center message detail drawer.
- Workflow fix so release tag builds update server `CARGO_PKG_VERSION` before server binary/image builds.

## Acceptance

- Full local verification passes: Rust fmt/clippy/test/build, Web lint/typecheck/test/build, docs build, workflow contract tests, source-size, Docker server/web/docs builds, diff check.
- Commit follows Lore protocol.
- Push main, tag next `v0.2.x`, push tag.
- GitHub Actions main and tag-triggered workflows finish green; record release and Docker publication evidence in `.memory/session-log.md`.

## Guardrails

- Do not create a second notification delivery subsystem.
- Do not leak `secretRefs`, webhook URLs, SMTP passwords, PagerDuty routing keys, or authorization headers.
- Treat Alerts as rules/incidents and Notifications as channels/templates/policies/messages/delivery.
- Use `bun`/`bunx` for web/docs commands.

## Closeout evidence — 2026-06-13

- Implemented and released as `v0.2.9`; `v0.2.8` is recorded only as a failed release attempt caused by release-only Cargo.lock version drift.
- All expected GitHub Actions for commit `cecec2aa2ec54750d2558c6e288ade9043802119` completed successfully: CI, Coverage, GitHub assets, Docker server/web/docs, and Rust/Java/Python/Go/Node SDK publish workflows.
- Docker Hub images verified by pull: `yhyzgn/tikeo-server:v0.2.9`, `yhyzgn/tikeo-web:v0.2.9`, `yhyzgn/tikeo-docs:v0.2.9`.
- Server image runtime version verified: `docker run --rm yhyzgn/tikeo-server:v0.2.9 --version` -> `tikeo 0.2.9`.
- Future work should continue from the next product slice; do not reopen the release-version issue unless a later tag regresses the lockfile-aware sync contract.
