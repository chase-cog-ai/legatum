// A Fort dug by a unit (core `rules::forts`, `TileView.fort`): an earth rampart with a timber
// palisade and corner towers around the tile's centre, drawn on the ground under the unit that
// holds it. A Fort held by an enemy at War with the viewer also rings the tiles next to it,
// where the viewer's units must stop.
import type { PlayerView } from '../types';
import { centerX, centerY, HEX_SIZE } from './grid';
import { hostileForts } from '../forts';
import type { Camera } from './render';

const EARTH = '#6b5a3e', TIMBER = '#c9a877', INK = '#2b2118', STOP = '#d9483b';

function fort(ctx: CanvasRenderingContext2D, x: number, y: number, r: number, alpha: number): void {
  const rx = r * 0.66, ry = r * 0.5;
  ctx.save();
  ctx.globalAlpha = alpha;
  ctx.lineJoin = 'round';
  // Rampart: a dark earth band, then the palisade on its crest.
  ctx.beginPath();
  ctx.ellipse(x, y, rx, ry, 0, 0, Math.PI * 2);
  ctx.lineWidth = Math.max(3, r * 0.2);
  ctx.strokeStyle = INK;
  ctx.stroke();
  ctx.lineWidth = Math.max(2, r * 0.15);
  ctx.strokeStyle = EARTH;
  ctx.stroke();
  ctx.lineWidth = Math.max(1, r * 0.055);
  ctx.strokeStyle = TIMBER;
  ctx.setLineDash([Math.max(1.5, r * 0.07), Math.max(1, r * 0.035)]);
  ctx.stroke();
  ctx.setLineDash([]);
  // Corner towers.
  const t = Math.max(2.5, r * 0.13);
  for (let k = 0; k < 6; k++) {
    const a = ((60 * k) * Math.PI) / 180;
    const tx = x + rx * Math.cos(a), ty = y + ry * Math.sin(a);
    ctx.fillStyle = TIMBER;
    ctx.strokeStyle = INK;
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.rect(tx - t / 2, ty - t * 0.8, t, t);
    ctx.fill();
    ctx.stroke();
  }
  ctx.restore();
}

/** Draw after improvements, before units and city banners. */
export function drawForts(ctx: CanvasRenderingContext2D, view: PlayerView, cam: Camera, width: number, height: number, dpr: number): void {
  if (!view.tiles.some((t) => t.fort)) return;
  const r = HEX_SIZE * cam.zoom, W = view.width;
  const at = (i: number) => ({ x: (centerX(i, W) - cam.x) * cam.zoom + width / 2, y: (centerY(i, W) - cam.y) * cam.zoom + height / 2 });
  ctx.save();
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  for (let i = 0; i < view.tiles.length; i++) {
    const tile = view.tiles[i];
    if (!tile.fort || !tile.explored) continue;
    const { x, y } = at(i);
    if (x < -r * 2 || x > width + r * 2 || y < -r * 2 || y > height + r * 2) continue;
    fort(ctx, x, y, r, tile.visible ? 1 : 0.6);
  }
  // The stop ring: one dashed line round the six tiles next to an enemy-held Fort.
  for (const i of hostileForts(view)) {
    const { x, y } = at(i);
    if (x < -r * 4 || x > width + r * 4 || y < -r * 4 || y > height + r * 4) continue;
    ctx.beginPath();
    ctx.arc(x, y, r * 2.45, 0, Math.PI * 2);
    ctx.lineWidth = 2;
    ctx.setLineDash([6, 5]);
    ctx.strokeStyle = STOP;
    ctx.globalAlpha = 0.8;
    ctx.stroke();
  }
  ctx.restore();
}
