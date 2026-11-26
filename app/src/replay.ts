import type { PlayerView } from './types';

export type ReplayMoment =
  | { kind: 'founded' | 'captured'; city: string; player: number | null }
  | { kind: 'war'; a: number | null; b: number | null }
  | { kind: 'declaredWar'; from: number | null; against: number | null }
  | { kind: 'eliminated'; player: number | null }
  | { kind: 'scored'; gained: number; total: number }
  | { kind: 'hold' | 'won'; team: number | null }
  | { kind: 'draw' };

export interface ReplayInfo {
  first: number;
  last: number;
  partial: boolean;
  milestones: { turn: number; moment: ReplayMoment }[];
}

export type ReplayRequest = { type: 'open'; recording: string; player: number; profile: boolean } | { type: 'seek'; turn: number };
export type ReplayResponse =
  | { type: 'ready'; info: ReplayInfo; view: PlayerView }
  | { type: 'frame'; turn: number; view: PlayerView; timing?: [wasmMs: number, parseMs: number, sentAt: number] }
  | { type: 'failed' };

const REPLAY_TURN_MS = 1200;

export function replayProfilingEnabled(): boolean {
  return new URLSearchParams(window.location.search).has('replayProfile');
}

export function nextReplayDeadline(previous: number | null, now: number, speed: number): number {
  const interval = REPLAY_TURN_MS / speed;
  return Math.max((previous ?? now) + interval, now + interval / 2);
}

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
