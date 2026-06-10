---
title: Kubernetes controller-specific runbook
description: Source-backed Nginx, Envoy Gateway, Traefik, and Gateway API production runbook for Tikeo HTTP, Web, and Worker Tunnel exposure.
---

# Kubernetes controller-specific runbook

This page finishes the controller-specific production docs slice for the Helm chart. It is source-backed by `deploy/helm/tikeo/values.yaml`, `deploy/helm/tikeo/examples/values-ingress-tls.yaml`, `deploy/helm/tikeo/examples/values-gateway-api-worker-tunnel.yaml`, `deploy/helm/tikeo/templates/server.yaml`, `deploy/helm/tikeo/templates/gateway-api.yaml`, and `deploy/helm/tikeo/templates/networkpolicy.yaml`.

The non-negotiable Tikeo boundary remains unchanged: business Workers do not expose inbound Services. Operators may expose the Tikeo Server management API, Web console, and Worker Tunnel endpoint, but Workers still connect outbound to the server-side Worker Tunnel. The rendered Service and `GRPCRoute` carry `tikeo.yhyzgn.com/worker-networking: "workers-connect-outbound-only"` to make that boundary visible in cluster inventory.

## Source-backed Helm knobs

Use these chart values rather than ad-hoc manifests:

| Surface | Helm values | Rendered resource |
|---|---|---|
| Management API | `server.ingress.enabled`, `server.ingress.className`, `server.ingress.annotations`, `server.ingress.hosts`, `server.ingress.tls` | `Ingress` named `tikeo-server` targeting the HTTP Service. |
| Web console | `web.ingress.enabled`, `web.ingress.className`, `web.ingress.hosts`, `web.ingress.tls` | `Ingress` named `tikeo-web` targeting the Web Service. |
| Worker Tunnel Service | `server.workerTunnelService.type`, `server.workerTunnelService.port`, `server.workerTunnelService.annotations` | `Service` named `tikeo-worker-tunnel`, port name `grpc-worker-tunnel`. |
| Listener TLS | `server.tls.http.enabled`, `server.tls.http.existingSecret`, `server.tls.workerTunnel.enabled`, `server.tls.workerTunnel.existingSecret` | Secret mounts plus `[transport_security.http]` and `[transport_security.worker_tunnel]` config. |
| Worker mTLS | `server.tls.workerTunnel.mtlsRequired`, `server.tls.workerTunnel.clientCaSecret` | Worker Tunnel listener requires client certificates signed by `ca.crt`. |
| Gateway API | `gatewayApi.enabled`, `gatewayApi.gateway.*`, `gatewayApi.workerTunnelRoute.*` | `Gateway` and `GRPCRoute` forwarding to the `tikeo-worker-tunnel` Service. |
| NetworkPolicy | `networkPolicy.enabled`, `networkPolicy.server.httpFrom`, `networkPolicy.server.workerTunnelFrom`, `networkPolicy.web.ingressFrom` | Ingress-only policies for server and web pods. |

The example overlays already encode the safest starting points:

```bash
# API/Web ingress plus listener TLS/mTLS wiring.
cat deploy/helm/tikeo/examples/values-ingress-tls.yaml

# Gateway API GRPCRoute for the Worker Tunnel h2/gRPC endpoint.
cat deploy/helm/tikeo/examples/values-gateway-api-worker-tunnel.yaml
```

## Nginx Ingress

Use Nginx Ingress for the browser Web console and the HTTP management API. The shipped `values-ingress-tls.yaml` sets:

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

The annotation is only correct when `server.tls.http.enabled=true` and the API ingress backend really speaks HTTPS. If you terminate TLS at Nginx and leave the Tikeo HTTP listener plaintext, remove or override `nginx.ingress.kubernetes.io/backend-protocol: HTTPS` for that environment. Keep ingress TLS and listener TLS as separate boundaries; do not assume one implies the other.

Render and install:

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

Smoke the API and Web surfaces after DNS points at the controller:

```bash
kubectl -n tikeo rollout status deploy/tikeo-server
kubectl -n tikeo rollout status deploy/tikeo-web
curl -fsS https://api.tikeo.example.com/readyz
curl -fsS https://tikeo.example.com/ | grep -i tikeo
kubectl -n tikeo get ingress tikeo-server tikeo-web
```

## Envoy Gateway

Prefer Envoy Gateway or another Gateway API controller for the Worker Tunnel because the tunnel is gRPC/HTTP2 and should not be squeezed through generic HTTP/1 ingress defaults. The source example sets `gatewayApi.enabled=true`, `gatewayApi.gateway.className=envoy-gateway`, listener name `grpc-worker-tunnel`, and a `GRPCRoute` backend pointing to the `tikeo-worker-tunnel` Service.

Start with the shipped overlay:

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

Smoke the rendered Gateway API resources:

```bash
kubectl -n tikeo get gateway tikeo-worker-tunnel
kubectl -n tikeo get grpcroute tikeo-worker-tunnel
kubectl -n tikeo describe grpcroute tikeo-worker-tunnel
kubectl -n tikeo get svc tikeo-worker-tunnel -o jsonpath='{.spec.ports[?(@.name=="grpc-worker-tunnel")].port}'
```

For an actual Worker connection, configure Worker SDK or demo processes with the externally reachable Worker Tunnel URL. The Helm chart does not deploy those business Workers; they still run outside this chart and connect outbound.

## Traefik

Traefik can serve the Web console and management API through Kubernetes Ingress or, in Gateway API mode, through compatible Gateway API resources. For the chart-owned Ingress path, set `server.ingress.className=traefik` and `web.ingress.className=traefik`; add controller-specific TLS/router annotations through `server.ingress.annotations` and `web.ingress.annotations` instead of forking manifests.

Example overlay:

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

Smoke it the same way as Nginx:

```bash
helm template tikeo ./deploy/helm/tikeo --namespace tikeo -f ./values-traefik.yaml
helm upgrade --install tikeo ./deploy/helm/tikeo --namespace tikeo -f ./values-traefik.yaml
kubectl -n tikeo get ingress
curl -fsS https://api.tikeo.example.com/readyz
curl -fsS https://tikeo.example.com/ | grep -i tikeo
```

Do not use Traefik IngressRoute CRDs as the only documented path in this repository unless the chart grows first-class templates for them. Until then, keep controller-specific custom resources as operator-owned overlays and keep the chart source of truth in `server.ingress.*`, `web.ingress.*`, `server.workerTunnelService.annotations`, and `gatewayApi.*`.

## Generic Gateway API

For Gateway API controllers other than Envoy Gateway, keep the chart shape and change only the values:

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

Then render and inspect the exact manifests before applying:

```bash
helm template tikeo ./deploy/helm/tikeo \
  --namespace tikeo \
  -f deploy/helm/tikeo/examples/values-gateway-api-worker-tunnel.yaml \
  | grep -E 'kind: Gateway|kind: GRPCRoute|grpc-worker-tunnel|workers-connect-outbound-only'
```

Smoke expectations:

```bash
kubectl -n tikeo get gateway,grpcroute
kubectl -n tikeo describe gateway tikeo-worker-tunnel
kubectl -n tikeo describe grpcroute tikeo-worker-tunnel
curl -fsS https://api.tikeo.example.com/readyz
```

`curl -fsS` verifies HTTP surfaces only. Worker Tunnel gRPC verification must use a real Worker SDK/demo process or a future gRPC health/reflection probe; do not substitute an HTTP GET for Worker registration proof.

## TLS and mTLS matrix

| Layer | Values | Production recommendation |
|---|---|---|
| Browser/Web ingress TLS | `web.ingress.tls` | Terminate at Nginx, Traefik, or Gateway with a public/browser certificate. |
| Management API ingress TLS | `server.ingress.tls` | Terminate at the edge; optionally re-encrypt to Tikeo HTTP listener. |
| Tikeo HTTP listener TLS | `server.tls.http.enabled` and `server.tls.http.existingSecret` | Enable when ingress/backend traffic must also be encrypted. |
| Worker Tunnel listener TLS | `server.tls.workerTunnel.enabled` and `server.tls.workerTunnel.existingSecret` | Enable for shared clusters and any remote Worker path. |
| Worker Tunnel mTLS | `server.tls.workerTunnel.mtlsRequired` and `server.tls.workerTunnel.clientCaSecret` | Enable when Workers run outside the trust boundary or across clusters/VPCs. |
| Gateway TLS | `gatewayApi.gateway.tls.certificateRefs` | Terminate or pass through according to the controller; keep it explicit in values. |

If `server.tls.workerTunnel.mtlsRequired=true`, Worker clients must carry certificates signed by the configured `ca.crt`. Registration failures in that mode are expected until the Worker deployment or VM/systemd process receives the matching client identity.

## NetworkPolicy and Worker boundary smoke

`networkPolicy.enabled=true` limits ingress to server and web pods. It does not deploy business Workers and does not require Worker inbound Services. The Worker Tunnel ingress rule uses `networkPolicy.server.workerTunnelFrom`; leaving it empty renders a broad namespace selector so remote namespaces can still connect outbound to the tunnel Service. Tighten it only after you know the Worker namespaces or gateway namespaces.

Smoke the boundary:

```bash
helm template tikeo ./deploy/helm/tikeo \
  --namespace tikeo \
  -f deploy/helm/tikeo/examples/values-ops-hardening.yaml \
  | grep -E 'kind: NetworkPolicy|workers-connect-outbound-only|workerTunnelFrom'

kubectl -n tikeo get networkpolicy
kubectl -n tikeo get svc tikeo-worker-tunnel -o yaml | grep -E 'grpc-worker-tunnel|workers-connect-outbound-only'
```

A valid production smoke includes three separate claims: API/Web HTTP readiness through `curl -fsS`, Gateway/Ingress resources accepted by the controller, and at least one real Worker process registered through outbound Worker Tunnel. Keep those claims separate in incident notes and release evidence.
