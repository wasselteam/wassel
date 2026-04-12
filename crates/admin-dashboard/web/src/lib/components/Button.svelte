<script module lang="ts">
</script>

<script lang="ts">
  let { label, onclick } = $props();
</script>

<button {onclick}>{label}</button>

<style>
  button {
    background: var(--color-primary-light);
    padding: 0 0.3em;
    border-radius: 0.3em;
    border: 1px solid gray;
  }
</style>
