# 154 — Docs CI, publish target, and reference-depth follow-up

## Current baseline

The standalone docs site has a verified P0 content/localization/deployment baseline:

- Docusaurus 3.10.1 TypeScript + Bun app in `website/`.
- Default docs deployment target is GitHub Pages project hosting (`https://yhyzgn.github.io/tikeo/`) through `baseUrl=/tikeo/`.
- Custom standalone docs domains are supported with `TIKEO_DOCS_URL` and `TIKEO_DOCS_BASE_URL=/`.
- English P0 docs have contract-enforced minimum evaluation depth.
- zh-CN P0 docs exist for every current P0 route and have contract-enforced localized depth.
- SDK docs cover Rust, Go, Java Spring Boot, Python, and Node.js.
- Deployment docs include copy-paste runbooks for single binary/systemd, Docker Compose SQLite/PostgreSQL/MySQL, Helm dev/prod/TLS/ops overlays, and configuration parameters.
- Local default `/tikeo/` and custom root `/` builds/serve smokes are green for zh-CN language-switch routes.

## Recommended next slice

1. Add docs verification to CI.
   - Decide whether to extend main CI or create a docs-specific workflow.
   - Minimum commands: `python3 .github/tests/docs_site_contract_test.py`, `cd website && bun install --frozen-lockfile`, `bun run docs:typecheck`, and `bun run docs:build`.
   - For GitHub Pages deployment, keep the default `TIKEO_DOCS_BASE_URL=/tikeo/`; for custom domains, set `TIKEO_DOCS_BASE_URL=/`.
2. Select and document final docs hosting.
   - If using GitHub Pages project hosting: verify `/tikeo/zh-CN/...` after deployment.
   - If using a standalone domain: set canonical URL, `TIKEO_DOCS_URL`, and `TIKEO_DOCS_BASE_URL=/`.
3. Expand source-backed reference depth.
   - SDK overview and cross-language parity guide.
   - User guide pages for Dashboard, Jobs, Instances, Workers, Workflows, Scripts, Audit, and Settings.
   - Generated or source-derived OpenAPI/protobuf references.
   - Configuration/environment variable matrix generated from committed config structures or examples.

## Guardrails

- Do not advertise unverified runtime behavior.
- Keep Python/Node docs tied to actual `sdks/*` and `examples/*` commands.
- Do not manually invent API schemas if OpenAPI/protobuf generation can own the reference.
- Keep zh-CN pages complete for any new P0 sidebar route added.
- Keep deployment commands copy-pasteable and state exactly which values must be replaced for production.

## Verification entrypoint

Before committing any next docs slice, run:

```bash
python3 .github/tests/docs_site_contract_test.py
python3 scripts/check-source-size.py
cd website
bun install --frozen-lockfile
bun run docs:typecheck
bun run docs:build
```

For route/baseUrl changes, also run `bun run docs:serve` and curl affected English and zh-CN routes under both `/tikeo/` and `/` when relevant.
