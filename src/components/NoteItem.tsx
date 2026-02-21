import React from 'react';
import { format } from 'date-fns';
import { FileText, Calendar } from 'lucide-react';
import { NoteItemProps } from '../types';
import './NoteItem.css';

const NoteItem: React.FC<NoteItemProps> = ({ note, isActive, onClick }) => {
    const getDisplayDate = () => {
        if (typeof note.date_time === 'string') return note.date_time;
        const ms = parseInt(note.date_time.$date.$numberLong);
        return format(new Date(ms), 'MMM d, yyyy');
    };

    return (
        <div
            className={`note-item ${isActive ? 'active' : ''}`}
            onClick={onClick}
        >
            <div className="note-item-icon">
                <FileText size={18} />
            </div>
            <div className="note-item-content">
                <h3 className="note-item-title">{note.title || 'Untitled Note'}</h3>
                <div className="note-item-meta">
                    <Calendar size={12} />
                    <span>{getDisplayDate()}</span>
                </div>
            </div>
        </div>
    );
};

export default NoteItem;
