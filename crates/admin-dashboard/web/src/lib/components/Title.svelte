<script lang="ts">
  let { text } = $props();
</script>

<h1>{text}</h1>

<style>
  h1 {
    font-size: 1.5em;
    font-weight: bold;
    text-shadow: 2px 2px 1px #dddddd;
    text-align: center;
    margin: 0.2em;
  }
</style>
