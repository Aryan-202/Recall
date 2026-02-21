import React from 'react';
import { StickyNote, Plus } from 'lucide-react';
import './EmptyState.css';

interface EmptyStateProps {
    onNewNote: () => void;
}

const EmptyState: React.FC<EmptyStateProps> = ({ onNewNote }) => {
    return (
        <div className="empty-state">
            <div className="empty-state-content">
                <div className="empty-icon-wrapper">
                    <StickyNote size={48} className="empty-icon" />
                </div>
                <h2>Select a note to view</h2>
                <p>Choose a note from the list on the left or create a new one to started.</p>
                <button className="create-first-btn" onClick={onNewNote}>
                    <Plus size={18} />
                    <span>Create New Note</span>
                </button>
            </div>
        </div>
    );
};

export default EmptyState;
