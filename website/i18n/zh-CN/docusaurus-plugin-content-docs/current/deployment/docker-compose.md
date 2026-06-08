---
title: Docker Compose
description: Tikeo 本地评估与接近生产形态 smoke test 的 Docker Compose 入口。
---

# Docker Compose

Tikeo 可以通过 Docker Compose 进行本地评估、镜像打包验证和数据库 overlay smoke test。Compose 不是生产调度集群的替代品，但能快速证明 Server、Web、存储和 Worker Tunnel 的基础连接。

## SQLite 开发路径

```bash
DOCKER_BUILDKIT=1 docker compose --env-file .env up -d --build
curl -fsS http://127.0.0.1:${TIKEO_HTTP_PORT:-9090}/readyz
```

## 外部数据库 overlay

仓库包含 PostgreSQL 和 MySQL Compose profile。需要验证更接近生产的 schema 行为时，应使用这些 overlay，而不是手动修改 SQLite 表结构。

## 端口

| 端口 | 用途 |
|---|---|
| `9090` | HTTP API 与 Server/Web proxy target |
| `9998` | Worker Tunnel gRPC/HTTP2 listener |
| `80` | Web console 容器内部端口 |

## 清理

```bash
docker compose down --remove-orphans
```

## 何时选择 Compose

Compose 适合本地产品评估、镜像 smoke test、数据库 overlay 验证和演示准备。它可以帮助验证镜像入口、环境变量、readyz、Web 到 Server 的连接、Worker 到 tunnel 的连接。

## Worker 连接规则

Worker 仍然主动连接 Server Worker Tunnel。Compose 应让 Worker 能访问 Server tunnel endpoint，但不应反转架构，让业务 Worker 暴露任意入站执行调用。

## 验证清单

确认 `docker compose config` 渲染正常，Server readiness 成功，Web 容器能访问 Server endpoint，Worker demo 能连接 tunnel，并且 `docker compose down --remove-orphans` 不留下陈旧服务。

## 演示建议

做公开演示时，可以先用 Compose 展示一键启动和 readiness，再切换到 Web 控制台展示 Worker、Jobs、Instances 与 Audit。若需要展示数据库能力，使用 PostgreSQL 或 MySQL overlay，比手工准备 SQLite 行更可信。

这条路径也适合给贡献者复现问题：提交 issue 时附上 compose profile、端口、数据库后端和健康检查输出。
