# Latest completed slice

- 2026-06-11: Notification Center / Alerting boundary planning is complete. `design/notification-center-alerting-plan.md` now defines the source-backed split: Alerting owns abnormal-condition rules/events/silence/recovery/escalation, while Notification Center owns reusable outbound channels/templates/policies/messages/provider delivery/retry/DLQ and job/workflow/alert touchpoints. Follow-up prompt: `.prompt/165-notification-center-alerting-boundary.md`.

# Next Work

## Current priority direction

当前优先级仍是功能/模块测试验收阶段，不收缩、不臆造。新的重点缺口是通知触达能力：现有 Alert 模块已有规则、事件、provider delivery、retry/DLQ，但还缺少独立可复用的 Notification Center 来支撑告警规则可配置、渠道可配置、任务执行状态触达和工作流 notification 节点触达。

## Immediate next slice

1. Implement `.prompt/165-notification-center-alerting-boundary.md`: start with generic `notification_channels` and `notification_policies` storage/API, redaction, provider metadata, and compatibility with existing alert APIs.
2. Keep `alert_rules.channels_json` as compatibility/migration data; do not delete existing alert routes or provider delivery behavior.
3. After channels/policies are stable, extract provider adapters from `crates/tikeo-server/src/alert.rs` into a generic notification provider module, then add job status policies for `retry_scheduled`, `retry_exhausted`, `failed`, `partial_failed`, `succeeded`, and `cancelled`.
4. The docs Docker publish digest for `yhyzgn/tikeo-docs` was triggered by tag `v0.2.2` but not awaited; record the digest later if that release verification is resumed.

## Current verified baseline

- Alerting baseline: `alert_rules`, `alert_events`, `alert_delivery_attempts`, `AlertDispatcher`, webhook/Slack/DingTalk/Feishu/WeCom/PagerDuty/Email/plugin webhook provider support, redacted readiness, retry/DLQ, background retry ownership gating, alert delivery UI, and metrics summary exist.
- Job/runtime baseline: job instance statuses are `pending`, `dispatching`, `running`, `succeeded`, `partial_failed`, `failed`, and `cancelled`; Worker task results and built-in processors update instance status and schedule/exhaust retries.
- Workflow baseline: workflow UI has a `notification` node kind with raw channel/target/template shape; it must migrate to registered Notification Center channels/templates.
- Docs site module baseline remains `docs/`, with Docker publish workflow targeting `yhyzgn/tikeo-docs`.
- Source-size cleanup: `scripts/check-source-size.py` covers ordinary `.rs` / `.ts` / `.tsx` source and excludes generated/dependency directories.

## Standing constraints

- Functional/module testing acceptance phase: do not shrink scope; if anything missing/incomplete/untested/hallucinated is found, fill it production-grade or record a real blocker. Keep durable context fresh and source-backed.
- Alerting and Notification Center vocabulary is now a project boundary: Alerts = rules/events/incidents; Notifications = channels/templates/policies/messages/delivery. Inbound webhook event sources are job triggers, not outbound notification channels.
- Worker 重要可见性状态不得只在内存。
- 禁止约定命名匹配；必须使用结构化字段、labels 或 structuredCapabilities。
- 中文 i18n 必须完整中文，英文 i18n 必须英文，不要中英混合 label。
- Go/Rust/Java/Python/Node SDK demo 能力广告必须真实；不可执行 sandbox 只能 fail-closed，不能作为 capability 暴露。
- 新 schema 变更必须进入显式 SeaORM migration；不得在 `connect_and_migrate` 后挂未记录的兼容补丁。
- Helm chart 不能部署业务 Worker 或创建业务 Worker 入站 Service；Worker 只能主动出站连接 Tikeo Worker Tunnel。
- 源文件 <=1500 行；`mod.rs` / `lib.rs` 等入口文件只做声明和 re-export；后续源码变更必须保持审计通过。
- Web/frontend package management and command execution must use `bun` / `bunx` unless explicitly overridden.
