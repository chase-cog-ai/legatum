import { expect, it } from 'vitest';
import { RULES, TECHS } from '../data';
import type { PlayerView, TechId, TechOption } from '../types';
import { goalForecast, treeColumns, treeLinks } from './TechTree';

const eraOf = (t: TechId): number => (RULES.techs as unknown as Record<TechId, { era: number }>)[t].era;
const catalog: TechOption[] = TECHS.map((tech) => ({ tech, era: eraOf(tech), cost: { total: 100, parts: [] } as never, turns: 1, researchable: false, known_by: 0 }));

it('lays out every technology once, five to an era, with the spear line on top', () => {
  const cols = treeColumns(catalog);
  expect(cols[1][0]).toBe('Mathematics');
  expect(cols[2][0]).toBe('Feudalism');
  expect(cols.flat().sort()).toEqual([...TECHS].sort());
  expect(cols.every((c) => c.length === 5)).toBe(true);
});

it('links each unit and building line forward across eras, never inside one era', () => {
  const links = treeLinks(eraOf);
  expect(links.length).toBeGreaterThan(20);
  for (const l of links) expect(eraOf(l.to)).toBeGreaterThan(eraOf(l.from));
  expect(new Set(links.map((l) => l.line))).toEqual(new Set(['Melee', 'Ranged', 'Mounted', 'Air', 'Economy']));
});

it('forecasts a goal from every unknown technology before its era, at one a turn at least', () => {
  const cols = treeColumns(catalog);
  const goal = cols[2][0];
  const view = { turn: 10, economy: { science_per_turn: { total: 1000 } }, research: { known: cols[0], catalog, progress: 0, era: 1 } } as unknown as PlayerView;
  const f = goalForecast(view, goal, cols);
  expect(f.before).toBe(5);
  expect(f.turns).toBe(6);
  expect(f.arrival).toBe(16);
  const slow = { ...view, economy: { science_per_turn: { total: 50 } } } as unknown as PlayerView;
  expect(goalForecast(slow, goal, cols).turns).toBe(12);
  const none = { ...view, economy: { science_per_turn: { total: 0 } } } as unknown as PlayerView;
  expect(goalForecast(none, goal, cols).arrival).toBeNull();
});
