<script lang="ts">
  import '$lib/assets/unstyle.css';
  import '$lib/assets/main.css';
  import favicon from '$lib/assets/favicon.svg';
  import House from '@lucide/svelte/icons/house';
  import ScrollText from '@lucide/svelte/icons/scroll-text';
  import Gauge from '@lucide/svelte/icons/gauge';
  import Cable from '@lucide/svelte/icons/cable';
  import { Sidebar } from '$lib/components';

  let { children } = $props();
  let links = [
    { href: '/dashboard', title: 'Dashboard', icon: House },
    { href: '/logs', title: 'Logs', icon: ScrollText },
    { href: '/performance', title: 'Performance', icon: Gauge },
    { href: '/maintenance', title: 'Maintenance', icon: Cable },
  ];
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

<main>
  <Sidebar {links} />
  <div class="content">
    {@render children()}
  </div>
</main>

<style>
  main {
    position: fixed;
    width: 100vw;
    height: 100vh;

    display: flex;
    flex-direction: row;
  }

  div.content {
    margin: 1em;
    width: calc(100vw - 250px);
  }
</style>
