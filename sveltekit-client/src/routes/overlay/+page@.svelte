<script lang="ts">
  import { onMount } from "svelte";
  import wsStore, { connect } from "../../stores/ws";

  let audioElem: HTMLAudioElement = null;
  onMount(() => {
    connect();
  });

  let video: HTMLVideoElement;
  let active = false;
  let paused: boolean;

  $: if (paused) {
    active = false;
  }

  const resetAudioPlayer = (audio: HTMLAudioElement) => {
    audio.pause();
    audio.currentTime = 0;
  };

  const playSound = async (audio: HTMLAudioElement, source: string) => {
    audio.src = source;
    try {
      await audio.play();
    } catch (e) {
      console.error("Error playing sound", e);
    }
  };

  $: if ($wsStore.messages[0] === "420") {
    if (audioElem !== null) {
      resetAudioPlayer(audioElem);
      playSound(audioElem, "/420blazeit.mp3");
    }
  }

  $: if ($wsStore.messages[0] === "Death") {
    if (audioElem !== null) {
      resetAudioPlayer(audioElem);
      playSound(audioElem, "/mario_death.mp3");
    }
  }

  $: if ($wsStore.messages[0] === "Nice") {
    if (audioElem !== null) {
      resetAudioPlayer(audioElem);
      playSound(audioElem, "/noice.mp3");
    }
  }
</script>

<audio bind:this={audioElem} />

<video class:active bind:this={video} src="/derp.mp4" bind:paused>
  <track kind="captions" />
</video>

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
<style>
  video {
    display: none;
  }

  .active {
    display: inline-block;
  }
</style>
