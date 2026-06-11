# Dashboard user guide

## Overview

Dashboard is the first operating surface after login. It summarizes job inventory, enabled jobs, pending instances, online Workers, and broadcast activity so an operator can decide where to investigate next. It is read-only: do not look for create, update, retry, or cancel actions here.

Implementation anchors: `web/src/pages/Dashboard.tsx` loads `/api/v1/jobs`, `/api/v1/workers`, and `/api/v1/jobs/{job}/instances`; it also listens to `/api/v1/instances/stream` and `/api/v1/workers/stream`. Raw health and capacity checks can be compared with `/api/v1/metrics/summary` and `/api/v1/cluster`.

## Prerequisites

- You are logged in with read access to Jobs, Instances, and Workers.
- The Management API reachable by the Web console is the same environment you are investigating.
- At least one Job or Worker exists; an empty environment only proves that the page renders.
- `curl` and `jq` are available if you want to compare the cards with API output.

```bash
curl -fsS http://127.0.0.1:9090/api/v1/workers \
  -H "authorization: Bearer $TIKEO_TOKEN" | jq '.data.online'
```

## Open the page

1. Log in to the Web console.
2. Select **Dashboard** or open `/dashboard`.
3. Wait for the cards to load and for stream updates to arrive.
4. If numbers do not change, check the browser Network tab for the two SSE streams and the list APIs.

## Common tasks

### Check Worker availability

Start with online Workers. If the number is zero, open Workers and inspect outbound Worker Tunnel sessions, recent lifecycle events, and registered namespace/app scope.

### Check queue pressure

Pending instances should be temporary. If pending count keeps rising, open Jobs, check whether the job is enabled, then compare processor names and selectors with Workers. Open Instances to determine whether pending work later succeeds, fails, or becomes partial broadcast failure.

### Check broadcast activity

Broadcast instances are fan-out work. High broadcast count is not automatically bad, but `partial_failed` instances require node-by-node investigation in Instances.

## Verify

- `/dashboard` renders total jobs, enabled jobs, pending instances, online Workers, and broadcast instances.
- Worker connect/disconnect changes appear through `/api/v1/workers/stream` or after refresh.
- New instance state appears through `/api/v1/instances/stream` or after refresh.
- The page remains read-only; all mutating work happens in Jobs, Instances, Workers, or Settings.

## Troubleshooting

| Symptom | Action |
| --- | --- |
| Online Workers is zero | Open Workers and verify outbound Worker Tunnel connection and scope. |
| Pending instances increase | Open Jobs scheduling advice, then compare Worker structured capabilities. |
| Cards are stale | Check SSE streams and fallback API calls in browser Network. |
| Dashboard disagrees with a table page | Trust the table/detail page and API response; refresh Dashboard. |
| Page is hidden | Check RBAC read permissions for the relevant resources. |

## Production checklist

- [ ] Dashboard is used only for triage, not as the system of record.
- [ ] Worker count matches the Workers page and API output.
- [ ] Pending count has an understood cause during incidents.
- [ ] SSE failure has an API fallback and is visible in browser/server logs.
- [ ] Operators know which page owns each follow-up action.
