<script lang="ts">
  import type { Snippet } from 'svelte';

  type Variant = 'primary' | 'accent' | 'secondary' | 'tertiary' | 'danger';

  let {
    variant = 'primary',
    type = 'submit',
    href = undefined,
    disabled = false,
    class: className = '',
    children
  }: {
    variant?: Variant;
    type?: 'button' | 'submit' | 'reset';
    href?: string;
    disabled?: boolean;
    class?: string;
    children: Snippet;
  } = $props();
</script>

{#if href}
  <a class="btn {variant} {className}" {href} aria-disabled={disabled || undefined}>
    {@render children()}
  </a>
{:else}
  <button class="btn {variant} {className}" {type} {disabled}>
    {@render children()}
  </button>
{/if}

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-2);
    min-height: 2.75rem;
    padding: 0.65rem 1.15rem;
    border-radius: var(--radius-control);
    font-family: var(--font-body);
    font-size: var(--text-compact);
    font-weight: 500;
    text-decoration: none;
    cursor: pointer;
    border: 1px solid transparent;
    transition: background 0.15s ease, border-color 0.15s ease, color 0.15s ease;
  }
  .btn:disabled,
  .btn[aria-disabled='true'] {
    opacity: 0.55;
    pointer-events: none;
  }
  .primary {
    background: var(--color-brand);
    color: #fff;
  }
  .primary:hover {
    background: var(--color-brand-strong);
    color: #fff;
  }
  .accent {
    background: var(--color-accent);
    color: #15202b;
  }
  .accent:hover {
    background: var(--color-accent-hover);
    color: #fff;
  }
  .secondary {
    background: var(--color-surface);
    color: var(--color-brand);
    border-color: var(--color-border-strong);
  }
  .secondary:hover {
    border-color: var(--color-brand);
    color: var(--color-brand-strong);
  }
  .tertiary {
    background: transparent;
    color: var(--color-brand);
    padding-inline: 0.5rem;
  }
  .danger {
    background: var(--color-surface);
    color: var(--color-danger);
    border-color: color-mix(in srgb, var(--color-danger) 45%, var(--color-border));
  }
  .danger:hover {
    background: var(--color-danger-soft);
  }
</style>
