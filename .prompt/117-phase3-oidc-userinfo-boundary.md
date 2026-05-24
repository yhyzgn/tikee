# 117 — Phase 3 OIDC UserInfo boundary

## Goal
Advance the OIDC callback from token exchange to provider discovery/UserInfo retrieval while preserving fail-closed session issuance.

## Scope
- Fetch the OpenID Provider Configuration document from the configured issuer.
- Require and validate a `userinfo_endpoint` value.
- Fetch the UserInfo payload and reject empty key sets.
- Continue failing closed before accepting the `access_token` because signature/claims validation is not yet implemented.
- Extend the local mock IdP test to prove token, discovery, and UserInfo endpoints are all hit exactly once.

## Out of scope
- Signature, issuer, audience, nonce, and expiry validation.
- OIDC user/role/tenant mapping.
- Session issuance from IdP identity.
