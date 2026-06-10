# Latest completed slice

- 2026-06-10: Source-derived API/protocol reference docs are now in place. English and zh-CN docs include Management OpenAPI and Worker Tunnel protobuf reference pages, SDK helper docs link to exact create/trigger/instance/log endpoint anchors and `DispatchTask`, and docs contracts guard the source tokens, sidebar entries, and links. Follow-up prompt: `.prompt/161-docs-publishing-search-user-guide-followup.md`.

# Next Work

## Current priority direction

当前优先级：功能/模块测试验收阶段继续保持不收缩原则。独立 Docusaurus docs 站点已经完成 scaffold、P0 内容深度、zh-CN 路由镜像、部署 runbook、SDK create+trigger 文档、Management API trigger e2e smoke，以及 source-derived Management OpenAPI / Worker Tunnel protobuf reference。下一步应优先补 docs publishing/search/SEO readiness 与用户指南深度，所有内容必须从已验证 UI/backend/SDK artifacts 或源码派生，不能发明未实现能力。

## Immediate next slice

1. Add docs publishing/search/SEO readiness once hosting target is selected or safely defaulted:
   - canonical URL / baseUrl policy
   - robots policy
   - OpenGraph/social image wiring
   - local search or DocSearch plan
   - maintained `llms.txt` / `llms-full.txt` strategy
2. Expand user-guide depth from verified artifacts:
   - Dashboard
   - Jobs
   - Instances
   - Workers
   - Workflows
   - Scripts
   - Audit
   - Settings
3. Optional quality follow-up: add a contributor runbook for `scripts/management-trigger-e2e-smoke.sh` if a lighter local-only smoke wrapper is useful.
4. Kubernetes 后续可继续补真实控制器专项文档：Nginx/Envoy/Traefik/Gateway API controller 的实际生产 values、证书模式和 smoke runbook。
5. 迁移工具（PowerJob/XXL-JOB）仍维持最低优先级 backlog，核心服务体验稳定后再做。

## Current verified baseline

- Docs site P0 content/localization/deployment：默认 `/` 为英文站，`/zh-CN/` 为中文站；Docusaurus navbar/footer/sidebar/homepage/blog 均已本地化；`website/docs/` 当前 P0 英文页面通过最小深度/section 契约；`website/i18n/zh-CN/docusaurus-plugin-content-docs/current/` 覆盖所有当前 P0 route，并通过 zh-CN 内容深度契约；SDK docs 覆盖 Rust、Go、Java Spring Boot、Python、Node.js，并已包含 source-backed Management API create+trigger examples、`x-tikeo-api-key` / `TIKEO_MANAGEMENT_API_KEY`、默认 `triggerType=api` + `executionMode=single` 与显式 broadcast selector helper 文档；部署 docs 覆盖 single binary/systemd、Compose SQLite/PostgreSQL/MySQL（含完整 docker-compose*.yml）、Helm dev/prod/TLS/ops 和配置参数。
- Source-derived reference：`website/docs/reference/management-openapi.md` / zh-CN mirror document `/api-docs/openapi.json`, `/api/v1/jobs`, `/api/v1/jobs/{job}:trigger`, `/api/v1/instances/{instance}`, `/api/v1/instances/{instance}/logs`, `CreateJobRequest`, `TriggerJobRequest`, `ApiResponse`, and `x-tikeo-api-key`; `website/docs/reference/worker-tunnel-protobuf.md` / zh-CN mirror document `WorkerTunnelService`, `OpenTunnel`, `SubscribeTaskLogs`, `RegisterWorker`, `Heartbeat`, `WorkerRegistered`, `DispatchTask`, `TaskLog`, `TaskResult`, `TaskCheckpoint`, `assignment_token`, and `processor_name`.
- Docs/e2e verification：`python3 .github/tests/docs_site_contract_test.py`、`python3 .github/tests/workflow_contract_test.py`、`python3 .github/tests/management_smoke_contract_test.py`、`python3 scripts/check-source-size.py`、`cd website && bun install --frozen-lockfile && bun run docs:typecheck && bun run docs:build` 均通过；latest docs build output was checked for no `broken anchor` warnings.
- Main CI baseline：main CI contains `workflow-policy` repository contract tests, `Docs site` job, cross-language worker parity smoke, and management-trigger e2e smoke artifact upload.
- Source-size cleanup：`scripts/check-source-size.py` 已覆盖普通 `.rs` / `.ts` / `.tsx` 源码并排除 `.git`、`.dev`、`target`、`node_modules`、`dist`、`coverage` 等生成/依赖目录；当前全仓库审计通过，且已接入 main CI `workflow-policy` 快速门禁。

## Standing constraints

- Functional/module testing acceptance phase: do not shrink scope; if anything missing/incomplete/untested/hallucinated is found, fill it production-grade or record a real blocker. Keep durable context fresh and source-backed.
- Worker 重要可见性状态不得只在内存。
- 禁止约定命名匹配；必须使用结构化字段、labels 或 structuredCapabilities。
- 中文 i18n 必须完整中文，英文 i18n 必须英文，不要中英混合 label。
- Go/Rust/Java/Python/Node SDK demo 能力广告必须真实；不可执行 sandbox 只能 fail-closed，不能作为 capability 暴露。
- 新 schema 变更必须进入显式 SeaORM migration；不得在 `connect_and_migrate` 后挂未记录的兼容补丁。
- Helm chart 不能部署业务 Worker 或创建业务 Worker 入站 Service；Worker 只能主动出站连接 Tikeo Worker Tunnel。
- 源文件 <=1500 行；`mod.rs` / `lib.rs` 等入口文件只做声明和 re-export；后续源码变更必须保持审计通过。
- Web/frontend package management and command execution must use `bun` / `bunx` unless explicitly overridden.
