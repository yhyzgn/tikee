---
title: Docker Compose
description: SQLite、PostgreSQL、MySQL、Web、Worker Tunnel 与 Prometheus 的复制即用 Compose 部署命令。
---

# Docker Compose

Docker Compose 适合本地或 VM 上做可重复 smoke 环境，包含打包后的 Server 与 Web 容器。仓库提供三套独立 stack：`docker-compose.yml`（SQLite）、`docker-compose.postgres.yml`（PostgreSQL）、`docker-compose.mysql.yml`（MySQL）。

## 1. 准备 `.env`

```bash
cp deploy/compose/tikeo.env.example .env
```

默认暴露 HTTP `9090`、Worker Tunnel `9998`、Web `8080`、可选 Prometheus `9091`。

## 2. SQLite 一键启动

```bash
DOCKER_BUILDKIT=1 docker compose --env-file .env up -d --build
curl -fsS http://127.0.0.1:${TIKEO_HTTP_PORT:-9090}/readyz
curl -fsS http://127.0.0.1:${TIKEO_WEB_PORT:-8080}/ >/dev/null
```

打开 `http://127.0.0.1:${TIKEO_WEB_PORT:-8080}` 查看 Web 控制台。

## 3. PostgreSQL stack

```bash
DOCKER_BUILDKIT=1 docker compose --env-file .env -f docker-compose.postgres.yml up -d --build
curl -fsS http://127.0.0.1:${TIKEO_HTTP_PORT:-9090}/readyz
docker compose --env-file .env -f docker-compose.postgres.yml ps
```

可覆盖参数：`TIKEO_POSTGRES_PORT`、`TIKEO_POSTGRES_DB`、`TIKEO_POSTGRES_USER`、`TIKEO_POSTGRES_PASSWORD`、`TIKEO_POSTGRES_DATA_VOLUME`。

## 4. MySQL stack

```bash
DOCKER_BUILDKIT=1 docker compose --env-file .env -f docker-compose.mysql.yml up -d --build
curl -fsS http://127.0.0.1:${TIKEO_HTTP_PORT:-9090}/readyz
docker compose --env-file .env -f docker-compose.mysql.yml ps
```

可覆盖参数：`TIKEO_MYSQL_PORT`、`TIKEO_MYSQL_DATABASE`、`TIKEO_MYSQL_USER`、`TIKEO_MYSQL_PASSWORD`、`TIKEO_MYSQL_ROOT_PASSWORD`、`TIKEO_MYSQL_DATA_VOLUME`。MySQL stack 已设置 `utf8mb4`。

## 5. 可选 Prometheus

```bash
docker compose --env-file .env --profile observability up -d prometheus
curl -fsS http://127.0.0.1:${TIKEO_PROMETHEUS_PORT:-9091}/-/ready
```

## Compose 参数表

| 变量 | 默认值 | 含义 |
|---|---:|---|
| `TIKEO_IMAGE` | `yhyzgn/tikeo-server:dev` | Server 镜像。 |
| `TIKEO_WEB_IMAGE` | `yhyzgn/tikeo-web:dev` | Web 镜像。 |
| `TIKEO_HTTP_PORT` | `9090` | HTTP API / health host 端口。 |
| `TIKEO_WORKER_TUNNEL_PORT` | `9998` | Worker Tunnel host 端口。 |
| `TIKEO_WEB_PORT` | `8080` | Web UI host 端口。 |
| `TIKEO_PROMETHEUS_PORT` | `9091` | Prometheus host 端口。 |
| `TIKEO_WORKER_TUNNEL_PUBLIC_ENDPOINT` | `http://127.0.0.1:9998` | 外部 demo Worker 主动连接地址。 |
| `TIKEO__STORAGE__DATABASE_URL` | 未设置 | 覆盖 Server 数据库 URL。 |

## Worker 连接规则

Worker 仍然主动连接 Server Worker Tunnel。本地 Rust demo 示例：

```bash
TIKEO_WORKER_TUNNEL_ENDPOINT=${TIKEO_WORKER_TUNNEL_PUBLIC_ENDPOINT:-http://127.0.0.1:9998}   cargo run --manifest-path examples/rust/worker-demo/Cargo.toml
```

不要为业务 Worker 暴露任意入站端口。

## 清理

```bash
docker compose --env-file .env down --remove-orphans
# 删除 SQLite 数据卷：
docker compose --env-file .env down --remove-orphans -v
```

PostgreSQL/MySQL 清理时要带上启动时使用的 `-f` 文件。

## 适用边界

Compose 的目标是让评估者复制命令后快速得到可用环境，而不是替代生产编排。共享环境应改用外部数据库、平台 Secret 和更严格的网络边界。每次演示前建议运行 readiness、Web 首页、Worker Tunnel demo 三个检查，避免只证明容器启动而没有证明调度链路可用。

## 参数替换建议

首次运行可以只复制 `.env.example`。多人共享或对外演示时，至少替换数据库密码、宿主机端口、镜像 tag 和数据卷名称。如果 Worker 不在同一台机器上，务必把 `TIKEO_WORKER_TUNNEL_PUBLIC_ENDPOINT` 改成 Worker 主机可访问的地址，而不是保留 `127.0.0.1`。
