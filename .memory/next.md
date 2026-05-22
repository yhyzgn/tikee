# Next Work

## Immediate next slice
- Continue with `.prompt/076-script-execution-governance-and-live-runner-smoke.md`.
- Focus areas:
  1. Add script-bound execution governance visibility: audit/result classification for missing worker capability, missing runner, policy rejection, digest mismatch, timeout, and output-limit failures.
  2. Add an optional live smoke path for containerized script runner execution when Docker/compatible runtime is available, while keeping CI/unit tests deterministic without Docker.
  3. Keep Server as metadata dispatcher only; all script execution remains Worker-side and opt-in.

## Current status
- Phase 075 slice completed. Rust SDK now includes an opt-in `ContainerScriptRunner` for non-WASM scripts that builds Docker-compatible default-deny execution commands (`--network=none`, `--read-only`, no host mounts, stdin script content, scheduler metadata env, whitelisted env only). Unit tests verify the command boundary and reject dangerous policy before spawning any runtime.
