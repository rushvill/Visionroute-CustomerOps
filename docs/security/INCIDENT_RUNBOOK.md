# Incident response runbook — Customer Ops

## Severity

| Level | Examples | Response |
|-------|----------|----------|
| SEV1 | Data breach, mass auth bypass, ransomware on DB | Page owner immediately; take system offline if needed |
| SEV2 | Partial outage, elevated login failures, suspected abuse | Investigate within 1 hour |
| SEV3 | Single-user issues, non-security bugs | Normal queue |

## First 15 minutes

1. **Confirm** — reproduce; check `/health`, `/ready`, hosting status, recent deploys.
2. **Contain** — if credentials leaked: rotate `SESSION_SECRET` + DB password; force logout by truncating `sessions` if required.
3. **Preserve** — export recent `audit_events` and application logs; do not wipe before capture.
4. **Communicate** — notify stakeholders with facts only (what, when, impact).

## Common playbooks

### Suspected credential stuffing
- Check audit for burst of `login_failure`
- Confirm rate limiter is active
- Temporarily lower `LOGIN_RATE_LIMIT_MAX` via env and restart API
- Reset affected user passwords after verification

### Privilege escalation / IDOR
- Identify actor from `audit_events` (`permission_denied`, `scope_denied`)
- Disable `is_active` on the user row
- Patch authz; add regression test; redeploy

### Database compromise / restore
- Follow [OPERATIONS.md](OPERATIONS.md) restore steps in a **staging** clone first when possible
- Rotate all secrets after restore from an untrusted period

## After-action

Within 5 business days: root cause, timeline, customer impact, permanent fix, test coverage added. Update this runbook if the playbook was wrong.
