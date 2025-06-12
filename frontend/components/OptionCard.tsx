import React from 'react';
import { motion } from 'framer-motion';
import { PollOption, VoteDirection } from '../types';
import { MIN_USER_VOTE, MAX_USER_VOTE, MAX_USER_VOTE_EFFECT_MAGNITUDE } from '../constants';
import { useDoubleTap } from '../hooks/useDoubleTap';
import { PlusIcon, MinusIcon, MultimediaIcon, TrendingUpIcon, TrendingDownIcon } from './icons';

interface OptionCardProps {
  option: PollOption;
  onVote: (optionId: string, direction: VoteDirection) => void;
  imageSeed?: string | number; // For unique placeholder image from Unsplash
}

function getBackgroundStyle(userVote: number, imageUrl: string): React.CSSProperties {
  const normalizedVote = Math.max(MIN_USER_VOTE, Math.min(MAX_USER_VOTE, userVote));
  const intensity = normalizedVote / MAX_USER_VOTE_EFFECT_MAGNITUDE; 

  let brightnessValue = 1.0; 

  if (intensity > 0) {
    brightnessValue = 1.0 + intensity * 0.5; 
  } else if (intensity < 0) {
    brightnessValue = 1.0 + intensity * 0.5; 
  }
  
  return { 
    backgroundImage: `url(${imageUrl})`,
    filter: `blur(8px) brightness(${brightnessValue.toFixed(2)}) saturate(0.5)`,
    // Removed explicit transition here, Framer Motion will handle transitions on layout prop
  };
}

const OptionCard: React.FC<OptionCardProps> = ({ option, onVote, imageSeed }) => {
  const handleUpvote = useDoubleTap({ onDoubleTap: () => onVote(option['option-id'], VoteDirection.UP) });
  const handleDownvote = useDoubleTap({ onDoubleTap: () => onVote(option['option-id'], VoteDirection.DOWN) });

  const placeholderImageUrl = `https://source.unsplash.com/random/400x300?sig=${imageSeed || option['option-id']}&grayscale&blur=2`;
  const backgroundStyle = getBackgroundStyle(option.user_latest_vote, placeholderImageUrl);

  const textShadowClass = "shadow-black/50 dark:shadow-black/70 [text-shadow:_0_1px_2px_var(--tw-shadow-color)]";
  const textColorClass = "text-neutral-50";

  return (
    <motion.div 
      layout // Enable automatic animation when layout changes
      initial={{ opacity: 0.8, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
      exit={{ opacity: 0, scale: 0.9 }}
      transition={{ duration: 0.4, type: "spring" }}
      className="relative rounded-lg shadow-md hover:shadow-xl dark:shadow-neutral-800 dark:hover:shadow-neutral-700/70 overflow-hidden group aspect-[4/3] sm:aspect-auto sm:min-h-[220px] flex flex-col"
    >
      <div
        className="absolute inset-0 w-full h-full bg-cover bg-center -z-20"
        style={backgroundStyle}
        aria-hidden="true"
      />
      <div 
        className="absolute inset-0 w-full h-full bg-gradient-to-t from-black/60 via-black/30 to-transparent -z-10" 
        aria-hidden="true" 
      />

      <div 
        className="absolute top-0 left-0 h-full w-1/2 cursor-pointer z-10 flex items-center justify-start p-2 sm:p-3"
        onClick={handleDownvote}
        aria-label={`Double tap to decrease score for ${option.metadata.title}`}
        role="button"
        tabIndex={0}
      >
        <MinusIcon className={`w-10 h-10 sm:w-12 sm:h-12 opacity-25 group-hover:opacity-70 transition-opacity duration-200 ${textColorClass} ${textShadowClass}`} />
      </div>
      <div 
        className="absolute top-0 right-0 h-full w-1/2 cursor-pointer z-10 flex items-center justify-end p-2 sm:p-3"
        onClick={handleUpvote}
        aria-label={`Double tap to increase score for ${option.metadata.title}`}
        role="button"
        tabIndex={0}
      >
        <PlusIcon className={`w-10 h-10 sm:w-12 sm:h-12 opacity-25 group-hover:opacity-70 transition-opacity duration-200 ${textColorClass} ${textShadowClass}`} />
      </div>

      <div className={`mt-auto p-3 sm:p-4 relative z-0 ${textColorClass} ${textShadowClass}`}>
        <div className="flex justify-between items-start mb-1.5">
          <h3 className={`font-bold text-lg sm:text-xl line-clamp-2`}>{option.metadata.title}</h3>
          {option['ft-id'] && (
            <div className="ml-2 flex-shrink-0" title={`Multimedia available for ${option.metadata.title}`}>
              <MultimediaIcon className={`w-5 h-5 opacity-80`} />
            </div>
          )}
        </div>
        <p className={`text-sm sm:text-base opacity-85 line-clamp-2 sm:line-clamp-3 mb-2.5`}>{option.metadata.description}</p>
        
        <div className="flex justify-start items-center h-6"> {/* Fixed height to prevent layout shifts */}
          {option.user_latest_vote !== 0 && (
             <div className="flex items-center" title={`You voted on this item`}>
                {option.user_latest_vote > 0 ? 
                  <TrendingUpIcon className={`w-5 h-5 sm:w-6 sm:h-6`} /> : 
                  <TrendingDownIcon className={`w-5 h-5 sm:w-6 sm:h-6`} />}
             </div>
          )}
        </div>
      </div>
    </motion.div>
  );
};

export default OptionCard;