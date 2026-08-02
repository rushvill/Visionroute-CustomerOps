# Secure Rust Web Tool — Cursor Engineering Guide

> **Purpose:** This document is the implementation contract for Cursor when designing, building, reviewing, testing, and deploying a fast, secure web application.
>
> **Default stack:** SvelteKit + TypeScript frontend, Rust + Axum backend, PostgreSQL database, Redis only when justified, Docker for local/production parity.
>
> **Security target:** OWASP ASVS 5.0 Level 2 by default, plus selected Level 3 controls for administrators, credentials, external integrations, audit logs, and high-impact operations.

---

## 1. Instructions to Cursor

Cursor must follow these rules throughout the project:

1. Read this entire document before generating architecture or code.
2. Work in small, reviewable phases. Do not generate the entire application in one uncontrolled pass.
3. Before changing code, summarize:
   - what will change;
   - which files will change;
   - security implications;
   - tests that will prove the change works.
4. Never silently weaken authentication, authorization, validation, encryption, logging, testing, or browser security to make a feature work.
5. Never hard-code passwords, API keys, tokens, private keys, encryption keys, database URLs, or environment-specific secrets.
6. Never expose stack traces, SQL errors, secrets, internal paths, or provider responses to end users.
7. Never trust frontend validation. Repeat all important validation and authorization on the Rust server.
8. Prefer established, maintained libraries over custom cryptography, custom session formats, custom password hashing, or custom security middleware.
9. Pin important dependency versions. Explain major dependency additions.
10. Do not use `unsafe` Rust unless absolutely necessary, documented, reviewed, and isolated.
11. Do not use `unwrap()`, `expect()`, `panic!()`, or unchecked indexing in normal request-processing paths.
12. Do not invent database columns, API fields, roles, or business rules. Update schemas and contracts deliberately.
13. A feature is incomplete until it has:
    - authorization;
    - validation;
    - error handling;
    - logging;
    - tests;
    - relevant documentation.
14. Keep the application runnable after every phase.
15. Ask for clarification only when a missing business decision would materially change security, data ownership, or architecture. Otherwise choose the safest reasonable default and document it.

---

## 2. Required Engineering Skills

The project should apply these skills, whether performed manually or assisted by Cursor.

### Foundation

- HTML semantics and accessibility
- Modern CSS, responsive design, and layout
- TypeScript with strict mode
- Svelte and SvelteKit routing, loading, forms, and server hooks
- Rust ownership, borrowing, traits, enums, error handling, and async programming
- Axum routing, extractors, middleware, state, and response handling
- SQL and relational data modeling
- Git, pull requests, code review, and release discipline
- Linux, HTTP, DNS, TLS, reverse proxies, and containers

### Intermediate

- REST API design and OpenAPI
- Authentication, sessions, and role-based access control
- Database migrations and transactional operations
- Background jobs with retry and idempotency
- Structured logging, tracing, metrics, and alerting
- Unit, integration, end-to-end, and security testing
- Browser security headers and Content Security Policy
- CI/CD, dependency scanning, and container hardening
- Performance profiling and database query optimization
- Secure third-party API integration

### Advanced

- Threat modeling and abuse-case analysis
- Object-level and field-level authorization
- Multi-tenant isolation, when applicable
- Fine-grained permissions or policy-based authorization
- Secret rotation and key management
- Audit-log integrity and privileged action review
- Rate-limit design and denial-of-service resilience
- Supply-chain security, SBOMs, artifact signing, and provenance
- Disaster recovery, backup restoration tests, and incident response
- Horizontal scaling, queues, caching, and distributed tracing
- Penetration testing and OWASP ASVS verification

---

## 3. Recommended Architecture

```text
Browser / PWA
    |
    | HTTPS
    v
Reverse Proxy / Load Balancer
    |
    +--------------------------+
    |                          |
    v                          v
SvelteKit Frontend        Rust Axum API
SSR + client islands      Business logic and authorization
                               |
                  +------------+------------+
                  |                         |
                  v                         v
             PostgreSQL               External APIs
                  |
                  v
          Optional job worker
```

### Architecture principles

- Use a **modular monolith first**. Do not begin with microservices.
- Keep domain logic independent from HTTP handlers and database details.
- Keep frontend presentation separate from backend authorization and rules.
- Use server-side rendering for initial speed, SEO where relevant, and reduced client work.
- Hydrate only interactive components that need browser behavior.
- Keep API contracts explicit and versioned.
- Place long-running or retryable external work in a job worker rather than blocking requests.
- Add Redis only for a demonstrated need such as distributed rate limiting, caching, or a queue.
- Use PostgreSQL in production. SQLite may be used only for constrained local development or tests when behavior remains compatible.
- Default to one deployment region near primary users and the database.

---

## 4. Suggested Repository Structure

```text
project/
├── apps/
│   ├── web/                         # SvelteKit
│   │   ├── src/
│   │   │   ├── lib/
│   │   │   │   ├── api/
│   │   │   │   ├── components/
│   │   │   │   ├── schemas/
│   │   │   │   ├── stores/
│   │   │   │   └── utils/
│   │   │   ├── routes/
│   │   │   ├── hooks.server.ts
│   │   │   └── app.html
│   │   ├── static/
│   │   ├── tests/
│   │   └── package.json
│   └── api/                         # Rust Axum
│       ├── src/
│       │   ├── main.rs
│       │   ├── app.rs
│       │   ├── config.rs
│       │   ├── error.rs
│       │   ├── telemetry.rs
│       │   ├── auth/
│       │   ├── domain/
│       │   ├── http/
│       │   │   ├── middleware/
│       │   │   ├── routes/
│       │   │   └── extractors/
│       │   ├── services/
│       │   ├── repositories/
│       │   ├── integrations/
│       │   └── jobs/
│       ├── migrations/
│       ├── tests/
│       ├── Cargo.toml
│       └── deny.toml
├── packages/
│   └── contracts/                   # OpenAPI/generated shared types if used
├── deploy/
│   ├── docker/
│   ├── reverse-proxy/
│   └── monitoring/
├── docs/
│   ├── architecture/
│   ├── decisions/
│   ├── security/
│   ├── operations/
│   └── api/
├── .github/workflows/
├── .env.example
├── compose.yaml
├── SECURITY.md
├── CONTRIBUTING.md
└── README.md
```

### Rust boundaries

Use a dependency direction similar to:

```text
HTTP handlers
    -> application services
        -> domain types and rules
            -> repository/integration traits

Infrastructure implementations
    -> repository/integration traits
```

Handlers must stay thin. They should extract input, call a service, and map the result to an HTTP response.

---

## 5. Default Technology Choices

### Frontend

- SvelteKit
- TypeScript with `"strict": true`
- Vite
- Tailwind CSS or well-scoped CSS
- Zod or equivalent for client-side usability validation
- Playwright for end-to-end tests
- Vitest for unit/component tests
- Generated OpenAPI client or a small typed API wrapper
- Accessible component primitives when needed

### Backend

- Stable Rust toolchain
- Axum
- Tokio
- Tower and tower-http
- Serde
- SQLx with PostgreSQL and compile-time checked queries where practical
- `tracing` and `tracing-subscriber`
- `thiserror` for typed internal errors
- `secrecy` or equivalent for secret-bearing values
- Argon2id for locally stored passwords
- A maintained session or token library rather than hand-written formats
- `uuid` or equivalent non-sequential public identifiers
- `time` or `chrono` with UTC storage

### Operations

- Docker multi-stage builds
- Non-root production containers
- Reverse proxy or managed load balancer with TLS
- PostgreSQL with automated backups
- OpenTelemetry-compatible tracing where useful
- Prometheus-compatible metrics or managed equivalent
- Error tracking with secret and personal-data scrubbing

Do not add a library merely because it is popular. Confirm maintenance status, license, security history, and necessity.

---

## 6. Product and UX Best Practices

### Before coding

Create:

- problem statement;
- target users and roles;
- core workflows;
- data classification;
- permissions matrix;
- non-functional requirements;
- threat model;
- success metrics;
- out-of-scope list.

### Interface quality

- Mobile-first for user-facing pages.
- Desktop-first may be acceptable for administration, but it must remain usable at narrower widths.
- Use semantic HTML before ARIA.
- All controls must work with keyboard navigation.
- Visible focus states are mandatory.
- Every input needs an accessible label.
- Use sufficient contrast.
- Respect reduced-motion preferences.
- Provide loading, empty, partial, error, offline, and success states.
- Do not rely only on color to communicate status.
- Prevent duplicate submissions.
- Preserve user-entered data after recoverable errors.
- Confirm destructive actions and clearly describe their scope.
- Provide undo where safe and practical.
- Dates, currencies, and units must be localized at display time while stored canonically.

### Accessibility target

Target WCAG 2.2 AA for production-facing interfaces.

---

## 7. API Design Rules

- Prefix stable APIs, for example `/api/v1`.
- Use nouns for resources.
- Use correct HTTP methods and status codes.
- Return a consistent error envelope.
- Include a request ID in responses and logs.
- Validate:
  - path parameters;
  - query parameters;
  - headers;
  - JSON/form bodies;
  - uploaded files.
- Enforce request-body size limits.
- Use pagination with bounded maximum page size.
- Use filtering and sorting allowlists.
- Do not expose database internals in public identifiers.
- Use idempotency keys for retried create/payment/external-action operations.
- Use optimistic concurrency or version checks where lost updates matter.
- Document API contracts in OpenAPI.
- Do not return fields merely because they exist in the database.
- Use explicit response DTOs to prevent accidental data leakage.
- Apply timeouts to database and external-service operations.
- Retry only operations that are safe to retry, using bounded exponential backoff and jitter.
- Use circuit breaking or temporary failure isolation for unstable external dependencies.

### Standard error shape

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Some fields are invalid.",
    "fields": {
      "email": "Enter a valid email address."
    },
    "request_id": "01J..."
  }
}
```

Production responses must not contain stack traces, SQL statements, internal hostnames, secret values, or raw third-party errors.

---

## 8. Authentication Strategy

Choose one primary model deliberately.

### Recommended for a first-party browser application

Use opaque server-side sessions:

- Send the session identifier only in a cookie.
- Cookie flags:
  - `HttpOnly`;
  - `Secure`;
  - `SameSite=Lax` or stricter when compatible;
  - narrow `Path`;
  - no unnecessary `Domain`.
- Rotate the session ID:
  - after login;
  - after privilege changes;
  - after password reset;
  - after sensitive account recovery.
- Store only a cryptographic hash of session tokens in the database where practical.
- Implement absolute and idle expiration.
- Revoke sessions on password reset, suspected compromise, or administrative action.
- Allow users to inspect and revoke active sessions.
- Require recent reauthentication for sensitive operations.

### JWT use

Use JWTs only when there is a concrete distributed or delegated authorization requirement.

When JWTs are necessary:

- use asymmetric signing where multiple services verify tokens;
- pin accepted algorithms;
- validate issuer, audience, signature, expiry, and not-before;
- keep access tokens short-lived;
- rotate refresh tokens and detect reuse;
- store browser tokens in secure HttpOnly cookies, not local storage;
- maintain revocation strategy for compromised sessions;
- never place secrets or sensitive personal data in token payloads.

### Passwords

- Hash passwords with Argon2id using parameters calibrated for the production environment.
- Enforce a reasonable minimum length.
- Permit long passwords and password managers.
- Do not impose arbitrary composition rules.
- Check new passwords against known-compromised password data when feasible.
- Rate-limit login, password reset, verification, and recovery.
- Use generic responses to reduce account enumeration.
- Password reset tokens must be random, single-use, short-lived, and stored hashed.
- Do not log passwords, reset tokens, session tokens, or authorization headers.

### Multi-factor authentication

For administrators and high-impact roles:

- Prefer WebAuthn/passkeys.
- TOTP may be a fallback.
- Treat SMS as a weaker fallback, not the preferred method.
- Protect MFA enrollment and removal with recent authentication.
- Generate one-time recovery codes and store them hashed.
- Notify users of MFA and recovery changes.

---

## 9. Authorization

Authentication answers “who are you?” Authorization answers “may you perform this exact action on this exact object?”

### Required rules

- Deny by default.
- Enforce authorization on every protected backend route.
- Validate object ownership or tenant membership for every object ID.
- Never rely on hidden buttons or frontend route guards as security.
- Avoid scattered string comparisons such as `role == "admin"`.
- Centralize permissions in typed policies or service-layer checks.
- Separate:
  - read;
  - create;
  - update;
  - delete;
  - approve;
  - export;
  - impersonate;
  - manage users;
  - manage credentials.
- Protect field-level access when some fields are more sensitive.
- Recheck authorization when state changes make a previously valid action invalid.
- Log privileged and denied actions without leaking secrets.

### Permissions matrix template

| Resource | Action | User | Operator | Admin | System |
|---|---:|---:|---:|---:|---:|
| Own profile | Read/update | Yes | Yes | Yes | Limited |
| Other users | Read | No | Scoped | Yes | Limited |
| Credentials | View plaintext | No | No | Prefer never | Runtime only |
| Credentials | Rotate | No | Scoped | Yes | Yes |
| Audit logs | Read | No | Scoped | Yes | Append |
| Configuration | Update | No | Scoped | Yes | No |

The final matrix must be adapted to the actual product.

---

## 10. Input Validation and Injection Prevention

- Define typed request DTOs.
- Reject unknown fields for security-sensitive operations when practical.
- Validate lengths before expensive parsing.
- Normalize carefully; do not normalize identifiers in ways that merge distinct accounts.
- Use allowlists for enums, sort fields, export columns, MIME types, redirects, and provider names.
- Use SQLx bind parameters. Never construct SQL by concatenating untrusted input.
- If dynamic SQL is unavoidable, map requested fields to fixed server-side SQL fragments.
- Escape output according to its destination.
- Do not insert untrusted HTML with `{@html}`.
- Sanitize user-authored rich text with a maintained allowlist sanitizer.
- Validate URLs and block dangerous schemes.
- Protect against server-side request forgery:
  - permit only required schemes;
  - use domain/host allowlists where possible;
  - resolve and reject loopback, private, link-local, metadata, and reserved IP ranges;
  - revalidate redirects;
  - cap redirects, response size, and time;
  - use controlled outbound networking in production.
- Protect command execution:
  - avoid shell invocation;
  - pass arguments separately;
  - use strict allowlists;
  - run with minimal privileges.
- Protect templates from server-side template injection by never compiling untrusted template code.

---

## 11. Browser and HTTP Security

### Required response headers

Set centrally and test them:

```text
Strict-Transport-Security: max-age=31536000; includeSubDomains
X-Content-Type-Options: nosniff
Referrer-Policy: strict-origin-when-cross-origin
Permissions-Policy: camera=(), microphone=(), geolocation=()
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Resource-Policy: same-site
Content-Security-Policy: ...
```

Add `frame-ancestors 'none'` or an explicit allowlist in CSP unless embedding is required.

### Content Security Policy

Start in report-only mode, fix violations, then enforce.

A strong starting policy may resemble:

```text
default-src 'self';
base-uri 'none';
object-src 'none';
frame-ancestors 'none';
form-action 'self';
script-src 'self' 'nonce-{RANDOM_PER_RESPONSE}';
style-src 'self' 'nonce-{RANDOM_PER_RESPONSE}';
img-src 'self' data: https:;
font-src 'self';
connect-src 'self' https://required-api.example;
upgrade-insecure-requests;
```

Adapt it to real dependencies. Avoid `unsafe-eval`. Avoid `unsafe-inline`; use SvelteKit-supported nonces or hashes.

### CSRF

For cookie-authenticated state-changing requests:

- retain SvelteKit origin checking where applicable;
- verify `Origin` for unsafe methods;
- use CSRF tokens when origin checks alone are insufficient;
- use `SameSite` cookies as defense in depth, not as the only defense;
- never perform state changes through GET requests.

### CORS

- Prefer same-origin frontend and API deployment.
- Do not use `Access-Control-Allow-Origin: *` with credentials.
- Allow only known production origins.
- Allow only required methods and headers.
- Keep preflight caching bounded.
- Do not reflect arbitrary origins.

### TLS

- Redirect HTTP to HTTPS.
- Use modern TLS configuration managed by a trusted proxy/platform.
- Enable HSTS only after confirming all covered subdomains support HTTPS.
- Validate certificates for outbound requests.
- Never disable certificate validation to solve development problems.

---

## 12. Database Security

- Use a dedicated application database user.
- Do not grant superuser, schema-owner, or unrelated database privileges to the runtime user.
- Use migrations committed to version control.
- Review destructive migrations.
- Back up before risky production migrations.
- Wrap related state changes in transactions.
- Use unique constraints and foreign keys to enforce invariants.
- Use row-level security as defense in depth for high-risk multi-tenant systems, but do not treat it as a replacement for application authorization.
- Encrypt database connections.
- Encrypt sensitive data at the application layer when database administrators or backups must not reveal it.
- Keep encryption keys outside the database.
- Store timestamps in UTC.
- Use soft deletion only when the business requires recovery or retention; otherwise securely delete.
- Define retention and deletion rules for every sensitive dataset.
- Test backup restoration regularly.
- Prevent sensitive production data from being copied into development environments.

### Sensitive data classes

Classify each field:

1. Public
2. Internal
3. Confidential
4. Restricted

Restricted examples include passwords, tokens, private keys, recovery codes, government identifiers, payment data, and highly sensitive customer records.

Do not collect data without a defined purpose.

---

## 13. Secrets and Cryptography

- Store secrets in a platform secret manager or protected environment variables.
- Keep `.env` files out of Git.
- Commit only `.env.example` with safe placeholders.
- Validate required secrets at startup.
- Reject known placeholder values in production.
- Rotate credentials and encryption keys.
- Support key versioning for encrypted records.
- Use envelope encryption or a managed key service for high-value secrets.
- Use authenticated encryption such as AES-GCM or ChaCha20-Poly1305 through a maintained library.
- Generate random values using a cryptographically secure random generator.
- Never design custom encryption, signatures, password hashing, or token generation.
- Never log decrypted secrets.
- Avoid returning stored credentials to the frontend. Prefer server-side use.
- Zeroize highly sensitive in-memory values where supported and useful.

---

## 14. File Upload Security

When uploads are required:

- Require authentication and authorization.
- Limit total request size and per-file size.
- Allowlist extensions and independently verify content signatures.
- Generate server-side filenames.
- Store outside the web root.
- Prevent path traversal.
- Reject archives unless required.
- If archives are required, cap extracted file count, depth, and total size.
- Scan files for malware when risk warrants it.
- Re-encode images when possible.
- Strip unneeded metadata.
- Serve downloads with safe `Content-Disposition`.
- Use a separate download domain or object storage for untrusted active content.
- Never execute uploaded content.
- Apply retention and deletion policies.

---

## 15. Rate Limiting and Abuse Resistance

Implement layered limits:

- per IP;
- per user/account;
- per session;
- per endpoint;
- global safety limit.

Apply stricter policies to:

- login;
- signup;
- password reset;
- email/phone verification;
- MFA;
- search;
- exports;
- file uploads;
- expensive reports;
- external API triggers;
- AI/LLM operations, if present.

Requirements:

- Return `429 Too Many Requests`.
- Include a reasonable `Retry-After`.
- Use bounded queues.
- Set request, database, and outbound timeouts.
- Cap body size, pagination, concurrency, and export size.
- Prevent users from creating unbounded background work.
- Use quotas for costly tenant operations.
- Degrade safely rather than fail open.
- Do not reveal whether an account exists through materially different throttling responses.

For multi-instance deployments, use a shared rate-limit store or edge enforcement.

---

## 16. External Integrations

- Treat every provider response as untrusted input.
- Use typed provider clients.
- Keep provider-specific DTOs out of the domain model.
- Set connect and total timeouts.
- Limit response size.
- Validate content type.
- Retry only safe operations.
- Use idempotency for provider writes when supported.
- Verify webhook signatures using the raw request body.
- Reject replayed webhooks using timestamps/nonces/event IDs.
- Store processed event IDs to enforce idempotency.
- Use least-privilege provider credentials.
- Separate credentials by environment.
- Rotate and revoke credentials.
- Redact provider secrets and personal data from logs.
- Implement circuit breakers or temporary backoff.
- Expose provider failures as stable internal error codes, not raw messages.
- Record integration audit events.

---

## 17. Logging, Audit, Monitoring, and Incident Detection

### Application logs

Use structured JSON logs with:

- timestamp;
- severity;
- service/version;
- environment;
- request ID;
- trace ID;
- route template;
- status code;
- duration;
- authenticated subject ID, when appropriate;
- tenant ID, when appropriate;
- stable error code.

Never log:

- passwords;
- session tokens;
- access/refresh tokens;
- authorization headers;
- reset links;
- MFA secrets;
- private keys;
- full payment data;
- raw sensitive request bodies.

### Security audit log

Create a separate append-oriented audit trail for:

- login success/failure;
- logout and session revocation;
- password or MFA changes;
- role and permission changes;
- user creation, suspension, or deletion;
- secret creation, access, and rotation;
- exports;
- configuration changes;
- privileged reads;
- impersonation;
- destructive actions;
- webhook verification failures.

Each event should contain:

- actor;
- action;
- target;
- timestamp;
- result;
- request/trace ID;
- source IP or derived network context where legally appropriate;
- before/after summary without sensitive values.

Restrict audit-log access. Define retention. Protect logs from tampering.

### Metrics and alerts

Track at minimum:

- request count, error rate, and latency;
- database pool saturation and query latency;
- login and authorization failures;
- rate-limit events;
- job queue depth and failures;
- provider errors;
- resource saturation;
- deployment health;
- backup failures.

Alert on actionable conditions, not every isolated error.

---

## 18. Error Handling and Failure Safety

- Use typed errors internally.
- Map errors to stable public error codes.
- Fail closed for authentication, authorization, webhook verification, and security configuration.
- Do not continue startup in production when critical configuration is invalid.
- Use graceful shutdown.
- Bound retries.
- Avoid partial writes with transactions.
- Use idempotency for retried requests and jobs.
- Send failed jobs to a dead-letter state after bounded attempts.
- Make recovery actions explicit and auditable.
- Test exceptional paths:
  - database unavailable;
  - provider timeout;
  - invalid session;
  - stale permission;
  - duplicate request;
  - malformed input;
  - disk or queue pressure;
  - corrupted provider data.

---

## 19. Performance Requirements

Security and performance must be designed together.

### Frontend performance

- Set performance budgets:
  - initial JavaScript;
  - total page weight;
  - image weight;
  - largest contentful paint;
  - interaction latency;
  - layout shift.
- Use SSR or prerendering appropriately.
- Keep client-side state minimal.
- Code-split heavy features.
- Lazy-load maps, editors, charts, and admin-only modules.
- Optimize and size images correctly.
- Use modern image formats.
- Preload only critical assets.
- Avoid unnecessary third-party scripts.
- Virtualize very large lists.
- Paginate server-side.
- Debounce only where it improves UX; do not hide slow APIs.
- Cache immutable assets with content hashes.
- Use a service worker only with a deliberate update and cache strategy.

### Backend performance

- Use async I/O for network and database work.
- Do not hold locks across `.await`.
- Move CPU-heavy work to bounded blocking workers.
- Configure database pools from measured load.
- Avoid N+1 queries.
- Select only required columns.
- Add indexes based on query plans.
- Paginate large results.
- Stream large exports where safe.
- Compress suitable responses at the proxy or middleware.
- Cache only measured hot paths.
- Define cache invalidation before implementation.
- Protect expensive endpoints with limits.
- Run load tests before scaling architecture.

### Suggested targets

Adapt to product requirements:

- p95 API latency under 300 ms for ordinary reads in-region;
- p95 API latency under 600 ms for ordinary writes;
- no unbounded query or response;
- error rate below agreed service objective;
- Core Web Vitals in the “good” range for primary user flows.

Do not claim a performance target is achieved without measurements.

---

## 20. Testing Strategy

### Rust unit tests

Test:

- domain rules;
- permission policies;
- validators;
- status transitions;
- token/session utilities;
- error mapping;
- retry decisions.

### Backend integration tests

Use a real ephemeral PostgreSQL instance or isolated test database.

Test:

- migrations;
- repository queries;
- transactions;
- authentication;
- authorization;
- object ownership;
- validation;
- rate limiting;
- CSRF/origin behavior;
- headers;
- idempotency;
- external provider mocks;
- failure paths.

### Frontend tests

Test:

- rendering states;
- keyboard navigation;
- validation messages;
- disabled/loading behavior;
- permission-based navigation as UX only;
- error recovery;
- important components.

### End-to-end tests

Cover:

1. account creation or provisioning;
2. login and logout;
3. password reset;
4. MFA for privileged users;
5. primary business workflow;
6. authorization boundary between users/tenants;
7. administrator workflow;
8. destructive action confirmation;
9. session expiry;
10. inaccessible protected routes.

### Security tests

Automate checks for:

- missing authorization;
- insecure direct object reference;
- SQL injection payloads;
- stored and reflected XSS;
- CSRF;
- unsafe redirects;
- SSRF controls;
- oversized requests;
- malicious uploads;
- brute force/rate limiting;
- sensitive error leakage;
- security headers;
- dependency vulnerabilities;
- secret scanning.

### Additional methods

- Property-based tests for parsers, validators, and permission invariants.
- Fuzz untrusted parsers and complex payloads.
- Mutation testing for critical rules where practical.
- Load tests with realistic data volumes.
- Restore a backup in a test environment.
- Perform manual penetration testing before high-risk production releases.

A test that does not assert meaningful behavior is not sufficient.

---

## 21. Rust Quality Gates

Run locally and in CI:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo audit
cargo deny check
```

Recommended policies:

- committed `Cargo.lock` for applications;
- minimal feature flags;
- review duplicate dependency versions;
- deny unknown or unacceptable licenses;
- deny vulnerable, unmaintained, or yanked dependencies according to policy;
- document exceptions with owner and expiration date;
- forbid unsafe code at crate level where possible:

```rust
#![forbid(unsafe_code)]
```

Production profiles should be deliberate, for example:

```toml
[profile.release]
lto = "thin"
codegen-units = 1
panic = "abort"
strip = "symbols"
```

Benchmark before accepting slower compilation for marginal runtime gains.

---

## 22. Frontend Quality Gates

Run locally and in CI:

```bash
npm ci
npm run check
npm run lint
npm run test
npm run test:e2e
npm run build
npm audit --omit=dev
```

Also include:

- dependency review;
- secret scanning;
- bundle-size checks;
- accessibility checks;
- browser security-header tests against a deployed test environment.

Do not blindly apply automated dependency fixes that introduce breaking changes. Review and test them.

---

## 23. Supply-Chain Security

- Minimize dependencies.
- Use lockfiles.
- Use protected branches.
- Require review for production changes.
- Pin CI actions to immutable commit SHAs where feasible.
- Restrict CI token permissions.
- Do not expose secrets to untrusted pull requests.
- Separate build and deployment permissions.
- Generate an SBOM for releases.
- Scan dependencies and container images.
- Sign release artifacts or container images where supported.
- Preserve build provenance.
- Use trusted base images with minimal packages.
- Rebuild regularly for security updates.
- Define a vulnerability response SLA.
- Remove abandoned dependencies.
- Review install/build scripts in high-risk dependencies.

---

## 24. Container and Deployment Hardening

### Container rules

- Use multi-stage builds.
- Use a minimal runtime image.
- Run as a non-root numeric user.
- Use a read-only root filesystem when practical.
- Drop Linux capabilities.
- Do not mount the Docker socket.
- Do not bake secrets into layers.
- Add health/readiness endpoints that reveal no sensitive details.
- Set CPU and memory limits.
- Use a temporary writable directory only where required.
- Scan the final image.
- Pin base image by digest for controlled releases.

### Environment separation

Maintain separate:

- development;
- test;
- staging;
- production

accounts, credentials, databases, domains, and encryption keys.

Production data must not be used casually in development.

### Deployment process

1. Build once.
2. Run all quality and security gates.
3. Produce immutable artifact.
4. Deploy the same artifact promoted through environments.
5. Run migrations with a controlled strategy.
6. Run smoke tests.
7. Monitor errors and latency.
8. Support quick rollback.
9. Record deployment metadata and operator.

Use rolling, blue/green, or canary deployment when service criticality justifies it.

---

## 25. Backups, Recovery, and Business Continuity

- Define RPO: acceptable data loss.
- Define RTO: acceptable recovery time.
- Automate encrypted backups.
- Store backups separately from the primary system.
- Restrict and audit backup access.
- Test restoration on a schedule.
- Document:
  - database restoration;
  - secret/key restoration;
  - rollback;
  - provider outage behavior;
  - compromised credential rotation;
  - incident communications.
- Ensure deletion and retention requirements also apply to backups where legally required.
- Do not call backups “working” until restoration is proven.

---

## 26. Privacy and Data Governance

- Collect the minimum data required.
- Document purpose and retention for each sensitive field.
- Obtain appropriate consent where needed.
- Provide data export/correction/deletion workflows where applicable.
- Restrict internal access by role and purpose.
- Mask data in support/admin interfaces.
- Do not expose sensitive values in URLs.
- Avoid third-party analytics on authenticated or sensitive pages unless explicitly reviewed.
- Review cross-border storage and provider processing requirements.
- Maintain an inventory of subprocessors and external services.
- Establish breach-response procedures.

---

## 27. Security Maturity Levels

### Level 1 — Minimum development baseline

- HTTPS in deployed environments
- secure cookies
- password hashing
- backend authorization
- validation and parameterized SQL
- basic rate limiting
- secure headers
- secret management
- structured error handling
- dependency scanning
- unit and integration tests
- automated backups

### Level 2 — Required production baseline

- OWASP ASVS 5.0 Level 2 review
- threat model
- MFA for administrators
- centralized permission policies
- audit logging
- CSP enforcement
- CSRF and CORS tests
- session management and revocation
- file/SSRF protections where relevant
- CI security gates
- image/container scanning
- monitoring and alerts
- tested backup restoration
- documented incident response
- external security review for important releases

### Level 3 — High-value or high-risk system

- selected ASVS Level 3 verification
- phishing-resistant MFA/passkeys
- stronger tenant isolation
- field-level authorization
- key management service and key rotation
- tamper-resistant audit storage
- network egress controls
- WAF/API gateway where justified
- SBOM signing and build provenance
- regular penetration tests
- fuzzing and property testing of risky components
- formal privileged-access review
- disaster-recovery exercises
- security architecture review for every major integration

---

## 28. Development Phases for Cursor

### Phase 0 — Discovery

Produce:

- `docs/product-requirements.md`
- `docs/security/data-classification.md`
- `docs/security/permissions-matrix.md`
- `docs/security/threat-model.md`
- `docs/architecture/system-context.md`
- initial acceptance criteria

Do not code core features before roles, sensitive data, and trust boundaries are identified.

### Phase 1 — Foundation

Create:

- repository structure;
- SvelteKit app;
- Axum API;
- PostgreSQL;
- migrations;
- configuration validation;
- health/readiness endpoints;
- tracing and request IDs;
- Docker development environment;
- CI quality gates.

Acceptance criteria:

- clean build;
- tests run;
- invalid production configuration fails startup;
- no secrets committed;
- containers run as non-root.

### Phase 2 — Authentication

Implement:

- user model;
- Argon2id password hashing;
- opaque sessions;
- login/logout;
- password reset;
- email verification if required;
- rate limits;
- secure cookies;
- session rotation and revocation;
- security audit events.

Acceptance criteria include enumeration resistance, session fixation prevention, expiry tests, CSRF tests, and brute-force limits.

### Phase 3 — Authorization

Implement:

- roles;
- permission policies;
- object ownership or tenant scope;
- protected frontend UX;
- protected backend routes;
- denied-action audit events.

Test horizontal and vertical privilege escalation.

### Phase 4 — Core Domain

For each feature:

1. define domain types and invariants;
2. write migration;
3. implement repository;
4. implement service with authorization;
5. implement handler and DTOs;
6. implement frontend;
7. add unit, integration, and E2E tests;
8. update OpenAPI and documentation.

### Phase 5 — Hardening

Implement and verify:

- CSP;
- all security headers;
- stricter request limits;
- CORS allowlist;
- SSRF controls;
- file handling controls;
- dependency/security scans;
- audit UI;
- monitoring;
- backups and restoration;
- incident runbook.

### Phase 6 — Performance

- establish representative dataset;
- profile frontend bundle;
- profile API and SQL;
- remove N+1 queries;
- add necessary indexes;
- conduct load tests;
- set capacity limits;
- document measured results.

### Phase 7 — Release

- complete ASVS checklist;
- resolve high/critical findings;
- review permissions;
- rotate initial credentials;
- validate production CSP;
- restore backup;
- run penetration test;
- finalize rollback plan;
- deploy;
- monitor.

---

## 29. Threat Model Template

For each feature, document:

```markdown
## Feature

### Assets
What data or capability must be protected?

### Actors
Anonymous user, normal user, operator, administrator, external provider, attacker, insider.

### Entry points
Browser form, API route, webhook, file upload, background job, admin action.

### Trust boundaries
Browser/server, app/database, app/provider, tenant/tenant, operator/admin.

### Abuse cases
- user accesses another user's object;
- attacker repeats a privileged request;
- provider sends malformed data;
- attacker causes expensive work;
- compromised operator exports restricted data;
- malicious input reaches SQL, HTML, shell, URL fetch, or logs.

### Controls
Authentication, authorization, validation, encryption, limits, auditing, alerting.

### Tests
Specific automated and manual tests proving the controls.
```

Use STRIDE or another structured method for important components, but prioritize concrete abuse cases over paperwork.

---

## 30. Definition of Done

A story is done only when:

- [ ] Business acceptance criteria pass.
- [ ] Backend authorization is implemented and tested.
- [ ] Input validation and limits are implemented.
- [ ] Expected error and failure states are handled.
- [ ] No secret or sensitive value is exposed.
- [ ] Unit tests cover domain rules.
- [ ] Integration tests cover database/API behavior.
- [ ] Important user flow has E2E coverage.
- [ ] Security logging is included where applicable.
- [ ] Accessibility is checked.
- [ ] Performance impact is considered.
- [ ] API/documentation is updated.
- [ ] Migrations are reversible or have a documented recovery path.
- [ ] `fmt`, `clippy`, tests, audits, lint, and builds pass.
- [ ] No unresolved critical/high vulnerability remains without documented approval.
- [ ] Code has been reviewed by a human.

---

## 31. Pull Request Checklist

```markdown
## Purpose
What problem does this change solve?

## Architecture
Why is this design appropriate?

## Security
- Authentication impact:
- Authorization impact:
- Data classification:
- New trust boundaries:
- Abuse cases considered:
- Secrets/PII handling:
- Logging/audit impact:

## Database
- Migration:
- Constraints/indexes:
- Rollback/recovery:

## Performance
- Expected load:
- Query count:
- Payload/bundle impact:
- Measurements:

## Testing
- Unit:
- Integration:
- E2E:
- Security/failure paths:

## Deployment
- Configuration:
- Migration order:
- Monitoring:
- Rollback:
```

---

## 32. Cursor Prompt for Each Feature

Copy and adapt this prompt:

```markdown
Implement the following feature according to `CURSOR_SECURE_RUST_WEB_GUIDE.md`.

Feature:
[Describe the feature.]

Users and permissions:
[Who can do what?]

Sensitive data:
[List any confidential or restricted fields.]

Acceptance criteria:
[List observable behavior.]

Before coding:
1. Inspect the existing architecture and reuse established patterns.
2. Identify threats, abuse cases, authorization boundaries, validation, limits, audit events, and failure states.
3. List files to change and provide a concise implementation plan.
4. Identify migrations and API-contract changes.
5. Do not weaken existing security controls.

During implementation:
- Keep handlers thin.
- Put business rules and authorization in services/policies.
- Use typed DTOs and parameterized SQL.
- Return stable, non-sensitive errors.
- Add structured logs and required audit events.
- Add unit, integration, E2E, and relevant security tests.
- Keep all quality gates passing.

After implementation:
1. Run format, lint, test, audit, and build commands.
2. Show results.
3. Summarize security decisions.
4. State any unresolved risks or assumptions.
5. Update documentation and OpenAPI.
```

---

## 33. Initial Cursor Bootstrap Prompt

```markdown
Read `CURSOR_SECURE_RUST_WEB_GUIDE.md` completely and treat it as the project's engineering and security contract.

Create only Phase 0 and Phase 1 first. Do not implement product-specific core features yet.

Use:
- SvelteKit + strict TypeScript for the frontend;
- Rust + Axum + Tokio for the API;
- PostgreSQL + SQLx;
- opaque server-side cookie sessions as the planned browser authentication model;
- Docker Compose for development;
- structured tracing and request IDs;
- OpenAPI;
- CI with formatting, linting, tests, dependency audits, secret scanning, and builds.

First produce:
1. assumptions;
2. architecture decision record;
3. directory plan;
4. data classification template;
5. permissions matrix template;
6. threat-model skeleton;
7. phased task list;
8. files that will be created.

Then scaffold the foundation in small steps. After each step, run the relevant checks and report failures honestly. Do not use placeholder security that could accidentally ship to production.
```

---

## 34. Commands Reference

### Rust

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo audit
cargo deny check
cargo build --release
```

### SvelteKit

```bash
npm ci
npm run check
npm run lint
npm run test
npm run test:e2e
npm run build
```

### Database

```bash
sqlx migrate run
sqlx migrate info
cargo sqlx prepare --workspace
```

### Containers

```bash
docker compose config
docker compose build --pull
docker compose up
docker compose ps
docker compose logs
```

Never run destructive database or container commands against production without explicit environment verification and a recovery plan.

---

## 35. Authoritative Security and Framework References

Cursor should consult current official documentation before implementing version-sensitive behavior:

- OWASP Application Security Verification Standard 5.0
- OWASP Top 10:2025
- OWASP Cheat Sheet Series
- Rust and Cargo official documentation
- Axum, Tokio, Tower, and tower-http official documentation
- SvelteKit official documentation, especially CSP and CSRF configuration
- SQLx official documentation
- RustSec Advisory Database and `cargo audit`
- `cargo-deny` documentation
- WCAG 2.2
- NIST guidance where relevant to authentication, cryptography, and incident response

This document is a baseline, not a substitute for product-specific threat modeling or professional security review.
