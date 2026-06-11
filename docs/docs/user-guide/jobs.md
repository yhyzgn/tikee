# Jobs user guide

## Overview

The Jobs page manages job definitions: schedule type, namespace/app ownership, SDK processor/script/plugin binding, retry policy, calendars, canary target, version history, rollback, single execution, broadcast execution, and scheduling advice. Treat a Job as the contract that tells Tikeo what should run and which Workers are eligible to run it.

Implementation anchors: `web/src/pages/JobsPage.tsx` uses `/api/v1/jobs`, `/api/v1/jobs/{job}`, `/api/v1/jobs/{job}:trigger`, `/api/v1/jobs/{job}/versions`, `/api/v1/jobs/{job}/rollback`, `/api/v1/jobs/{job}/scheduling-advice`, `/api/v1/jobs/topology`, and `/api/v1/jobs/{job}/impact`.

## Prerequisites

- `jobs:read` to view Jobs; `jobs:write` to create, edit, delete, or rollback.
- `instances:execute` to trigger work.
- Namespace, app, Worker pool, processor, script, plugin processor, and calendar are already prepared.
- Workers advertise matching structured capabilities; job names are not routing rules.

```bash
curl -fsS http://127.0.0.1:9090/api/v1/jobs \
  -H "authorization: Bearer $TIKEO_TOKEN" | jq '.data.items[] | {id,name,namespace,app,enabled}'
```

## Open the page

1. Log in to the console.
2. Select **Jobs** or open `/jobs`.
3. Filter by keyword, namespace, app, or schedule type.
4. Use topology or impact views before changing high-fan-out jobs.

## Common tasks

### Create a job

Choose namespace/app first. Select API, cron, fixed rate, fixed delay, once, or daily time interval. Bind the executor: SDK processor, approved script, or enabled plugin processor. Configure retry policy, misfire policy, calendar, worker pool, and optional canary target. Save, then open scheduling advice.

### Edit and version a job

Changing scope, processor, script, calendar, or retry policy creates operational impact. Confirm authorization for both source and destination scope when moving a job. After saving, inspect versions and note the version number for rollback.

### Trigger single execution

API triggers use `triggerType=api` and `executionMode=single`. After triggering, open Instances and inspect `/api/v1/instances/{instance}` and `/api/v1/instances/{instance}/logs`.

### Trigger broadcast execution

Broadcast is opt-in and requires `broadcastSelector`. Use tags, region, cluster, or labels only if Workers actually advertise them. Inspect every execution node; one failed node can produce `partial_failed`.

## Verify

- You can create an API job and trigger it through `/api/v1/jobs/{job}:trigger`.
- Versions show changes after edits and rollback restores a selected version.
- Scheduling advice shows eligible Workers or explains why none match.
- Instance logs prove processor execution after a trigger.
- Broadcast selectors target only expected Workers.

## Troubleshooting

| Symptom | Action |
| --- | --- |
| Job remains pending | Compare processor, namespace/app, and selector with Workers. |
| Trigger unauthorized | Check `instances:execute` and the credential type. |
| Processor options are missing | Start a Worker that advertises the processor or plugin. |
| Rollback changes too much | Re-open version history and compare created time/author before retrying. |
| Broadcast hits wrong nodes | Tighten selector labels/region/cluster and verify Worker metadata. |

## Production checklist

- [ ] Every production Job has an owner namespace/app and a clear processor binding.
- [ ] Retry and misfire policies are deliberate, not defaults copied blindly.
- [ ] Broadcast selectors are tested against live Worker metadata.
- [ ] Version history and rollback are part of release procedures.
- [ ] Trigger evidence includes instance ID, logs, and audit entries.
