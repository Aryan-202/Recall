<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  const get_notes = async () => {
    const notes = await invoke("get_notes");
    return notes;
  };

  const notesPromise = get_notes();
</script>

<div class="sidebar">
  {#await notesPromise}
    <p>Loading notes...</p>
  {:then notes}
    {#each notes as note}
      <p>{note}</p>
    {/each}
  {:catch error}
    <p style="color: red">{error.message}</p>
  {/await}
</div>

<style>
  .sidebar {
    width: 200px;
    height: 100vh;
    background-color: #f0f0f0;
  }
</style>
