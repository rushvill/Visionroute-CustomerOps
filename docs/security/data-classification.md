# Data classification

| Field / class | Classification | Handling |
|---------------|----------------|----------|
| Company name, device label, ticket subject | Internal | Authz required; OK in admin UI |
| Contact email/phone | Confidential | Authz; mask in logs |
| Billing amounts, invoices | Confidential | Admin only in MVP |
| Passwords, session tokens, reset tokens | Restricted | Hash only; never log/return |
| Credential encryption keys, DB URLs | Restricted | Env/secret manager only |
| ICCID / MSISDN | Confidential | Authz; customers see own assigned only |
| Admin notes, internal ticket comments | Confidential | Admin only |
| Audit events | Internal / Confidential | Append-oriented; restricted read |
| Signup IP/UA | Confidential | Store hashed only |

## Retention (initial)

- Signup requests: keep after conversion for audit (≥ 1 year) or anonymize; store privacy consent timestamp + notice version.
- Accounts/devices/SIMs: retain while account active + business retention policy.
- Invoices/payments: retain for tax/accounting needs.
- Privacy / DSAR requests: retain until handled + audit trail.
- Tickets: retain with account.
- Sessions: expire (idle + absolute); hashed token storage.
- Backups: encrypted; same classification as live DB.

See also: [docs/privacy/PRIVACY_NOTICE.md](../privacy/PRIVACY_NOTICE.md).
