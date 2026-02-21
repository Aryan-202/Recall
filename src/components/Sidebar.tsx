import React from 'react';
import { Plus, Search, Layers } from 'lucide-react';
import { Note } from '../types';
import NoteItem from './NoteItem';
import './Sidebar.css';

interface SidebarProps {
    notes: Note[];
    activeNoteId: string | null;
    onNoteSelect: (id: string) => void;
    onNewNote: () => void;
    searchQuery: string;
    onSearchChange: (query: string) => void;
}

const Sidebar: React.FC<SidebarProps> = ({
    notes,
    activeNoteId,
    onNoteSelect,
    onNewNote,
    searchQuery,
    onSearchChange
}) => {
    return (
        <aside className="sidebar">
            <div className="sidebar-header">
                <div className="brand">
                    <Layers className="brand-icon" size={20} />
                    <h1>Recall</h1>
                </div>
                <button className="new-note-btn" onClick={onNewNote} title="New Note">
                    <Plus size={20} />
                </button>
            </div>

            <div className="search-container">
                <Search className="search-icon" size={16} />
                <input
                    type="text"
                    placeholder="Search notes..."
                    value={searchQuery}
                    onChange={(e) => onSearchChange(e.target.value)}
                />
            </div>

            <div className="notes-list">
                {notes.length === 0 ? (
                    <div className="empty-search">
                        <p>No notes found</p>
                    </div>
                ) : (
                    notes.map(note => (
                        <NoteItem
                            key={note.bson_uuid}
                            note={note}
                            isActive={activeNoteId === note.bson_uuid}
                            onClick={() => onNoteSelect(note.bson_uuid)}
                        />
                    ))
                )}
            </div>
        </aside>
    );
};

export default Sidebar;
