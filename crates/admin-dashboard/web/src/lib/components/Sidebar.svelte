<script module lang="ts">
  import type { Component } from 'svelte';

  export type SidebarProps = {
    links: Link[];
  };

  export type Link = {
    title: string;
    href: string;
    icon: Component;
  };
</script>

<script lang="ts">
  import Title from './Title.svelte';

  let { links }: SidebarProps = $props();
</script>

<div class="content">
  <Title text="Wassel Admin"></Title>

  <div class="links">
    {#each links as link (link)}
      <div class="link">
        <link.icon color="black" strokeWidth={2} size={24} />
        <a href={link.href}>{link.title} </a>
      </div>
    {/each}
  </div>
</div>

<style>
  .content {
    width: 250px;
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--color-primary-light);
    color: var(--color-text-primary);
  }

  div.links {
    display: flex;
    flex-direction: column;
    margin: 0 1em;
  }

  a {
    width: 100%;
    padding: 0 0.2em;
    background: none;
  }

  .link {
    height: 1.5em;

    a {
      vertical-align: top;
    }
  }

  .link:hover {
    background: grey;
  }
</style>
