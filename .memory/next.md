# Latest completed slice

- 2026-06-11: Notification Center provider-schema and first-class template hardening is complete with an additional rich-provider fail-closed patch pending final replacement release verification in the current session. Reusable notification channels/policies/messages/delivery attempts, job lifecycle materialization, generic retry/DLQ, provider-specific message schema/rendering, `notification_templates` storage/API/render preview, Web template editor/preview, policy template selector, docs, and safety fixes are implemented and locally verified.

# Next Work

## Current priority direction

当前仍是功能/模块测试验收与发布阶段，不收缩、不臆造。Notification Center 的渠道、策略、消息、投递、provider schema、以及 reusable templates 已落地；Alert 自动迁移、Workflow notification 节点迁移、真实 test-send endpoint、外部 SaaS live smoke 仍不能被说成完成。

## Immediate next slice after this release

1. Migrate alert delivery toward Notification Center: create/backfill `notification_policies(owner_type='alert_rule')` from `alert_rules.channels_json`, dual-write or bridge attempts during migration, and preserve existing alert routes until a documented breaking release.
2. Migrate workflow `notification` nodes from raw `channel/target/template` fields to registered Notification Center channel/template refs. Default must remain non-blocking unless explicitly configured otherwise.
3. Add delivery lease/idempotency hardening so crash recovery avoids both lost notifications and duplicate provider calls. Current ordering is at-least-once: safer than loss, but may duplicate if crashing after result insert before old attempt consumption.
4. Add real channel `:test` endpoint only when it persists attempts and redacts results; until then `supportsTestSend=false` is correct.
5. After the current notification-center release, execute the user-requested docs-site rewrite: make the docs human-oriented, step-by-step deploy/config/integration documentation rather than AI-oriented source notes, then test, commit, build Docker image, push/tag release, and monitor GitHub Actions.

## Current verified baseline

- Notification Center baseline: explicit schema/repositories, routes/OpenAPI, config defaults, background worker, Web page, docs, RBAC/menu seed, job lifecycle materialization, redaction, generic retry/DLQ, and provider delivery for webhook-style/Slack/DingTalk/Feishu/WeCom/PagerDuty/Email/plugin webhook-compatible providers.
- Template baseline: `notification_templates` has an explicit SeaORM migration/entity/repository, CRUD/list/get/delete API, `/api/v1/notification-templates/{id}/render` preview endpoint, safe token replacement, provider/message-type validation, Web template drawer/preview, and policy template AutoComplete options restricted to enabled stored templates matching selected channel providers.
- Alerting baseline remains compatible: alert rules/events/delivery attempts/retry UI still exist; Alerting owns incident semantics while Notification Center owns reusable outbound delivery.
- Docs site module baseline remains `docs/`, with Docker publish workflow targeting `yhyzgn/tikeo-docs`.

## Standing constraints

- Functional/module testing acceptance phase: do not shrink scope; if anything missing/incomplete/untested/hallucinated is found, fill it production-grade or record a real blocker. Keep durable context fresh and source-backed.
- Alerts = rules/events/incidents; Notifications = channels/templates/policies/messages/delivery. Inbound webhook event sources are job triggers, not outbound notification channels.
- Never leak webhook tokens, SMTP passwords, PagerDuty keys, header credentials, or secret ref values in API responses/UI/docs/logs.
- Worker 重要可见性状态不得只在内存。
- 禁止约定命名匹配；必须使用结构化字段、labels 或 structuredCapabilities。
- 中文 i18n 必须完整中文，英文 i18n 必须英文，不要中英混合 label。
- 新 schema 变更必须进入显式 SeaORM migration；不得在 `connect_and_migrate` 后挂未记录的兼容补丁。
- Helm chart 不能部署业务 Worker 或创建业务 Worker 入站 Service；Worker 只能主动出站连接 Tikeo Worker Tunnel。
- 源文件 <=1500 行；`mod.rs` / `lib.rs` 等入口文件只做声明和 re-export。
- Web/frontend/docs package management and command execution must use `bun` / `bunx` unless explicitly overridden.

## Next after provider schema/template hardening

1. Commit/push the rich-provider fail-closed patch and publish a replacement `v0.2.xxx` release tag superseding `v0.2.4`.
2. Monitor remote CI and release/Docker workflows until green.
3. Start the human-oriented docs-site rewrite as the next automatic task, then test, commit, build, tag, publish, and monitor its release.
4. Continue alert-rule dual-write/migration and workflow notification node migration after the docs rewrite release unless the user reprioritizes.
