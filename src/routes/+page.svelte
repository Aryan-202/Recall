<script lang="ts">
  import { onMount } from "svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import Editor from "$lib/components/Editor.svelte";
  import { loadNotes, loadTags } from "$lib/store";

  onMount(async () => {
    await Promise.all([loadNotes(), loadTags()]);
  });
</script>

<div class="app-layout">
  <Sidebar />
  <Editor />
</div>

<style>
  .app-layout {
    display: flex;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
  }
</style>
