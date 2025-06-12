import React from 'react';
import { PollData } from '../types';
import { XIcon, SunIcon, MoonIcon } from './icons';

interface MenuProps {
  isOpen: boolean;
  onClose: () => void;
  polls: PollData[];
  selectedPollId: string | null;
  onSelectPoll: (pollId: string) => void;
  darkMode: boolean;
  toggleDarkMode: () => void;
}

const Menu: React.FC<MenuProps> = ({ 
  isOpen, 
  onClose, 
  polls, 
  selectedPollId, 
  onSelectPoll,
  darkMode,
  toggleDarkMode 
}) => {
  return (
    <>
      {/* Overlay */}
      <div 
        className={`fixed inset-0 bg-black/60 dark:bg-black/70 z-40 transition-opacity duration-300 ease-in-out ${isOpen ? 'opacity-100' : 'opacity-0 pointer-events-none'}`}
        onClick={onClose}
        aria-hidden="true"
      />

      {/* Menu Panel */}
      <aside 
        className={`fixed top-0 left-0 w-72 sm:w-80 h-full bg-neutral-100 dark:bg-neutral-800 shadow-xl z-50 transform transition-transform duration-300 ease-in-out flex flex-col
                    ${isOpen ? 'translate-x-0' : '-translate-x-full'}`}
        role="dialog"
        aria-modal="true"
        aria-labelledby="menu-title"
      >
        <header className="p-4 flex justify-between items-center border-b border-neutral-300 dark:border-neutral-700 flex-shrink-0">
          <h2 id="menu-title" className="text-lg font-semibold text-neutral-800 dark:text-neutral-100">
            Select Poll
          </h2>
          <button 
            onClick={onClose} 
            className="p-1.5 rounded-md text-neutral-600 dark:text-neutral-300 hover:bg-neutral-200 dark:hover:bg-neutral-700 transition-colors"
            aria-label="Close menu"
          >
            <XIcon className="w-5 h-5" />
          </button>
        </header>

        <nav className="flex-grow p-3 space-y-1.5 overflow-y-auto">
          {polls.map(poll => (
            <button
              key={poll['poll-id']}
              onClick={() => onSelectPoll(poll['poll-id'])}
              className={`w-full text-left px-3 py-2 rounded-md transition-all duration-150 text-sm 
                ${selectedPollId === poll['poll-id'] 
                  ? 'bg-neutral-700 text-white dark:bg-neutral-300 dark:text-neutral-900 font-medium scale-[1.02]' // Accent for selected: using darker/lighter neutrals
                  : 'text-neutral-700 dark:text-neutral-200 hover:bg-neutral-200 dark:hover:bg-neutral-700/60'
                }`}
              aria-current={selectedPollId === poll['poll-id'] ? 'page' : undefined}
            >
              {poll.header}
            </button>
          ))}
          {polls.length === 0 && (
            <p className="text-neutral-500 dark:text-neutral-400 text-sm px-3 py-2">No polls available.</p>
          )}
        </nav>

        <footer className="p-3 border-t border-neutral-300 dark:border-neutral-700 flex-shrink-0">
          <button 
            onClick={toggleDarkMode} 
            className="w-full flex items-center justify-between px-3 py-2 rounded-md text-sm text-neutral-700 dark:text-neutral-200 hover:bg-neutral-200 dark:hover:bg-neutral-700/60 transition-colors"
            aria-label={`Switch to ${darkMode ? 'light' : 'dark'} mode`}
          >
            <span>{darkMode ? 'Light Mode' : 'Dark Mode'}</span>
            {darkMode ? <SunIcon className="w-5 h-5" /> : <MoonIcon className="w-5 h-5" />}
          </button>
        </footer>
      </aside>
    </>
  );
};

export default Menu;