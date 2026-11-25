import { describe, expect, it } from 'vitest';
import { parseAiTimeline, rivalStories, type AiTimeline } from './rivalPlans';
import type { PostGame, ReportEmpire } from './postgame';
import type { PlayerView } from './types';

const empire = (id: number, name: string, mine = false): ReportEmpire =>
  ({ id, team: id, faction: 'Athens', name, label: mine ? 'You' : name, ink: '#000', mine, winner: false, alive: true });
const pg = { empires: [empire(0, 'Switzerland', true), empire(1, 'Sparta'), empire(2, 'Japan')] } as unknown as PostGame;
const view = { me: 0, team: 0 } as unknown as PlayerView;
const bern = { id: 3, name: 'Bern', owner: 0 };
const kyoto = { id: 9, name: 'Kyoto', owner: 2 };

const timeline: AiTimeline = { seats: [{
  player: 1,
  plans: [
    { turn: 1, plan: 'economy', against: [], city: null, reason: 'start', chance: 5_956 },
    { turn: 40, plan: 'campaign', against: [0], city: bern, reason: 'better', chance: 6_100 },
    { turn: 52, plan: 'peace', against: [0], city: null, reason: 'infeasible', chance: 4_000 },
    { turn: 70, plan: 'campaign', against: [2], city: { id: 11, name: null, owner: null }, reason: 'better', chance: 4_500 },
  ],
  campaigns: [
    { turn: 40, city: bern, end: null },
    { turn: 52, city: bern, end: 'walls_before_arrival' },
    { turn: 90, city: kyoto, end: null },
    { turn: 96, city: kyoto, end: 'captured' },
  ],
  threats: [
    { kind: 'top_threat', turn: 33, rival: 0 },
    { kind: 'treaty', turn: 35, with: [2], treaty: 'Alliance', accepted: false, rival: 0 },
    { kind: 'treaty', turn: 36, with: [2], treaty: 'Alliance', accepted: false, rival: 0 },
    { kind: 'walls', turn: 36, city: { id: 5, name: 'Sparta', owner: 1 }, rival: 0 },
    { kind: 'second_front', turn: 41, against: [0], rival: 0 },
  ],
}] };

describe('rival plans', () => {
  const [story] = rivalStories(view, pg, timeline);
  const lines = story.entries.map((e) => `${e.turn} ${e.text}`);

  it('tells one rival\'s decisions in turn order, in plain sentences', () => {
    expect(lines).toEqual([
      '1 Began by growing its own empire',
      '33 Came to fear you most',
      '35 Offered Japan an Alliance, wary of you',
      '36 Started Walls in Sparta against you',
      '40 Set out to take Bern from you',
      '41 Declared war on you, the rival it feared most',
      '52 Called off the march on Bern: Walls went up before it arrived',
      '52 Decided to seek peace with you, because its last plan could no longer work',
      '70 Set out to take a city of Japan (since razed)',
      '90 Marched on Kyoto',
      '96 Took Kyoto',
    ]);
  });

  it('says a march once: the plan that names the city stands for its start', () => {
    expect(lines.filter((l) => l.includes('Bern') && l.startsWith('40'))).toHaveLength(1);
  });

  it('marks what was aimed at the viewer and counts it', () => {
    expect(story.entries.find((e) => e.text.startsWith('Set out to take Bern'))?.tone).toBe('against');
    expect(story.entries.find((e) => e.text === 'Took Kyoto')?.tone).toBe('neutral');
    expect(story.againstYou).toBe(story.entries.filter((e) => e.tone === 'against').length);
    expect(story.againstYou).toBeGreaterThan(0);
  });

  it('carries the rival\'s own win estimate on plan changes only', () => {
    expect(story.entries[0].chance).toBe(60);
    expect(story.entries.find((e) => e.text === 'Took Kyoto')?.chance).toBeUndefined();
  });

  it('sums the story up', () => {
    expect(story.summary).toBe('Feared you most at the end; marched on 2 cities and took 1.');
  });

  it('has nothing to show without a record', () => {
    expect(parseAiTimeline(null)).toBeNull();
    expect(parseAiTimeline('not json')).toBeNull();
    expect(parseAiTimeline('{}')).toBeNull();
    expect(rivalStories(view, pg, null)).toEqual([]);
    expect(rivalStories(view, pg, { seats: [{ player: 1, plans: [], campaigns: [], threats: [] }] })).toEqual([]);
  });
});
