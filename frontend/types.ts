export interface OptionMetadata {
  title: string;
  description: string;
}

export interface PollOption {
  "option-id": string;
  "ft-id"?: string; // For multimedia link indicator
  metadata: OptionMetadata;
  score: number;
  user_latest_vote: number; // User's specific vote contribution, clamped
}

export interface PollData {
  "poll-id": string;
  header: string;
  options: PollOption[];
}

export enum VoteDirection {
  UP = 'UP',
  DOWN = 'DOWN',
}