<script lang="ts">
  import { humanreadableSize, statsService, type SystemStats } from '$lib';
  import { onDestroy } from 'svelte';

  let { data } = $props();
  let { cpuUsage, memory: memoryRaw, startTime } = $derived(data);
  let memory = $derived(humanreadableSize(memoryRaw));
  let startup = $derived(new Date(startTime * 1000.0));

  let handler = (stats: SystemStats) => {
    memoryRaw = stats.memory;
    cpuUsage = stats.cpuUsage;
    startTime = stats.startTime;
  };
  statsService.addSystemEventListener(handler);

  onDestroy(() => {
    statsService.removeSystemEventListener(handler);
  });
</script>

<h1>Actual dashboard page</h1>
<p>CPU: {cpuUsage}%</p>
<p>Memory: {memory}</p>
<p>Startup time: {startup}</p>
