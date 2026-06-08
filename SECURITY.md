# Security Policy

Tikeo is an orchestration platform that touches worker execution, scripts, credentials, audit logs, and deployment configuration. Please report security issues responsibly.

## Supported versions

Tikeo is still evolving quickly. Security fixes are prioritized on the `main` branch and on the latest published release line when releases are available.

## Reporting a vulnerability

Preferred path:

1. Use GitHub private vulnerability reporting / Security Advisories if it is enabled for the repository.
2. If private reporting is not available, open a minimal public issue that says you have a security concern, but do **not** include exploit details, secrets, credentials, or private infrastructure data.

Please include enough context for maintainers to reproduce safely:

- Affected component: server, worker tunnel, Web UI, SDK, deployment, scripts, RBAC, API keys, or storage.
- Impact summary.
- Reproduction outline without exposing real secrets.
- Suggested mitigation if known.

## Security scope

High-priority areas include:

- Authentication, session, RBAC, and API-key bypasses.
- Worker tunnel authorization and dispatch isolation.
- Script sandbox escape or unsafe runtime fallback.
- Secret exposure in logs, audit events, UI, examples, or generated artifacts.
- SSRF, command injection, path traversal, SQL injection, and unsafe deserialization.
- Deployment defaults that expose privileged services unexpectedly.

## Safe handling guidelines

- Do not paste real tokens, database URLs, private keys, production logs, or customer data into issues.
- Redact screenshots and terminal output before sharing.
- Prefer minimal repro projects over production traces.
