<script lang="ts">
  import Button from '$lib/components/Button.svelte';
  import Title from '$lib/components/Title.svelte';
  import { Badge } from '$lib/components';
  import { plugins } from '$lib/stores';
  import { SearchInput } from '$lib/components';
  import ClockArrowUp from '@lucide/svelte/icons/clock-arrow-up';

  function onRestart() {}

  function onReloadAll() {}
</script>

<div class="content">
  <Title text="Maintenance" />

  <div class="buttons">
    <Button onclick={onRestart} label="Restart server" />
    <Button onclick={onReloadAll} label="Reload all plugins" />
  </div>

  <div class="plugin-list">
    <div class="heading">
      <h2>Plugins</h2>
      <div class="search">
        <SearchInput value={''} />
      </div>
    </div>

    <div class="plugins">
      {#each $plugins as [id, plugin], index (id)}
        <div class="plugin {index % 2 === 0 ? 'even' : 'odd'}">
          <Badge background="#83c06f" foreground="white" text="normal" />
          <p>{id} v{plugin.version}</p>
          <div class="last-reload">
            <ClockArrowUp />
            <span>04:15:12 3/3/2026</span>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .content {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    gap: 1em;
  }

  .plugin-list {
    background: var(--color-surface-light);
    border: 1px grey solid;
    border-radius: 0.3em;
    flex-grow: 1;
  }

  .heading {
    background: var(--color-primary-light);
    display: flex;
    padding: 0.2em;
  }

  .search {
    position: absolute;
    left: 50%;
  }

  .heading h2 {
    font-weight: bold;
    font-size: 1.2em;
  }

  .plugin {
    display: flex;
    padding: 0.2em;
    border-bottom: 1px grey solid;
    gap: 0.5em;
  }

  .plugin.even {
    background: white;
  }

  .last-reload {
    margin-left: auto;
    display: flex;
    gap: 0.2em;
  }
</style>
