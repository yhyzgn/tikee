# Phase 4 P2 plugin system closed loop

This phase implements the P2 plugin system item for custom processor types and custom alert channels.

Implemented boundaries:
- Storage: `plugins` table with processor type declarations and alert channel type declarations.
- HTTP: `/api/v1/plugins` list/create/update/delete, guarded by tenant read/manage permissions.
- Jobs: `processorType` field added to create/update/summary/storage/version snapshots. Custom processor types resolve to Worker capability `plugin-processor:<type>` while preserving `processorName` as the task processor name.
- Scheduling/dispatch: scheduling advice and Worker dispatch require the plugin processor capability for custom processor jobs.
- Alerts: plugin alert channel types are visible in delivery readiness and can be materialized into webhook-compatible notification channels with simple `{{message}}`, `{{resource_id}}`, `{{resource_type}}`, `{{severity}}` template replacement.
- Web: `/plugins` management page, menu route, API client types, and Jobs create/edit plugin processor selector.
- Demo: Java Spring worker advertises `plugin-processor:sql` and includes `billing.sql-sync`; Rust demo can advertise the same capability with `TIKEE_ENABLE_PLUGIN_SQL=1`.

Validation anchors:
- `cargo test -p tikee-storage plugin_repository_resolves_custom_processor_and_alert_channel_types -- --nocapture`
- `cargo test -p tikee-server plugin_registry_supports_custom_processor_types_and_alert_channels -- --nocapture`
- `cd web && bun test src/pages/__tests__/PluginsPage.test.tsx`
- `cd web && bun run lint && bun run build`

Next phases should not reintroduce hard-coded custom processor enums. Use the plugin registry as the source of truth for UI choices and capability resolution.
