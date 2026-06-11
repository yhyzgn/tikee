# Workers user guide

## Overview

The Workers page shows execution capacity: active outbound Worker Tunnel sessions, structured capabilities, lifecycle history, and dispatch queue context. It is the first page to open when instances are pending or a processor cannot be found.

Implementation anchors: `web/src/pages/WorkersPage.tsx` reads `/api/v1/workers`, `/api/v1/workers/history`, `/api/v1/workers/stream`, and dispatch queue views. Runtime execution uses `Worker Tunnel`, `WorkerTunnelService`, `OpenTunnel`, `RegisterWorker`, `Heartbeat`, `DispatchTask`, `TaskLog`, `TaskResult`, and `TaskCheckpoint`.

## Prerequisites

- `workers:read` permission.
- A Worker process can reach the Server Worker Tunnel endpoint outbound.
- Worker registration has correct namespace, app, cluster, region, worker pool, labels, and capabilities.
- Jobs reference capabilities that the Worker can actually execute.

```bash
curl -fsS http://127.0.0.1:9090/api/v1/workers \
  -H "authorization: Bearer $TIKEO_TOKEN" | jq '.data.items[] | {workerId,status,namespace,app,structuredCapabilities}'
```

## Open the page

1. Select **Workers** or open `/workers`.
2. Review summary counts, Worker table, and lifecycle history.
3. Open dispatch queue when pending work needs deeper triage.
4. Compare Worker metadata with the Job that is waiting.

## Common tasks

### Confirm outbound connectivity

Workers dial the Server; the Server does not call business Workers. If a Worker is missing, check endpoint, TLS mode, network egress, namespace/app, and logs in the Worker process.

### Verify capabilities

Inspect `sdkProcessors`, `scriptRunners`, `pluginProcessors`, tags, labels, region, cluster, and worker pool. Do not advertise a runner or processor unless the runtime can execute it safely.

### Diagnose no eligible worker

Take the processor/script/plugin from the Job, then match it against Worker capabilities in the same namespace/app. For broadcast, also match selector labels/region/cluster.

## Verify

- Online count increases when a Worker connects through the outbound tunnel.
- Lifecycle history records disconnects/reconnects.
- Structured capabilities match the runtime and selected jobs.
- Dispatch queue explains pending, running, done, or failed handoff states.
- No business Worker inbound Service is required.

## Troubleshooting

| Symptom | Action |
| --- | --- |
| Worker invisible | Verify `TIKEO_WORKER_ENDPOINT`, TLS/plaintext mode, and network egress. |
| Worker reconnects repeatedly | Inspect lifecycle history and Worker logs for heartbeat or auth errors. |
| Job cannot dispatch | Match namespace/app and structured capabilities. |
| Script job pending | Confirm script runner language/backend is advertised and installed. |
| Broadcast selects too many Workers | Tighten labels, tags, region, or cluster. |

## Production checklist

- [ ] Worker identity uses namespace/app/cluster/region/worker pool/labels.
- [ ] Capabilities are derived from installed and tested runtime support.
- [ ] Worker Tunnel uses the expected TLS/mTLS policy.
- [ ] Lifecycle history survives routine restarts enough for operators to diagnose.
- [ ] Workers are deployed as outbound clients, not inbound scheduler targets.
