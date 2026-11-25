import { expect, it } from 'vitest';
import { hexAt, hexToPixel, SQRT3 } from '../hex';
import type { PlayerView } from '../types';
import { MAX_ZOOM } from './gestures';
import { fitReplayCamera, replayBounds } from './replayCamera';
import { HEX_SIZE, worldToScreen } from './render';

it('centers seen terrain, not the world, and fits it at either canvas size', () => {
  const width = 30;
  const seen = [0, 3 * width + 8];
  const view = {
    width, height: 20,
    tiles: Array.from({ length: width * 20 }, (_, index) => ({ explored: seen.includes(index) })),
  } as PlayerView;
  const bounds = replayBounds(view);
  const far = hexToPixel(hexAt(seen[1], width), HEX_SIZE);
  expect(bounds).toEqual({
    minX: -HEX_SIZE * SQRT3 / 2, maxX: far.x + HEX_SIZE * SQRT3 / 2,
    minY: -HEX_SIZE, maxY: far.y + HEX_SIZE,
  });
  for (const [w, h] of [[320, 200], [540, 180]]) {
    const cam = fitReplayCamera(bounds, w, h);
    const start = worldToScreen(cam, w, h, bounds.minX, bounds.minY);
    const end = worldToScreen(cam, w, h, bounds.maxX, bounds.maxY);
    expect(start.x).toBeCloseTo(w - end.x);
    expect(start.y).toBeCloseTo(h - end.y);
    expect(Math.min(start.x, start.y)).toBeCloseTo(Math.min(w, h) / 8);
  }
  expect(fitReplayCamera(bounds, 5000, 5000).zoom).toBe(MAX_ZOOM);
});
