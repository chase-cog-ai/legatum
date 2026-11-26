import { createHash } from 'node:crypto';
import { existsSync } from 'node:fs';
import { expect, it } from 'vitest';
import { unmetPlayer, unmetTeam } from '../empires';
import type { ReplayInfo } from '../replay';
import type { PlayerView } from '../types';
import { loadReplayEngine, replayFixture } from './replayFixture';

function mapDigest({ tiles, units, cities, diplomacy }: PlayerView) {
  return createHash('sha256').update(JSON.stringify({ tiles, units, cities, diplomacy })).digest('hex');
}

it.skipIf(!existsSync(new URL('../wasm-pkg/legatum_bg.wasm', import.meta.url)))('replays the original fog on every fixture turn and redacts unknown turning-point actors', async () => {
  const original: string[] = [];
  const { game } = await replayFixture((view) => original.push(mapDigest(view)));
  const { Game, GameReplay } = await loadReplayEngine();
  const finished = Game.load(game);
  try {
    const replay = new GameReplay(finished.replay_recording(0), 0);
    try {
      const { first, last, milestones } = JSON.parse(replay.info()) as ReplayInfo;
      expect(first).toBe(0);
      expect(last + 1).toBe(original.length);
      expect(milestones.length).toBeGreaterThan(0);
      for (let turn = first; turn <= last; turn++) {
        const view = JSON.parse(replay.frame(turn)) as PlayerView;
        expect(mapDigest(view), `map after turn ${turn}`).toBe(original[turn]);
        for (const { moment } of milestones.filter((entry) => entry.turn === turn)) {
          if ('player' in moment && moment.player !== null) expect(unmetPlayer(view, moment.player)).toBe(false);
          if ('team' in moment && moment.team !== null) expect(unmetTeam(view, moment.team)).toBe(false);
          const teams = moment.kind === 'war' ? [moment.a, moment.b]
            : moment.kind === 'declaredWar' ? [moment.from, moment.against] : [];
          for (const team of teams) if (team !== null) expect(unmetTeam(view, team)).toBe(false);
        }
      }
    } finally {
      replay.free();
    }
  } finally {
    finished.free();
  }
}, 120_000);
