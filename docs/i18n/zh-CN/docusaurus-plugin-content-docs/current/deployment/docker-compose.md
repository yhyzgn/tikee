---
title: Docker Compose
description: SQLite、PostgreSQL、MySQL、Web、Worker Tunnel 与 Prometheus 的完整 Compose 文件和复制即用命令。
---

# Docker Compose

Docker Compose 适合本地或 VM 上做可重复 smoke 环境，包含打包后的 Server 与 Web 容器。本页把仓库中提交的 **完整** `docker-compose*.yml` 全部写出，用户可以直接整段复制。

## 快速启动

```bash
cp deploy/compose/tikeo.env.example .env
DOCKER_BUILDKIT=1 docker compose --env-file .env up -d --build
curl -fsS http://127.0.0.1:${TIKEO_HTTP_PORT:-9090}/readyz
curl -fsS http://127.0.0.1:${TIKEO_WEB_PORT:-8080}/ >/dev/null
```

Web 控制台地址：`http://127.0.0.1:${TIKEO_WEB_PORT:-8080}`。

## 完整 `docker-compose.yml`

SQLite 是最快的本地评估路径，数据保存在 `tikeo-data` named volume。

```yaml
services:
  tikeo:
    build:
      context: .
      dockerfile: Dockerfile
    image: ${TIKEO_IMAGE:-yhyzgn/tikeo-server:local}
    command: ["serve", "--config", "/app/config/container.toml"]
    environment:
      TZ: ${TZ:-Asia/Shanghai}
      MIMALLOC_PURGE_DELAY: ${MIMALLOC_PURGE_DELAY:-0}
      MIMALLOC_PURGE_DECOMMITS: ${MIMALLOC_PURGE_DECOMMITS:-1}
      MIMALLOC_ABANDONED_PAGE_PURGE: ${MIMALLOC_ABANDONED_PAGE_PURGE:-1}
    ports:
      - "${TIKEO_HTTP_PORT:-9090}:9090"
      - "${TIKEO_WORKER_TUNNEL_PORT:-9998}:9998"
    volumes:
      - tikeo-data:/data
    healthcheck:
      test: ["CMD-SHELL", "curl -fsS http://127.0.0.1:9090/readyz >/dev/null"]
      interval: 5s
      timeout: 5s
      retries: 30
      start_period: 10s
    restart: unless-stopped

  web:
    build:
      context: ./web
      dockerfile: Dockerfile
    image: ${TIKEO_WEB_IMAGE:-yhyzgn/tikeo-web:local}
    depends_on:
      tikeo:
        condition: service_healthy
    ports:
      - "${TIKEO_WEB_PORT:-8080}:80"
    healthcheck:
      test: ["CMD-SHELL", "wget -qO- http://127.0.0.1/ >/dev/null"]
      interval: 5s
      timeout: 5s
      retries: 30
      start_period: 5s
    restart: unless-stopped

  prometheus:
    image: prom/prometheus:v3.0.1
    profiles: ["observability"]
    depends_on:
      tikeo:
        condition: service_healthy
    command:
      - "--config.file=/etc/prometheus/prometheus.yml"
      - "--web.enable-lifecycle"
    volumes:
      - ./observability/prometheus/prometheus.yml:/etc/prometheus/prometheus.yml:ro
      - ./observability/prometheus/tikeo-recording-rules.yml:/etc/prometheus/tikeo-recording-rules.yml:ro
    ports:
      - "${TIKEO_PROMETHEUS_PORT:-9091}:9090"
    restart: unless-stopped

volumes:
  tikeo-data:
    name: ${TIKEO_DATA_VOLUME:-tikeo-data}
```

## PostgreSQL stack

```bash
DOCKER_BUILDKIT=1 docker compose --env-file .env -f docker-compose.postgres.yml up -d --build
curl -fsS http://127.0.0.1:${TIKEO_HTTP_PORT:-9090}/readyz
docker compose --env-file .env -f docker-compose.postgres.yml ps
```

常用 `.env` 覆盖：

```dotenv
TIKEO_POSTGRES_PORT=15432
TIKEO_POSTGRES_DB=tikeo
TIKEO_POSTGRES_USER=tikeo
TIKEO_POSTGRES_PASSWORD=change-me
TIKEO_POSTGRES_DATA_VOLUME=tikeo-postgres-data
```

## 完整 `docker-compose.postgres.yml`

```yaml
services:
  tikeo:
    build:
      context: .
      dockerfile: Dockerfile
    image: ${TIKEO_IMAGE:-yhyzgn/tikeo-server:local}
    command: ["serve", "--config", "/app/config/postgres.toml"]
    depends_on:
      postgres:
        condition: service_healthy
    environment:
      TZ: ${TZ:-Asia/Shanghai}
      MIMALLOC_PURGE_DELAY: ${MIMALLOC_PURGE_DELAY:-0}
      MIMALLOC_PURGE_DECOMMITS: ${MIMALLOC_PURGE_DECOMMITS:-1}
      MIMALLOC_ABANDONED_PAGE_PURGE: ${MIMALLOC_ABANDONED_PAGE_PURGE:-1}
    ports:
      - "${TIKEO_HTTP_PORT:-9090}:9090"
      - "${TIKEO_WORKER_TUNNEL_PORT:-9998}:9998"
    healthcheck:
      test: ["CMD-SHELL", "curl -fsS http://127.0.0.1:9090/readyz >/dev/null"]
      interval: 5s
      timeout: 5s
      retries: 30
      start_period: 10s
    restart: unless-stopped

  web:
    build:
      context: ./web
      dockerfile: Dockerfile
    image: ${TIKEO_WEB_IMAGE:-yhyzgn/tikeo-web:local}
    depends_on:
      tikeo:
        condition: service_healthy
    ports:
      - "${TIKEO_WEB_PORT:-8080}:80"
    healthcheck:
      test: ["CMD-SHELL", "wget -qO- http://127.0.0.1/ >/dev/null"]
      interval: 5s
      timeout: 5s
      retries: 30
      start_period: 5s
    restart: unless-stopped

  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: ${TIKEO_POSTGRES_DB:-tikeo}
      POSTGRES_USER: ${TIKEO_POSTGRES_USER:-tikeo}
      POSTGRES_PASSWORD: ${TIKEO_POSTGRES_PASSWORD:-tikeo}
    ports:
      - "${TIKEO_POSTGRES_PORT:-15432}:5432"
    volumes:
      - tikeo-postgres-data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U $${POSTGRES_USER} -d $${POSTGRES_DB}"]
      interval: 5s
      timeout: 5s
      retries: 30
      start_period: 10s
    restart: unless-stopped

  prometheus:
    image: prom/prometheus:v3.0.1
    profiles: ["observability"]
    depends_on:
      tikeo:
        condition: service_healthy
    command:
      - "--config.file=/etc/prometheus/prometheus.yml"
      - "--web.enable-lifecycle"
    volumes:
      - ./observability/prometheus/prometheus.yml:/etc/prometheus/prometheus.yml:ro
      - ./observability/prometheus/tikeo-recording-rules.yml:/etc/prometheus/tikeo-recording-rules.yml:ro
    ports:
      - "${TIKEO_PROMETHEUS_PORT:-9091}:9090"
    restart: unless-stopped

volumes:
  tikeo-postgres-data:
    name: ${TIKEO_POSTGRES_DATA_VOLUME:-tikeo-postgres-data}
```

## MySQL stack

```bash
DOCKER_BUILDKIT=1 docker compose --env-file .env -f docker-compose.mysql.yml up -d --build
curl -fsS http://127.0.0.1:${TIKEO_HTTP_PORT:-9090}/readyz
docker compose --env-file .env -f docker-compose.mysql.yml ps
```

常用 `.env` 覆盖：

```dotenv
TIKEO_MYSQL_PORT=13306
TIKEO_MYSQL_DATABASE=tikeo
TIKEO_MYSQL_USER=tikeo
TIKEO_MYSQL_PASSWORD=change-me
TIKEO_MYSQL_ROOT_PASSWORD=change-root
TIKEO_MYSQL_DATA_VOLUME=tikeo-mysql-data
```

## 完整 `docker-compose.mysql.yml`

```yaml
services:
  tikeo:
    build:
      context: .
      dockerfile: Dockerfile
    image: ${TIKEO_IMAGE:-yhyzgn/tikeo-server:local}
    command: ["serve", "--config", "/app/config/mysql.toml"]
    depends_on:
      mysql:
        condition: service_healthy
    environment:
      TZ: ${TZ:-Asia/Shanghai}
      MIMALLOC_PURGE_DELAY: ${MIMALLOC_PURGE_DELAY:-0}
      MIMALLOC_PURGE_DECOMMITS: ${MIMALLOC_PURGE_DECOMMITS:-1}
      MIMALLOC_ABANDONED_PAGE_PURGE: ${MIMALLOC_ABANDONED_PAGE_PURGE:-1}
    ports:
      - "${TIKEO_HTTP_PORT:-9090}:9090"
      - "${TIKEO_WORKER_TUNNEL_PORT:-9998}:9998"
    healthcheck:
      test: ["CMD-SHELL", "curl -fsS http://127.0.0.1:9090/readyz >/dev/null"]
      interval: 5s
      timeout: 5s
      retries: 30
      start_period: 10s
    restart: unless-stopped

  web:
    build:
      context: ./web
      dockerfile: Dockerfile
    image: ${TIKEO_WEB_IMAGE:-yhyzgn/tikeo-web:local}
    depends_on:
      tikeo:
        condition: service_healthy
    ports:
      - "${TIKEO_WEB_PORT:-8080}:80"
    healthcheck:
      test: ["CMD-SHELL", "wget -qO- http://127.0.0.1/ >/dev/null"]
      interval: 5s
      timeout: 5s
      retries: 30
      start_period: 5s
    restart: unless-stopped

  mysql:
    image: mysql:8.4
    environment:
      MYSQL_DATABASE: ${TIKEO_MYSQL_DATABASE:-tikeo}
      MYSQL_USER: ${TIKEO_MYSQL_USER:-tikeo}
      MYSQL_PASSWORD: ${TIKEO_MYSQL_PASSWORD:-tikeo}
      MYSQL_ROOT_PASSWORD: ${TIKEO_MYSQL_ROOT_PASSWORD:-root}
    command:
      - "--character-set-server=utf8mb4"
      - "--collation-server=utf8mb4_0900_ai_ci"
    ports:
      - "${TIKEO_MYSQL_PORT:-13306}:3306"
    volumes:
      - tikeo-mysql-data:/var/lib/mysql
    healthcheck:
      test: ["CMD-SHELL", "mysqladmin ping -h 127.0.0.1 -uroot -p$${MYSQL_ROOT_PASSWORD} --silent"]
      interval: 5s
      timeout: 5s
      retries: 60
      start_period: 20s
    restart: unless-stopped

  prometheus:
    image: prom/prometheus:v3.0.1
    profiles: ["observability"]
    depends_on:
      tikeo:
        condition: service_healthy
    command:
      - "--config.file=/etc/prometheus/prometheus.yml"
      - "--web.enable-lifecycle"
    volumes:
      - ./observability/prometheus/prometheus.yml:/etc/prometheus/prometheus.yml:ro
      - ./observability/prometheus/tikeo-recording-rules.yml:/etc/prometheus/tikeo-recording-rules.yml:ro
    ports:
      - "${TIKEO_PROMETHEUS_PORT:-9091}:9090"
    restart: unless-stopped

volumes:
  tikeo-mysql-data:
    name: ${TIKEO_MYSQL_DATA_VOLUME:-tikeo-mysql-data}
```

## 可选 Prometheus

三套 Compose 文件都包含 `observability` profile 下的 Prometheus 服务。

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

Worker 仍然主动连接 Server Worker Tunnel。本地 Rust demo：

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

## 生产替换提醒

上面的完整 YAML 与仓库文件保持一致，适合直接复制做本地评估。真正用于共享环境时，请至少替换镜像 tag、数据库密码、宿主机端口、数据卷名称和 Worker Tunnel 对外地址。如果 Worker 运行在另一台机器或另一个网络中，`TIKEO_WORKER_TUNNEL_PUBLIC_ENDPOINT` 不能保留 `127.0.0.1`，必须改成 Worker 可访问的域名或 IP。Compose 不负责证书签发、Secret 管理或滚动发布；这些能力应交给 Kubernetes、systemd 周边工具或企业部署平台。

## 验证顺序

建议按顺序验证：Compose 服务状态、Server `readyz`、Web 首页、Worker Tunnel demo、实例日志。只看到容器处于 running 不代表调度链路可用；至少要启动一个 Worker demo 并确认任务日志/结果能回传。

## 前置条件

执行本页命令前，请先满足页面列出的安装、认证和权限要求。本地示例默认 Server 使用 `config/dev.toml`，客户端访问 `127.0.0.1`，令牌保存在 shell 变量中，不写入文件或截图。

## 验收

完成本页步骤后，用对应 API、UI、构建、smoke 或部署检查验证结果。有效验收至少包含执行的命令、检查的路由或文件，以及观察到的状态或产物。

## 故障排查

步骤失败时，先保留完整命令、响应状态和 Server 日志时间窗口，再检查认证、namespace/app scope、Worker 匹配、存储 readiness 和代理行为，不要直接修改生产配置。

## 生产检查清单

- [ ] 密钥通过环境变量或平台 Secret 引用管理，不写入示例。
- [ ] 已把本地 `127.0.0.1` 命令替换成真实域名、TLS 和认证方式。
- [ ] 已记录变更面的回滚和证据采集方式。
- [ ] 运维人员可以在没有隐藏 shell 历史或隐式状态的情况下复现验收。
