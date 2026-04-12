<script module lang="ts">
  export type DropdownProps = {
    variants: string[];
    value: string;
    width?: string;
  };
</script>

<script lang="ts">
  import ListChevronsUpDown from '@lucide/svelte/icons/list-chevrons-up-down';
  let { variants, width = '100px', value = $bindable() }: DropdownProps = $props();
</script>

<div class="content" style="width: {width};">
  <select bind:value>
    {#each variants as variant}
      <option value={variant}>{variant}</option>
    {/each}
  </select>
  <ListChevronsUpDown size="1.5em" />
</div>

<style>
  .content {
    width: fit-content;
    display: flex;
    flex-direction: row;
    gap: 0.15em;
    border: 1px solid gray;
    border-radius: 0.3em;
    text-align: center;
  }

  select {
    background: var(--color-primary-light);
    padding: 0 0.3em;
    border-radius: 0.3em;
    width: 100%;
  }
</style>
