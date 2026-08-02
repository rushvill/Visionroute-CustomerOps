<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    tone = 'info',
    title,
    children
  }: {
    tone?: 'success' | 'warning' | 'danger' | 'info';
    title: string;
    children?: Snippet;
  } = $props();
</script>

<div class="alert {tone} ops-feedback" role="status">
  <p class="title">{title}</p>
  {#if children}
    <div class="body">{@render children()}</div>
  {/if}
</div>

<style>
  .alert {
    padding: var(--space-4) var(--space-5);
    border-radius: var(--radius-panel);
    border: 1px solid var(--color-border);
  }
  .title {
    margin: 0;
    font-weight: 600;
    font-family: var(--font-display);
    font-size: var(--text-compact);
  }
  .body {
    margin-top: var(--space-2);
    color: var(--color-text-secondary);
    font-size: var(--text-compact);
  }
  .success {
    background: var(--color-success-soft);
    border-color: color-mix(in srgb, var(--color-success) 25%, var(--color-border));
  }
  .warning {
    background: var(--color-warning-soft);
    border-color: color-mix(in srgb, var(--color-warning) 25%, var(--color-border));
  }
  .danger {
    background: var(--color-danger-soft);
    border-color: color-mix(in srgb, var(--color-danger) 25%, var(--color-border));
  }
  .info {
    background: var(--color-info-soft);
    border-color: color-mix(in srgb, var(--color-info) 25%, var(--color-border));
  }
</style>
