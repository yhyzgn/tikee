---
title: Kubernetes and Helm
description: Kubernetes, Helm, TLS, mTLS, and worker identity deployment boundaries.
---

# Kubernetes and Helm

Tikeo includes Kubernetes and Helm assets for production deployment planning.

## Helm chart baseline

The local chart under `deploy/helm/tikeo` supports:

- external database Secret injection;
- conditional SQLite PVC for development;
- HTTP listener TLS Secret mounts;
- Worker Tunnel TLS/mTLS Secret mounts;
- Ingress;
- probes, resources, security contexts;
- optional PodDisruptionBudget, NetworkPolicy, ServiceMonitor, and Gateway API GRPCRoute;
- `values.schema.json` validation.

## Worker rule

The chart must not deploy business Workers or create business Worker inbound Services. Workers connect outbound to the Worker Tunnel.

## Local validation

```bash
helm lint deploy/helm/tikeo
helm template tikeo deploy/helm/tikeo --namespace tikeo
```

## When to choose Kubernetes

Kubernetes is the right target for production-like evaluation of identity, network policy, TLS/mTLS, ingress, observability, and database Secret injection. The chart is designed to deploy Tikeo infrastructure, not arbitrary business Worker workloads.

## Security boundaries

HTTP TLS and Worker Tunnel TLS/mTLS are separate concerns. Worker identity should combine namespace/app/worker-pool scope, logical worker identity, session generation, fencing token, and capability snapshots. NetworkPolicy should protect Server endpoints without requiring workers to expose business execution Services.

## Operational overlays

Use Helm examples to test external database, ingress TLS, ops hardening, and Gateway API paths. ServiceMonitor support is optional and assumes Prometheus Operator CRDs are installed.

## Verification checklist

- `helm lint deploy/helm/tikeo` passes.
- `helm template` renders expected Secrets, mounts, Services, and optional resources.
- External database values render no SQLite PVC.
- TLS/mTLS values mount certificate paths into generated transport security config.
- Gateway API examples are used only when matching CRDs/controllers exist.
