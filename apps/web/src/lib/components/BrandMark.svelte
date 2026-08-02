<script lang="ts">
  let {
    light = false,
    compact = false
  }: {
    light?: boolean;
    compact?: boolean;
  } = $props();
</script>

<a class="brand" class:light class:compact href="/" aria-label="VisionRoute home">
  <svg class="mark" viewBox="0 0 40 40" aria-hidden="true">
    <path
      class="route-line"
      d="M6 28 C12 12, 18 12, 22 20 S32 34, 34 12"
      fill="none"
      stroke="currentColor"
      stroke-width="2.2"
      stroke-linecap="round"
    />
    <circle cx="34" cy="12" r="2.4" fill="currentColor" />
  </svg>
  <span class="word">VisionRoute</span>
</a>

<style>
  .brand {
    display: inline-flex;
    align-items: center;
    gap: 0.55rem;
    text-decoration: none;
    color: var(--color-brand-strong);
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 1.05rem;
  }
  .brand.light {
    color: #fff;
  }
  .mark {
    width: 1.65rem;
    height: 1.65rem;
    flex-shrink: 0;
  }
  .compact .word {
    font-size: 0.95rem;
  }
</style>
