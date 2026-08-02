<script lang="ts">
  import { page } from '$app/state';
  import BrandMark from '$lib/components/BrandMark.svelte';

  let { data, children } = $props();

  let menuOpen = $state(false);

  const links = [
    { href: '/admin#overview', label: 'Overview' },
    { href: '/admin#signups', label: 'Signup Inbox' },
    { href: '/admin#accounts', label: 'Accounts' },
    { href: '/admin#billing', label: 'Billing' },
    { href: '/admin#subscriptions', label: 'Subscriptions' },
    { href: '/admin#sims', label: 'SIM Inventory' },
    { href: '/admin#sim-costs', label: 'SIM Data Costs' },
    { href: '/admin#coverage', label: 'Coverage' },
    { href: '/admin#tickets', label: 'Tickets' },
    { href: '/admin#users', label: 'Users' },
    { href: '/admin#audit', label: 'Audit' },
    { href: '/admin#privacy', label: 'Privacy' }
  ];

  const hash = $derived(page.url.hash || '#overview');
</script>

<div class="ops" class:menu-open={menuOpen}>
  <aside id="ops-sidebar" class="sidebar" aria-label="Customer Operations">
    <div class="side-brand">
      <BrandMark light compact />
      <p class="ops-label">Customer Operations</p>
    </div>
    <nav class="side-nav">
      {#each links as link}
        <a
          href={link.href}
          class:active={hash === `#${link.href.split('#')[1]}`}
          onclick={() => (menuOpen = false)}
        >
          {link.label}
        </a>
      {/each}
    </nav>
    <div class="side-foot">
      <p class="who">Signed in as {data.user?.fullName ?? data.user?.username ?? 'Admin'}</p>
      <form method="POST" action="/?/logout">
        <button type="submit">Log out</button>
      </form>
      <a class="portal-link" href="/">Public site</a>
    </div>
  </aside>

  <div class="workspace">
    <header class="topbar">
      <button
        type="button"
        class="menu-btn"
        onclick={() => (menuOpen = !menuOpen)}
        aria-expanded={menuOpen}
        aria-controls="ops-sidebar"
      >
        Menu
      </button>
      <p class="top-title">VisionRoute · Customer Operations</p>
    </header>
    <div class="backdrop" onclick={() => (menuOpen = false)} role="presentation"></div>
    {@render children()}
  </div>
</div>

<style>
  .ops {
    min-height: 100vh;
    display: grid;
    grid-template-columns: var(--sidebar-width) 1fr;
    background: var(--color-canvas);
  }
  .sidebar {
    background: var(--color-brand-strong);
    color: #fff;
    display: flex;
    flex-direction: column;
    padding: var(--space-6) var(--space-5);
    position: sticky;
    top: 0;
    height: 100vh;
  }
  .side-brand {
    margin-bottom: var(--space-8);
  }
  .ops-label {
    margin: var(--space-2) 0 0;
    font-size: var(--text-meta);
    color: rgba(255, 255, 255, 0.7);
  }
  .side-nav {
    display: grid;
    gap: 0.25rem;
    flex: 1;
  }
  .side-nav a {
    color: rgba(255, 255, 255, 0.82);
    text-decoration: none;
    padding: 0.55rem 0.7rem;
    border-radius: var(--radius-control);
    font-size: var(--text-compact);
    font-weight: 500;
    min-height: 2.75rem;
    display: flex;
    align-items: center;
  }
  .side-nav a:hover,
  .side-nav a.active {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
  }
  .side-foot {
    border-top: 1px solid rgba(255, 255, 255, 0.12);
    padding-top: var(--space-4);
    font-size: var(--text-meta);
  }
  .who {
    margin: 0 0 var(--space-3);
    color: rgba(255, 255, 255, 0.75);
  }
  .side-foot button,
  .portal-link {
    display: flex;
    align-items: center;
    width: 100%;
    text-align: left;
    border: 0;
    background: transparent;
    color: #fff;
    font: inherit;
    cursor: pointer;
    padding: 0.35rem 0;
    text-decoration: none;
    min-height: 2.75rem;
  }
  .workspace {
    min-width: 0;
  }
  .topbar {
    display: none;
    align-items: center;
    gap: var(--space-4);
    padding: var(--space-4) clamp(1rem, 3vw, 2rem);
    background: var(--color-surface);
    border-bottom: 1px solid var(--color-border);
  }
  .menu-btn {
    border: 1px solid var(--color-border);
    background: var(--color-surface);
    border-radius: var(--radius-control);
    min-height: 2.75rem;
    min-width: 2.75rem;
    padding: 0.45rem 0.85rem;
    font: inherit;
    font-weight: 500;
    cursor: pointer;
  }
  .top-title {
    margin: 0;
    font-family: var(--font-display);
    font-weight: 600;
    font-size: var(--text-compact);
    line-height: 1.3;
  }
  .backdrop {
    display: none;
  }

  @media (max-width: 900px) {
    .ops {
      grid-template-columns: 1fr;
    }
    .sidebar {
      position: fixed;
      inset: 0 auto 0 0;
      width: min(18rem, 88vw);
      z-index: 50;
      transform: translateX(-105%);
      transition: transform 0.2s ease;
      height: 100dvh;
    }
    .ops.menu-open .sidebar {
      transform: translateX(0);
    }
    .topbar {
      display: flex;
      position: sticky;
      top: 0;
      z-index: 30;
    }
    .ops.menu-open .backdrop {
      display: block;
      position: fixed;
      inset: 0;
      background: rgba(9, 42, 67, 0.35);
      z-index: 45;
    }
  }

  @media (max-width: 480px) {
    .top-title {
      font-size: var(--text-meta);
    }
  }
</style>
