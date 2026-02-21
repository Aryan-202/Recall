import { motion, AnimatePresence } from 'framer-motion';
import Sidebar from './components/Sidebar';
import Editor from './components/Editor';
import EmptyState from './components/EmptyState';
import { useNotes } from './hooks/useNotes';
import './App.css';

function App() {
  const {
    notes,
    activeNote,
    activeNoteId,
    setActiveNoteId,
    searchQuery,
    setSearchQuery,
    handleNewNote,
    handleUpdateNote,
    handleDeleteNote,
    loading
  } = useNotes();

  if (loading) {
    return (
      <div className="loading-screen">
        <motion.div
          animate={{ scale: [1, 1.2, 1] }}
          transition={{ repeat: Infinity, duration: 1.5 }}
          className="loading-logo"
        >
          Recall
        </motion.div>
      </div>
    );
  }

  return (
    <div className="app-container">
      <Sidebar
        notes={notes}
        activeNoteId={activeNoteId}
        onNoteSelect={setActiveNoteId}
        onNewNote={handleNewNote}
        searchQuery={searchQuery}
        onSearchChange={setSearchQuery}
      />

      <div className="main-viewport">
        <AnimatePresence mode="wait">
          {activeNote ? (
            <motion.div
              key={activeNote.bson_uuid}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              transition={{ duration: 0.2 }}
              style={{ height: '100%', width: '100%' }}
            >
              <Editor
                note={activeNote}
                onUpdate={(title, body) => handleUpdateNote(activeNote.bson_uuid, title, body)}
                onDelete={handleDeleteNote}
              />
            </motion.div>
          ) : (
            <motion.div
              key="empty"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              style={{ height: '100%', width: '100%' }}
            >
              <EmptyState onNewNote={handleNewNote} />
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
}

export default App;
