<script lang="ts">
  import Title from '$lib/components/Title.svelte';
  import { traces, type Trace } from '$lib/stores';
</script>

<div class="content">
  <Title text="Logs" />
  <div class="table">
    <table>
      <tbody>
        {#each $traces as trace (trace.timestamp)}
          {@render message(trace)}
        {/each}
      </tbody>
    </table>
  </div>
</div>

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
  div.content {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
  }

  div.table {
    flex-grow: 1;
    overflow: auto;
    background: var(--color-surface-light);
    border-radius: 0.3em;
    border: grey 1px solid;
  }

  table {
    border-collapse: separate;
    border-spacing: 10px 2px;
    font-family: monospace;
    font-size: 0.7em;
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
