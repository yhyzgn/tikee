# 154 — Docs CI and reference-depth follow-up

## Current baseline

The standalone docs site has a verified P0 content/localization baseline:

- Docusaurus 3.10.1 TypeScript + Bun app in `website/`.
- English P0 docs have contract-enforced minimum evaluation depth.
- zh-CN P0 docs exist for every current P0 route and have contract-enforced localized depth.
- SDK docs cover Rust, Go, Java Spring Boot, Python, and Node.js.
- Local docs build and zh-CN route smoke are green.

## Recommended next slice

1. Add docs verification to CI.
   - Decide whether to extend main CI or create a docs-specific workflow.
   - Minimum commands: `python3 .github/tests/docs_site_contract_test.py`, `cd website && bun install --frozen-lockfile`, `bun run docs:typecheck`, `bun run docs:build`.
   - Keep docs CI independent from deployment-provider selection.
2. Add publish-readiness metadata after the hosting target is selected.
   - Canonical URL.
   - robots policy.
   - OpenGraph/Twitter preview image.
   - sitemap review.
   - local search or DocSearch plan.
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
- Keep deployment-provider config separate until the final docs hosting target is chosen.

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

For route changes, also run `bun run docs:serve` and curl the affected English and zh-CN routes.
