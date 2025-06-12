export const BASE_VOTE_INCREMENT = 1; // Base change in user_latest_vote per tap before streak bonus
export const RAPID_VOTE_WINDOW_MS = 350; // Milliseconds window for a tap to be considered "rapid"
export const MAX_STREAK_MULTIPLIER = 5; // Maximum multiplier for vote increment due to streak

export const MIN_USER_VOTE = -127;
export const MAX_USER_VOTE = 127;

// This constant defines the magnitude of user_latest_vote at which the visual brightness effect is at its maximum/minimum.
// Setting it to MAX_USER_VOTE means the full brightness/darkness effect is achieved when the vote is +/-127.
export const MAX_USER_VOTE_EFFECT_MAGNITUDE = 127;