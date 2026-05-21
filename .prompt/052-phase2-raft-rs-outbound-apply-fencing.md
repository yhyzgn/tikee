# 052 — Phase 2 raft-rs outbound transport, apply bookkeeping, and fencing prep

## Context
The project uses TiKV raft-rs (`raft` crate 0.7.x) for the scheduler server cluster direction. Completed safe slices:
- stable string `node_id` -> non-zero raft `u64` mapping and `RawNode` bootstrap validation;
- no-FK durable raft metadata/member/log/snapshot records;
- `/api/v1/raft/append-entries` DTO validation and conversion into `eraftpb::Message`;
- `RaftRuntimeCoordinator` ticker loop that drives `RawNode::tick()` every 100ms;
- Ready persistence order skeleton: HardState -> log entries -> snapshot -> `advance()`;
- inbound runtime inbox: validated HTTP messages are submitted to a bounded mpsc channel and stepped by the runtime loop.

## Hard safety rule
Do **not** set `can_schedule=true`, do **not** emit `leader_fencing_token`, and do **not** let dispatch/tick ownership run from raft mode until real raft-rs leader state plus persisted fencing token are implemented and consumed by the existing dispatch gates.

## Required next work
1. Add an outbound peer transport abstraction that can send raft-rs `Ready.messages()` to configured peer endpoints over HTTP, compatible with Docker bridge / K8s Service / LB / WAF layers.
2. Convert outbound `eraftpb::Message` values back into the existing wire DTO shape, with base64 payload encoding and no Swagger UI.
3. Implement Ready apply/state-machine bookkeeping for committed entries and applied index persistence, while still avoiding scheduler authority changes.
4. Add tests for outbound message serialization, transport failure handling, and applied-index persistence.
5. Update `design/scheduler-architecture-design.md`, `.memory/*`, and roadmap checkboxes.
6. Run full verification (`cargo fmt`, clippy, workspace tests, `cargo run -- --help`, web typecheck/build), commit with rich Lore trailers, and push.

## Current constraints
- API responses must always use `{ code, message, data }`.
- DB全库严禁外键；只能软关联字段。
- Backend crates stay under `crates/`; backend entrypoint remains at repo root.
- Web stays under `web/` with React + Ant Design + Bun.
- Go SDK + Python SDK remain Phase4.
