<script lang="ts">
  import type { ActionData } from './$types';
  import Button from '$lib/components/ui/Button.svelte';
  import Field from '$lib/components/ui/Field.svelte';
  import Alert from '$lib/components/ui/Alert.svelte';

  let { form }: { form: ActionData } = $props();
</script>

<main id="main" class="auth">
  <aside class="brand-panel" aria-hidden="true">
    <div class="brand-copy">
      <p class="eyebrow">VisionRoute</p>
      <h1>Track your vehicles and fleet.</h1>
    </div>
    <div class="panel-texture"></div>
  </aside>

  <section class="form-panel">
    {#if form?.success}
      <Alert tone="success" title="Thanks — we received your signup.">
        <p>
          Our team will review your request and call you. When everything is set up, you’ll get
          customer access to see your plan and GPS units.
        </p>
        <p><a href="/">Back to home</a></p>
      </Alert>
    {:else}
      <header class="intro">
        <h1>Sign up</h1>
        <p>
          For individuals checking vehicles or businesses managing a fleet. We’ll call you to
          finish setup.
        </p>
      </header>

      <form method="POST" class="form">
        <fieldset>
          <legend>Contact</legend>
          <Field label="Full name" name="fullName" required value={form?.fullName ?? ''} />
          <Field
            label="Company or name"
            name="companyName"
            required
            value={form?.companyName ?? ''}
            hint="Use your business name, or your own name if this is personal."
          />
          <Field
            label="Email"
            name="email"
            type="email"
            autocomplete="email"
            required
            value={form?.email ?? ''}
          />
          <Field
            label="Phone"
            name="phone"
            type="tel"
            autocomplete="tel"
            value={form?.phone ?? ''}
            hint="So we can call you about your request."
          />
        </fieldset>

        <fieldset>
          <legend>What you want to track</legend>
          <Field
            label="Estimated vehicles / GPS units"
            name="estimatedDevices"
            type="number"
            min="0"
            step="1"
            value={form?.estimatedDevices ?? ''}
            hint="How many vehicles do you want to track?"
          />
          <Field
            label="Anything we should know?"
            name="message"
            rows={3}
            value={form?.message ?? ''}
          />
        </fieldset>

        {#if form?.error}
          <Alert tone="danger" title="Could not submit signup">
            <p>{form.error}</p>
          </Alert>
        {/if}

        <label class="consent">
          <input type="checkbox" name="privacyAccepted" value="true" required />
          <span>
            I have read and accept the
            <a href="/privacy" target="_blank" rel="noopener noreferrer">privacy notice</a>
            (version 2026-08-02).
          </span>
        </label>

        <Button variant="accent">Sign up</Button>
        <p class="alt">Already a customer? <a href="/login">Sign in</a></p>
      </form>
    {/if}
  </section>
</main>

<style>
  .auth {
    display: grid;
    grid-template-columns: 0.42fr 0.58fr;
    min-height: calc(100vh - 0px);
  }
  .brand-panel {
    position: relative;
    background: linear-gradient(160deg, #092a43, #123b5d 60%, #0d3a58);
    color: #fff;
    padding: 7rem clamp(1.25rem, 3vw, 2.5rem) var(--space-10);
    overflow: hidden;
  }
  .brand-copy {
    position: relative;
    z-index: 1;
    max-width: 22rem;
  }
  .eyebrow {
    text-transform: uppercase;
    letter-spacing: 0.1em;
    font-size: var(--text-meta);
    font-weight: 600;
    margin: 0 0 var(--space-4);
    color: var(--color-accent-soft);
  }
  .brand-panel h1 {
    font-size: clamp(1.75rem, 2.4vw, 2.35rem);
    font-weight: 700;
    line-height: 1.2;
  }
  .panel-texture {
    position: absolute;
    inset: 20% -10% -10% 20%;
    background: repeating-linear-gradient(
      -20deg,
      transparent,
      transparent 18px,
      rgba(255, 255, 255, 0.04) 18px,
      rgba(255, 255, 255, 0.04) 19px
    );
  }
  .form-panel {
    padding: 6.5rem clamp(1.25rem, 4vw, 3rem) var(--space-12);
    background: var(--color-canvas);
  }
  .intro {
    margin-bottom: var(--space-8);
    max-width: 28rem;
  }
  .intro h1 {
    font-size: var(--text-page);
    margin-bottom: var(--space-3);
  }
  .intro p {
    margin: 0;
    color: var(--color-text-secondary);
  }
  .form {
    display: grid;
    gap: var(--space-5);
    max-width: 28rem;
  }
  fieldset {
    margin: 0;
    padding: 0;
    border: 0;
    display: grid;
    gap: var(--space-4);
  }
  legend {
    font-family: var(--font-display);
    font-weight: 600;
    font-size: var(--text-compact);
    margin-bottom: var(--space-2);
    padding: 0;
  }
  .alt {
    margin: 0;
    font-size: var(--text-compact);
    color: var(--color-text-muted);
  }
  .consent {
    display: flex;
    gap: var(--space-3);
    align-items: flex-start;
    font-size: var(--text-compact);
    color: var(--color-text-secondary);
    line-height: 1.45;
  }
  .consent input {
    margin-top: 0.25rem;
    width: 1.1rem;
    height: 1.1rem;
    flex-shrink: 0;
  }
  .consent a {
    font-weight: 500;
  }

  @media (max-width: 860px) {
    .auth {
      grid-template-columns: 1fr;
      min-height: auto;
    }
    .brand-panel {
      padding: 5.5rem 1.25rem var(--space-8);
      min-height: auto;
    }
    .form-panel {
      padding: var(--space-8) 1.25rem var(--space-12);
    }
    .form,
    .intro {
      max-width: none;
    }
    .form :global(.btn) {
      width: 100%;
    }
  }

  @media (max-width: 480px) {
    .brand-panel {
      padding: 5rem 1rem var(--space-6);
    }
    .brand-panel h1 {
      font-size: 1.5rem;
    }
  }
</style>
