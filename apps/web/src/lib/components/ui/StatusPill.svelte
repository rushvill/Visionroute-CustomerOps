<script lang="ts">
  let {
    tone = 'neutral',
    label
  }: {
    tone?: 'neutral' | 'success' | 'warning' | 'danger' | 'info';
    label: string;
  } = $props();
</script>

<span class="pill {tone}"><span class="dot" aria-hidden="true"></span>{label}</span>

<style>
  .pill {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.2rem 0.65rem;
    border-radius: var(--radius-pill);
    font-size: var(--text-meta);
    font-weight: 500;
    color: var(--color-text);
    background: var(--color-canvas-subtle);
  }
  .dot {
    width: 0.45rem;
    height: 0.45rem;
    border-radius: 50%;
    background: var(--color-text-muted);
  }
  .success {
    background: var(--color-success-soft);
  }
  .success .dot {
    background: var(--color-success);
  }
  .warning {
    background: var(--color-warning-soft);
  }
  .warning .dot {
    background: var(--color-warning);
  }
  .danger {
    background: var(--color-danger-soft);
  }
  .danger .dot {
    background: var(--color-danger);
  }
  .info {
    background: var(--color-info-soft);
  }
  .info .dot {
    background: var(--color-info);
  }
</style>
