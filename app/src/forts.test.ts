import { describe, expect, it } from 'vitest';
import { fortStops, hostileForts } from './forts';
import { reachTiles } from './map/reach';
import { pathTurns } from './path';
import { fromOffset, tileIndex } from './hex';
import type { Hex, PlayerView, TileView } from './types';

const W = 12, H = 5;
/** Open grassland; player 1's Spearman stands in a Fort at offset (6, 2). `war` sets the relation. */
function view(war: boolean, garrison = true): PlayerView {
  const tiles: TileView[] = Array.from({ length: W * H }, () => ({ explored: true, visible: true, terrain: 'Grassland', ruin: false, owner: null }));
  const fort = fromOffset(6, 2);
  tiles[tileIndex(fort, W, H)].fort = true;
  return {
    width: W, height: H, tiles, me: 0, team: 0, cities: [],
    players: [{ id: 0, team: 0 }, { id: 1, team: 1 }],
    diplomacy: { relations: [{ a: 0, b: 1, state: war ? 'War' : 'Peace' }] },
    units: garrison ? [{ id: 9, owner: 1, pos: fort, kind: 'Spearman', military: true }] : [],
  } as unknown as PlayerView;
}
const row = (from: number, to: number): Hex[] => Array.from({ length: to - from }, (_, k) => fromOffset(from + 1 + k, 2));

describe('Forts block the road (core rules::forts::blocker)', () => {
  it('only a Fort held by an enemy at War stops a step, and only on entering its reach', () => {
    expect(hostileForts(view(true))).toEqual([tileIndex(fromOffset(6, 2), W, H)]);
    expect(fortStops(view(false))).toBeUndefined();
    expect(fortStops(view(true, false))).toBeUndefined();
    const stops = fortStops(view(true))!;
    expect(stops(fromOffset(4, 2), fromOffset(5, 2))).toBe(true);
    // Already beside the Fort: walking round it is free.
    expect(stops(fromOffset(5, 2), fromOffset(5, 1))).toBe(false);
    expect(stops(fromOffset(2, 2), fromOffset(3, 2))).toBe(false);
  });

  it('the route counts the stop as the end of the turn', () => {
    const from = fromOffset(3, 2), path = row(3, 5);
    const cost = () => 1;
    const open = pathTurns(path, cost, 4, 4, from);
    expect(open.at(-1)!.turn).toBe(1);
    const blocked = pathTurns([...path, fromOffset(5, 1)], cost, 4, 4, from, undefined, undefined, undefined, fortStops(view(true)));
    expect(blocked.find((s) => s.hex.q === fromOffset(5, 2).q && s.hex.r === fromOffset(5, 2).r)!.turn).toBe(1);
    expect(blocked.at(-1)!.turn).toBe(2);
  });

  it('the reach overlay ends at the tiles next to the Fort', () => {
    const u = { pos: fromOffset(3, 2), kind: 'Horseman', max_moves: 6 } as never;
    // The tile behind the Fort: every way there first enters the Fort's reach.
    const far = tileIndex(fromOffset(7, 2), W, H);
    expect(reachTiles(view(false), u).has(far)).toBe(true);
    const reach = reachTiles(view(true), u);
    expect(reach.has(tileIndex(fromOffset(5, 2), W, H))).toBe(true);
    expect(reach.has(far)).toBe(false);
  });
});
