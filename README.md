# Recall

Recall is a modern, high-performance, and distraction-free note-taking application built for those who value speed and simplicity. It leverages the power of **Tauri** for a lightweight footprint, **Rust** for a robust backend, and **SvelteKit** for a reactive, beautiful frontend.

## ✨ Features

- **Distraction-Free Editor**: A clean interface focused purely on your writing.
- **Auto-Save**: Never lose a thought; changes are saved automatically as you type.
- **Fast Search**: Instantly filter through your notes with title and content search.
- **Tagging System**: Organize your notes with flexible tags (Backend ready).
- **Local-First**: All data is stored locally in an SQLite database for privacy and speed.
- **Dark Mode**: A generic, sleek dark theme designed for extended coding/writing sessions.
- **Import/Export**: detailed markdown import and export capabilities.

## 🛠️ Tech Stack

- **Frontend**: SvelteKit, TypeScript, Vanilla CSS (Variables)
- **Backend**: Rust (Tauri), SQLite (Rusqlite)
- **State Management**: Svelte Stores
- **Database**: SQLite

## 🚀 Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (or Bun)
- [Rust](https://www.rust-lang.org/tools/install)
- [VS Code](https://code.visualstudio.com/) (Recommended) with Tauri and Rust Analyzer extensions.

### Installation

1.  Clone the repository:

    ```bash
    git clone https://github.com/yourusername/recall.git
    cd recall
    ```

2.  Install frontend dependencies:

    ```bash
    npm install
    # or
    bun install
    ```

3.  Run the application in development mode:
    ```bash
    npm run tauri dev
    # or
    bun tauri dev
    ```

## 📂 Project Structure

- `src/`: SvelteKit frontend source code.
  - `routes/`: Application pages and layout.
  - `lib/`: Shared components, stores, and backend API wrappers.
    - `components/`: UI components (Sidebar, Editor).
    - `db.ts`: Type-safe wrappers for Tauri invoke commands.
    - `store.ts`: Reactive state management.
- `src-tauri/`: Rust backend source code.
  - `src/`: Rust source files.
    - `db/`: Database schema, models, and operations.
    - `commands/`: Tauri commands exposed to the frontend.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License.
