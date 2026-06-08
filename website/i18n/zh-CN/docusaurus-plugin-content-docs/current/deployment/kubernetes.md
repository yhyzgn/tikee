---
title: Kubernetes 与 Helm
description: Tikeo Kubernetes、Helm、TLS、mTLS 与 Worker identity 部署边界。
---

# Kubernetes 与 Helm

Tikeo 提供 Kubernetes 与 Helm 资产，用于生产部署规划和运维硬化验证。当前 chart 聚焦部署 Tikeo Server/Web 基础设施，不负责部署任意业务 Worker。

## Helm chart 基线

`deploy/helm/tikeo` 支持：

- 外部数据库 Secret 注入；
- 开发态 SQLite PVC 条件创建；
- HTTP listener TLS Secret mount；
- Worker Tunnel TLS/mTLS Secret mount；
- Ingress；
- probes、resources、security contexts；
- 可选 PodDisruptionBudget、NetworkPolicy、ServiceMonitor、Gateway API GRPCRoute；
- `values.schema.json` 校验。

## Worker 规则

chart 不应部署业务 Worker，也不应创建业务 Worker 入站 Service。Worker 必须主动出站连接 Worker Tunnel。这条规则是 Tikeo 区别于传统 server-to-executor callback 的核心部署边界。

## 本地验证

```bash
helm lint deploy/helm/tikeo
helm template tikeo deploy/helm/tikeo --namespace tikeo
```

## 何时选择 Kubernetes

当评估目标包含 identity、NetworkPolicy、TLS/mTLS、Ingress、观测性、外部数据库 Secret 注入、Gateway API 或 Prometheus Operator 集成时，应使用 Kubernetes/Helm 路径。

## 安全边界

HTTP TLS 与 Worker Tunnel TLS/mTLS 是不同关注点。Worker identity 应结合 namespace、app、worker pool、逻辑 worker identity、session generation、fencing token 与 capability snapshot。NetworkPolicy 应保护 Server endpoint，而不是要求 Worker 暴露业务执行 Service。

## 运维 overlay

Helm examples 可用于验证外部数据库、Ingress TLS、ops hardening 与 Gateway API。ServiceMonitor 是可选能力，依赖 Prometheus Operator CRD 已安装。

## 验证清单

确认 `helm lint` 通过，`helm template` 渲染出预期 Secret、mount、Service 和可选资源；外部数据库 values 不再渲染 SQLite PVC；TLS/mTLS values 能写入生成的 transport security config；Gateway API 只在集群具备对应 CRD/controller 时启用。

## 生产评估建议

生产评估应把 chart 渲染结果与运行时行为一起看：Secret 是否只通过引用传递，证书路径是否进入 transport security config，NetworkPolicy 是否保护 Server endpoint，ServiceMonitor 是否只在 CRD 存在时启用，Worker 是否仍保持主动出站连接模型。
