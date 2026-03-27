<script lang="ts">
  import { traces, type Trace } from '$lib';
</script>

<h1>Server Logs</h1>
<table>
  <tbody>
    {#each $traces as trace (trace.timestamp)}
      {@render message(trace)}
    {/each}
  </tbody>
</table>

{#snippet message(trace: Trace)}
  <tr>
    <td>
      {trace.timestamp}
    </td>
    <td class="level {trace.level}">
      {trace.level}
    </td>
    <td>
      {#if trace.message}
        {trace.message}
      {/if}
    </td>
    <td>
      {#each Object.entries(trace.fields) as [key, value]}
        <span class="field-key">{key}</span>=<span>{value}</span> <span></span>
      {/each}
    </td>
  </tr>
{/snippet}

<style>
  table {
    border-collapse: separate;
    border-spacing: 10px 2px;
    font-family: monospace;
  }

  .field-key {
    font-style: italic;
  }

  td {
    vertical-align: top;
  }

  td.level {
    text-align: right;
  }

  td.level.TRACE {
    color: purple;
  }

  td.level.DEBUG {
    color: cornflowerblue;
  }

  td.level.INFO {
    color: green;
  }

  td.level.WARN {
    color: yellow;
  }

  td.level.ERROR {
    color: red;
  }
</style>
