import React from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { PollData, VoteDirection } from '../types';
import OptionCard from './OptionCard';

interface PollDisplayProps {
  poll: PollData; 
  onVote: (pollId: string, optionId: string, direction: VoteDirection) => void;
}

const PollDisplay: React.FC<PollDisplayProps> = ({ poll, onVote }) => {
  const handleOptionVote = (optionId: string, direction: VoteDirection) => {
    onVote(poll['poll-id'], optionId, direction);
  };

  return (
    <motion.div 
      layout // Animate layout changes of the grid container itself
      className="w-full"
    >
      <AnimatePresence> {/* Useful if options could be added/removed dynamically, less critical for pure reordering */}
        <motion.div 
          layout // This helps animate the grid itself if its properties change
          className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 sm:gap-5"
        >
          {poll.options.map((option, index) => (
            <OptionCard 
              key={option['option-id']} // Stable key is crucial for React and Framer Motion
              option={option} 
              onVote={handleOptionVote}
              imageSeed={`${poll['poll-id']}-${option['option-id']}`} // Simpler seed, index not needed if id is unique
            />
          ))}
        </motion.div>
      </AnimatePresence>
    </motion.div>
  );
};

export default PollDisplay;