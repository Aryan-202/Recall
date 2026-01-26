<script lang="ts">
  import {
    selectedNote,
    saveCurrentNote,
    removeNote,
    selectedNoteId,
  } from "../store";
  import { onDestroy } from "svelte";
  import type { Note } from "../types";

  let title = "";
  let content = "";
  let currentId: string | null = null;
  let timer: any;
  let isSaving = false;

  // React to store changes
  $: if ($selectedNote && $selectedNote.id !== currentId) {
    currentId = $selectedNote.id;
    title = $selectedNote.title;
    content = $selectedNote.content;
  }

  function save() {
    if (!currentId) return;
    isSaving = true;
    saveCurrentNote(currentId, title, content).finally(() => {
      setTimeout(() => (isSaving = false), 500);
    });
  }

  function handleChange() {
    if (!currentId) return;
    clearTimeout(timer);
    timer = setTimeout(save, 800);
  }

  function handleDelete() {
    if (currentId && confirm("Are you sure you want to delete this note?")) {
      removeNote(currentId);
    }
  }

  onDestroy(() => clearTimeout(timer));
</script>

<div class="editor-container">
  {#if $selectedNote}
    <div class="toolbar">
      <span class="status">
        {#if isSaving}Saving...{:else}Saved{/if}
      </span>
      <button class="delete-btn" on:click={handleDelete}>Delete</button>
    </div>
    <div class="editor-content">
      <input
        type="text"
        class="title-input"
        placeholder="Note Title"
        bind:value={title}
        on:input={handleChange}
      />
      <textarea
        class="content-input"
        placeholder="Start typing..."
        bind:value={content}
        on:input={handleChange}
      ></textarea>
    </div>
  {:else}
    <div class="empty-view">
      <div class="empty-icon">📝</div>
      <h2>Select a note to view</h2>
      <p>Or create a new one to get started</p>
    </div>
  {/if}
</div>

<style>
  .editor-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-primary);
    position: relative;
  }

  .toolbar {
    padding: 10px 20px;
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 15px;
  }

  .status {
    font-size: 0.8rem;
    color: var(--text-muted);
  }

  .delete-btn {
    background: transparent;
    color: #ef4444;
    font-size: 0.85rem;
    padding: 6px 12px;
    border-radius: 6px;
  }

  .delete-btn:hover {
    background: rgba(239, 68, 68, 0.1);
  }

  .editor-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 0 40px 40px;
    max-width: 900px;
    margin: 0 auto;
    width: 100%;
  }

  .title-input {
    font-size: 2.5rem;
    font-weight: 700;
    margin-bottom: 20px;
    color: var(--text-primary);
    width: 100%;
  }

  .title-input::placeholder {
    color: var(--bg-tertiary);
  }

  .content-input {
    flex: 1;
    resize: none;
    font-size: 1.1rem;
    line-height: 1.8;
    color: var(--text-secondary);
  }

  .empty-view {
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
  }

  .empty-icon {
    font-size: 4rem;
    margin-bottom: 20px;
    opacity: 0.5;
  }

  .empty-view h2 {
    font-size: 1.5rem;
    margin-bottom: 10px;
    color: var(--text-secondary);
  }
</style>
