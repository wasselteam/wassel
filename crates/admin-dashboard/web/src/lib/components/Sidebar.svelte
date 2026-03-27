<script lang="ts">
  import type { SidebarProps } from './Sidebar';

  let { links }: SidebarProps = $props();
</script>

<div class="content">
  {#each links as link (link)}
    <a href={link.href}>{link.title}</a>
  {/each}
</div>

<style>
  .content {
    display: flex;
    flex-direction: column;
  }

  a {
    width: 100%;
    color: white;
    background: blue;
  }
</style>
