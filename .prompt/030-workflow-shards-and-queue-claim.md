# 030 — Workflow shards and queue claim continuation

## Current phase context
- 025 has started. Worker `TaskResult` now automatically maps a completed `job_instance_id` back to its soft-linked `workflow_node_instance`, marks that node `succeeded` / `failed`, and calls workflow advance to queue eligible downstream nodes.
- `dispatch_queue` now has `lease_owner` and `lease_until` fields in the SeaORM entity, migration, SQLite compatibility path, and API summary model.
- SSE workflow instance events already exist via `instance_events` and `/api/v1/workflow-instances/{id}/events` / stream.

## Remaining 025 work
1. Implement real atomic queue claim / lease / visibility-timeout behavior instead of only adding fields.
2. Dispatch `workflow_shards` as concrete worker tasks or shard queue rows; persist shard result output/status.
3. Add MapReduce reduce semantics: when all map shards succeed, auto-advance / queue the reduce node; failed shards should support retry/failure propagation.
4. Map child workflow terminal status back to the parent workflow node and auto-advance parent successors.
5. Consider job log SSE follow endpoint if current job logs remain pull-only.

## Hard constraints
- No DB foreign keys; all relationships remain soft-linked by ids.
- HTTP envelope remains `{ code, message, data }`.
- Swagger UI remains forbidden.
- After changes: run cargo fmt/clippy/test/build, bun lint/typecheck/test/build, docker compose config if deployment files change, update design/.memory/.prompt, commit and push.

## 追加：审计约束
- Workflow 管理/执行动作必须记录 audit log：create/update/validate/dry-run/run/advance/materialize/recover。
- 后续新增 shard dispatch、shard result、child workflow callback、queue claim/release 等管理或执行动作时，也要同步写入 audit log；普通读取接口可不审计，避免刷屏。
