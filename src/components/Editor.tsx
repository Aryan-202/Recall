import React, { useEffect, useState } from 'react';
import { Save, Trash2, Clock, Share2 } from 'lucide-react';
import { Note } from '../types';
import './Editor.css';

interface EditorProps {
    note: Note;
    onUpdate: (title: string, body: string) => void;
    onDelete: (id: string) => void;
}

const Editor: React.FC<EditorProps> = ({ note, onUpdate, onDelete }) => {
    const [title, setTitle] = useState(note.title);
    const [body, setBody] = useState(note.body);

    useEffect(() => {
        setTitle(note.title);
        setBody(note.body);
    }, [note.bson_uuid, note.title, note.body]);

    const handleSave = () => {
        onUpdate(title, body);
    };

    return (
        <main className="editor">
            <header className="editor-header">
                <div className="editor-actions-left">
                    <div className="last-saved">
                        <Clock size={14} />
                        <span>Last saved: Just now</span>
                    </div>
                </div>
                <div className="editor-actions-right">
                    <button className="icon-btn" title="Share">
                        <Share2 size={18} />
                    </button>
                    <button className="icon-btn danger" onClick={() => onDelete(note.bson_uuid)} title="Delete">
                        <Trash2 size={18} />
                    </button>
                    <button className="save-btn" onClick={handleSave}>
                        <Save size={16} />
                        <span>Save</span>
                    </button>
                </div>
            </header>

            <div className="editor-content">
                <input
                    className="editor-title-input"
                    type="text"
                    placeholder="Note Title"
                    value={title}
                    onChange={(e) => setTitle(e.target.value)}
                />
                <textarea
                    className="editor-body-textarea"
                    placeholder="Start writing..."
                    value={body}
                    onChange={(e) => setBody(e.target.value)}
                />
            </div>
        </main>
    );
};

export default Editor;
