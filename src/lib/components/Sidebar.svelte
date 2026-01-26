<script lang="ts">
  import {
    notes,
    selectedNoteId,
    addNote,
    performSearch,
    searchQuery,
  } from "../store";
  import type { Note } from "../types";

  let timer: any;
  function handleSearch(e: Event) {
    const query = (e.target as HTMLInputElement).value;
    clearTimeout(timer);
    timer = setTimeout(() => {
      performSearch(query);
    }, 300);
  }
</script>

<aside class="sidebar">
  <div class="header">
    <div class="brand">Recall</div>
    <button class="add-btn" on:click={addNote} aria-label="Create New Note">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="20"
        height="20"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        ><line x1="12" y1="5" x2="12" y2="19"></line><line
          x1="5"
          y1="12"
          x2="19"
          y2="12"
        ></line></svg
      >
    </button>
  </div>

  <div class="search-box">
    <input
      type="text"
      placeholder="Search notes..."
      value={$searchQuery}
      on:input={handleSearch}
    />
  </div>

  <div class="note-list">
    {#each $notes as note (note.id)}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div
        class="note-item"
        class:active={$selectedNoteId === note.id}
        on:click={() => ($selectedNoteId = note.id)}
      >
        <div class="note-title">{note.title || "Untitled"}</div>
        <div class="note-date">
          {new Date(note.updated_at).toLocaleDateString()}
        </div>
        <div class="note-preview">
          {note.content.substring(0, 50).replace(/\n/g, " ")}...
        </div>
      </div>
    {/each}
    {#if $notes.length === 0}
      <div class="empty-state">No notes found</div>
    {/if}
  </div>
</aside>

<style>
  .sidebar {
    width: 300px;
    height: 100%;
    background: var(--bg-secondary);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
  }

  .header {
    padding: 20px;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .brand {
    font-size: 1.5rem;
    font-weight: 700;
    letter-spacing: -0.5px;
    color: var(--text-primary);
  }

  .add-btn {
    background: var(--accent);
    color: white;
    width: 32px;
    height: 32px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .add-btn:hover {
    background: var(--accent-hover);
  }

  .search-box {
    padding: 0 20px 20px;
  }

  .search-box input {
    width: 100%;
    background: var(--bg-primary);
    padding: 10px 12px;
    border-radius: 8px;
    color: var(--text-primary);
    font-size: 0.9rem;
    border: 1px solid transparent;
    transition: border-color 0.2s;
  }

  .search-box input:focus {
    border-color: var(--accent);
  }

  .note-list {
    flex: 1;
    overflow-y: auto;
    padding: 0 10px;
  }

  .note-item {
    padding: 12px 14px;
    margin-bottom: 8px;
    border-radius: 8px;
    cursor: pointer;
    transition: background 0.2s;
    border: 1px solid transparent;
  }

  .note-item:hover {
    background: var(--bg-tertiary);
  }

  .note-item.active {
    background: var(--bg-tertiary);
    border-color: var(--accent);
  }

  .note-title {
    font-weight: 600;
    font-size: 0.95rem;
    color: var(--text-primary);
    margin-bottom: 4px;
  }

  .note-date {
    font-size: 0.75rem;
    color: var(--text-muted);
    float: right;
  }

  .note-preview {
    font-size: 0.8rem;
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .empty-state {
    padding: 20px;
    text-align: center;
    color: var(--text-muted);
    font-size: 0.9rem;
  }
</style>
