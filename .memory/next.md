# Next Work

## Current priority direction
Continue P1 production governance only after preserving the source-size/module-entry rule: every source file must stay <= 1500 lines, and `mod.rs` / `lib.rs` files should remain module entry/re-export surfaces instead of implementation dumps.

## P1 — production hardening / common enterprise use
1. Full script approval/signature/KMS plus URL/File/Secret grants and production release gates.
   - Done foundation: fail-closed policy/signature gates, blocked audit materialization, and read-only release-gate preview API.
   - Next: design local verification boundary for signed approval artifacts without introducing external KMS dependency by default.
2. OIDC tenant/app/role binding and advanced tenant isolation UI.
3. Prometheus/Grafana recording-rule validation and operational runbooks.
4. Go/Python SDKs; Node.js SDK after Worker identity/lifecycle semantics stabilize.

## P2 — ecosystem / advanced differentiation
- PowerJob and XXL-JOB migration tooling.
- Terraform Provider, GitOps/IaC, K8s CRD.
- Task dependency discovery/topology, workflow replay, intelligent scheduling.
- Plugin system, advanced webhook/event sources, task versioning/canary rollback.

## Recently completed
- P0 service-usability lane completed and pushed before this cleanup.
- HTTP/mod.rs and other oversized Rust files were split; max source file line count is now 1495.
- Script release-gate preview endpoint was added for local production-gate visibility.
