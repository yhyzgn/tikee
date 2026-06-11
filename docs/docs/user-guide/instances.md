# Instances user guide

## Overview

The Instances page is the execution evidence surface for Jobs and Workflows. Use it to inspect status, attempts, Worker nodes, logs, result payloads, broadcast partial failures, and cancellation. When a job “does not work,” collect evidence here before changing the job or Worker.

Implementation anchors: `web/src/pages/InstancesPage.tsx` reads `/api/v1/jobs`, `/api/v1/jobs/{job}/instances`, `/api/v1/instances/{instance}`, `/api/v1/instances/{instance}/attempts`, `/api/v1/instances/{instance}/logs`, and `/api/v1/instances/{instance}/cancel`; it also uses `/api/v1/instances/stream` and `/api/v1/instances/{instance}/logs/stream`.

## Prerequisites

- You have permission to read instances; cancellation requires execute/control permission.
- You know the job, workflow, or instance ID you are investigating.
- Workers send `TaskLog` and `TaskResult` through the outbound Worker Tunnel.
- Browser/network policy allows SSE, or you can refresh via normal APIs.

```bash
curl -fsS http://127.0.0.1:9090/api/v1/instances/INSTANCE_ID \
  -H "authorization: Bearer $TIKEO_TOKEN" | jq '.data | {id,status,result}'
```

## Open the page

1. Select **Instances** or open `/instances`.
2. Filter by job, status, or time window.
3. Open the detail drawer for attempts, logs, and results.
4. Copy instance ID and Worker ID when escalating.

## Common tasks

### Read status

`pending` means dispatch has not completed. `running` means a Worker accepted work or logs are arriving. `succeeded`, `failed`, and `partial_failed` are terminal evidence states. Broadcast instances can have mixed node results.

### Inspect logs and attempts

Open the details drawer, read attempts first, then logs grouped by Worker. For broadcast, compare successful and failed nodes by Worker ID, region, cluster, labels, and runner capability.

### Cancel active work

Cancel only active instances and only when you understand the business effect. After the API accepts cancellation, refresh details; Workers may still send cleanup logs.

## Verify

- A successful instance shows result, attempt, and Worker logs.
- A failed instance shows error or log evidence.
- Broadcast/partial instances show multiple execution nodes.
- Cancel is available only with permission and active status.
- Stream or refresh reflects status changes.

## Troubleshooting

| Symptom | Action |
| --- | --- |
| Instance stays pending | Check Jobs scheduling advice and Worker capabilities. |
| No logs | Verify Worker is online and sends `TaskLog`. |
| Status and logs disagree | Refresh detail API and inspect latest attempt. |
| Cancel seems ineffective | Check whether the instance already reached terminal state or Worker is cleaning up. |
| Broadcast partially fails | Compare successful and failed Worker metadata. |

## Production checklist

- [ ] Incident records include instance ID, job ID, Worker ID, attempt, and time window.
- [ ] Logs are captured from Tikeo instance logs, not only container stdout.
- [ ] Cancellation procedure defines who can cancel and when.
- [ ] Broadcast failures are reviewed per execution node.
- [ ] Audit entries are correlated for manual triggers and cancellations.
