import type { PlayerView } from './types';

export type ReplayMoment =
  | { kind: 'founded' | 'captured'; city: string; player: number }
  | { kind: 'war'; a: number; b: number; declarer: number | null }
  | { kind: 'eliminated'; player: number }
  | { kind: 'scored'; gained: number; total: number }
  | { kind: 'hold' | 'won'; team: number }
  | { kind: 'draw' };

export interface ReplayInfo {
  first: number;
  last: number;
  partial: boolean;
  milestones: { turn: number; moment: ReplayMoment }[];
}

export type ReplayRequest = { type: 'open'; recording: string; player: number } | { type: 'seek'; turn: number };
export type ReplayResponse =
  | { type: 'ready'; info: ReplayInfo; view: PlayerView }
  | { type: 'frame'; turn: number; view: PlayerView }
  | { type: 'failed' };

export function playbackTurn(key: string, turn: number, first: number, last: number): number | undefined {
  if (key === 'Home') return first;
  if (key === 'End') return last;
  if (key === 'ArrowLeft') return Math.max(first, turn - 1);
  if (key === 'ArrowRight') return Math.min(last, turn + 1);
  return undefined;
}

export function replayTurnLabel(turn: number, first: number, partial: boolean): string {
  if (turn !== first) return `After turn ${turn}`;
  return partial ? `Before turn ${turn + 1}` : 'Opening map';
}
