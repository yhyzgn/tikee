# Scripts user guide

## Overview

The Scripts page manages script drafts, execution policy, approval/publish flow, immutable versions, rollback, and content/policy `diff` previews. Jobs can bind only to approved script versions. The Server stores and dispatches script metadata; Workers execute inside controlled runtimes.

Implementation anchors: `web/src/pages/ScriptsPage.tsx` uses `/api/v1/scripts`, `/api/v1/scripts/{id}`, `/api/v1/scripts/{id}/publish`, `/api/v1/scripts/{id}/rollback`, `/api/v1/scripts/{id}/versions`, and `/api/v1/scripts/{id}/diff`.

## Prerequisites

- `scripts:read` to view; `scripts:manage` for create, edit, publish, rollback, or delete.
- A Worker advertises the required script language and sandbox backend.
- Release notes or approval evidence are ready for publish/rollback.
- Policies contain only allowed env var names and secret references, never raw secrets.

```bash
curl -fsS http://127.0.0.1:9090/api/v1/scripts \
  -H "authorization: Bearer $TIKEO_TOKEN" | jq '.data.items[] | {id,name,language,status,version}'
```

## Open the page

1. Select **Scripts** or open `/scripts`.
2. Create a script or open `/scripts/{id}/edit`.
3. Filter by language, status, or keyword.
4. Use diff before publish or rollback.

## Common tasks

### Create a script

Enter name, language, content, timeout, memory, output limit, env allowlist, filesystem policy, network policy, secret refs, and sandbox backend. Keep network and filesystem denied unless explicitly approved.

### Publish a version

Preview `diff`, record approval, then publish. Publishing creates an immutable approved version that Jobs can reference.

### Roll back

Select a known approved version, execute rollback, and treat the change as production-impacting because future job instances may use the restored version.

## Verify

- Draft creation saves content and policy fields.
- `diff` shows both content and policy changes.
- Publish creates an approved immutable version.
- Rollback points the script to the selected approved version.
- A matching Worker advertises the language/backend before jobs use the script.

## Troubleshooting

| Symptom | Action |
| --- | --- |
| Save fails | Check required fields, limits, language, and permission. |
| Publish fails | Check approval fields and policy validation. |
| Script job pending | Verify Worker `scriptRunners`. |
| Script execution fails | Inspect instance logs; do not look for execution on the Server. |
| Rollback appears ignored | Confirm new instance creation time and released version pointer. |

## Production checklist

- [ ] Network/filesystem are denied by default.
- [ ] Timeout, memory, and output limits are set.
- [ ] Env vars are allowlisted by name only.
- [ ] Worker runner matches language and sandbox backend.
- [ ] Publish and rollback have approval records and version IDs.
