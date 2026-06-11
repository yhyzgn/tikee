# Workflows user guide

## Overview

The Workflows page manages DAG definitions, visual editing, JSON/YAML definition views, validation, dry-run, execution, replay, shard inspection, and failed-node recovery. Use validation and dry-run before executing production workflows.

Implementation anchors: `web/src/pages/WorkflowsPage.tsx` uses `/api/v1/workflows`, `/api/v1/workflows/{id}`, `/api/v1/workflows/{id}/validate`, `/api/v1/workflows/dry-run`, `/api/v1/workflows/{id}/run`, workflow-instance routes, and event streams. Workflows are `DAG` graphs whose job nodes ultimately create ordinary job instances.

## Prerequisites

- `workflows:read` to view; manage/execute permissions for create, edit, run, or recover.
- Referenced Jobs exist and have eligible Workers.
- Test input and rollback/recovery expectations are known.
- Operators understand which nodes can be retried, skipped, or failed.

```bash
curl -fsS http://127.0.0.1:9090/api/v1/workflows \
  -H "authorization: Bearer $TIKEO_TOKEN" | jq '.data[] | {id,name}'
```

## Open the page

1. Select **Workflows** or open `/workflows`.
2. Use `/workflows/new` for a new DAG or `/workflows/{id}/edit` to edit.
3. Switch between visual canvas and definition view to verify graph shape.
4. Validate before saving or running.

## Common tasks

### Build a small DAG

Start with a job-backed node and explicit edges. Add condition, parallel, join, delay, approval, notification, compensation, map, map_reduce, or sub_workflow nodes only when the operational behavior is clear.

### Validate and dry-run

Validation checks structure. Dry-run checks expected start nodes, node count, and edge count before materializing runtime work. Fix validation errors before execution.

### Run and recover

Running creates a workflow instance. Inspect node status, shards, and underlying job instances. Recovery is an operational action; confirm failed node, input context, and downstream effect before retry/skip/fail.

## Verify

- A small DAG can be saved and validated.
- Dry-run returns expected graph metadata.
- Running creates a workflow instance.
- Job nodes can be traced to Instances and logs.
- Replay and recovery actions are visible only to authorized operators.

## Troubleshooting

| Symptom | Action |
| --- | --- |
| Validation fails | Fix missing nodes, invalid edges, duplicate keys, or bad node config. |
| Dry-run differs from canvas | Compare JSON/YAML definition with visual layout. |
| Job node pending | Check referenced Job and Worker eligibility. |
| Recovery unsafe | Stop and get business approval before skip/fail. |
| Notification node surprises operators | Use Notification Center channel/template/policy references, not inline secrets. |

## Production checklist

- [ ] Workflow definitions are reviewed as code or change records.
- [ ] Every job node has an eligible Worker path.
- [ ] Recovery procedures are documented per node type.
- [ ] Notification nodes do not contain raw secrets.
- [ ] Replay evidence is kept for incidents.
