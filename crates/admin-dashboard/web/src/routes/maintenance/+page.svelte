<script lang="ts">
  let { data } = $props();
  let { plugins } = $derived(data);
</script>

<h1>Actual maintenance page</h1>

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
        <td>{plugin.name}</td>
        <td>{plugin.endpoint}</td>
        <td>{plugin.version}</td>
        <td>{plugin.description}</td>
      </tr>
    {/each}
  </tbody>
</table>
