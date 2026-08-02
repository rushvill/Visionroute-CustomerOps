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
      <h1>Track your vehicles.</h1>
      <p>Sign in to see your subscription, promo, and GPS units.</p>
    </div>
  </aside>

  <section class="form-panel">
    <header class="intro">
      <h1>Welcome back</h1>
      <p>Sign in to your VisionRoute customer account.</p>
    </header>

    <form method="POST" class="form">
      <Field
        label="Username or email"
        name="usernameOrEmail"
        autocomplete="username"
        required
        value={form?.usernameOrEmail ?? ''}
      />
      <Field
        label="Password"
        name="password"
        type="password"
        autocomplete="current-password"
        required
      />

      {#if form?.error}
        <Alert tone="danger" title="Sign in failed">
          <p>{form.error}</p>
        </Alert>
      {/if}

      <Button variant="primary">Sign in</Button>
      <p class="alt">New here? <a href="/signup">Sign up</a></p>
    </form>
  </section>
</main>

<style>
  .auth {
    display: grid;
    grid-template-columns: 0.42fr 0.58fr;
    min-height: 100vh;
  }
  .brand-panel {
    background: linear-gradient(160deg, #092a43, #123b5d);
    color: #fff;
    padding: 7rem clamp(1.25rem, 3vw, 2.5rem) var(--space-10);
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
    margin-bottom: var(--space-4);
  }
  .brand-panel p:last-child {
    margin: 0;
    color: rgba(255, 255, 255, 0.8);
    max-width: 22rem;
  }
  .form-panel {
    padding: 6.5rem clamp(1.25rem, 4vw, 3rem) var(--space-12);
    background: var(--color-canvas);
  }
  .intro {
    margin-bottom: var(--space-8);
    max-width: 26rem;
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
    gap: var(--space-4);
    max-width: 24rem;
  }
  .alt {
    margin: 0;
    font-size: var(--text-compact);
    color: var(--color-text-muted);
  }

  @media (max-width: 860px) {
    .auth {
      grid-template-columns: 1fr;
      min-height: auto;
    }
    .brand-panel {
      padding: 5.5rem 1.25rem var(--space-8);
    }
    .brand-panel p:last-child {
      display: none;
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
