# 040 — Go / Python SDK implementation plan

## Context
Phase 039 completed first-class processor binding: Job definitions and Workflow job/map nodes now carry optional `processor_name`; dispatcher resolves workflow node binding -> job binding -> legacy `job_id`. Rust/Java SDKs already route on `DispatchTask.processor_name`.

## Goal
Start Phase 2 SDK completion by planning and implementing independent Go and Python Worker SDKs under the standardized SDK layout.

## Required work
1. Create/verify independent SDK layout:
   - `sdks/go/<sdk-name>`
   - `sdks/python/<sdk-name>`
   - demos under `examples/go/<demo-name>` and `examples/python/<demo-name>` when useful for live/smoke validation.
2. Each SDK must be independently buildable/testable/publishable and must not depend on server `crates/*` path modules.
3. Implement Worker Tunnel basics consistent with Rust/Java:
   - client_instance_id registration, server-assigned worker_id capture
   - heartbeat loop
   - DispatchTask receive
   - processor registry keyed by `processor_name` with legacy fallback to `job_id`
   - TaskLog / TaskResult reporting
4. Add README snippets and demo smoke flows.
5. Update design roadmap and `.memory` after each substantial step.

## Validation
- Run language-native tests/builds for each SDK and demo.
- Re-run relevant server protocol tests if proto copies change.
- Keep root Dockerfile server-only; do not add SDK builds to root image.
- Commit and push with detailed Lore trailers.
