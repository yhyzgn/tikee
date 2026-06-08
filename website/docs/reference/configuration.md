---
title: Configuration reference
description: Where Tikeo configuration lives and which settings matter for first evaluation.
---

# Configuration reference

The default development configuration lives under `config/`. Public docs should keep examples small and link to committed config files instead of copying large TOML blocks.

## First local config

```bash
cargo run --bin tikeo -- serve --config config/dev.toml
```

## Important areas

- HTTP listener address and port.
- Worker Tunnel listener address and port.
- Storage database URL.
- Transport security: HTTP TLS and Worker Tunnel TLS/mTLS.
- Script governance and release signature secret reference.
- Alert retry worker settings.
- Observability and tracing exporters.

## Safety rule

Schema changes must go through explicit SeaORM migrations. Do not document manual database mutation as a supported configuration path.

## Configuration evaluation workflow

Treat configuration as part of the runtime contract. Start with `config/dev.toml`, then introduce one change at a time: database URL, HTTP listener, Worker Tunnel listener, TLS/mTLS, OIDC, alert retry, or tracing exporter.

## Environment variables

Containerized deployment paths may map environment variables into Tikeo config. Document the mapping in deployment-specific pages and keep secrets in Secret references or external secret stores. Do not put plaintext credentials into public docs examples.

## Migration boundary

Storage schema changes must be represented as explicit SeaORM migrations. Compatibility helpers should not silently mutate schema outside migration version tracking.

## Troubleshooting config

If the Server starts but `readyz` fails, inspect storage and migration readiness first. If Worker registration fails, inspect tunnel listener configuration, TLS/mTLS settings, and worker identity scope.

## Next reference work

A future docs phase should generate an environment-variable matrix and configuration table directly from committed config structures or examples.
