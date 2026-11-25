// Forts block the road (core `rules::forts::blocker`): a unit that steps from outside a Fort's
// reach onto a tile next to it stops for the turn, when a military unit at War with it holds the Fort.
import type { Hex, PlayerView } from './types';
import { atWar } from './diplomacy';
import { distance, hexAt, tileIndex } from './hex';

/** The tiles holding a Fort whose garrison is at War with the viewer. */
export function hostileForts(view: PlayerView): number[] {
  const out: number[] = [];
  for (const u of view.units) {
    if (!u.military || u.owner === view.me) continue;
    const i = tileIndex(u.pos, view.width, view.height);
    if (view.tiles[i]?.fort && !out.includes(i) && atWar(view, u.owner)) out.push(i);
  }
  return out;
}

/** Whether the viewer's unit stepping `from` → `to` is stopped by a Fort; undefined when no Fort can stop it. */
export function fortStops(view: PlayerView): ((from: Hex, to: Hex) => boolean) | undefined {
  const forts = hostileForts(view).map((i) => hexAt(i, view.width));
  if (!forts.length) return undefined;
  return (from, to) => forts.some((f) => distance(to, f) === 1 && distance(from, f) > 1);
}
