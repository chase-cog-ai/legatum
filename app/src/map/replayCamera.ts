import { hexAt, hexToPixel, SQRT3 } from '../hex';
import type { PlayerView } from '../types';
import { MAX_ZOOM } from './gestures';
import { HEX_SIZE, worldBounds, type Camera } from './render';

export function replayBounds(view: PlayerView) {
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  const halfWidth = HEX_SIZE * SQRT3 / 2;
  view.tiles.forEach((tile, index) => {
    if (!tile.explored) return;
    const { x, y } = hexToPixel(hexAt(index, view.width), HEX_SIZE);
    minX = Math.min(minX, x - halfWidth);
    maxX = Math.max(maxX, x + halfWidth);
    minY = Math.min(minY, y - HEX_SIZE);
    maxY = Math.max(maxY, y + HEX_SIZE);
  });
  return Number.isFinite(minX) ? { minX, minY, maxX, maxY } : worldBounds(view);
}

export function fitReplayCamera(bounds: ReturnType<typeof replayBounds>, w: number, h: number): Camera {
  const padding = Math.min(28, Math.min(w, h) / 8);
  return {
    x: (bounds.minX + bounds.maxX) / 2,
    y: (bounds.minY + bounds.maxY) / 2,
    zoom: Math.min(MAX_ZOOM, (w - 2 * padding) / (bounds.maxX - bounds.minX), (h - 2 * padding) / (bounds.maxY - bounds.minY)),
  };
}
