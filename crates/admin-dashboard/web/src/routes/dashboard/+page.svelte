<script lang="ts">
  import { humanreadableSize, systemStats } from '$lib';

  let { cpuUsage, memory, startTime } = $derived($systemStats);
</script>

<h1>Dashboard</h1>
<p>CPU: {cpuUsage}%</p>
<p>Memory: {humanreadableSize(memory)}</p>
<p>Startup time: {new Date(startTime * 1000.0)}</p>
