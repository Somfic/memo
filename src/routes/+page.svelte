<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Bar from "$components/Bar.svelte";
  import Content from "$components/Content.svelte";

  import { card } from "../state";
</script>

<Content />
<Bar />
