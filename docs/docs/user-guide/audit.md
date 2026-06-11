# Audit user guide

## Overview

The Audit page is the governance evidence surface for platform writes, authentication events, script governance actions, dispatch-related events, failure reasons, and release reviews. Use it to answer who changed what, when, from where, and with which result.

Implementation anchors: `web/src/pages/AuditLogsPage.tsx` reads `/api/v1/audit-logs` and exports filtered JSON through `/api/v1/audit-logs:export`.

## Prerequisites

- `audit:read` permission.
- You know at least one investigation dimension: time window, actor, action, resource type, resource ID, failure reason, or trace ID.
- Exported files are treated as sensitive operational records.
- Browser download is allowed if exporting evidence.

```bash
curl -fsS 'http://127.0.0.1:9090/api/v1/audit-logs?pageSize=20' \
  -H "authorization: Bearer $TIKEO_TOKEN" | jq '.data.items[] | {createdAt,actor,action,result}'
```

## Open the page

1. Select **Audit** or open `/audit`.
2. Filter by actor, action, resource type, resource ID, failure reason, and page size.
3. Open rows for before/after snapshots and trace information.
4. Export only after filters match the investigation.

## Common tasks

### Investigate a change

Filter by resource type/id and time window. Record actor, action, trace ID, before/after snapshot, and request identifiers.

### Investigate a failure

Filter failed rows or failure reason. Use trace ID and time to correlate with API/server logs.

### Export evidence

Export current filters as JSON. Keep the exported file in an authorized evidence store; screenshots are supplementary only.

## Verify

- Successful writes appear with actor, action, resource, and time.
- Failed writes expose a failure reason when available.
- Trace IDs can be correlated with server logs.
- Export preserves filters and result count.
- Before/after fields are present for covered resource types.

## Troubleshooting

| Symptom | Action |
| --- | --- |
| No rows | Widen time/filter and check permission. |
| Missing failure reason | Inspect the failing route and server logs. |
| Trace ID not found | Align log time range and request ID. |
| Export empty | Confirm filtered table has rows before export. |
| Snapshot missing | Confirm that resource type supports before/after audit. |

## Production checklist

- [ ] Production changes leave audit entries.
- [ ] High-risk failures include failure reasons.
- [ ] Exports are stored securely.
- [ ] Incident notes include trace ID, resource ID, and time window.
- [ ] Screenshots never replace JSON export for formal review.
