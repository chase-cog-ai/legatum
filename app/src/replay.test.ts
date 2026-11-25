import { expect, it } from 'vitest';
import { playbackTurn, replayTurnLabel } from './replay';

it('keeps replay shortcuts within the recorded turns', () => {
  expect(playbackTurn('ArrowLeft', 14, 14, 45)).toBe(14);
  expect(playbackTurn('ArrowRight', 45, 14, 45)).toBe(45);
  expect(playbackTurn('ArrowLeft', 28, 14, 45)).toBe(27);
  expect(playbackTurn('ArrowRight', 28, 14, 45)).toBe(29);
  expect(playbackTurn('Home', 28, 14, 45)).toBe(14);
  expect(playbackTurn('End', 28, 14, 45)).toBe(45);
  expect(playbackTurn('Tab', 28, 14, 45)).toBeUndefined();
});

it('distinguishes the opening and a partial anchor from a resolved turn', () => {
  expect(replayTurnLabel(0, 0, false)).toBe('Opening map');
  expect(replayTurnLabel(14, 14, true)).toBe('Before turn 15');
  expect(replayTurnLabel(15, 14, true)).toBe('After turn 15');
});
