<script module lang="ts">
  export type BadgeProps = {
    text: string;
    background: string;
    foreground: string;
  };
</script>

<script lang="ts">
  let { text, background, foreground }: BadgeProps = $props();
</script>

<span style:background style:color={foreground}>{text}</span>

<style>
  span {
    border-radius: 0.25em;
    padding: 0.1em 1em;
  }
</style>
