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
    hint = undefined,
    revealable = false
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
    /** When true (or type is password), show a Show/Hide control. */
    revealable?: boolean;
  } = $props();

  const id = $derived(`field-${name}`);
  const canReveal = $derived(revealable || type === 'password');
  let revealed = $state(false);

  function toggleReveal(event: MouseEvent) {
    event.preventDefault();
    event.stopPropagation();
    revealed = !revealed;
  }
</script>

<div class="field">
  <label class="label" for={id}>{label}</label>
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
  {:else if canReveal}
    <div class="control">
      <input
        {id}
        {name}
        type={revealed ? 'text' : 'password'}
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
      <button
        type="button"
        class="reveal"
        aria-pressed={revealed}
        aria-controls={id}
        aria-label={revealed ? 'Hide password' : 'Show password'}
        onclick={toggleReveal}
      >
        {revealed ? 'Hide' : 'Show'}
      </button>
    </div>
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
</div>

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
  .control {
    position: relative;
    display: grid;
  }
  .control input {
    padding-right: 4.25rem;
  }
  .reveal {
    position: absolute;
    top: 50%;
    right: 0.4rem;
    transform: translateY(-50%);
    z-index: 1;
    border: 0;
    background: transparent;
    color: var(--color-text-secondary);
    font: inherit;
    font-size: var(--text-meta);
    font-weight: 600;
    padding: 0.35rem 0.55rem;
    border-radius: var(--radius-control);
    cursor: pointer;
  }
  .reveal:hover {
    color: var(--color-text);
  }
  .reveal:focus-visible {
    outline: none;
    box-shadow: var(--focus-ring);
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
