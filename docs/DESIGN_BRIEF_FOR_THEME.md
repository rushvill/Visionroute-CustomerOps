# VisionRoute Customer Ops — project briefing for design / theme consultation

**Purpose of this doc:** Hand to another AI (or designer) so they can propose a visual theme and look-and-feel **before** we implement a presentability pass. Do not invent new product features here — recommend branding, layout, and UI language only.

**Repo / workspace:** `Visionroute-CustomerOps`  
**Related (on hold):** `Visionroute2` control layer — do not mix themes unless asked.  
**Date context:** August 2026

---

## 1. What this product is

**VisionRoute Customer Ops** is the go-to-market and customer-operations layer for a GPS fleet business.

VisionRoute already has **ready Tracksolid white-label GPS web/mobile apps**. The business was blocked because there was no clear way for customers to:

1. Reach / register with VisionRoute  
2. Be tracked as accounts (devices, Smart SIMs, data coverage, billing stubs, support tickets)

Customer Ops is that system of record. Long-term it may merge into the Visionroute2 control layer; short-term it is a **separate product**.

**Not in scope for this app (MVP):** live GPS maps, Tracksolid telemetry UI, payment gateway, carrier data-balance APIs.

---

## 2. Who uses it

| Audience | Role | What they do |
|----------|------|----------------|
| Prospect | Anonymous | Submit a signup / service request |
| Customer | Portal user | Log in; see account, devices, SIMs, coverage basics; open tickets |
| Admin / staff | VisionRoute ops | Approve signups, manage accounts/SIMs/coverage, handle tickets, view audit |

**Demo logins (local):** password for both is `VisionRouteDemo26!`  
- Admin: `admin`  
- Customer: `customer` (Demo Fleet Co fixtures)

---

## 3. What we already built (engineering status)

Followed a secure stack guide in phases:

| Phase | Status | Summary |
|-------|--------|---------|
| 0 Spec | Done | Product, threat model, permissions matrix, schema docs |
| 1 Foundation | Done | SvelteKit web + Rust Axum API + Postgres + Compose + health |
| 2 Auth | Done | Argon2id, opaque HttpOnly cookie sessions, login/logout, `/me` |
| 3 Authz | Done | Role permissions, account scope, admin gates, denied-action audit |
| 4 Domain CRM | Done | Signup → account, devices, Smart SIMs, subscriptions/coverage, tickets |
| 5 Hardening | Done | Security headers/CSP, stricter body limits, CI, ops/incident docs |
| 6 Performance | Not started | Deferred |
| **Presentability UX** | **Planned, not built** | Waiting on theme / look direction from this consultation |

**Stack:** SvelteKit (TypeScript) + Rust/Axum API + PostgreSQL. Auth is cookie-based (BFF/server actions), not JWT in the browser.

**Local URLs (typical):**  
- Web: `http://127.0.0.1:5173`  
- API: `http://127.0.0.1:8080`

---

## 4. Current screens (what exists today)

These are **functional but scaffold-looking** (plain slate UI, Segoe/system fonts, little brand presence).

### Public
- **Home (`/`)** — product blurb, links, session box, API health (engineering chrome still visible)
- **Signup (`/signup`)** — prospect request form (name, company, email, phone, devices, message)
- **Login (`/login`)** — username/email + password

### Customer
- **Portal (`/portal`)** — account summary, devices list, SIMs, subscription/coverage (no prices), tickets + create-ticket form

### Admin
- **Admin (`/admin`)** — long single page with anchor sections:
  - Signup inbox (approve with username/password, or reject)
  - Accounts
  - SIM inventory (add / assign)
  - Coverage expiring
  - Tickets
  - Users
  - Audit events

Shared header: text brand “VisionRoute Customer Ops” + nav links.

---

## 5. Presentability goals (next UX pass)

We want the tool to feel showable to:

1. **Customers / prospects** — trust, clarity, VisionRoute brand  
2. **Staff who will operate it daily** — clear ops console, not a developer scaffold  

### Draft product decisions already leaning this way
- Customer-facing brand emphasis: **VisionRoute** first; “Customer Ops” more for staff context  
- Polish **both** public/customer surfaces and admin (not customer-only)  
- **No new backend domain features** in the presentability pass  
- Keep mobile usable  

### Visual direction draft (not final — please challenge or refine)
A prior internal draft suggested a **fleet / road** feel: deep navy + asphalt gray + amber accent, light atmospheric background, expressive non-default fonts. Avoid generic “AI UI” clichés (purple gradients, cream+terracotta serif kits, newspaper broadsheet layouts, dark-mode-first, glow spam, pill clusters).

**Please propose:** color system, typography pairing, layout principles for home / portal / admin, and any reference mood — so we can lock a theme before implementation.

---

## 6. Design constraints (hard rules for implementers)

When recommending a look, respect these:

- **Brand first on marketing surfaces:** home hero should still read as VisionRoute if the nav were removed.  
- **One composition** in the first viewport — not a dashboard of widgets.  
- **Hero budget:** brand, one headline, one short supporting line, CTA group, one dominant visual plane. No stats strips, promo chips, or floating badges on the hero.  
- **Cards:** default to no cards; only when they contain a real interaction.  
- **One job per section.**  
- **Real visual anchor** preferred over pure abstract decoration (fleet/road/ops atmosphere OK).  
- Ship **2–3 intentional motions**, not noise.  
- Works on **desktop and mobile**.  
- Do not require replacing the SvelteKit stack.

---

## 7. What we need back from the design AI

Please return a concrete theme proposal:

1. **Name the direction** (1 sentence mood).  
2. **Color tokens** (hex): background, surface, text, muted, brand, accent, success/warn/danger.  
3. **Typography:** two fonts (display + body) with usage rules — not Inter/Roboto/Arial/system.  
4. **Layout recipes** for: Home hero, Signup/Login, Customer portal, Admin console.  
5. **Component language:** buttons, status pills, tables/lists, empty states — how they should feel.  
6. **What not to do** for this brand.  
7. Optional: ASCII or short wireframe descriptions of the home first viewport and portal overview.

We will then implement in SvelteKit against that locked theme.

---

## 8. File map (if useful)

```
apps/web/                 SvelteKit UI (presentability work lands here)
apps/api/                 Rust Axum API (auth + CRM domain — leave stable)
docs/                     Product + security docs
docs/security/OPERATIONS.md
docs/security/INCIDENT_RUNBOOK.md
README.md                 How to run + seed users
```

---

## 9. One-line summary

**Customer Ops is a working secure CRM for VisionRoute signup, fleet accounts, Smart SIMs, coverage, and tickets — functionally ready to demo, visually still a scaffold; we need a locked brand theme before polishing UI for customers and staff.**
