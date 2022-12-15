<script context="module" lang="ts">
  import type { ErrorLoad } from "@sveltejs/kit";

  export const load: ErrorLoad = ({ error, status }) => {
    return {
      props: {
        error,
        status,
      },
    };
  };
</script>

<script lang="ts">
  export let error: Error;
  export let status: number;

  console.log("error", error);
  console.log("status", status);
</script>

<img src="https://http.cat/{status}" alt={`Status code: ${status}`} />
<h1>Oh noes! {status}</h1>
<h2>{error.message}</h2>
