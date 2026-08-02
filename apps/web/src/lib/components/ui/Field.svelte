<script lang="ts">
  let {
    label,
    name,
    type = 'text',
    value = '',
    required = false,
    autocomplete = undefined,
    minlength = undefined,
    min = undefined,
    step = undefined,
    rows = undefined,
    error = undefined,
    hint = undefined
  }: {
    label: string;
    name: string;
    type?: string;
    value?: string | number;
    required?: boolean;
    autocomplete?: AutoFill | null;
    minlength?: number;
    min?: number | string;
    step?: number | string;
    rows?: number;
    error?: string;
    hint?: string;
  } = $props();

  const id = $derived(`field-${name}`);
</script>

<label class="field" for={id}>
  <span class="label">{label}</span>
  {#if rows}
    <textarea
      {id}
      {name}
      {required}
      {rows}
      class:invalid={!!error}
      aria-invalid={error ? 'true' : undefined}
      aria-describedby={error ? `${id}-error` : hint ? `${id}-hint` : undefined}
    >{value}</textarea>
  {:else}
    <input
      {id}
      {name}
      {type}
      {required}
      {autocomplete}
      {minlength}
      {min}
      {step}
      {value}
      class:invalid={!!error}
      aria-invalid={error ? 'true' : undefined}
      aria-describedby={error ? `${id}-error` : hint ? `${id}-hint` : undefined}
    />
  {/if}
  {#if hint && !error}
    <span class="hint" id="{id}-hint">{hint}</span>
  {/if}
  {#if error}
    <span class="error" id="{id}-error" role="alert">{error}</span>
  {/if}
</label>

<style>
  .field {
    display: grid;
    gap: var(--space-2);
    font-size: var(--text-compact);
  }
  .label {
    font-weight: 500;
    color: var(--color-text);
  }
  input,
  textarea {
    width: 100%;
    min-height: 2.75rem;
    padding: 0.65rem 0.85rem;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-control);
    background: var(--color-surface);
    font: inherit;
    color: var(--color-text);
  }
  textarea {
    min-height: 6rem;
    resize: vertical;
  }
  input:focus-visible,
  textarea:focus-visible {
    border-color: var(--color-info);
    box-shadow: var(--focus-ring);
    outline: none;
  }
  .invalid {
    border-color: var(--color-danger);
  }
  .hint {
    color: var(--color-text-muted);
    font-size: var(--text-meta);
  }
  .error {
    color: var(--color-danger);
    font-size: var(--text-meta);
  }
</style>
