<script lang="ts">
  let { data } = $props();
  let { plugins } = $derived(data);
</script>

<h1>Maintenance</h1>

<h2>Plugin list:</h2>
<table>
  <thead>
    <tr>
      <td>Name</td>
      <td>Endpoint</td>
      <td>Version</td>
      <td>Description</td>
    </tr>
  </thead>

  <tbody>
    {#each plugins as plugin (plugin.id)}
      <tr>
        <td class="monospace">{plugin.name}</td>
        <td class="monospace">{plugin.endpoint}</td>
        <td class="monospace">{plugin.version}</td>
        <td>{plugin.description}</td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  table {
    border-collapse: collapse;
  }

  td {
    border: 2px black solid;
    padding: 0.2em;
  }

  td.monospace {
    font-family: monospace;
  }
</style>
