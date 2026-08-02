<script lang="ts">
  import Button from '$lib/components/ui/Button.svelte';

  let { data } = $props();
</script>

<main id="main">
  <section class="hero" aria-label="VisionRoute">
    <div class="hero-inner">
      <div class="hero-copy motion-rise">
        <p class="eyebrow">Vehicle &amp; fleet tracking</p>
        <h1>Track your vehicles with VisionRoute.</h1>
        <p class="lede">
          For individuals watching a few vehicles, and businesses managing a full fleet. Sign up —
          our team will call you, set everything up, and get you tracking.
        </p>
        <div class="cta">
          <Button href="/signup" variant="accent">Sign up</Button>
          <Button href="/login" variant="secondary">Customer sign in</Button>
        </div>
      </div>
      <div class="hero-visual" aria-hidden="true">
        <div class="atmosphere"></div>
        <svg class="route-art" viewBox="0 0 320 280" fill="none">
          <path
            class="route-line"
            d="M20 220 C70 80, 120 80, 160 150 S250 260, 300 60"
            stroke="rgba(232,154,36,0.9)"
            stroke-width="3"
            stroke-linecap="round"
          />
          <circle cx="300" cy="60" r="7" fill="#E89A24" />
        </svg>
      </div>
    </div>
  </section>

  <section id="how" class="section how">
    <div class="wrap motion-rise-delay">
      <h2>How it works</h2>
      <div class="columns">
        <article>
          <h3>1. Sign up</h3>
          <p>Tell us who you are and how many vehicles you want to track. It only takes a minute.</p>
        </article>
        <article>
          <h3>2. We call you</h3>
          <p>VisionRoute reviews your request, contacts you, and confirms the right setup.</p>
        </article>
        <article>
          <h3>3. Start tracking</h3>
          <p>Once your account is ready, sign in to see your plan, promo, and GPS units.</p>
        </article>
      </div>
    </div>
  </section>

  <section class="section preview">
    <div class="wrap motion-rise-delay">
      <h2>Built for every fleet size</h2>
      <p class="section-lede">
        Personal vehicles or business fleets — one VisionRoute service to track what matters.
      </p>
      <div class="preview-frame" role="img" aria-label="Who VisionRoute is for">
        <div class="preview-grid two">
          <div>
            <span>Individuals</span>
            <strong>Check your vehicles</strong>
          </div>
          <div>
            <span>Businesses</span>
            <strong>Manage your fleet</strong>
          </div>
        </div>
      </div>
    </div>
  </section>

  <section class="section cta-band">
    <div class="wrap band-inner">
      <h2>Ready to track with VisionRoute?</h2>
      <div class="cta">
        <Button href="/signup" variant="accent">Sign up</Button>
        <Button href="/login" variant="secondary">Already a customer? Sign in</Button>
      </div>
      {#if data.user}
        <p class="signed">Signed in as {data.user.fullName}</p>
      {/if}
    </div>
  </section>
</main>

<style>
  .hero {
    background:
      radial-gradient(ellipse at 70% 20%, rgba(232, 154, 36, 0.18), transparent 45%),
      linear-gradient(135deg, #092a43 0%, #123b5d 55%, #0b334f 100%);
    color: #fff;
    padding: 7.5rem 0 5rem;
    overflow: hidden;
  }
  .hero-inner,
  .wrap {
    max-width: var(--max-content);
    margin: 0 auto;
    padding-inline: clamp(1rem, 3vw, 2.5rem);
  }
  .hero-inner {
    display: grid;
    grid-template-columns: 1.1fr 0.9fr;
    gap: var(--space-10);
    align-items: center;
  }
  .eyebrow {
    margin: 0 0 var(--space-4);
    letter-spacing: 0.1em;
    text-transform: uppercase;
    font-size: var(--text-meta);
    font-weight: 600;
    color: rgba(255, 241, 216, 0.9);
  }
  h1 {
    font-size: var(--text-hero);
    font-weight: 700;
    max-width: 16ch;
  }
  .lede {
    margin: var(--space-5) 0 var(--space-8);
    max-width: 36rem;
    font-size: 1.125rem;
    line-height: 1.6;
    color: rgba(255, 255, 255, 0.86);
  }
  .cta {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-3);
  }
  .hero :global(.secondary) {
    background: transparent;
    color: #fff;
    border-color: rgba(255, 255, 255, 0.45);
  }
  .hero-visual {
    position: relative;
    min-height: 16rem;
  }
  .atmosphere {
    position: absolute;
    inset: 10% 5% 5% 15%;
    border-radius: 40% 60% 45% 55%;
    background:
      linear-gradient(160deg, rgba(255, 255, 255, 0.08), transparent 50%),
      repeating-linear-gradient(
        -18deg,
        transparent,
        transparent 14px,
        rgba(255, 255, 255, 0.04) 14px,
        rgba(255, 255, 255, 0.04) 15px
      );
    border: 1px solid rgba(255, 255, 255, 0.1);
  }
  .route-art {
    position: relative;
    width: min(100%, 22rem);
    margin-left: auto;
  }
  .section {
    padding: var(--space-20) 0;
  }
  .how {
    background: var(--color-canvas);
  }
  .how h2,
  .preview h2,
  .cta-band h2 {
    font-size: var(--text-section);
    margin-bottom: var(--space-8);
  }
  .columns {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-8);
  }
  .columns h3 {
    font-size: var(--text-panel);
    margin-bottom: var(--space-3);
  }
  .columns p {
    margin: 0;
    color: var(--color-text-secondary);
  }
  .preview {
    background: var(--color-surface);
    border-block: 1px solid var(--color-border);
  }
  .section-lede {
    margin: calc(var(--space-8) * -0.5) 0 var(--space-8);
    color: var(--color-text-secondary);
    max-width: 36rem;
  }
  .preview-frame {
    border: 1px solid var(--color-border);
    border-radius: var(--radius-panel);
    background: var(--color-surface-raised);
    padding: var(--space-6);
    box-shadow: var(--shadow-raised);
  }
  .preview-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: var(--space-4);
  }
  .preview-grid.two {
    grid-template-columns: 1fr 1fr;
  }
  .preview-grid span {
    display: block;
    color: var(--color-text-muted);
    font-size: var(--text-meta);
    margin-bottom: var(--space-1);
  }
  .preview-grid strong {
    font-family: var(--font-display);
  }
  .cta-band {
    background: var(--color-brand-strong);
    color: #fff;
  }
  .band-inner {
    display: grid;
    gap: var(--space-6);
  }
  .cta-band :global(.secondary) {
    background: transparent;
    color: #fff;
    border-color: rgba(255, 255, 255, 0.4);
  }
  .signed {
    margin: 0;
    color: rgba(255, 255, 255, 0.7);
    font-size: var(--text-compact);
  }

  @media (max-width: 900px) {
    .hero {
      padding: 5.5rem 0 3rem;
    }
    .hero-inner,
    .columns,
    .preview-grid,
    .preview-grid.two {
      grid-template-columns: 1fr;
    }
    .hero-visual {
      min-height: 7.5rem;
      order: -1;
    }
    .route-art {
      margin-inline: auto;
      width: min(100%, 16rem);
    }
    h1 {
      max-width: none;
      font-size: clamp(1.85rem, 7vw, 2.5rem);
    }
    .section {
      padding: var(--space-12) 0;
    }
    .cta {
      flex-direction: row;
      flex-wrap: wrap;
      gap: 0.65rem;
    }
    .cta :global(.btn) {
      flex: 0 1 auto;
      width: auto;
      min-height: 2.5rem;
      padding: 0.5rem 1rem;
      font-size: 0.875rem;
    }
  }

  @media (max-width: 480px) {
    .hero {
      padding-top: 4.75rem;
      padding-bottom: 2.5rem;
    }
    .lede {
      font-size: 1rem;
      margin-bottom: var(--space-5);
    }
  }

  @media (min-width: 560px) and (max-width: 900px) {
    .preview-grid.two {
      grid-template-columns: 1fr 1fr;
    }
  }
</style>
