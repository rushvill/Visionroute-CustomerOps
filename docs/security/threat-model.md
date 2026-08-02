# Threat model (skeleton)

## Assets

- Customer PII and company records
- Login credentials and sessions
- Smart SIM inventory and assignments
- Billing drafts and amounts
- Support tickets (may include operational details)
- Admin privileges

## Actors

Anonymous prospect, customer user, admin, attacker (external), malicious insider, future Tracksolid/provider integrations.

## Entry points

- Public signup form / API
- Login / logout / password reset (Phase 2)
- Customer portal device & ticket APIs
- Admin CRM APIs
- Future webhooks/jobs

## Trust boundaries

Browser ↔ SvelteKit ↔ Axum API ↔ PostgreSQL; tenant/account isolation; admin vs customer.

## Priority abuse cases

1. Horizontal access: customer reads another account’s devices/tickets/SIMs.
2. Vertical privilege: customer hits admin signup approval or SIM inventory.
3. Signup spam / enumeration via login or signup responses.
4. Session fixation / theft (insecure cookies).
5. SQL/XSS via ticket or signup fields.
6. Idempotent replay creating duplicate accounts/tickets.
7. Admin note or invoice amount leakage to customer DTOs.

## Controls (baseline)

- Opaque HttpOnly Secure sessions; Argon2id passwords
- Deny-by-default service policies; object ownership checks
- Typed DTOs; parameterized SQL; rate limits on auth/signup
- Explicit response DTOs; audit privileged actions
- ASVS L2-oriented headers/CSP as Phase 5 hardens

## Tests (as features land)

IDOR tests per resource; admin-only route tests; signup rate limit; session cookie flags; validation fuzz on public forms.
