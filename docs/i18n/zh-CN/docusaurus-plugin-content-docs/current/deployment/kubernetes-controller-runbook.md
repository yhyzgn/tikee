---
title: Kubernetes 控制器专项运行手册
description: 面向 Nginx、Envoy Gateway、Traefik 与 Gateway API 的 Tikeo 生产暴露、TLS/mTLS 与 smoke runbook。
---

# Kubernetes 控制器专项运行手册

本页补齐 Helm chart 的 controller-specific 生产文档。事实来源是 `deploy/helm/tikeo/values.yaml`、`deploy/helm/tikeo/examples/values-ingress-tls.yaml`、`deploy/helm/tikeo/examples/values-gateway-api-worker-tunnel.yaml`、`deploy/helm/tikeo/templates/server.yaml`、`deploy/helm/tikeo/templates/gateway-api.yaml`、`deploy/helm/tikeo/templates/networkpolicy.yaml`。

Tikeo 的边界不变：业务 Worker 不暴露入站 Service。运维人员可以暴露 Tikeo Server 管理 API、Web 控制台和 Worker Tunnel endpoint，但业务 Worker 仍然主动出站连接 Server 侧 Worker Tunnel。渲染出的 Service 与 `GRPCRoute` 带有 `tikeo.yhyzgn.com/worker-networking: "workers-connect-outbound-only"`，用于在集群资产盘点时明确这个边界。

## Helm 参数事实表

不要手写分叉 manifest，优先使用这些 chart values：

| 暴露面 | Helm values | 渲染资源 |
|---|---|---|
| 管理 API | `server.ingress.enabled`、`server.ingress.className`、`server.ingress.annotations`、`server.ingress.hosts`、`server.ingress.tls` | 指向 HTTP Service 的 `tikeo-server` Ingress。 |
| Web 控制台 | `web.ingress.enabled`、`web.ingress.className`、`web.ingress.hosts`、`web.ingress.tls` | 指向 Web Service 的 `tikeo-web` Ingress。 |
| Worker Tunnel Service | `server.workerTunnelService.type`、`server.workerTunnelService.port`、`server.workerTunnelService.annotations` | `tikeo-worker-tunnel` Service，端口名 `grpc-worker-tunnel`。 |
| listener TLS | `server.tls.http.enabled`、`server.tls.http.existingSecret`、`server.tls.workerTunnel.enabled`、`server.tls.workerTunnel.existingSecret` | Secret mount 与 `[transport_security.http]` / `[transport_security.worker_tunnel]` 配置。 |
| Worker mTLS | `server.tls.workerTunnel.mtlsRequired`、`server.tls.workerTunnel.clientCaSecret` | Worker Tunnel listener 要求客户端证书由 `ca.crt` 签发。 |
| Gateway API | `gatewayApi.enabled`、`gatewayApi.gateway.*`、`gatewayApi.workerTunnelRoute.*` | 转发到 `tikeo-worker-tunnel` Service 的 `Gateway` 与 `GRPCRoute`。 |
| NetworkPolicy | `networkPolicy.enabled`、`networkPolicy.server.httpFrom`、`networkPolicy.server.workerTunnelFrom`、`networkPolicy.web.ingressFrom` | server / web Pod 入站策略。 |

已提交的 overlay 是最小安全起点：

```bash
cat deploy/helm/tikeo/examples/values-ingress-tls.yaml
cat deploy/helm/tikeo/examples/values-gateway-api-worker-tunnel.yaml
```

## Nginx Ingress

Nginx Ingress 适合 Web 控制台和 HTTP 管理 API。仓库里的 `values-ingress-tls.yaml` 设置了：

```yaml
server:
  ingress:
    enabled: true
    className: nginx
    annotations:
      nginx.ingress.kubernetes.io/backend-protocol: HTTPS
web:
  ingress:
    enabled: true
    className: nginx
```

`nginx.ingress.kubernetes.io/backend-protocol: HTTPS` 只有在 `server.tls.http.enabled=true` 且 API ingress 后端真实使用 HTTPS 时才正确。如果你的环境只在 Nginx 边缘终止 TLS，Tikeo HTTP listener 保持明文，就需要移除或覆盖这个 annotation。Ingress TLS 与 Tikeo listener TLS 是两个边界，不应互相假设。

渲染并安装：

```bash
helm template tikeo ./deploy/helm/tikeo \
  --namespace tikeo \
  -f deploy/helm/tikeo/examples/values-external-postgres.yaml \
  -f deploy/helm/tikeo/examples/values-ingress-tls.yaml

helm upgrade --install tikeo ./deploy/helm/tikeo \
  --namespace tikeo --create-namespace \
  -f deploy/helm/tikeo/examples/values-external-postgres.yaml \
  -f deploy/helm/tikeo/examples/values-ingress-tls.yaml
```

DNS 指向 controller 后做 smoke：

```bash
kubectl -n tikeo rollout status deploy/tikeo-server
kubectl -n tikeo rollout status deploy/tikeo-web
curl -fsS https://api.tikeo.example.com/readyz
curl -fsS https://tikeo.example.com/ | grep -i tikeo
kubectl -n tikeo get ingress tikeo-server tikeo-web
```

## Envoy Gateway

Worker Tunnel 是 gRPC/HTTP2，建议用 Envoy Gateway 或其他 Gateway API controller 暴露，不要强行塞进默认 HTTP/1 ingress 行为。仓库示例设置 `gatewayApi.enabled=true`、`gatewayApi.gateway.className=envoy-gateway`、listener name `grpc-worker-tunnel`，并通过 `GRPCRoute` 把流量转到 `tikeo-worker-tunnel` Service。

使用已提交 overlay：

```bash
kubectl -n tikeo create secret tls tikeo-worker-tunnel-gateway-tls \
  --cert=./certs/worker-tunnel-gateway.crt \
  --key=./certs/worker-tunnel-gateway.key

helm upgrade --install tikeo ./deploy/helm/tikeo \
  --namespace tikeo --create-namespace \
  -f deploy/helm/tikeo/examples/values-external-postgres.yaml \
  -f deploy/helm/tikeo/examples/values-ingress-tls.yaml \
  -f deploy/helm/tikeo/examples/values-gateway-api-worker-tunnel.yaml
```

检查 Gateway API 资源：

```bash
kubectl -n tikeo get gateway tikeo-worker-tunnel
kubectl -n tikeo get grpcroute tikeo-worker-tunnel
kubectl -n tikeo describe grpcroute tikeo-worker-tunnel
kubectl -n tikeo get svc tikeo-worker-tunnel -o jsonpath='{.spec.ports[?(@.name=="grpc-worker-tunnel")].port}'
```

真实 Worker 连接需要把 Worker SDK 或 demo 进程配置为外部可达的 Worker Tunnel URL。Helm chart 不部署这些业务 Worker；它们仍然在 chart 外部主动出站连接。

## Traefik

Traefik 可以通过 Kubernetes Ingress 暴露 Web 和管理 API，也可以在 Gateway API 模式下使用兼容的 Gateway API 资源。对于 chart 自带 Ingress 路径，设置 `server.ingress.className=traefik` 和 `web.ingress.className=traefik`；Traefik 特有 TLS/router annotation 通过 `server.ingress.annotations` 与 `web.ingress.annotations` 注入，不要复制模板。

示例 overlay：

```yaml
server:
  ingress:
    enabled: true
    className: traefik
    annotations:
      traefik.ingress.kubernetes.io/router.entrypoints: websecure
      traefik.ingress.kubernetes.io/router.tls: "true"
web:
  ingress:
    enabled: true
    className: traefik
    annotations:
      traefik.ingress.kubernetes.io/router.entrypoints: websecure
      traefik.ingress.kubernetes.io/router.tls: "true"
```

Smoke 与 Nginx 类似：

```bash
helm template tikeo ./deploy/helm/tikeo --namespace tikeo -f ./values-traefik.yaml
helm upgrade --install tikeo ./deploy/helm/tikeo --namespace tikeo -f ./values-traefik.yaml
kubectl -n tikeo get ingress
curl -fsS https://api.tikeo.example.com/readyz
curl -fsS https://tikeo.example.com/ | grep -i tikeo
```

除非 chart 未来增加 Traefik IngressRoute 一等模板，否则不要把 Traefik CRD 作为仓库唯一推荐路径。自定义 CRD 应由 operator overlay 管理，chart 内仍以 `server.ingress.*`、`web.ingress.*`、`server.workerTunnelService.annotations`、`gatewayApi.*` 作为事实源。

## 通用 Gateway API

对 Envoy Gateway 之外的 Gateway API controller，保持 chart 结构不变，只替换 values：

```yaml
gatewayApi:
  enabled: true
  gateway:
    create: true
    className: <your-gateway-class>
    listenerName: grpc-worker-tunnel
    hostname: worker-tunnel.tikeo.example.com
    port: 443
    tls:
      mode: Terminate
      certificateRefs:
        - kind: Secret
          name: tikeo-worker-tunnel-gateway-tls
  workerTunnelRoute:
    enabled: true
    hostnames:
      - worker-tunnel.tikeo.example.com
```

应用前先渲染检查：

```bash
helm template tikeo ./deploy/helm/tikeo \
  --namespace tikeo \
  -f deploy/helm/tikeo/examples/values-gateway-api-worker-tunnel.yaml \
  | grep -E 'kind: Gateway|kind: GRPCRoute|grpc-worker-tunnel|workers-connect-outbound-only'
```

Smoke 预期：

```bash
kubectl -n tikeo get gateway,grpcroute
kubectl -n tikeo describe gateway tikeo-worker-tunnel
kubectl -n tikeo describe grpcroute tikeo-worker-tunnel
curl -fsS https://api.tikeo.example.com/readyz
```

`curl -fsS` 只能证明 HTTP 暴露面。Worker Tunnel gRPC 需要真实 Worker SDK/demo 进程或未来 gRPC health/reflection probe 验证，不能用 HTTP GET 代替 Worker 注册证据。

## TLS/mTLS 矩阵

| 层级 | Values | 生产建议 |
|---|---|---|
| 浏览器/Web ingress TLS | `web.ingress.tls` | 在 Nginx、Traefik 或 Gateway 终止浏览器证书。 |
| 管理 API ingress TLS | `server.ingress.tls` | 在边缘终止；必要时再加密到 Tikeo HTTP listener。 |
| Tikeo HTTP listener TLS | `server.tls.http.enabled`、`server.tls.http.existingSecret` | ingress 到后端也要加密时启用。 |
| Worker Tunnel listener TLS | `server.tls.workerTunnel.enabled`、`server.tls.workerTunnel.existingSecret` | 共享集群或远程 Worker 必须启用。 |
| Worker Tunnel mTLS | `server.tls.workerTunnel.mtlsRequired`、`server.tls.workerTunnel.clientCaSecret` | Worker 跨信任边界、跨集群或跨 VPC 时启用。 |
| Gateway TLS | `gatewayApi.gateway.tls.certificateRefs` | 根据 controller 选择 terminate 或透传，并在 values 中显式表达。 |

如果 `server.tls.workerTunnel.mtlsRequired=true`，Worker client 必须携带由配置的 `ca.crt` 签发的证书。没有证书时注册失败是预期安全行为，不应该通过关闭校验绕过。

## NetworkPolicy 与 Worker 边界 smoke

`networkPolicy.enabled=true` 会限制 server/web Pod 入站，不会部署业务 Worker，也不会要求 Worker 入站 Service。Worker Tunnel ingress 规则由 `networkPolicy.server.workerTunnelFrom` 控制；为空时会渲染较宽的 namespace selector，方便远端 namespace 仍然主动连接 Tunnel。只有确认 Worker namespace 或 gateway namespace 后才收紧。

边界 smoke：

```bash
helm template tikeo ./deploy/helm/tikeo \
  --namespace tikeo \
  -f deploy/helm/tikeo/examples/values-ops-hardening.yaml \
  | grep -E 'kind: NetworkPolicy|workers-connect-outbound-only|workerTunnelFrom'

kubectl -n tikeo get networkpolicy
kubectl -n tikeo get svc tikeo-worker-tunnel -o yaml | grep -E 'grpc-worker-tunnel|workers-connect-outbound-only'
```

合格的生产 smoke 要拆成三个独立结论：API/Web 通过 `curl -fsS` readiness，Gateway/Ingress 资源被 controller 接受，至少一个真实 Worker 进程通过出站 Worker Tunnel 注册。发布记录和事故记录都应分开记录这些证据。
