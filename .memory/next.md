# Next Work

## Immediate next slice
- Continue with `.prompt/075-script-runner-container-and-execution-governance.md`.
- Focus areas:
  1. Add a safer containerized Worker-side script runner option for non-WASM scripts, preserving explicit opt-in and default-deny network/filesystem/secrets.
  2. Add execution governance hooks for script-bound tasks: audit fields, worker capability visibility, and clear failure reasons for missing runner/capability/policy rejection.
  3. Keep the Server as dispatcher/metadata authority only; it must never execute user code and must only bind released immutable `script_versions` snapshots.

## Current status
- Phase 074 slice completed. Worker Tunnel now supports non-WASM `ScriptProcessorBinding` payloads from released immutable script versions, dispatcher filters workers by `script:<language>` / `script:*` / `*`, Rust SDK executes only through explicitly registered `ScriptRunner`s, Java SDK explicitly rejects script bindings for now, and Web script details document the required worker capabilities.
