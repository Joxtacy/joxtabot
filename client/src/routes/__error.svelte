<script lang="ts" context="module">
  export function load({ error, status }: { error: Error; status: number }) {
    return {
      props: {
        error,
        status,
      },
    };
  }
</script>

<script lang="ts">
  export let error: Error;
  export let status: number;
</script>

<h1>rip {status}</h1>
<h2>{error.name} - {error.message}</h2>
<h3>{error.stack}</h3>
