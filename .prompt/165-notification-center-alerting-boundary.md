# 165 — Notification Center and alerting boundary

## Context

The acceptance-stage planning pass on 2026-06-11 found that Tikeo already has a real alerting foundation, but it is alert-centric:

- `alert_rules` stores inline `channels_json`.
- `alert_events` stores alert incident/recovery history.
- `alert_delivery_attempts` stores alert-specific provider attempts and retry/DLQ state.
- `AlertDispatcher` already supports webhook, Slack, DingTalk, Feishu/Lark, WeChat Work/WeCom, PagerDuty, plugin webhook, and Email.
- Job/instance status transitions exist, and workflow UI already has a `notification` node shape, but there is no reusable Notification Center that lets jobs/workflows/alerts share channels, templates, policies, and delivery history.

Canonical planning artifact: `design/notification-center-alerting-plan.md`.

## Goal

Implement the next production-grade slice of Notification Center without breaking existing alert APIs.

## Required boundary

- **Alerting** owns abnormal condition evaluation, alert events, severity, dedupe, silence, recovery, and escalation.
- **Notification Center** owns reusable outbound channels, templates, policies/subscriptions, messages, provider delivery, retry, DLQ, channel tests, and redacted delivery history.
- **Inbound webhook event sources** are job triggers and must not be confused with outbound webhook notification channels.

## Recommended first implementation slice

1. Add explicit SeaORM migration/entities/repositories for `notification_channels` and `notification_policies` first. If a smaller slice is needed, defer templates/messages but do not skip channel reuse and redaction.
2. Add `GET/POST/PATCH/DELETE /api/v1/notification-channels` with provider validation, redacted target responses, and secret-ref-only credential handling.
3. Add `GET /api/v1/notification-channel-types` that returns built-in providers and plugin-provided channel type metadata. Keep existing plugin `alertChannelTypes` as compatibility; add generic naming in the new API.
4. Add tests proving raw URLs/tokens/passwords are not leaked through channel list/detail/readiness responses.
5. Keep existing `/api/v1/alert-*` routes green; do not remove `alert_rules.channels_json` in this slice.

## Follow-up slices

- Extract provider adapters from `crates/tikeo-server/src/alert.rs` into a generic notification provider module, then make alert delivery use it.
- Add `notification_messages` and generic `notification_delivery_attempts`, then dual-write alert attempts during migration.
- Add job-level notification policies for `succeeded`, `failed`, `partial_failed`, `cancelled`, `retry_scheduled`, `retry_exhausted`, `no_eligible_worker`, and `script_governance_failure`.
- Migrate workflow `notification` nodes from raw channel/target/template fields to registered channels/templates.
- Update Web UI: split Alerts (rules/events/silence) from Notifications (channels/templates/policies/delivery/DLQ/test send).
- Add docs/reference and user guides after APIs land.

## Constraints

- No database foreign keys; use soft links and repository validation.
- New schema must be an explicit SeaORM migration, not a hidden startup compatibility patch.
- Source files must remain under 1500 lines; split storage/API/provider/UI by responsibility.
- All HTTP business responses keep `{code,message,data}`.
- Web/frontend commands use `bun`/`bunx`.
- Never store or return raw webhook tokens, SMTP passwords, PagerDuty routing keys, or secret values in operator-facing responses/logs/audit records.
- Do not emit terminal job failure notifications while a retry is scheduled; use `retry_scheduled` until retries are exhausted.

## Expected verification

- Targeted storage tests for channel create/list/update/delete, redaction, scope soft links, and delete-refusal when policies reference a channel.
- Targeted HTTP tests for envelope, RBAC, OpenAPI schema registration, provider validation, and non-leakage of secrets.
- Existing alert route/provider/retry tests remain green.
- If Web changes: `cd web && bun run lint && bun run typecheck && bun test && bun run build`.
- Standard Rust gates: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, `cargo build --workspace --all-features`.
- `python3 scripts/check-source-size.py`.

## Completion notes

- Update `design/notification-center-alerting-plan.md` if implementation decisions change.
- Update `design/tikeo-architecture-design.md`, `.memory/progress.md`, `.memory/session-log.md`, `.memory/decisions.md`, `.memory/next.md`, and `.memory/risks.md`.
- Create the next `.prompt/166-*.md` before commit.
- Commit with Lore trailers and push.
