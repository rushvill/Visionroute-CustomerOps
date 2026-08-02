# VisionRoute Customer Ops — locked UI theme: Route Command

**Status:** Locked for implementation (approved design review)  
**Scope:** Presentability only — no new backend features  
**Related brief:** [DESIGN_BRIEF_FOR_THEME.md](./DESIGN_BRIEF_FOR_THEME.md)  
**Plan:** Presentable UX pass (Route Command)

---

## 1. Direction

**Route Command** — professional fleet-operations identity inspired by navigation, road infrastructure, dispatch, and reliable logistics.

Feel: dependable, operational, modern — not futuristic, not generic SaaS, not decorative.

Metaphor: a well-managed route — clear direction, visible status, predictable actions — not “cars on a map.”

### Brand hierarchy

| Surface | Primary | Subtitle |
|---------|---------|----------|
| Public + customer | **VisionRoute** | Fleet services and customer support |
| Admin | **VisionRoute** | Customer Operations |

Customers should feel they use the VisionRoute portal, not an internal CRM.

---

## 2. Color tokens

### Core

| Token | Hex | Usage |
|-------|-----|--------|
| `--color-canvas` | `#F4F6F8` | Main background |
| `--color-canvas-subtle` | `#E9EDF1` | Alternate sections, table headers |
| `--color-surface` | `#FFFFFF` | Forms, interactive containers |
| `--color-surface-raised` | `#FAFBFC` | Elevated controls |
| `--color-text` | `#15202B` | Primary text |
| `--color-text-secondary` | `#526171` | Supporting text |
| `--color-text-muted` | `#748292` | Labels, metadata |
| `--color-border` | `#D5DCE3` | Borders |
| `--color-border-strong` | `#AEB9C4` | Emphasized boundaries |
| `--color-brand` | `#123B5D` | VisionRoute navy |
| `--color-brand-strong` | `#092A43` | Hero, nav, strong emphasis |
| `--color-brand-soft` | `#DCE9F2` | Selected rows, soft brand bg |
| `--color-accent` | `#E89A24` | Amber accent (~10% of screen) |
| `--color-accent-hover` | `#CC7C0E` | Accent hover |
| `--color-accent-soft` | `#FFF1D8` | Soft highlight |

### Semantic

| Token | Hex |
|-------|-----|
| `--color-success` / soft | `#18794E` / `#E1F3E9` |
| `--color-warning` / soft | `#A96108` / `#FFF0D2` |
| `--color-danger` / soft | `#B33A3A` / `#FBE5E5` |
| `--color-info` / soft | `#176B87` / `#DDF0F5` |

Dark navy only for: public nav over hero, home hero, admin sidebar, auth visual panel. Default daily UI stays light.

Amber = acquisition CTAs, pending work, small route highlights — not every control.

---

## 3. Typography

- **Display:** Sora — headlines, titles, brand, key numbers (600/700)
- **Body:** Source Sans 3 — UI, forms, tables (400/500/600)

Type scale (desktop / mobile): hero 56–64 / 38–44; page title 32–36 / 28–32; section 22–24 / 20–22; body 16; compact 14; meta 12–13.

Heading line-height ~1.1–1.2; body ~1.5–1.65.

---

## 4. Layout

- Max width ~1280px; 12-column desktop grid
- Spacing: 4, 8, 12, 16, 20, 24, 32, 40, 48, 64, 80, 96
- Radius: controls 8px; panels 10–12px; dialogs 14px; pills full
- Prefer borders over shadows; shadow only for dropdowns/dialogs/auth panel

---

## 5. Shells

1. **Public** — horizontal nav: VisionRoute · Services · Customer Portal · Sign In  
2. **Portal** — Overview · Devices · Smart SIMs · Coverage · Support (+ customer menu)  
3. **Admin** — navy left sidebar (~240–260px): Overview, Signup Inbox, Accounts, SIM Inventory, Coverage, Tickets, Users, Audit; light workspace; mobile drawer

Remove from public UI: API health, session debug, phase notes, engineering env labels.

---

## 6. Page recipes (summary)

- **Home:** one hero composition — eyebrow, headline “Keep every fleet connection on course.”, support line, Get started + Customer sign in; atmospheric road/fleet plane; then How it helps / portal preview / CTA. No hero cards.
- **Signup/Login:** ~42% brand panel / 58% form; group Contact + Fleet request; confirmation after signup.
- **Portal:** one account summary surface; devices/SIMs lists; coverage summary; tickets list + create.
- **Admin:** attention queue from real counts; tables; approve primary / reject labeled danger; audit as event log.

---

## 7. Components

Primary buttons: navy for app actions; **amber** for public acquisition. Secondary bordered navy. Danger for reject/destructive.

Inputs: white, 8px radius, navy/info focus ring, visible labels, ≥44px mobile height.

Status pills: compact semantic soft backgrounds + text (not oversized).

Tables: soft header, row separators, ~48–56px rows, sticky header when long.

Cards/panels only for forms, account summary, selected record, workflows — not every block.

---

## 8. Motion (exactly three families)

1. Hero route reveal (~700–900ms)  
2. Marketing section entrance (subtle fade/rise)  
3. Ops feedback (drawer/form success ~150–220ms)  

Respect `prefers-reduced-motion`.

---

## 9. Do not

Purple AI gradients, glassmorphism, fake KPIs/charts, live-map cosplay, dark-mode-default for customers, pill spam, engineering chrome on public pages, unsupported “AI/real-time GPS” claims.

---

## 10. Copy principle

Direct, calm, operational. Prefer “Your coverage is active through …” over internal status jargon.

Every screen: **Where am I? / What is the state? / What is the next action?**
