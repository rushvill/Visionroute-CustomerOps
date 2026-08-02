# VisionRoute Customer Ops — Database Schema

**Purpose:** Small CRM-style control layer for customer acquisition, accounts, GPS devices, Smart SIM lifecycle, billing, and support ticketing.  
**Storage target:** PostgreSQL (recommended). SQLite acceptable for local prototype only.  
**Audience:** Admin (full access) · Customer/operator (own account: devices, tickets, basic plan/SIM info).

This schema is designed to **rival a light CRM** and later integrate into the VisionRoute control layer without a rewrite.

---

## 1. Design principles

1. **Account is the center** — every device, SIM, subscription, bill, and ticket hangs off a customer account.
2. **Admin vs customer visibility** — same tables; APIs expose full vs trimmed DTOs.
3. **Soft deletes / status flags** — prefer `status` over hard delete for commercial and SIM records.
4. **UUID primary keys** — portable across services and future control-layer merge.
5. **Audit fields** — `created_at`, `updated_at`, `created_by`, `updated_by` where mutations matter.
6. **Tracksolid stays external** — store provider refs (account name, device id); do not duplicate GPS telemetry here in MVP.

---

## 2. Entity relationship overview

```text
signup_requests ──(approve)──► accounts ──► account_users (login)
                                  │
                                  ├── contacts
                                  ├── devices ──────────────┐
                                  │                         │
                                  ├── sim_cards ◄───────────┤ (optional device_id)
                                  │         │               │
                                  ├── subscriptions ◄───────┘
                                  │         │
                                  ├── billing_invoices / billing_line_items
                                  │
                                  └── tickets ──► ticket_comments
                                                  ticket_attachments (optional)
```

---

## 3. Enumerations (use DB enums or CHECK constraints)

| Name | Values |
|------|--------|
| `account_status` | `pending`, `active`, `suspended`, `churned` |
| `user_role` | `admin`, `operator`, `customer`, `viewer` |
| `signup_status` | `new`, `reviewing`, `approved`, `rejected` |
| `device_status` | `pending_install`, `active`, `inactive`, `retired` |
| `sim_status` | `inventory`, `assigned`, `active`, `suspended`, `exhausted`, `retired` |
| `sim_carrier` | `smart`, `globe`, `tnt`, `other` (MVP: mostly `smart`) |
| `subscription_status` | `trial`, `active`, `past_due`, `paused`, `cancelled`, `expired` |
| `coverage_policy` | `shouldered_by_us`, `customer_paid`, `undecided` |
| `invoice_status` | `draft`, `issued`, `partial`, `paid`, `void`, `overdue` |
| `ticket_status` | `open`, `in_progress`, `waiting_customer`, `resolved`, `closed` |
| `ticket_priority` | `p1`, `p2`, `p3`, `p4` |
| `ticket_category` | `device`, `sim_data`, `billing`, `login`, `install`, `other` |

---

## 4. Tables (detailed)

### 4.1 `accounts` — CRM company / fleet customer

The commercial customer (tenant) in your CRM.

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `account_code` | VARCHAR(32) UNIQUE | Human code e.g. `VR-00042` |
| `company_name` | VARCHAR(200) NOT NULL | |
| `display_name` | VARCHAR(200) | Short label |
| `status` | `account_status` NOT NULL DEFAULT `pending` | |
| `industry` | VARCHAR(100) NULL | Optional |
| `tax_id` | VARCHAR(64) NULL | TIN / business ID |
| `billing_email` | VARCHAR(255) NULL | |
| `operations_email` | VARCHAR(255) NULL | |
| `phone` | VARCHAR(40) NULL | |
| `address_line1` | VARCHAR(255) NULL | |
| `address_line2` | VARCHAR(255) NULL | |
| `city` | VARCHAR(100) NULL | |
| `province` | VARCHAR(100) NULL | |
| `postal_code` | VARCHAR(20) NULL | |
| `country` | VARCHAR(2) NOT NULL DEFAULT `PH` | ISO |
| `notes` | TEXT NULL | Admin-only |
| `tracksolid_account_ref` | VARCHAR(120) NULL | White-label portal username/account label |
| `source` | VARCHAR(64) NULL | `web_signup`, `referral`, `walk_in`, `admin` |
| `approved_at` | TIMESTAMPTZ NULL | |
| `approved_by` | UUID NULL → `users.id` | |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |
| `updated_at` | TIMESTAMPTZ NOT NULL | |
| `created_by` | UUID NULL | |
| `updated_by` | UUID NULL | |

**Indexes:** `status`, `company_name`, `billing_email`

---

### 4.2 `users` — login identities

People who can sign in (admin staff or customer portal users).

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `account_id` | UUID NULL → `accounts.id` | NULL for platform admins |
| `username` | VARCHAR(64) UNIQUE NOT NULL | |
| `email` | VARCHAR(255) UNIQUE NOT NULL | |
| `password_hash` | TEXT NOT NULL | bcrypt/argon2 |
| `full_name` | VARCHAR(200) NOT NULL | |
| `phone` | VARCHAR(40) NULL | |
| `role` | `user_role` NOT NULL | |
| `is_active` | BOOLEAN NOT NULL DEFAULT true | |
| `last_login_at` | TIMESTAMPTZ NULL | |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |
| `updated_at` | TIMESTAMPTZ NOT NULL | |

**Rules:**
- Platform `admin`: `account_id` IS NULL  
- Customer portal user: `account_id` NOT NULL, role `customer` or `operator`  
- Customer may only see rows scoped to their `account_id`

**Indexes:** `account_id`, `role`

---

### 4.3 `contacts` — additional people on an account (CRM)

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `account_id` | UUID NOT NULL → `accounts.id` ON DELETE CASCADE | |
| `full_name` | VARCHAR(200) NOT NULL | |
| `title` | VARCHAR(120) NULL | |
| `email` | VARCHAR(255) NULL | |
| `phone` | VARCHAR(40) NULL | |
| `is_primary` | BOOLEAN NOT NULL DEFAULT false | |
| `notes` | TEXT NULL | |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |
| `updated_at` | TIMESTAMPTZ NOT NULL | |

---

### 4.4 `signup_requests` — how customers reach you

Public/inbound registration before an account exists.

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `status` | `signup_status` NOT NULL DEFAULT `new` | |
| `full_name` | VARCHAR(200) NOT NULL | |
| `company_name` | VARCHAR(200) NOT NULL | |
| `email` | VARCHAR(255) NOT NULL | |
| `phone` | VARCHAR(40) NULL | |
| `requested_username` | VARCHAR(64) NULL | |
| `estimated_devices` | INT NULL | |
| `message` | TEXT NULL | |
| `preferred_contact` | VARCHAR(32) NULL | `email`, `phone`, `viber` |
| `ip_hash` | VARCHAR(128) NULL | Privacy-preserving |
| `user_agent_hash` | VARCHAR(128) NULL | |
| `reviewed_by` | UUID NULL → `users.id` | |
| `reviewed_at` | TIMESTAMPTZ NULL | |
| `rejection_reason` | TEXT NULL | |
| `converted_account_id` | UUID NULL → `accounts.id` | Set on approve |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |
| `updated_at` | TIMESTAMPTZ NOT NULL | |

**Indexes:** `status`, `email`, `created_at`  
**Partial unique (optional):** unique `(email)` where `status IN ('new','reviewing')`

---

### 4.5 `devices` — GPS units under an account

What the customer sees in “My devices.”

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `account_id` | UUID NOT NULL → `accounts.id` ON DELETE CASCADE | |
| `name` | VARCHAR(120) NOT NULL | e.g. plate / label |
| `plate_number` | VARCHAR(32) NULL | |
| `imei` | VARCHAR(32) NULL | |
| `provider` | VARCHAR(32) NOT NULL DEFAULT `tracksolid` | |
| `provider_device_id` | VARCHAR(64) NULL | Tracksolid device id |
| `provider_account_ref` | VARCHAR(120) NULL | Which WL sub-account |
| `status` | `device_status` NOT NULL DEFAULT `pending_install` | |
| `install_date` | DATE NULL | |
| `notes` | TEXT NULL | Admin |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |
| `updated_at` | TIMESTAMPTZ NOT NULL | |

**Indexes:** `account_id`, `imei`, `provider_device_id`, `status`  
**Unique (optional):** `(provider, provider_device_id)` where not null

---

### 4.6 `sim_cards` — Smart SIM inventory & assignment

Central to your operational pain point.

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `carrier` | `sim_carrier` NOT NULL DEFAULT `smart` | |
| `iccid` | VARCHAR(32) UNIQUE NULL | Prefer storing when available |
| `msisdn` | VARCHAR(20) UNIQUE NULL | Mobile number |
| `sim_label` | VARCHAR(64) NULL | Internal sticker/code |
| `status` | `sim_status` NOT NULL DEFAULT `inventory` | |
| `purchase_date` | DATE NULL | |
| `purchase_cost_cents` | INT NULL | Admin/finance |
| `data_plan_label` | VARCHAR(120) NULL | e.g. “Smart Bro 1yr promo” |
| `account_id` | UUID NULL → `accounts.id` | NULL = still in inventory |
| `device_id` | UUID NULL → `devices.id` | Assigned device |
| `activated_at` | TIMESTAMPTZ NULL | |
| `last_status_check_at` | TIMESTAMPTZ NULL | Manual or future automation |
| `data_exhausted_at` | TIMESTAMPTZ NULL | When known empty |
| `notes` | TEXT NULL | |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |
| `updated_at` | TIMESTAMPTZ NOT NULL | |

**Constraints:**
- At least one of `iccid` or `msisdn` required (CHECK)
- If `device_id` set, `account_id` must match device’s account (enforce in app or trigger)

**Indexes:** `status`, `account_id`, `device_id`, `carrier`

---

### 4.7 `plans` — service packages (catalog)

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `code` | VARCHAR(32) UNIQUE NOT NULL | `BASIC`, `PRO` |
| `name` | VARCHAR(120) NOT NULL | |
| `description` | TEXT NULL | |
| `price_cents` | INT NOT NULL DEFAULT 0 | Admin-visible |
| `currency` | CHAR(3) NOT NULL DEFAULT `PHP` | |
| `billing_cycle` | VARCHAR(16) NOT NULL DEFAULT `monthly` | `monthly`,`yearly`,`one_time` |
| `device_limit` | INT NOT NULL DEFAULT 1 | |
| `included_sims` | INT NOT NULL DEFAULT 1 | |
| `includes_data_months` | INT NULL | e.g. `12` if package shoulders 1 year |
| `is_active` | BOOLEAN NOT NULL DEFAULT true | |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |
| `updated_at` | TIMESTAMPTZ NOT NULL | |

---

### 4.8 `promos` — discounts / campaigns

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `code` | VARCHAR(32) UNIQUE NOT NULL | |
| `name` | VARCHAR(120) NOT NULL | |
| `description` | TEXT NULL | |
| `discount_type` | VARCHAR(24) NOT NULL | `percent`,`fixed`,`free_months` |
| `discount_value` | NUMERIC(12,2) NOT NULL DEFAULT 0 | |
| `starts_at` | TIMESTAMPTZ NULL | |
| `ends_at` | TIMESTAMPTZ NULL | |
| `max_redemptions` | INT NULL | |
| `redemption_count` | INT NOT NULL DEFAULT 0 | |
| `is_active` | BOOLEAN NOT NULL DEFAULT true | |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |
| `updated_at` | TIMESTAMPTZ NOT NULL | |

---

### 4.9 `subscriptions` — service + data coverage window

Ties account (and optionally device/SIM) to plan and **1-year shouldering** policy.

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `account_id` | UUID NOT NULL → `accounts.id` ON DELETE CASCADE | |
| `plan_id` | UUID NOT NULL → `plans.id` | |
| `promo_id` | UUID NULL → `promos.id` | |
| `status` | `subscription_status` NOT NULL DEFAULT `active` | |
| `coverage_policy` | `coverage_policy` NOT NULL DEFAULT `shouldered_by_us` | |
| `starts_at` | TIMESTAMPTZ NOT NULL | Commercial / coverage start |
| `ends_at` | TIMESTAMPTZ NULL | Soft end |
| `data_coverage_starts_at` | DATE NULL | When we start shouldering data |
| `data_coverage_ends_at` | DATE NULL | e.g. start + 1 year |
| `continue_shouldering` | BOOLEAN NULL | Admin decision near expiry |
| `renews_at` | TIMESTAMPTZ NULL | Next billing renew |
| `amount_cents` | INT NULL | Negotiated / discounted |
| `currency` | CHAR(3) NOT NULL DEFAULT `PHP` | |
| `notes` | TEXT NULL | Admin |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |
| `updated_at` | TIMESTAMPTZ NOT NULL | |

**Indexes:** `account_id`, `status`, `data_coverage_ends_at`, `renews_at`

**Customer portal fields (trimmed):** plan name, status, `data_coverage_ends_at`, renew date — **not** `amount_cents`, `notes`, purchase costs.

---

### 4.10 `subscription_sims` — which SIMs are covered by a subscription

Many-to-many (one subscription may cover multiple SIMs).

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `subscription_id` | UUID NOT NULL → `subscriptions.id` ON DELETE CASCADE | |
| `sim_card_id` | UUID NOT NULL → `sim_cards.id` ON DELETE CASCADE | |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |

**Unique:** `(subscription_id, sim_card_id)`

---

### 4.11 `billing_invoices`

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `account_id` | UUID NOT NULL → `accounts.id` | |
| `subscription_id` | UUID NULL → `subscriptions.id` | |
| `invoice_number` | VARCHAR(32) UNIQUE NOT NULL | `INV-2026-0001` |
| `status` | `invoice_status` NOT NULL DEFAULT `draft` | |
| `issue_date` | DATE NULL | |
| `due_date` | DATE NULL | |
| `currency` | CHAR(3) NOT NULL DEFAULT `PHP` | |
| `subtotal_cents` | INT NOT NULL DEFAULT 0 | |
| `tax_cents` | INT NOT NULL DEFAULT 0 | |
| `total_cents` | INT NOT NULL DEFAULT 0 | |
| `amount_paid_cents` | INT NOT NULL DEFAULT 0 | |
| `notes` | TEXT NULL | |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |
| `updated_at` | TIMESTAMPTZ NOT NULL | |

**Indexes:** `account_id`, `status`, `due_date`

---

### 4.12 `billing_line_items`

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `invoice_id` | UUID NOT NULL → `billing_invoices.id` ON DELETE CASCADE | |
| `description` | VARCHAR(255) NOT NULL | |
| `quantity` | NUMERIC(12,2) NOT NULL DEFAULT 1 | |
| `unit_amount_cents` | INT NOT NULL | |
| `line_total_cents` | INT NOT NULL | |
| `device_id` | UUID NULL → `devices.id` | Optional link |
| `sim_card_id` | UUID NULL → `sim_cards.id` | Optional link |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |

---

### 4.13 `billing_payments` (optional Phase 1b)

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `invoice_id` | UUID NOT NULL → `billing_invoices.id` | |
| `paid_at` | TIMESTAMPTZ NOT NULL | |
| `amount_cents` | INT NOT NULL | |
| `method` | VARCHAR(32) NULL | `cash`,`gcash`,`bank`,`other` |
| `reference` | VARCHAR(120) NULL | |
| `notes` | TEXT NULL | |
| `recorded_by` | UUID NULL → `users.id` | |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |

---

### 4.14 `tickets` — customer-raised issues (CRM support)

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `account_id` | UUID NOT NULL → `accounts.id` | |
| `number` | VARCHAR(32) UNIQUE NOT NULL | `TKT-000123` |
| `created_by_user_id` | UUID NOT NULL → `users.id` | Portal user |
| `assigned_to_user_id` | UUID NULL → `users.id` | Admin/staff |
| `device_id` | UUID NULL → `devices.id` | Optional |
| `sim_card_id` | UUID NULL → `sim_cards.id` | Optional |
| `subject` | VARCHAR(200) NOT NULL | |
| `description` | TEXT NULL | |
| `status` | `ticket_status` NOT NULL DEFAULT `open` | |
| `priority` | `ticket_priority` NOT NULL DEFAULT `p2` | |
| `category` | `ticket_category` NOT NULL DEFAULT `other` | |
| `resolved_at` | TIMESTAMPTZ NULL | |
| `closed_at` | TIMESTAMPTZ NULL | |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |
| `updated_at` | TIMESTAMPTZ NOT NULL | |

**Indexes:** `account_id`, `status`, `priority`, `created_by_user_id`, `assigned_to_user_id`

**Portal rules:**
- Customer creates tickets only for their `account_id`
- Customer lists only their account’s tickets
- Admin lists all

---

### 4.15 `ticket_comments`

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `ticket_id` | UUID NOT NULL → `tickets.id` ON DELETE CASCADE | |
| `author_user_id` | UUID NOT NULL → `users.id` | |
| `body` | TEXT NOT NULL | |
| `is_internal` | BOOLEAN NOT NULL DEFAULT false | Admin-only notes if true |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |

**Portal:** customers never see `is_internal = true`.

---

### 4.16 `ticket_attachments` (optional)

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `ticket_id` | UUID NOT NULL → `tickets.id` ON DELETE CASCADE | |
| `uploaded_by` | UUID NOT NULL → `users.id` | |
| `file_name` | VARCHAR(255) NOT NULL | |
| `content_type` | VARCHAR(120) NOT NULL | |
| `storage_key` | TEXT NOT NULL | S3/R2 path |
| `byte_size` | INT NOT NULL | |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |

---

### 4.17 `audit_events` — CRM activity trail (admin)

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `account_id` | UUID NULL → `accounts.id` | |
| `actor_user_id` | UUID NULL → `users.id` | |
| `entity_type` | VARCHAR(64) NOT NULL | `account`,`sim`,`ticket`,… |
| `entity_id` | UUID NOT NULL | |
| `action` | VARCHAR(64) NOT NULL | `created`,`status_changed`,… |
| `summary` | VARCHAR(255) NOT NULL | |
| `metadata` | JSONB NULL | Non-secret diffs |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |

**Indexes:** `account_id`, `(entity_type, entity_id)`, `created_at`

---

### 4.18 `idempotency_records` (API safety)

Same pattern as Visionroute2 mutations: signup, ticket create, invoice create, SIM assign.

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `scope_id` | VARCHAR(64) NOT NULL | account or `global` |
| `user_id` | UUID NULL | |
| `action` | VARCHAR(64) NOT NULL | |
| `idempotency_key_hash` | CHAR(64) NOT NULL | |
| `request_hash` | CHAR(64) NOT NULL | |
| `response_status` | INT NULL | |
| `response_body` | TEXT NULL | |
| `operation_status` | VARCHAR(24) NOT NULL | `in_progress`,`completed`,`failed` |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |
| `expires_at` | TIMESTAMPTZ NOT NULL | |

**Unique:** `(scope_id, action, idempotency_key_hash)`

---

## 5. Visibility matrix

| Data | Admin | Customer portal |
|------|-------|-----------------|
| All accounts | Yes | Own account only |
| Signup inbox | Yes | No (public form only) |
| Devices | All | Own devices |
| SIM inventory (unassigned) | Yes | No |
| Assigned SIM MSISDN/ICCID | Yes | Own assigned SIMs (basic) |
| Plan price / invoice amounts | Yes | Optional later; MVP = hide amounts |
| Coverage end date | Yes | Yes (basic) |
| Tickets | All | Own account tickets |
| Internal ticket comments | Yes | No |
| Audit events | Yes | No |

---

## 6. Customer portal screens → tables

| Screen | Primary tables |
|--------|----------------|
| Sign up / request | `signup_requests` |
| Login | `users` |
| Dashboard | `accounts`, `subscriptions`, open `tickets` count |
| My devices | `devices` (+ optional linked `sim_cards`) |
| My plan / coverage | `subscriptions`, `plans` (name only) |
| Raise ticket | `tickets`, `ticket_comments` |
| My tickets | `tickets` filtered by `account_id` |

---

## 7. Admin CRM screens → tables

| Screen | Primary tables |
|--------|----------------|
| Signup inbox | `signup_requests` |
| Customers | `accounts`, `contacts`, `users` |
| Customer 360 | account + devices + sims + subscriptions + invoices + tickets |
| SIM inventory | `sim_cards` where `status = inventory` |
| Coverage monitor | `subscriptions` ordered by `data_coverage_ends_at` |
| Billing | `billing_invoices`, line items, payments |
| Support console | `tickets` (all accounts) |
| Plans / promos | `plans`, `promos` |

---

## 8. MVP subset (build first)

If you want the smallest schema that still unblocks launch:

1. `signup_requests`  
2. `accounts`  
3. `users`  
4. `devices`  
5. `sim_cards`  
6. `subscriptions` (+ `plans`)  
7. `tickets` + `ticket_comments`  
8. `billing_invoices` + `billing_line_items` (even if drafts only)

Defer: `promos`, `payments`, `attachments`, rich `audit_events` (can add quickly later).

---

## 9. Example approval flow (data writes)

1. Prospect submits → insert `signup_requests` (`new`)  
2. Admin approves → insert `accounts` (`active`) + `users` (customer login) + set `converted_account_id`  
3. Admin adds `devices`  
4. Admin assigns `sim_cards` from inventory → set `account_id` / `device_id` / `status=assigned|active`  
5. Admin creates `subscriptions` with `data_coverage_starts_at` / `data_coverage_ends_at` (= +1 year)  
6. Optional `billing_invoices` draft  
7. Customer logs in → sees devices, coverage basics, creates `tickets`

---

## 10. Future control-layer integration

When merging into Visionroute2 / control layer:

- Map `accounts` ↔ existing `Customer`  
- Map `devices` ↔ `Vehicle` + Tracksolid provider ids  
- Map `tickets` ↔ native tickets  
- Keep `sim_cards` + coverage as the **new** domain Visionroute2 does not have yet  

Stable UUIDs and clear `provider_*` columns make that merge safer.

---

## 11. Open decisions (confirm before coding DDL)

1. **Coverage start trigger:** install date vs payment date vs SIM activation date?  
2. **One subscription per account vs per device/SIM?** (schema supports account-level + `subscription_sims`)  
3. **Customer sees invoice amounts in MVP?** (recommend **no** at first)  
4. **Username vs email login?** (schema supports both)  
5. **Must ICCID always be captured, or is MSISDN enough for Smart?**  

---

Once you confirm the open decisions (especially coverage start + subscription grain), we can generate the actual PostgreSQL DDL migration files in this workspace and scaffold the app around this schema.
