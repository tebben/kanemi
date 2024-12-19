<script lang="ts">
  import "../app.css";
  import { app } from "$lib/app";
  let { children } = $props();

  app.init();
</script>

{@render children()}
