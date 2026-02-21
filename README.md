# Recall

A modern, native note-taking application built with Tauri, React, and TypeScript. Recall combines the performance of a native app with the elegance of a modern web interface, providing a seamless note-taking experience.

<!-- ![Recall App](screenshot-placeholder.png) -->

## ✨ Features

- **Native Performance**: Built with Tauri (Rust backend) for small bundle sizes and high performance
- **Beautiful UI**: Dark theme with smooth animations powered by Framer Motion
- **Real-time Search**: Instantly filter notes by title or content
- **Persistent Storage**: Notes are stored locally using BSON serialization
- **Keyboard Friendly**: Optimized for fast note-taking and navigation
- **Responsive Design**: Clean, distraction-free interface that adapts to your workflow

## 🚀 Getting Started

### Prerequisites

- [Bun](https://bun.sh) (or Node.js + npm/yarn/pnpm)
- [Rust](https://www.rust-lang.org/tools/install) (for Tauri development)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/recall.git
   cd recall
   ```

2. Install dependencies:
   ```bash
   bun install
   ```

3. Run the development server:
   ```bash
   bun run tauri dev
   ```

### Building for Production

```bash
bun run tauri build
```

The compiled application will be available in `src-tauri/target/release/`.

## 🏗️ Project Structure

```
recall/
├── src/                    # Frontend source code
│   ├── components/         # React components
│   ├── hooks/              # Custom React hooks
│   ├── services/           # Tauri backend communication
│   ├── styles/             # CSS styles and theme
│   ├── types/              # TypeScript type definitions
│   ├── App.tsx             # Main application component
│   └── main.tsx            # Application entry point
├── src-tauri/              # Tauri backend
│   ├── src/
│   │   ├── commands/       # Tauri command implementations
│   │   ├── lib.rs          # Backend entry point
│   │   └── main.rs          # Application entry point
│   └── tauri.conf.json     # Tauri configuration
├── index.html              # HTML template
├── package.json            # Frontend dependencies
├── tsconfig.json           # TypeScript configuration
└── vite.config.ts          # Vite build configuration
```

## 🛠️ Technology Stack

### Frontend
- **React 19** - UI library
- **TypeScript** - Type safety
- **Vite** - Build tool and dev server
- **Framer Motion** - Smooth animations
- **Lucide React** - Beautiful icons
- **date-fns** - Date formatting

### Backend
- **Tauri 2** - Native application framework
- **Rust** - High-performance backend
- **BSON** - Binary JSON for data storage

## 💡 Usage

### Creating a Note
- Click the **+** button in the sidebar or use the "Create New Note" button in the empty state
- Give your note a title and start writing

### Managing Notes
- **Search**: Use the search bar to filter notes by title or content
- **Edit**: Click any note in the sidebar to edit its content
- **Delete**: Click the trash icon in the editor toolbar
- **Save**: Changes are saved manually with the save button

### Data Storage
Notes are stored locally using BSON serialization through Tauri commands, ensuring efficient binary storage while maintaining human-readable access through the frontend.

## 🔧 Development

### Adding New Features

1. **Frontend Components**: Add new React components in `src/components/`
2. **Backend Commands**: Implement new Tauri commands in `src-tauri/src/commands/`
3. **Styling**: Use the existing CSS variables in `src/styles/theme.css` for consistent theming

### Available Scripts

- `bun run dev` - Start Vite development server
- `bun run build` - Build the frontend
- `bun run tauri dev` - Run Tauri development app
- `bun run tauri build` - Build production app

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) for the amazing framework
- [Lucide](https://lucide.dev/) for the beautiful icons
- [Framer Motion](https://www.framer.com/motion/) for the smooth animations

---

Built with ❤️ using Tauri and React