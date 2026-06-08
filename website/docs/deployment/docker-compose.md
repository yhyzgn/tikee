---
title: Docker Compose
description: Copy-paste Docker Compose deployment paths for SQLite, PostgreSQL, MySQL, Web, Worker Tunnel, and Prometheus.
---

# Docker Compose

Use Docker Compose when you want a reproducible local or VM smoke environment with packaged Server and Web containers. The repository ships three standalone stacks:

| Stack | File | Database | Use it for |
|---|---|---|---|
| SQLite | `docker-compose.yml` | named volume `tikeo-data` | fastest local evaluation |
| PostgreSQL | `docker-compose.postgres.yml` | `postgres:16-alpine` | production-like relational smoke |
| MySQL | `docker-compose.mysql.yml` | `mysql:8.4` | MySQL compatibility smoke |

## 1. Prepare `.env`

```bash
cp deploy/compose/tikeo.env.example .env
```

Edit only what you need. The defaults expose HTTP on `9090`, Worker Tunnel on `9998`, Web on `8080`, and optional Prometheus on `9091`.

## 2. SQLite one-command stack

```bash
DOCKER_BUILDKIT=1 docker compose --env-file .env up -d --build
curl -fsS http://127.0.0.1:${TIKEO_HTTP_PORT:-9090}/readyz
curl -fsS http://127.0.0.1:${TIKEO_WEB_PORT:-8080}/ >/dev/null
```

Open the Web console at `http://127.0.0.1:${TIKEO_WEB_PORT:-8080}`.

## 3. PostgreSQL stack

```bash
DOCKER_BUILDKIT=1 docker compose --env-file .env -f docker-compose.postgres.yml up -d --build
curl -fsS http://127.0.0.1:${TIKEO_HTTP_PORT:-9090}/readyz
docker compose --env-file .env -f docker-compose.postgres.yml ps
```

Useful `.env` overrides:

```dotenv
TIKEO_POSTGRES_PORT=15432
TIKEO_POSTGRES_DB=tikeo
TIKEO_POSTGRES_USER=tikeo
TIKEO_POSTGRES_PASSWORD=change-me
TIKEO_POSTGRES_DATA_VOLUME=tikeo-postgres-data
```

## 4. MySQL stack

```bash
DOCKER_BUILDKIT=1 docker compose --env-file .env -f docker-compose.mysql.yml up -d --build
curl -fsS http://127.0.0.1:${TIKEO_HTTP_PORT:-9090}/readyz
docker compose --env-file .env -f docker-compose.mysql.yml ps
```

Useful `.env` overrides:

```dotenv
TIKEO_MYSQL_PORT=13306
TIKEO_MYSQL_DATABASE=tikeo
TIKEO_MYSQL_USER=tikeo
TIKEO_MYSQL_PASSWORD=change-me
TIKEO_MYSQL_ROOT_PASSWORD=change-root
TIKEO_MYSQL_DATA_VOLUME=tikeo-mysql-data
```

The MySQL stack starts with `utf8mb4` settings so Unicode payloads and logs are safe.

## 5. Optional Prometheus

```bash
docker compose --env-file .env --profile observability up -d prometheus
curl -fsS http://127.0.0.1:${TIKEO_PROMETHEUS_PORT:-9091}/-/ready
```

Prometheus reads committed files under `observability/prometheus/`.

## Compose parameter reference

| Variable | Default | Used by | Meaning |
|---|---:|---|---|
| `TIKEO_IMAGE` | `yhyzgn/tikeo-server:dev` | Server | Server image tag to build/use. |
| `TIKEO_WEB_IMAGE` | `yhyzgn/tikeo-web:dev` | Web | Web image tag to build/use. |
| `TIKEO_HTTP_PORT` | `9090` | Server | Host port for HTTP API and health checks. |
| `TIKEO_WORKER_TUNNEL_PORT` | `9998` | Server | Host port for outbound Worker Tunnel clients. |
| `TIKEO_WEB_PORT` | `8080` | Web | Host port for browser UI. |
| `TIKEO_PROMETHEUS_PORT` | `9091` | Prometheus | Host port for optional Prometheus. |
| `TIKEO_DATA_VOLUME` | `tikeo-data` | SQLite | Named volume for SQLite data. |
| `TIKEO_WORKER_TUNNEL_PUBLIC_ENDPOINT` | `http://127.0.0.1:9998` | Workers | Endpoint external demo workers should dial. |
| `TIKEO__STORAGE__DATABASE_URL` | unset | Server | Optional config override for external DB URLs. |

## Worker connectivity

Workers still dial out to the Server Worker Tunnel. For a local Rust demo:

```bash
TIKEO_WORKER_TUNNEL_ENDPOINT=${TIKEO_WORKER_TUNNEL_PUBLIC_ENDPOINT:-http://127.0.0.1:9998} \
  cargo run --manifest-path examples/rust/worker-demo/Cargo.toml
```

Do not expose arbitrary business Worker ports. The only public Worker-facing endpoint should be Tikeo's Server tunnel.

## Cleanup and reset

Stop containers but keep data:

```bash
docker compose --env-file .env down --remove-orphans
```

Delete local SQLite data volume:

```bash
docker compose --env-file .env down --remove-orphans -v
```

For PostgreSQL/MySQL stacks, include the same `-f` file you used for startup.
