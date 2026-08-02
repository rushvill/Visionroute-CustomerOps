# Product requirements — VisionRoute Customer Ops

## Problem statement

VisionRoute already has ready Tracksolid white-label GPS web and mobile apps. The business cannot start because there is no reliable path for customers to **reach us, register, and be tracked** as accounts — including Smart SIM coverage, billing stubs, and support tickets.

## Target users and roles

| Role | Who | Goal |
|------|-----|------|
| Anonymous prospect | Potential fleet customer | Submit a signup / service request |
| Customer portal user | Approved account member | Log in, see devices, raise tickets, view basic plan/coverage |
| Operator (account staff) | Customer-side fleet contact | Same as customer, optionally multi-user later |
| Admin | VisionRoute staff | Approve signups, manage accounts, SIMs, coverage, billing, all tickets |
| System | Jobs / integrations | Status checks, reminders (later) |

## Core workflows (MVP)

1. Prospect submits signup request online.
2. Admin reviews and approves → account + login user created.
3. Admin records devices and assigns Smart SIMs.
4. Admin sets subscription / data coverage window (often ~1 year shouldered by us).
5. Customer logs in → sees devices and basic coverage; creates support tickets.
6. Admin monitors SIM inventory, coverage expiry, tickets, and billing drafts.

## Non-functional requirements

- Online web application (not spreadsheet-as-SoR).
- PostgreSQL system of record.
- OWASP ASVS 5.0 Level 2 security baseline (see engineering guide).
- Mobile-usable customer flows; admin usable on desktop and narrower widths.
- Modular monolith: SvelteKit + Rust Axum.

## Success metrics (initial)

- A prospect can submit a signup without staff creating a sheet row by hand.
- Admin can convert signup → active account in one guided flow.
- Customer can log in and see at least one device and open a ticket.
- Admin can list SIMs by status and see coverage ending within 30/60/90 days.

## Out of scope (MVP)

- Replacing Tracksolid GPS maps/telemetry UI.
- Live Tracksolid API sync (store provider refs only).
- Payment gateway / automatic charging.
- Automated carrier data-balance APIs (manual status first).
- Microservices, Redis (unless a measured need appears).
- Full Visionroute2 merge (planned later).

## Assumptions (safest defaults until decided)

- Coverage start defaults to **SIM activation date** when set; else subscription `starts_at`.
- Subscription grain: **per account**, with SIMs linked via `subscription_sims`.
- Customer portal **does not** show prices/invoice amounts in MVP.
- Smart SIM: store **MSISDN and/or ICCID** (at least one required).
- Login: **username or email** + password.
