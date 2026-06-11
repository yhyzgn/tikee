# Settings and governance guide

## Overview

Settings are distributed across governance pages instead of one monolithic screen. Operators manage users, roles, tenant scopes, API-Key/service-account access, calendars, GitOps/IaC, notification governance, alert review, and audit through route-specific pages.

Implementation anchors: `web/src/routes.tsx` defines the menu and permission metadata. Current governance paths include `/users`, `/roles`, `/scopes`, `/api-keys`, `/calendars`, `/gitops`, `/notifications`, `/alerts`, and `/audit`. Menu visibility and actions are controlled by `RBAC` resource/action rules.

## Prerequisites

- You are logged in.
- Your role has read/manage/write permission for the page you need.
- Namespace/app/service-account scope is known before changing API-Key access.
- Newly created API-Key values are captured once into a secure system; they are not shown again.

```bash
curl -fsS http://127.0.0.1:9090/api/v1/jobs \
  -H "x-tikeo-api-key: $TIKEO_MANAGEMENT_API_KEY" | jq '.code'
```

## Open the page

1. Log in to the console.
2. Open the **Governance** menu group.
3. Select Users, Roles, Scopes, Calendars, API-Key, GitOps/IaC, Notifications, Alerts, or Audit.
4. If a menu item or button is missing, inspect the route permission and role catalog before assuming the feature is absent.

## Common tasks

### Manage users and roles

Use Users for account assignment and Roles for permission matrices, menu permissions, and UI action permissions. Test changes with viewer, operator, and admin accounts.

### Manage tenant scopes

Use Scopes to manage namespace, app, worker pool, and related references. Jobs, service accounts, Worker pools, secret references, and canary targets depend on scope.

### Manage API-Key access

Create service accounts and app-scoped API keys in `/api-keys`. SDK Management API uses `x-tikeo-api-key`, not browser bearer tokens. Rotate or revoke keys after exposure, service retirement, or ownership transfer.

## Verify

- A viewer sees only authorized menus/actions.
- Operators can perform daily tasks without high-risk manage permissions.
- Admins can manage roles, scopes, and API-Key entries.
- Full API-Key values are displayed only at creation time.
- Revoked or rotated keys stop working for Management API calls.

## Troubleshooting

| Symptom | Action |
| --- | --- |
| Menu missing | Check `web/src/routes.tsx` permission and user role. |
| Button hidden | Check UI action permission and backend catalog. |
| API-Key unauthorized | Check key state, service-account scope, and `x-tikeo-api-key` header. |
| Cross-scope operation fails | Confirm access to both source and destination scopes. |
| User sees too much | Revert role change and audit permission catalog. |

## Production checklist

- [ ] Least privilege is enforced by roles and menu/action permissions.
- [ ] Service accounts are app-scoped and owned by teams.
- [ ] API keys are rotated and revoked on schedule.
- [ ] Scope changes are reviewed for Job, Worker, and secret impact.
- [ ] RBAC changes are validated with non-admin accounts and audit evidence.
