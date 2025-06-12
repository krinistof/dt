import React, { useState, useCallback, useEffect } from 'react';
import { PollData, VoteDirection, PollOption } from './types';
import PollDisplay from './components/PollDisplay';
import Menu from './components/Menu';
import { MenuIcon } from './components/icons';
import { 
  BASE_VOTE_INCREMENT, 
  MIN_USER_VOTE, 
  MAX_USER_VOTE,
  RAPID_VOTE_WINDOW_MS,
  MAX_STREAK_MULTIPLIER
} from './constants';
import { GreeterClient } from './src/grpc';

console.log(await GreeterClient.sayHello());
// Sample data (can be fetched from an API in a real app)
const initialPollsData: PollData[] = [
  {
    "poll-id": "3933107e-affb-455f-8594-a0970e66db85",
    "header": "Best Fruit?",
    "options": [
      { "option-id": "fruit-banana", "metadata": { "title": "Banana", "description": "Curved yellow fruit, rich in potassium." }, "score": 57, "user_latest_vote": 18 },
      { "option-id": "fruit-watermelon", "metadata": { "title": "Watermelon", "description": "Large, juicy, and refreshing." }, "score": 46, "user_latest_vote": -28 },
      { "option-id": "fruit-kiwi", "ft-id": "kiwi-info", "metadata": { "title": "Kiwi", "description": "Fuzzy brown, green inside, tangy-sweet." }, "score": 35, "user_latest_vote": 0 },
      { "option-id": "fruit-raspberry", "metadata": { "title": "Raspberry", "description": "Small, red, sweet-tart berry." }, "score": -4, "user_latest_vote": 74 }
    ]
  },
  {
    "poll-id": "prog-lang-poll-456",
    "header": "Favorite Programming Language?",
    "options": [
      { "option-id": "lang-ts", "metadata": { "title": "TypeScript", "description": "JavaScript with static types." }, "score": 150, "user_latest_vote": 20 },
      { "option-id": "lang-py", "ft-id": "python-details", "metadata": { "title": "Python", "description": "Readable, versatile, great for AI." }, "score": 120, "user_latest_vote": -10 },
      { "option-id": "lang-rs", "metadata": { "title": "Rust", "description": "Memory safety without garbage collection." }, "score": 90, "user_latest_vote": 0 }
    ]
  }
];

interface VoteStreakInfo {
  lastTapMs: number;
  streak: number;
  direction: VoteDirection | null;
}

const App: React.FC = () => {
  const [polls, setPolls] = useState<PollData[]>(() => 
    initialPollsData.map(poll => ({
      ...poll,
      options: [...poll.options].sort((a,b) => b.score - a.score) // Initial sort, ensure new array
    }))
  );
  const [darkMode, setDarkMode] = useState<boolean>(false);
  const [isMenuOpen, setIsMenuOpen] = useState<boolean>(false);
  const [selectedPollId, setSelectedPollId] = useState<string | null>(
    initialPollsData.length > 0 ? initialPollsData[0]['poll-id'] : null
  );
  const [voteStreaks, setVoteStreaks] = useState<Record<string, VoteStreakInfo>>({});

  useEffect(() => {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    setDarkMode(prefersDark);
    
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handleChange = (e: MediaQueryListEvent) => setDarkMode(e.matches);
    mediaQuery.addEventListener('change', handleChange);
    return () => mediaQuery.removeEventListener('change', handleChange);
  }, []);

  useEffect(() => {
    if (darkMode) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }, [darkMode]);

  const handleVote = useCallback((pollId: string, optionId: string, direction: VoteDirection) => {
    const nowMs = Date.now();
    const currentStreakInfo = voteStreaks[optionId] || { lastTapMs: 0, streak: 0, direction: null };
    
    let newStreakCount = 1;
    if (
      (nowMs - currentStreakInfo.lastTapMs) <= RAPID_VOTE_WINDOW_MS &&
      currentStreakInfo.direction === direction
    ) {
      newStreakCount = Math.min(currentStreakInfo.streak + 1, MAX_STREAK_MULTIPLIER);
    }

    setVoteStreaks(prev => ({
      ...prev,
      [optionId]: { lastTapMs: nowMs, streak: newStreakCount, direction }
    }));

    const voteAmount = BASE_VOTE_INCREMENT * newStreakCount;

    setPolls(prevPolls =>
      prevPolls.map(poll => {
        if (poll['poll-id'] === pollId) {
          const updatedOptions = poll.options.map(opt => {
            if (opt['option-id'] === optionId) {
              const voteChange = direction === VoteDirection.UP ? voteAmount : -voteAmount;
              let newVoteForOption = opt.user_latest_vote + voteChange;
              newVoteForOption = Math.max(MIN_USER_VOTE, Math.min(MAX_USER_VOTE, newVoteForOption));
              
              const actualVoteDelta = newVoteForOption - opt.user_latest_vote;
              
              return {
                ...opt,
                score: opt.score + actualVoteDelta,
                user_latest_vote: newVoteForOption,
              };
            }
            return opt;
          });

          // Sort options by score descending after updating. Create new array for react reconciliation.
          const sortedOptions = [...updatedOptions].sort((a, b) => b.score - a.score);
          return { ...poll, options: sortedOptions };
        }
        return poll;
      })
    );
  }, [voteStreaks]); // Added voteStreaks to dependency array

  const selectedPoll = polls.find(p => p['poll-id'] === selectedPollId) || null;

  return (
    <div className="flex flex-col min-h-screen">
      <Menu 
        isOpen={isMenuOpen} 
        onClose={() => setIsMenuOpen(false)}
        polls={polls}
        selectedPollId={selectedPollId}
        onSelectPoll={(pollId) => {
          setSelectedPollId(pollId);
          setIsMenuOpen(false); 
        }}
        darkMode={darkMode}
        toggleDarkMode={() => setDarkMode(prev => !prev)}
      />

      <header className="sticky top-0 z-30 bg-neutral-100/90 dark:bg-neutral-800/90 backdrop-blur-sm shadow-sm">
        <div className="container mx-auto px-4 py-3.5 flex items-center justify-between">
          <button
            onClick={() => setIsMenuOpen(true)}
            className="p-2 rounded-md text-neutral-700 dark:text-neutral-200 hover:bg-neutral-200/70 dark:hover:bg-neutral-700/70 transition-colors"
            aria-label="Open menu"
            aria-expanded={isMenuOpen}
            aria-controls="app-menu"
          >
            <MenuIcon className="w-5 h-5 sm:w-6 sm:h-6" />
          </button>
          <h1 className="text-lg sm:text-xl font-semibold text-neutral-800 dark:text-neutral-100 truncate flex-1 text-center px-2" aria-live="polite">
            {selectedPoll ? selectedPoll.header : "Live Score Voting"}
          </h1>
          <div className="w-8 sm:w-9 h-8 sm:h-9"></div> {/* Placeholder for symmetry */}
        </div>
      </header>

      <main id="main-content" className="flex-grow container mx-auto px-4 py-6 sm:py-8">
        {selectedPoll ? (
          <PollDisplay poll={selectedPoll} onVote={handleVote} />
        ) : (
          <div className="text-center py-12">
            <p className="text-lg text-neutral-600 dark:text-neutral-400">
              {polls.length > 0 ? "Select a poll from the menu to get started." : "No polls currently available."}
            </p>
          </div>
        )}
      </main>
      
      <footer className="py-5 text-center text-xs text-neutral-500 dark:text-neutral-400 border-t border-neutral-200 dark:border-neutral-700">
        <a href="https://github.com/krinistof/dt">Democratic Tier</a> - {new Date().getFullYear()} 
      </footer>
    </div>
  );
};

export default App;
