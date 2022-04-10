<script lang="ts">
  import { onMount } from "svelte";
  import wsStore, { connect } from "../../stores/ws";

  let nice: HTMLAudioElement = null;
  onMount(() => {
    connect();
    nice = new Audio("/noice.mp3");
  });

  $: if ($wsStore.messages[0] === "Nice") {
    nice?.play();
  }
</script>

<!--
<main>
  <ul>
    {#each $wsStore.messages as msg}
      <li>{msg}</li>
    {/each}
  </ul>
</main>

<style>
  ul {
    margin: 0;
  }

  main {
    margin: auto;
    height: 100%;
    width: 100%;
    background-color: #04F404;
  }
</style>
-->
