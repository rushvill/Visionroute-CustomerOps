# System context

```text
Browser / PWA
    | HTTPS
    v
SvelteKit (apps/web)          Rust Axum API (apps/api)
SSR + BFF forms/API routes -> Business logic + authorization
                                    |
                                    v
                               PostgreSQL
```

## Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Architecture | Modular monolith | Guide default; ship faster, fewer trust boundaries |
| Frontend | SvelteKit + strict TS | Guide default; SSR for signup/login |
| Backend | Rust + Axum | Guide default; strong typing for authz |
| Database | PostgreSQL | Production SoR; SQLite not for prod |
| Auth (planned Phase 2) | Opaque server-side cookie sessions | First-party browser app |
| Deploy target later | Vercel (web) + Render (API+DB) or Compose | Online launch; not Sheets |
| Visionroute2 | Separate; integrate later | Unblock business without rewriting GPS layer |

## ADR: Google Sheets

Sheets may be used only as a temporary import/export aid. **System of record is PostgreSQL.**

## ADR: Control layer integration

Customer Ops is the acquisition + CRM + SIM + ticket ledger. Tracksolid remains the GPS product. Merge into Visionroute2 only after MVP workflows are proven.
