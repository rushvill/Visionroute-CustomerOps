# Deploy — Vercel (web) + Render (API + Postgres)

```
Browser → Vercel (SvelteKit BFF) → Render (Rust API) → Render Postgres
```

Session cookies are set on the **Vercel** domain via the BFF; the browser never talks to Render directly for login.

## Free-tier notes (Render)

- **Postgres free** expires after ~30 days — upgrade before real customer data.
- **Web free** sleeps after idle; first request can take ~30–60s to wake.
- **Do not compile Rust on Render free** if builds OOM. This repo builds the API image on **GitHub Actions** and Render runs `ghcr.io/rushvill/customerops-api:latest`.
- After pushing to `master`, wait for the **api-image** workflow to finish, then **Manual Sync** the Blueprint (or redeploy the service).
- If Render cannot pull the image, open GitHub → Packages → `customerops-api` → Package settings → **Change visibility** → Public.

## Prerequisites

- GitHub repo with this project pushed
- [Vercel](https://vercel.com) account
- [Render](https://render.com) account

**Note:** Render **free** Postgres expires after **30 days** (upgrade before then for real customer data). Free web services cold-start after idle.

## Order of operations

1. Push to GitHub  
2. Deploy Render (API + DB) — you can set a temporary `FRONTEND_ORIGIN` like `https://customerops.vercel.app` if the Vercel URL is not known yet  
3. Deploy Vercel with `BACKEND_API_BASE_URL` pointing at the Render API  
4. Set the real `FRONTEND_ORIGIN` on Render to the live Vercel `https://…` origin and redeploy the API  
5. Bootstrap admin → log in → remove `BOOTSTRAP_ADMIN_PASSWORD`  

## 1. Render — API + database

1. Dashboard → **New** → **Blueprint** → connect the GitHub repo → select `render.yaml`
2. Or: **Web Service** (Docker) with:
   - Root / Docker context: `apps/api`
   - Dockerfile: `apps/api/Dockerfile`
   - Health check: `/health`
3. Attach a **Postgres** instance; set `DATABASE_URL` from the connection string
4. Environment (production):

| Variable | Value |
|----------|--------|
| `APP_ENV` | `production` |
| `API_HOST` | `0.0.0.0` |
| `SESSION_SECRET` | `openssl rand -base64 32` (or Render generate) |
| `FRONTEND_ORIGIN` | Your Vercel URL, e.g. `https://customerops.vercel.app` (set after step 2) |
| `BOOTSTRAP_ADMIN_PASSWORD` | Strong password (12+), **remove after first login** |
| `BOOTSTRAP_ADMIN_USERNAME` | `admin` (optional) |
| `BOOTSTRAP_ADMIN_EMAIL` | Your real ops email |

5. Deploy and note the API URL: `https://customerops-api.onrender.com`

Demo seed users (`VisionRouteDemo26!`) **do not** run when `APP_ENV=production`.

## 2. Vercel — web

1. **Add New Project** → import the same GitHub repo
2. **Root Directory:** `apps/web`
3. Framework: SvelteKit (auto)
4. Environment variables:

| Variable | Value |
|----------|--------|
| `BACKEND_API_BASE_URL` | `https://customerops-api.onrender.com` |
| `PUBLIC_API_BASE_URL` | same as above (optional; BFF prefers `BACKEND_*`) |
| `FRONTEND_ORIGIN` | `https://<your-vercel-host>` (exact origin, no trailing slash) |

5. Deploy

## 3. Finish the handshake

1. On Render, set `FRONTEND_ORIGIN` to the live Vercel `https://…` URL and **redeploy** the API
2. Log in as bootstrap admin
3. **Delete** `BOOTSTRAP_ADMIN_PASSWORD` from Render env and redeploy
4. Create real customers via signup → admin approve (no demo password in prod)

## 4. Optional hardening

- Put Cloudflare (or similar) in front of the Vercel hostname
- Enable Render Postgres backups / paid plan before real customer data
- Custom domains on both Vercel and Render; keep `FRONTEND_ORIGIN` exact

## Local smoke before go-live

```bash
# API
cd apps/api && cargo test

# Web
cd apps/web && npm ci && npm run check && npm run build
```
