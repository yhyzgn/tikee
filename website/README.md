# Tikeo documentation site

This is the standalone Docusaurus 3 documentation site for Tikeo.

## Local development

The default build target is GitHub Pages project hosting at `https://yhyzgn.github.io/tikeo/`, so local serve URLs include `/tikeo/`.

```bash
bun install --frozen-lockfile
bun run docs:dev
```

For a custom root-domain preview, override the site URL and base URL:

```bash
TIKEO_DOCS_URL=http://127.0.0.1:13030 TIKEO_DOCS_BASE_URL=/ bun run docs:dev
```

## Verification

Default GitHub Pages-style build:

```bash
bun run docs:typecheck
bun run docs:build
bun run docs:serve -- --port 13030
```

Smoke URLs after default `docs:serve`:

```bash
curl -fsS http://127.0.0.1:13030/tikeo/
curl -fsS http://127.0.0.1:13030/tikeo/docs/
curl -fsS http://127.0.0.1:13030/tikeo/zh-CN/docs/
curl -fsS http://127.0.0.1:13030/tikeo/llms.txt
```

Custom root-domain build:

```bash
TIKEO_DOCS_URL=https://docs.example.com TIKEO_DOCS_BASE_URL=/ bun run docs:build
```

## Deployment configuration

| Environment variable | Default | Meaning |
| --- | --- | --- |
| `TIKEO_DOCS_URL` | `https://yhyzgn.github.io` | Docusaurus `url`. Use the origin only, without a path. |
| `TIKEO_DOCS_BASE_URL` | `/tikeo/` | Docusaurus `baseUrl`. Use `/` for a dedicated docs domain. |

If Chinese language switching returns “Page Not Found” on a static host, verify the deployed `baseUrl` matches the hosting path.

## Scope

The site currently covers the Phase A scaffold plus enriched P0 docs, complete current-route zh-CN localization, complete SDK list coverage, and copy-paste deployment paths for single binary/systemd, Docker Compose, and Helm/Kubernetes. Final deployment provider configuration remains environment-specific.
