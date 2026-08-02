# Permissions matrix

Deny by default. Frontend hides controls for UX only — backend enforces.

| Resource | Action | Anonymous | Customer | Admin | System |
|----------|--------|:---------:|:--------:|:-----:|:------:|
| Signup request | Create | Yes | No | Yes | No |
| Signup request | List/review/approve | No | No | Yes | No |
| Own account profile | Read | No | Yes | Yes | Limited |
| Other accounts | Read/update | No | No | Yes | Limited |
| Own devices | Read | No | Yes | Yes | Limited |
| Devices | Create/update/retire | No | No | Yes | No |
| SIM inventory | List unassigned | No | No | Yes | No |
| Own assigned SIMs | Read basic | No | Yes | Yes | Limited |
| SIMs | Assign/update status | No | No | Yes | Later |
| Own subscription basics | Read (no price) | No | Yes | Yes | Limited |
| Own invoices / payments | Read | No | Yes | Yes | No |
| Plans/prices/billing | Manage / read amounts | No | No | Yes | No |
| SIM data costs (VR spend) | Record / list | No | No | Yes | No |
| Own tickets | Create/read/comment | No | Yes | Yes | No |
| All tickets | Read/assign/resolve | No | No | Yes | No |
| Internal ticket notes | Read/write | No | No | Yes | No |
| Users/roles | Manage | No | No | Yes | No |
| Audit logs | Read | No | No | Yes | Append |
| Privacy / DSAR requests | Create | Yes | Yes | Yes | No |
| Privacy / DSAR requests | List / resolve | No | No | Yes | No |
| Health endpoints | Read | Yes | Yes | Yes | Yes |

Object-level rule: customers may only access rows where `account_id` matches their session account.
