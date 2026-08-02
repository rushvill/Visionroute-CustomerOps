<script lang="ts">
  import type { ActionData } from './$types';
  import Button from '$lib/components/ui/Button.svelte';
  import Field from '$lib/components/ui/Field.svelte';
  import Alert from '$lib/components/ui/Alert.svelte';

  let { form }: { form: ActionData } = $props();
</script>

<main id="main" class="privacy">
  <div class="wrap">
    <header class="intro">
      <p class="eyebrow">VisionRoute</p>
      <h1>Privacy notice</h1>
      <p>
        How we handle personal data in Customer Ops — aligned with common GDPR principles and the
        Philippines Data Privacy Act for small-business use.
      </p>
      <p class="meta">Notice version: 2026-08-02</p>
    </header>

    <section>
      <h2>What we collect</h2>
      <ul>
        <li>Contact details from signup (name, email, phone, company or personal name)</li>
        <li>Account login data (username; passwords are hashed)</li>
        <li>GPS unit details you see in the portal (labels, attach points, unit IDs)</li>
        <li>Subscription, promo, invoice, and payment records for your account</li>
        <li>Hashed technical signals on signup (to reduce abuse)</li>
      </ul>
    </section>

    <section>
      <h2>Why we use it</h2>
      <p>
        To set up and run VisionRoute service, contact you about your request, show your plan and
        GPS units, track bills and payments, and keep the system secure.
      </p>
    </section>

    <section>
      <h2>Cookies</h2>
      <p>
        We use an essential sign-in session cookie only. It is not used for advertising.
      </p>
    </section>

    <section>
      <h2>Your rights</h2>
      <p>
        You can ask to access, correct, or delete personal data (where the law allows), or raise
        another privacy request. Use the form below — VisionRoute staff will handle it.
      </p>
    </section>

    <section id="request" class="request">
      <h2>Submit a privacy request</h2>
      {#if form?.success}
        <Alert tone="success" title="Request received">
          <p>We logged your privacy request. VisionRoute will follow up using the email you provided.</p>
        </Alert>
      {:else}
        <form method="POST" class="form">
          <Field label="Your name" name="requesterName" value={form?.requesterName ?? ''} />
          <Field
            label="Email"
            name="requesterEmail"
            type="email"
            required
            autocomplete="email"
            value={form?.requesterEmail ?? ''}
          />
          <label class="select-field">
            <span>Request type</span>
            <select name="requestType" required>
              <option value="access">Access my data</option>
              <option value="rectification">Correct my data</option>
              <option value="erasure">Delete my data</option>
              <option value="portability">Export / portability</option>
              <option value="restriction">Restrict processing</option>
              <option value="objection">Object to processing</option>
              <option value="other">Other</option>
            </select>
          </label>
          <Field
            label="Details"
            name="details"
            rows={4}
            value={form?.details ?? ''}
            hint="Tell us what you need. Do not include passwords."
          />
          {#if form?.error}
            <Alert tone="danger" title="Could not submit"><p>{form.error}</p></Alert>
          {/if}
          <Button variant="primary">Submit request</Button>
        </form>
      {/if}
    </section>

    <p class="back"><a href="/">Back to home</a> · <a href="/signup">Sign up</a></p>
  </div>
</main>

<style>
  .privacy {
    padding: 6.5rem 0 var(--space-16);
  }
  .wrap {
    max-width: 42rem;
    margin: 0 auto;
    padding-inline: clamp(1rem, 3vw, 2.5rem);
  }
  .eyebrow {
    text-transform: uppercase;
    letter-spacing: 0.1em;
    font-size: var(--text-meta);
    font-weight: 600;
    color: var(--color-text-muted);
    margin: 0 0 var(--space-3);
  }
  .intro h1 {
    font-size: var(--text-page);
    margin-bottom: var(--space-3);
  }
  .intro p {
    color: var(--color-text-secondary);
  }
  .meta {
    font-size: var(--text-meta);
    color: var(--color-text-muted);
  }
  section {
    margin-top: var(--space-8);
  }
  h2 {
    font-size: var(--text-section);
    margin-bottom: var(--space-3);
  }
  ul {
    margin: 0;
    padding-left: 1.2rem;
    color: var(--color-text-secondary);
  }
  section p {
    margin: 0;
    color: var(--color-text-secondary);
  }
  .request {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-panel);
    padding: var(--space-6);
  }
  .form {
    display: grid;
    gap: var(--space-4);
  }
  .select-field {
    display: grid;
    gap: var(--space-2);
    font-size: var(--text-compact);
  }
  .select-field span {
    font-weight: 500;
  }
  select {
    min-height: 2.75rem;
    padding: 0.55rem 0.75rem;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-control);
    font: inherit;
    background: var(--color-surface);
  }
  .back {
    margin-top: var(--space-10);
    font-size: var(--text-compact);
  }
</style>
