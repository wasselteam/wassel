<script lang="ts">
  import '$lib/assets/unstyle.css';
  import favicon from '$lib/assets/favicon.svg';

  import Sidebar from '$lib/components/Sidebar.svelte';

  let { children } = $props();
  let links = [
    { href: '/dashboard', title: 'Dashboard' },
    { href: '/logs', title: 'Logs' },
    { href: '/performance', title: 'Performance' },
    { href: '/maintenance', title: 'Maintenance' },
  ];
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

<header>Some header content</header>
<main>
  <div class="sidebar">
    <Sidebar {links} />
  </div>
  <div class="content">
    {@render children()}
  </div>
</main>

<style>
  :global(body) {
    --header-height: 48px;
    --sidebar-width: 160px;

    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  header {
    position: fixed;
    left: 0;
    background: cornflowerblue;
    height: var(--header-height);
    width: 100%;
  }

  .sidebar {
    position: fixed;
    left: 0;
    top: var(--header-height);
    width: var(--sidebar-width);
    height: 100%;
    background: cornflowerblue;
  }

  main {
    display: flex;
    flex-direction: row;
    margin-top: var(--header-height);
    margin-left: var(--sidebar-width);
    flex-grow: 1;
  }

  div.content {
    flex-grow: 1;
  }
</style>
