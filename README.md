# VisionRoute Customer Ops

Customer acquisition, registration, billing, and Smart SIM lifecycle — the missing layer that unblocks business launch.

**Status:** Early commercial MVP (Customer Ops) — deploy guide: [`docs/DEPLOY.md`](docs/DEPLOY.md)  
**Relation to control layer (Visionroute2):** Build here first so the business can start; **integrate into the control layer later**. Do not block launch on full Visionroute2 commercial work.

**Engineering contract:** All design and implementation must follow [`docs/security/CURSOR_SECURE_RUST_WEB_GUIDE.md`](docs/security/CURSOR_SECURE_RUST_WEB_GUIDE.md) (also enforced via `.cursor/rules/secure-rust-web-guide.mdc`).

---

## Business summary (organized)

### Context
VisionRoute already has **ready GPS web and mobile apps** via a **Tracksolid white-label** (purchased and rebranded). Technically, fleet tracking can run today.

### Why the business is not starting yet
The GPS product is ready. What is missing is the **go-to-market and customer operations path**:

1. **How customers reach us** — no clear public signup / inquiry channel  
2. **How we capture and track them** — no structured customer record once they show interest or buy  
3. **How we run the account after signup** — SIMs, data coverage, billing, subscription start  

Until those exist, ready Tracksolid apps cannot convert into a running customer base.

### Core problem (restated)
**Not “build another GPS app.”**  
**Build the customer funnel + account ledger** that sits in front of (and later inside) the control layer:

- Can a prospect find us and sign up / request service?  
- Can admin see every lead and customer in one place?  
- Can we attach devices, Smart SIMs, coverage dates, and billing to that customer?  
- Can we monitor data/subscription risk after we go live?

### Operating model (current intent)
1. **Let customers reach and register** (signup / request form → admin review).  
2. **Onboard customers** with basic details needed for GPS fleet devices.  
3. **Issue / assign Smart SIM cards** purchased for those customers.  
4. **Shoulder mobile data for ~1 year** as part of the service package (policy may change later).  
5. **Monitor SIM lifecycle**: activation, subscription start, data exhaustion risk, renewal/continue-or-stop decisions.  
6. **Create customer billing records** so admin can see commercial status alongside technical fleet setup.  
7. **Later:** fold this into the VisionRoute control layer (Visionroute2), once acquisition and records are proven.

### What success looks like
Admin can answer, in one place:

| Question | Needed record |
|----------|----------------|
| Who is this account? | Customer profile |
| What devices do they have? | Device / IMEI / plate linkage |
| Which Smart SIM is on which device? | SIM inventory + assignment |
| When did we start shouldering data? | Subscription / data coverage window |
| Are they out of data or approaching expiry? | SIM status + alerts |
| What do we bill them for? | Billing / plan / invoice stub |

---

## Recommended plan (phased)

### Phase 0 — Spec & sheet bootstrap (this week)
- Finalize fields for Customer, Device, SIM, Subscription, Billing.
- Decide MVP storage (see Architecture below).
- Define admin workflows: register customer → assign device → assign SIM → set data coverage dates.

### Phase 1 — MVP Customer Ops (online, admin-first)
- Admin registers / imports customers.
- Admin records Smart SIMs (ICCID / MSISDN / purchase date / plan).
- Admin assigns SIM → customer → device.
- Admin sets **data coverage start** and **coverage end** (e.g. +1 year).
- Dashboard: SIMs expiring in 30/60/90 days; coverage ended; unassigned SIMs.

### Phase 2 — Customer-facing basics
- Customer can view **basic** account info (plan status, renew date, assigned SIMs) — not full admin pricing/inventory.
- Optional self-registration / signup request (admin approves).

### Phase 3 — Automation & policy
- Alerts when coverage nears expiry.
- Flag “continue shouldering data?” decisions.
- Deeper billing (invoices, payments) when ready.

**Visionroute2 stay on hold** for this track; integrate later only if Customer Ops needs live Tracksolid device sync.

---

## Online vs offline

| Mode | Fit | Verdict |
|------|-----|---------|
| **Offline-only** (Excel on one laptop) | Fine for personal notes; poor for multi-user, registration, alerts | Not recommended as the system of record |
| **Online spreadsheet** (Google Sheets) | Fast MVP, shared with staff, easy edits | **Good Phase 0 / temporary ops tool** |
| **Online app** (web) | Proper auth, forms, validation, dashboards, future customer portal | **Recommended Phase 1+ system of record** |

**Recommendation:** Start **online**. Use Google Sheets only as a **bootstrap or import source**, not the long-term backend—unless you deliberately want a no-code ops sheet for the first 2–4 weeks.

---

## Storage options

### Option A — Google Sheets (bootstrap)
**Pros:** Zero deploy, familiar, collaborative, free.  
**Cons:** Weak access control, easy to break formulas, hard to enforce “admin vs customer” visibility, poor APIs for registration UX, no strong audit trail.

**Use when:** validating fields and processes with the team before building an app.

### Option B — SQLite / local file DB (early app)
**Pros:** Simple, portable.  
**Cons:** Not ideal for multi-device cloud hosting without care.

### Option C — Postgres (Render / similar) — recommended for the app
**Pros:** Durable, multi-user, production-ready, fits admin + later customer login.  
**Cons:** Needs a small hosted backend.

### Option D — Airtable / Notion
Similar to Sheets: good ops UI, weaker as a secured product backend.

**Recommendation:**  
1. Optional **Google Sheet template** for field discovery.  
2. Build the product on **Postgres + small web app** as system of record.

---

## Proposed architecture

### Target (Phase 1+)

```
Admin / (later) Customer
        │
        ▼
   Web App (UI)
        │
        ▼
   API (auth + business rules)
        │
        ▼
   Postgres
   ├── customers
   ├── devices
   ├── sims          (Smart SIM inventory)
   ├── subscriptions (coverage window, policy flags)
   └── billing       (charges / status stubs)
```

**Out of scope for MVP:** replacing Tracksolid; live GPS maps; full Visionroute2 commercial layer.

**In scope for MVP:** customer records, SIM inventory (Smart), assignment, 1-year data coverage tracking, billing stubs, admin monitoring.

### Suggested core entities

1. **Customer** — name, company, contacts, address, status  
2. **Device** — IMEI / device id, plate/vehicle label, Tracksolid account ref (optional text)  
3. **SimCard** — Smart ICCID/MSISDN, purchase date, status (`inventory` / `assigned` / `suspended` / `exhausted`)  
4. **Subscription / Coverage** — customer (or device), start date, end date (e.g. +1 year), `dataShoulderedByUs` flag, renew decision  
5. **BillingRecord** — amount, period, status (`draft` / `due` / `paid`), linked customer  

### Visibility rule (same as prior intent)
- **Admin:** all customer, SIM, coverage, and billing detail  
- **Customer:** basic plan/coverage/SIM identity only (no internal inventory or full cost ledger until you choose otherwise)

---

## Google Sheets — if used as Phase 0

Suggested tabs:

1. `Customers`  
2. `Devices`  
3. `SIMs`  
4. `Coverage` (start/end, shouldered Y/N)  
5. `Billing`  
6. `Dashboard` (formulas: expiring in 30 days, unassigned SIMs)

Then import into the app when Phase 1 starts.

---

## Decision checklist

- [ ] Confirm Smart SIM fields (ICCID vs MSISDN vs both)  
- [ ] Confirm “1 year data” starts on: activation date vs install date vs payment date  
- [ ] Confirm who may register: admin-only vs public signup request  
- [ ] Confirm Phase 0: Sheet-only vs jump straight to app + Postgres  
- [ ] Confirm brand name for this workspace product (working title: **VisionRoute Customer Ops**)

---

## This workspace

Path: `/Users/rushymartinvillar/Documents/Visionroute-CustomerOps`  
Keep **Visionroute2** untouched while this track proceeds.

---

## Engineering status (secure guide Phase 0 → 5)

| Guide phase | Status |
|-------------|--------|
| Phase 0 — product/security/architecture docs | Done under `docs/` |
| Phase 1 — foundation (Compose, Axum health, SvelteKit shell) | Done |
| Phase 2 — opaque sessions / Argon2id auth (core) | Done |
| Phase 3 — authorization policies | Done |
| Phase 4 — core domain CRM | Done (MVP workflows) |
| Phase 5 — hardening | Done (headers/CSP, limits, CI, ops docs) |
| Phase 6 — performance | Not started |

**Ops docs:** [OPERATIONS.md](docs/security/OPERATIONS.md) · [INCIDENT_RUNBOOK.md](docs/security/INCIDENT_RUNBOOK.md)

### How to run locally

```bash
# 1. Env
cp .env.example .env
cp apps/web/.env.example apps/web/.env

# 2. Postgres only (API usually run on host for faster rustc)
docker compose up -d db

# 3. API (from apps/api)
cd apps/api
cargo run

# 4. Web (from apps/web)
cd apps/web
npm install
npm run dev -- --host 127.0.0.1 --port 5173
```

Open **http://127.0.0.1:5173** (must match `FRONTEND_ORIGIN`).

**Dev seed users** (password `VisionRouteDemo26!` for both):
- Admin: `admin` → [Admin](http://127.0.0.1:5173/admin) (signups, accounts, SIMs, coverage, tickets)
- Customer: `customer` → [Portal](http://127.0.0.1:5173/portal) (Demo Fleet Co devices/SIMs/tickets)

**Public:** [Signup](http://127.0.0.1:5173/signup)

Smoke checks:

- API live: `http://127.0.0.1:8080/health`
- API ready (DB): `http://127.0.0.1:8080/ready`
- Login: `http://127.0.0.1:5173/login`

Optional full stack in Docker: `docker compose up --build` (API on `:8080`, DB on `:5432`).
