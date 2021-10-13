<script lang="ts">
  let count = 0;

  const increment = () => {
    count++;
  };
</script>

<h1>Bajsballe {count}</h1>
<button on:click={increment}>bajsballe</button>
