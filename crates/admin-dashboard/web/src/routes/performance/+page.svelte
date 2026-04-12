<script lang="ts">
  import { Button, Dropdown, Title } from '$lib/components';
  import { plugins } from '$lib/stores';
  import { Line } from 'svelte-chartjs';
  import {
    Chart as ChartJS,
    Title as ChartTitle,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale,
    type ChartData,
  } from 'chart.js';

  ChartJS.register(
    ChartTitle,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale
  );

  let variants = $derived(['Global'].concat($plugins.keys().toArray()));
  let plugin = $state('Global');

  const data: ChartData<'line'> = $derived({
    labels: Array.from({ length: 7 }, (_, i) => i),
    datasets: [
      {
        label: plugin,
        fill: true,
        backgroundColor: '#d3d7e5',
        borderColor: '#435082',
        data: [65, 59, 80, 81, 56, 55, 40],
      },
    ],
  });

  function onDownloadCsv() {}
</script>

<div class="content">
  <Title text="Performance" />
  <div class="buttons">
    <Dropdown {variants} bind:value={plugin} width="250px" />
    <Button label="Download CSV" onclick={onDownloadCsv} />
  </div>

  {@render Plot('Requests per second', data)}
  {@render Plot('Delay', data)}
</div>

{#snippet Plot(name: string, data: ChartData<'line'>)}
  <div class="plot">
    <h2>{name}</h2>
    <Line {data} options={{ responsive: true, maintainAspectRatio: false }} />
  </div>
{/snippet}

<style>
  .content {
    width: 100%;
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: auto;
    gap: 0.5em;
  }

  .buttons {
    display: flex;
    justify-content: space-between;
  }

  h2 {
    font-weight: bold;
    text-align: center;
  }

  .plot {
    width: 100%;
    height: 35%;
    display: flex;
    flex-direction: column;
    margin-bottom: 5%;
  }
</style>
