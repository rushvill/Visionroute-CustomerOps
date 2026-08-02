<script lang="ts">
  let {
    eyebrow = undefined,
    title,
    description = undefined
  }: {
    eyebrow?: string;
    title: string;
    description?: string;
  } = $props();
</script>

<header class="page-header">
  {#if eyebrow}
    <p class="eyebrow">{eyebrow}</p>
  {/if}
  <h1>{title}</h1>
  {#if description}
    <p class="desc">{description}</p>
  {/if}
</header>

<style>
  .page-header {
    margin-bottom: var(--space-8);
  }
  .eyebrow {
    margin: 0 0 var(--space-2);
    font-size: var(--text-meta);
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--color-text-muted);
  }
  h1 {
    font-size: var(--text-page);
    color: var(--color-text);
  }
  .desc {
    margin: var(--space-3) 0 0;
    max-width: 40rem;
    color: var(--color-text-secondary);
  }
</style>
