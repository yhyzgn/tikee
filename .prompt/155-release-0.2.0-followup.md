# 155 — 0.2.0 release follow-up

## Current release baseline

The repository has been prepared for the formal `0.2.0` release:

- Version metadata is synchronized across the root Cargo workspace, Rust SDK/demo, Java SDK/demo, Python SDK/demo, Node SDK/demo, Web package, Website package, and Helm chart.
- README / README.zh-CN / Helm README install snippets point to `0.2.0` packages/images/tags.
- `CHANGELOG.md` includes a dated `0.2.0` release section.
- Local release validation passed for core Rust, Web, docs site, Java/Rust/Go/Node/Python SDKs, and Rust/Go/Node/Python/Java demos.

## Immediate handoff after tag push

1. Monitor GitHub Actions for the `v0.2.0` tag:
   - `release-github-assets.yml`
   - `publish-docker-server.yml`
   - `publish-docker-web.yml`
   - `publish-rust-sdk.yml`
   - `publish-go-sdk.yml`
   - `publish-java-sdk.yml`
   - `publish-python-sdk.yml`
   - `publish-nodejs-sdk.yml`
2. Confirm whether GitHub Release assets and registry artifacts are actually published. Do not claim registry availability until the corresponding workflow succeeds.
3. If a publish job fails because of missing credentials or registry-side configuration, record the failing job and required secret/config fix in `.memory/session-log.md` and `.memory/risks.md`; rerun the workflow after repair without moving the tag unless source changes are required.
4. Continue the docs work from `154-docs-ci-and-reference-depth-followup.md`: CI docs verification, final hosting config, search/SEO/OG, and source-backed API/protobuf references.

## Verification evidence from release prep

See `.memory/session-log.md` entry `2026-06-08 — 0.2.0 正式版发布准备` for the full command list.

## Guardrails

- Do not retag `v0.2.0` after pushing unless the tag push failed and no remote tag exists.
- Do not advertise Docker/npm/PyPI/crates/Maven artifacts as published until remote workflows prove it.
- Keep docs locale separation: `/` English, `/zh-CN/` Chinese.
- Keep release commits Lore-compliant and include concrete verification evidence.
