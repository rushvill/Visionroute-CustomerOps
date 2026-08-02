# Operations — backups, restore, incidents

## Backups (Postgres)

### What to back up
- PostgreSQL volume / managed DB snapshot for `customerops`
- Do **not** back up `.env` into the same bucket as public artifacts
- Application code is in git; secrets stay in the secret store

### Local / Compose snapshot

```bash
# From repo root, while db container is running:
docker compose exec -T db pg_dump -U customerops -d customerops -Fc > "backup-customerops-$(date +%Y%m%d).dump"
```

Store dumps encrypted at rest. Restrict who can download them.

### Managed hosting
Use the provider’s automated daily backups + point-in-time recovery when available (Render, RDS, etc.). Verify retention ≥ 7 days for MVP.

## Restore (test before you need it)

```bash
# WARNING: destroys current DB contents for that database name
docker compose exec -T db pg_restore -U customerops -d customerops --clean --if-exists < backup-customerops-YYYYMMDD.dump
```

After restore:
1. `curl http://127.0.0.1:8080/ready` → database ok
2. Log in as admin; spot-check accounts, SIMs, tickets
3. Record restore time and who performed it

## Monitoring (MVP)

| Signal | Where |
|--------|--------|
| Process up | `GET /health` |
| DB reachable | `GET /ready` |
| Auth failures / authz denials | JSON logs + `audit_events` |
| Admin audit UI | `/admin` → Audit section |

Alert on: API down, `/ready` failing, spike in `login_failure` / `permission_denied`.

## File uploads / SSRF

- Uploads are **disabled** (`files` module rejects until attachments ship).
- Outbound HTTP fetches are **deny-by-default** (`ssrf` allowlist empty). Tracksolid sync must add an explicit host allowlist.

## Secrets

- Rotate `SESSION_SECRET` and DB passwords on compromise.
- Dev seed passwords (`VisionRouteDemo26!`) must never be used in production.
