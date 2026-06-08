---
title: 配置参考
description: Tikeo 配置位置，以及首次评估最重要的设置。
---

# 配置参考

默认开发配置位于 `config/`。公开文档中的示例应保持简短，并链接到仓库内已提交配置文件，避免复制大段 TOML 后发生漂移。

## 首个本地配置

```bash
cargo run --bin tikeo -- serve --config config/dev.toml
```

## 重要配置区域

- HTTP listener 地址与端口。
- Worker Tunnel listener 地址与端口。
- Storage database URL。
- Transport security：HTTP TLS 与 Worker Tunnel TLS/mTLS。
- Script governance 与 release signature secret reference。
- Alert retry worker 设置。
- Observability 与 tracing exporter。

## 安全规则

schema 变更必须通过显式 SeaORM migration。不要把手工数据库 mutation 写成支持的配置路径。

## 配置评估流程

把配置看成运行时契约的一部分。从 `config/dev.toml` 开始，一次只引入一个变化：数据库 URL、HTTP listener、Worker Tunnel listener、TLS/mTLS、OIDC、alert retry 或 tracing exporter。

## 环境变量

容器化部署可能把环境变量映射到 Tikeo config。部署页面应记录映射关系，并把 secret 放进 Kubernetes Secret、外部 secret store 或 secret reference。公开示例不应包含明文生产凭据。

## Migration 边界

存储 schema 变化必须进入显式 SeaORM migration。兼容 helper 不应在 migration version tracking 之外静默改表。

## 排障方向

Server 启动但 `readyz` 失败时，优先检查 storage 和 migration readiness。Worker 注册失败时，优先检查 tunnel listener、TLS/mTLS 和 worker identity scope。

## 发布前检查

发布或演示前，记录所用 config 文件、数据库后端、HTTP 端口、Worker Tunnel 端口、TLS/mTLS 模式和认证开关。这样排查问题时可以复现实验条件，也能避免把开发态默认值误当成生产推荐配置。
