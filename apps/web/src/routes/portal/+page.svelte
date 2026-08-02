<script lang="ts">
  import type { ActionData } from './$types';
  import Button from '$lib/components/ui/Button.svelte';
  import Field from '$lib/components/ui/Field.svelte';
  import StatusPill from '$lib/components/ui/StatusPill.svelte';
  import Alert from '$lib/components/ui/Alert.svelte';
  import { fmtDate, fmtMoney, humanStatus, maskId, statusTone } from '$lib/format';

  let { data, form }: { data: import('./$types').PageData; form: ActionData } = $props();

  const isPromo = $derived(
    data.subscription?.status === 'trial' ||
      (data.subscription?.planName?.toLowerCase().includes('promo') ?? false)
  );
  const openBillCount = $derived(
    data.invoices.filter((i) => ['sent', 'partial', 'overdue'].includes(i.status)).length
  );
</script>

<main id="main" class="portal">
  <nav class="portal-nav" aria-label="Portal sections">
    <a href="#overview">Overview</a>
    <a href="#subscription">Subscription</a>
    <a href="#billing">Bills</a>
    <a href="#devices">GPS units</a>
    <a href="#privacy">Privacy</a>
  </nav>

  {#if data.noAccount}
    <Alert tone="warning" title="Account not ready yet">
      <p>
        VisionRoute has your signup. Our team will call you and create your customer access when
        everything is set up.
      </p>
    </Alert>
  {:else if !data.account}
    <Alert tone="danger" title="Unable to load account">
      <p>We could not load your account details. Try signing in again.</p>
    </Alert>
  {:else}
    <header class="portal-head" id="overview">
      <div>
        <h1>{data.account.companyName}</h1>
        <p class="sub">Your VisionRoute tracking account · {data.account.accountCode}</p>
      </div>
      <div class="head-actions">
        <StatusPill tone={statusTone(data.account.status)} label={humanStatus(data.account.status)} />
        {#if isPromo}
          <StatusPill tone="info" label="Promo" />
        {/if}
      </div>
    </header>

    <section class="summary" aria-label="Account overview">
      <p class="summary-label">At a glance</p>
      <div class="summary-metrics">
        <div>
          <span>GPS units</span>
          <strong>{data.devices.length}</strong>
        </div>
        <div>
          <span>Subscription</span>
          <strong
            >{data.subscription ? humanStatus(data.subscription.status) : 'Pending'}</strong
          >
        </div>
        <div>
          <span>Open bills</span>
          <strong>{openBillCount}</strong>
        </div>
      </div>
    </section>

    <section id="subscription" class="block">
      <h2>Promo &amp; subscription</h2>
      <p class="block-lede">Your current plan and service window.</p>

      {#if !data.subscription}
        <div class="empty">
          <h3>No subscription yet</h3>
          <p>When VisionRoute activates your plan, it will show here.</p>
        </div>
      {:else}
        <div class="coverage-panel">
          <div class="plan-row">
            <p class="plan">{data.subscription.planName ?? 'Service plan'}</p>
            <div class="plan-badges">
              <StatusPill
                tone={statusTone(data.subscription.status)}
                label={humanStatus(data.subscription.status)}
              />
              {#if isPromo}
                <StatusPill tone="info" label="Promo" />
              {/if}
            </div>
          </div>
          <p class="dates">
            {#if data.subscription.startsAt}
              Started {fmtDate(data.subscription.startsAt)}
            {/if}
            {#if data.subscription.endsAt}
              <span aria-hidden="true"> · </span>
              Ends {fmtDate(data.subscription.endsAt)}
            {:else if data.subscription.dataCoverageEndsAt}
              <span aria-hidden="true"> · </span>
              Active through <strong>{fmtDate(data.subscription.dataCoverageEndsAt)}</strong>
            {/if}
          </p>
          <div class="bar" aria-hidden="true"></div>
        </div>
      {/if}
    </section>

    <section id="billing" class="block">
      <h2>Bills &amp; payments</h2>
      <p class="block-lede">Your VisionRoute invoices and payments on record.</p>

      {#if data.invoices.length === 0}
        <div class="empty">
          <h3>No bills yet</h3>
          <p>When VisionRoute issues an invoice, it will appear here.</p>
        </div>
      {:else}
        <div class="records">
          {#each data.invoices as invoice}
            <article class="record">
              <h3>{invoice.number}</h3>
              <dl>
                <div><dt>Description</dt><dd>{invoice.description}</dd></div>
                <div><dt>Amount</dt><dd>{fmtMoney(invoice.amountCents, invoice.currency)}</dd></div>
                <div>
                  <dt>Status</dt>
                  <dd
                    ><StatusPill
                      tone={statusTone(invoice.status)}
                      label={humanStatus(invoice.status)}
                    /></dd
                  >
                </div>
                <div><dt>Due</dt><dd>{fmtDate(invoice.dueDate)}</dd></div>
              </dl>
            </article>
          {/each}
        </div>
      {/if}

      {#if data.payments.length > 0}
        <h3 class="subhead">Payments received</h3>
        <div class="records">
          {#each data.payments as payment}
            <article class="record quiet">
              <h3>{fmtMoney(payment.amountCents, payment.currency)}</h3>
              <dl>
                <div><dt>Date</dt><dd>{fmtDate(payment.paidAt)}</dd></div>
                <div><dt>Method</dt><dd>{humanStatus(payment.method)}</dd></div>
                {#if payment.reference}
                  <div><dt>Reference</dt><dd>{payment.reference}</dd></div>
                {/if}
              </dl>
            </article>
          {/each}
        </div>
      {/if}
    </section>

    <section id="devices" class="block">
      <h2>GPS units</h2>
      <p class="block-lede">How many units you have, their models, and where they are attached.</p>
      {#if data.devices.length === 0}
        <div class="empty">
          <h3>No GPS units yet</h3>
          <p>Units appear here after VisionRoute installs and activates them on your account.</p>
        </div>
      {:else}
        <div class="records">
          {#each data.devices as device}
            <article class="record">
              <h3>{device.name}</h3>
              <dl>
                <div>
                  <dt>Model</dt>
                  <dd>{device.provider ? device.provider : 'VisionRoute GPS'}</dd>
                </div>
                <div>
                  <dt>Attached to</dt>
                  <dd>{device.plateNumber ? device.plateNumber : 'Not assigned yet'}</dd>
                </div>
                {#if device.imei}
                  <div><dt>Unit ID</dt><dd>{maskId(device.imei, 4)}</dd></div>
                {/if}
                <div>
                  <dt>Status</dt>
                  <dd
                    ><StatusPill
                      tone={statusTone(device.status)}
                      label={humanStatus(device.status)}
                    /></dd
                  >
                </div>
              </dl>
            </article>
          {/each}
        </div>
      {/if}
    </section>

    <section id="privacy" class="block">
      <h2>Privacy</h2>
      <p class="block-lede">
        Request access, correction, or deletion of your personal data. See the full
        <a href="/privacy">privacy notice</a>.
      </p>
      {#if form?.privacySuccess}
        <Alert tone="success" title="Privacy request submitted">
          <p>VisionRoute will follow up using your email.</p>
        </Alert>
      {/if}
      <form method="POST" action="?/privacyRequest" class="ticket-form">
        <input type="hidden" name="requesterEmail" value={data.user.email} />
        <input type="hidden" name="requesterName" value={data.user.fullName} />
        <label class="select-field">
          <span>Request type</span>
          <select name="requestType" required>
            <option value="access">Access my data</option>
            <option value="rectification">Correct my data</option>
            <option value="erasure">Delete my data</option>
            <option value="portability">Export my data</option>
            <option value="other">Other</option>
          </select>
        </label>
        <Field label="Details" name="details" rows={3} value={form?.details ?? ''} />
        {#if form?.privacyError}
          <Alert tone="danger" title="Could not submit"><p>{form.privacyError}</p></Alert>
        {/if}
        <Button variant="secondary">Submit privacy request</Button>
      </form>
    </section>
  {/if}
</main>

<style>
  .portal {
    width: 100%;
    max-width: var(--max-content);
    margin: 0 auto;
    padding: var(--space-10) clamp(1rem, 3vw, 2.5rem) var(--space-16);
    box-sizing: border-box;
    overflow-x: clip;
  }
  .portal-nav {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-3) var(--space-5);
    margin-bottom: var(--space-8);
    font-size: var(--text-compact);
    font-weight: 500;
    max-width: 100%;
  }
  .portal-nav a {
    text-decoration: none;
    color: var(--color-text-secondary);
  }
  .portal-head {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    gap: var(--space-5);
    margin-bottom: var(--space-6);
    min-width: 0;
  }
  .portal-head > div:first-child {
    min-width: 0;
    flex: 1 1 12rem;
  }
  .portal-head h1 {
    font-size: var(--text-page);
    overflow-wrap: anywhere;
  }
  .sub {
    margin: var(--space-2) 0 0;
    color: var(--color-text-secondary);
    overflow-wrap: anywhere;
  }
  .head-actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-3);
    align-items: center;
  }
  .summary {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-panel);
    padding: var(--space-6);
    margin-bottom: var(--space-10);
    min-width: 0;
  }
  .summary-label {
    margin: 0 0 var(--space-4);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    font-size: var(--text-meta);
    font-weight: 600;
    color: var(--color-text-muted);
  }
  .summary-metrics {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: var(--space-4);
  }
  .summary-metrics span {
    display: block;
    color: var(--color-text-muted);
    font-size: var(--text-meta);
  }
  .summary-metrics strong {
    font-family: var(--font-display);
    font-size: 1.35rem;
  }
  .block {
    margin-bottom: var(--space-12);
    min-width: 0;
  }
  .block h2 {
    font-size: var(--text-section);
    margin-bottom: var(--space-2);
  }
  .block-lede {
    margin: 0 0 var(--space-5);
    color: var(--color-text-secondary);
    font-size: var(--text-compact);
    max-width: 40rem;
  }
  .subhead {
    font-size: var(--text-panel);
    margin: var(--space-8) 0 var(--space-4);
  }
  .record.quiet {
    background: var(--color-surface-raised);
  }
  .ticket-form {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-panel);
    padding: var(--space-6);
    display: grid;
    gap: var(--space-4);
    max-width: 28rem;
  }
  .select-field {
    display: grid;
    gap: var(--space-2);
    font-size: var(--text-compact);
  }
  .select-field span {
    font-weight: 500;
  }
  .select-field select {
    min-height: 2.75rem;
    padding: 0.55rem 0.75rem;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-control);
    font: inherit;
    background: var(--color-surface);
  }
  .plan-row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-3);
    margin-bottom: var(--space-3);
  }
  .plan-badges {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
  }
  .empty {
    padding: var(--space-6);
    border: 1px dashed var(--color-border-strong);
    border-radius: var(--radius-panel);
    background: var(--color-surface-raised);
  }
  .empty h3 {
    font-size: var(--text-panel);
    margin-bottom: var(--space-2);
  }
  .empty p {
    margin: 0;
    color: var(--color-text-secondary);
  }
  .records {
    display: grid;
    gap: var(--space-3);
  }
  .record {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-panel);
    padding: var(--space-5);
    min-width: 0;
  }
  .record h3 {
    font-size: var(--text-panel);
    margin-bottom: var(--space-3);
    overflow-wrap: anywhere;
  }
  dl {
    margin: 0;
    display: grid;
    gap: var(--space-2);
  }
  dl div {
    display: flex;
    gap: var(--space-3);
    align-items: center;
    font-size: var(--text-compact);
    min-width: 0;
  }
  dt {
    color: var(--color-text-muted);
    min-width: 4.5rem;
    flex-shrink: 0;
  }
  dd {
    margin: 0;
    min-width: 0;
    overflow-wrap: anywhere;
  }
  .coverage-panel {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-panel);
    padding: var(--space-6);
  }
  .plan {
    margin: 0;
    font-family: var(--font-display);
    font-weight: 600;
  }
  .dates {
    margin: 0 0 var(--space-4);
    color: var(--color-text-secondary);
  }
  .bar {
    height: 4px;
    border-radius: var(--radius-pill);
    background: linear-gradient(90deg, var(--color-brand), var(--color-accent));
  }

  @media (max-width: 860px) {
    .portal {
      padding: var(--space-5) 1rem var(--space-12);
    }
    .portal-nav {
      position: sticky;
      top: 0;
      z-index: 20;
      margin: 0 0 var(--space-5);
      padding: var(--space-2) 0;
      background: color-mix(in srgb, var(--color-canvas) 94%, transparent);
      backdrop-filter: blur(8px);
      flex-wrap: nowrap;
      overflow-x: auto;
      overscroll-behavior-x: contain;
      -webkit-overflow-scrolling: touch;
      scrollbar-width: none;
      gap: var(--space-2);
      max-width: 100%;
    }
    .portal-nav::-webkit-scrollbar {
      display: none;
    }
    .portal-nav a {
      flex: 0 0 auto;
      min-height: 2.4rem;
      padding: 0.4rem 0.75rem;
      border: 1px solid var(--color-border);
      border-radius: var(--radius-pill);
      background: var(--color-surface);
      white-space: nowrap;
    }
    .portal-head {
      flex-direction: column;
      align-items: stretch;
      gap: var(--space-4);
    }
    .summary-metrics {
      grid-template-columns: repeat(3, minmax(0, 1fr));
      gap: var(--space-3);
    }
    .summary-metrics strong {
      font-size: 1.15rem;
    }
  }

  @media (max-width: 480px) {
    .portal {
      padding-inline: 0.875rem;
    }
    .portal-head h1 {
      font-size: 1.5rem;
    }
    .summary {
      padding: var(--space-4);
    }
    .summary-metrics {
      grid-template-columns: 1fr;
      gap: var(--space-3);
    }
    .summary-metrics > div {
      display: flex;
      justify-content: space-between;
      align-items: baseline;
      gap: var(--space-3);
      padding-bottom: var(--space-2);
      border-bottom: 1px solid var(--color-border);
    }
    .summary-metrics > div:last-child {
      border-bottom: 0;
      padding-bottom: 0;
    }
    .summary-metrics span {
      margin: 0;
    }
    dl div {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--space-1);
    }
    dt {
      min-width: 0;
    }
  }

  @media (min-width: 560px) and (max-width: 860px) {
    .summary-metrics {
      grid-template-columns: repeat(3, minmax(0, 1fr));
    }
    .summary-metrics > div {
      display: block;
      border-bottom: 0;
      padding-bottom: 0;
    }
  }
</style>
