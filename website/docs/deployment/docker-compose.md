---
title: Docker Compose
description: Local and production-shaped Docker Compose entry points for Tikeo.
---

# Docker Compose

Tikeo can run with Docker Compose for local evaluation and production-shaped smoke tests.

## SQLite development path

```bash
DOCKER_BUILDKIT=1 docker compose --env-file .env up -d --build
curl -fsS http://127.0.0.1:${TIKEO_HTTP_PORT:-9090}/readyz
```

## External database overlays

The repository also includes PostgreSQL and MySQL Compose profiles. Use these when you need to verify schema behavior against a production-style database.

## Ports

| Port | Purpose |
|---|---|
| `9090` | HTTP API and Server/Web proxy target |
| `9998` | Worker Tunnel gRPC/HTTP2 listener |
| `80` | Web console container internal port |

## Cleanup

```bash
docker compose down --remove-orphans
```

## When to choose Compose

Docker Compose is best for local product evaluation, smoke testing packaged images, and validating database overlays without a Kubernetes cluster. It is not a substitute for production scheduling or cluster policy, but it gives a fast path for demonstrating Server, Web, storage, and Worker connectivity.

## Database choices

SQLite is suitable for a quick local evaluation. PostgreSQL and MySQL overlays are better when validating operational behavior closer to production. Always let Tikeo run its migration path; do not manually patch tables to make a demo pass.

## Worker connectivity

Workers still dial out to the Server Worker Tunnel. Compose should expose the Server tunnel endpoint to workers but should not invert the architecture by making business workers receive arbitrary inbound execution calls.

## Verification checklist

- `docker compose config` renders cleanly.
- Server readiness returns success.
- Web container can reach the configured Server endpoint.
- Worker demo can connect to the tunnel.
- `docker compose down --remove-orphans` leaves no stale local services.
