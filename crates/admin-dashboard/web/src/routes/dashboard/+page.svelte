<script lang="ts">
  import { systemStats } from '$lib/stores';
  import Gauge from '$lib/components/Gauge.svelte';
  import Title from '$lib/components/Title.svelte';
  import SearchInput from '$lib/components/SearchInput.svelte';
  import { plugins } from '$lib/stores';

  let { cpuUsage } = $derived($systemStats);
</script>

<div class="content">
  <Title text="Dashboard" />

  <div class="gauges">
    <Gauge ratio={cpuUsage / 100} text="CPU" />
    <Gauge ratio={cpuUsage / 100} text="Memory" />
    <Gauge ratio={cpuUsage / 100} text="Swap" />
    <Gauge ratio={cpuUsage / 100} text="Disk" />
  </div>

  <div class="plugins">
    <div class="plugins-header">
      <h2>Plugins</h2>
      <div class="search">
        <SearchInput value="" />
      </div>
    </div>

    <div class="plugins-table">
      <table>
        <thead>
          <tr>
            <td>Name</td>
            <td>Memory</td>
            <td>RPS</td>
            <td>Delay</td>
          </tr>
        </thead>
        <tbody>
          {#each $plugins as [id, plugin], index (id)}
            <tr class={index % 2 === 0 ? 'even' : 'odd'}>
              <td>{plugin.id} v{plugin.version}</td>
              <td>0</td>
              <td>0</td>
              <td>0</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</div>

<style>
  div.content {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    overflow: auto;
  }

  div.gauges {
    height: 25vh;
    display: grid;
    grid-template-columns: repeat(4, 1fr);
  }

  @media (max-width: 840px) {
    div.gauges {
      height: 50vh;
      grid-template-columns: repeat(2, 1fr);
    }
  }

  div.plugins {
    background: #f4f4f4;
    width: 100%;
    flex-grow: 1;
    border: grey 1px solid;
    border-radius: 0.3em;
  }

  .plugins-header {
    background: #d3d7e5;
    display: flex;
    flex-direction: row;
    padding: 0.3em;

    h2 {
      font-weight: bold;
      font-size: 1.2em;
    }
  }

  .search {
    position: absolute;
    left: 50%;
  }

  .plugins-table table {
    width: 100%;
    border-collapse: collapse;
    text-align: center;

    thead {
      width: 100%;
      background: var(--color-primary-light);
      border-top: 1px grey solid;
      border-bottom: 1px grey solid;

      td {
        border-right: 1px grey solid;
        padding: 0.2em;
      }

      td:last-child {
        border-right: 0;
      }
    }

    tbody {
      width: 100%;
      height: 100%;

      tr {
        border-bottom: grey 1px solid;
      }

      tr.even {
        background: white;
      }
    }
  }
</style>
