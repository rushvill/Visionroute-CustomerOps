<script lang="ts">
  import '$lib/styles/app.css';
  import { page } from '$app/state';
  import BrandMark from '$lib/components/BrandMark.svelte';
  import Button from '$lib/components/ui/Button.svelte';

  let { data, children } = $props();

  let menuOpen = $state(false);

  const pathname = $derived(page.url.pathname);
  const isAdminRoute = $derived(pathname.startsWith('/admin'));
  const isHome = $derived(pathname === '/');
  const isAdmin = $derived(data.user?.role === 'admin');
  const showPortal = $derived(
    !!data.user && (data.user.role === 'customer' || data.user.accountId !== null)
  );

  $effect(() => {
    pathname;
    menuOpen = false;
  });
</script>

<svelte:head>
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
  <link
    href="https://fonts.googleapis.com/css2?family=Sora:wght@500;600;700&family=Source+Sans+3:wght@400;500;600&display=swap"
    rel="stylesheet"
  />
  <meta name="theme-color" content="#092A43" />
  <title>VisionRoute</title>
</svelte:head>

<a class="skip-link" href="#main">Skip to content</a>

{#if isAdminRoute}
  {@render children()}
{:else}
  <div class="public-shell" class:nav-open={menuOpen}>
    <header class="public-nav" class:over-hero={isHome}>
      <div class="nav-inner">
        <BrandMark light={isHome} />
        <button
          type="button"
          class="nav-toggle"
          aria-expanded={menuOpen}
          aria-controls="public-menu"
          onclick={() => (menuOpen = !menuOpen)}
        >
          {menuOpen ? 'Close' : 'Menu'}
        </button>
        <nav id="public-menu" aria-label="Main">
          <a href="/#how" onclick={() => (menuOpen = false)}>Services</a>
          {#if showPortal}
            <a href="/portal" onclick={() => (menuOpen = false)}>Customer Portal</a>
          {:else if !data.user}
            <a href="/login" onclick={() => (menuOpen = false)}>Customer Portal</a>
          {/if}
          {#if data.user}
            {#if isAdmin}
              <a href="/admin" onclick={() => (menuOpen = false)}>Operations</a>
            {/if}
            <form method="POST" action="/?/logout" class="logout">
              <button type="submit">Sign out</button>
            </form>
          {:else}
            <a href="/login" onclick={() => (menuOpen = false)}>Sign in</a>
            <Button href="/signup" variant="accent" class="nav-cta">Sign up</Button>
          {/if}
        </nav>
      </div>
    </header>
    {#if menuOpen}
      <div class="nav-backdrop" onclick={() => (menuOpen = false)} role="presentation"></div>
    {/if}
    {@render children()}
    <footer class="site-footer">
      <div class="footer-inner">
        <BrandMark compact />
        <p>Vehicle &amp; fleet tracking</p>
        <a class="privacy-link" href="/privacy">Privacy</a>
      </div>
    </footer>
  </div>
{/if}

<style>
  .public-shell {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    overflow-x: clip;
    width: 100%;
    max-width: 100%;
  }
  .public-nav {
    position: sticky;
    top: 0;
    z-index: 40;
    background: var(--color-surface);
    border-bottom: 1px solid var(--color-border);
  }
  .public-nav.over-hero {
    background: transparent;
    border-bottom-color: transparent;
    position: absolute;
    inset-inline: 0;
  }
  .public-nav.over-hero :global(a:not(.btn)),
  .public-nav.over-hero :global(.logout button),
  .public-nav.over-hero .nav-toggle {
    color: rgba(255, 255, 255, 0.92);
  }
  .nav-inner {
    max-width: var(--max-content);
    margin: 0 auto;
    padding: 0.9rem clamp(1rem, 3vw, 2.5rem);
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
    position: relative;
  }
  .nav-toggle {
    display: none;
    min-height: 2.75rem;
    min-width: 2.75rem;
    padding: 0.45rem 0.85rem;
    border: 1px solid currentColor;
    border-radius: var(--radius-control);
    background: transparent;
    font: inherit;
    font-weight: 500;
    font-size: var(--text-compact);
    cursor: pointer;
    color: var(--color-text-secondary);
  }
  nav {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.85rem 1.15rem;
    font-size: var(--text-compact);
    font-weight: 500;
  }
  nav a {
    text-decoration: none;
    color: var(--color-text-secondary);
    min-height: 2.75rem;
    display: inline-flex;
    align-items: center;
  }
  nav a:hover {
    color: var(--color-brand-strong);
  }
  .logout {
    margin: 0;
  }
  .logout button {
    border: 0;
    background: transparent;
    font: inherit;
    font-weight: 500;
    color: var(--color-text-secondary);
    cursor: pointer;
    padding: 0;
    min-height: 2.75rem;
  }
  .nav-backdrop {
    display: none;
  }
  .site-footer {
    border-top: 1px solid var(--color-border);
    background: var(--color-surface);
    margin-top: auto;
  }
  .footer-inner {
    max-width: var(--max-content);
    margin: 0 auto;
    padding: var(--space-8) clamp(1rem, 3vw, 2.5rem);
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-4);
    align-items: center;
    justify-content: space-between;
  }
  .footer-inner p {
    margin: 0;
    color: var(--color-text-muted);
    font-size: var(--text-compact);
  }
  .privacy-link {
    font-size: var(--text-compact);
    font-weight: 500;
    text-decoration: none;
    color: var(--color-text-secondary);
  }
  .privacy-link:hover {
    color: var(--color-brand-strong);
  }

  @media (max-width: 720px) {
    .nav-toggle {
      display: inline-flex;
      align-items: center;
      justify-content: center;
    }
    nav {
      display: none;
      position: absolute;
      top: calc(100% + 0.35rem);
      right: clamp(1rem, 3vw, 2.5rem);
      left: clamp(1rem, 3vw, 2.5rem);
      flex-direction: column;
      align-items: stretch;
      gap: 0.25rem;
      padding: var(--space-3);
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-panel);
      box-shadow: var(--shadow-raised);
      z-index: 50;
    }
    .public-shell.nav-open nav {
      display: flex;
    }
    .public-nav.over-hero nav :global(a:not(.btn)),
    .public-nav.over-hero nav :global(.logout button) {
      color: var(--color-text-secondary);
    }
    nav a,
    .logout button {
      padding: 0.65rem 0.85rem;
      border-radius: var(--radius-control);
      width: 100%;
      justify-content: flex-start;
      text-align: left;
    }
    nav :global(.nav-cta) {
      width: 100%;
      margin-top: var(--space-2);
    }
    .nav-backdrop {
      display: block;
      position: fixed;
      inset: 0;
      background: rgba(9, 42, 67, 0.28);
      z-index: 35;
    }
  }
</style>
