<script module lang="ts">
  export type SearchInputProps = {
    value: string;
  };
</script>

<script lang="ts">
  import Search from '@lucide/svelte/icons/search';

  let { value = $bindable() }: SearchInputProps = $props();
</script>

<div class="plugin-search">
  <Search />
  <input type="search" placeholder="Search..." bind:value />
</div>

<style>
  .plugin-search {
    background: white;
    padding: 0.1em;
    border-radius: 0.3em;
    vertical-align: middle;
    display: flex;
    gap: 0.3em;
  }
</style>
