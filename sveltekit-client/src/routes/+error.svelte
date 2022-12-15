<script lang="ts">
  import { page } from "$app/stores";

  console.log("error", page.error);
  console.log("status", page.status);
</script>

<img src="https://http.cat/{$page.status}" alt={`Status code: ${$page.status}`} />
<h1>Oh noes! {$page.status}</h1>
<h2>{$page.error.message}</h2>
