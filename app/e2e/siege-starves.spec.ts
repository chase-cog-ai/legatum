import { mkdir } from 'node:fs/promises';
import type { Page } from '@playwright/test';
import type { useGame } from '../src/store';
import type { Hex } from '../src/types';
import { test, expect } from './fixtures';
import { coreFixture } from '../src/test/coreFixture';
import { siegeFixture } from '../src/test/siegeFixture';
import { spawnUnit, refreshVision } from '../src/test/fixtureEngine';
import { neighbors, hexEq, tileIndex } from '../src/hex';

// Siege starves the land (rules 63): an army at War takes the yield of the enemy tile it stands on.
// The tile card says so, the city's Food drops, and a city cut off from its land shows the
// starvation countdown with the cause. At the user's laptop window and a desktop one, on the 2D map and the 3D map.
type GameWindow = Window & {
  legatum: typeof useGame;
  legatumMap: { toScreen(hex: Hex): { x: number; y: number } };
  legatumMap3d: { project(hex: Hex): { x: number; y: number } | null };
};
const VIEWS = ['2d', '3d'] as const;
const SIZES = [{ width: 1000, height: 600 }, { width: 1440, height: 900 }] as const;
const shots = process.env.SIEGE_STARVES_SHOTS;

/** Waypost Seven behind Walls (the AI's Spearmen never assault a walled city), `ring` enemy Spearmen on the tiles beside it. */
function besieged(ring: number) {
  const setup = siegeFixture();
  const f = setup.fixture;
  const own = f.cities.find((c) => c.id === setup.own)!;
  own.population = 4;
  own.food = 4;
  own.hp = 200;
  own.lastAttackedTurn = undefined;
  const held = f.units.filter((u) => setup.attackers.includes(u.id)).map((u) => u.pos);
  for (const h of neighbors(own.pos)) {
    if (held.length >= ring) break;
    if (f.terrain[tileIndex(h, f.width, f.height)] === undefined || f.units.some((u) => hexEq(u.pos, h))) continue;
    try { spawnUnit(f, 1, 'Spearman', h, false); held.push(h); } catch { /* impassable */ }
  }
  refreshVision(f);
  return { ...setup, held, cityPos: own.pos };
}

async function start(page: Page, setup: ReturnType<typeof besieged>, view: (typeof VIEWS)[number]) {
  await page.addInitScript(({ game, view }) => {
    if (!location.protocol.startsWith('http')) return;
    localStorage.setItem('legatum.autosave.v1', JSON.stringify({ format: 'legatum-save', version: 1, client: 'wasm', turn: 2, names: ['Ada'], savedAt: new Date().toISOString(), game }));
    localStorage.setItem('legatum.first-game-hints.v2', JSON.stringify({ enabled: false, seen: [] }));
    localStorage.setItem('legatum.map-view', view);
  }, { game: coreFixture(setup.fixture), view });
  await page.goto('/?client=wasm');
  await page.getByRole('button', { name: /^Continue/ }).click();
  await expect(page.getByTestId('turn-number')).toHaveText('2');
  await expect(page.locator('.loading-screen')).toHaveCount(0);
  await expect(page.locator('.map3d-canvas')).toHaveCount(view === '3d' ? 1 : 0);
}

const city = (page: Page, id: number) => page.evaluate((id) => {
  const s = (window as unknown as GameWindow).legatum.getState();
  const v = s.views![s.seat];
  const c = v.cities.find((x) => x.id === id)!;
  return { food: c.detail!.yields.total.food, starve: c.detail!.turns_to_starve ?? null, labels: c.detail!.yields.sources.map((x) => x.label), holding: v.units.filter((u) => u.holds_tile).length };
}, id);

async function shot(page: Page, size: { width: number; height: number }, name: string) {
  if (!shots) return;
  await mkdir(shots, { recursive: true });
  await page.evaluate(() => document.fonts.ready.then(() => undefined));
  await page.waitForTimeout(300);
  await page.screenshot({ path: `${shots}/${name}-${size.width}x${size.height}.png`, animations: 'disabled' });
}

for (const view of VIEWS) for (const size of SIZES) test.describe(`siege starves the land ${view} ${size.width}x${size.height}`, () => {
  test.use({ viewport: size });
  // The 3D map draws in software in CI.
  if (view === '3d') test.slow();

  test('an army at War on a city tile takes its yield, and the tile card and the city say so', async ({ page }) => {
    const open = besieged(0), setup = besieged(6);
    // The same city with no army beside it, from the engine: what the land yields when it is free.
    await start(page, { ...open, fixture: { ...open.fixture, units: open.fixture.units.filter((u) => !open.attackers.includes(u.id)) } }, view);
    const free = await city(page, open.own);
    expect(free.labels.some((l) => l.startsWith('An enemy army holds the tile'))).toBe(false);

    await page.evaluate((id) => {
      const s = (window as unknown as GameWindow).legatum.getState();
      s.set({ techOpen: false });
      s.select({ kind: 'city', id }, true);
    }, open.own);
    await expect(page.locator('.city-panel').getByTestId('city-growth')).toBeVisible();
    await shot(page, size, (view === '3d' ? '3d-' : '') + 'city-free');
    await page.evaluate(() => localStorage.clear());
    await start(page, setup, view);
    const under = await city(page, setup.own);
    expect(under.holding).toBeGreaterThanOrEqual(setup.held.length);
    expect(under.food).toBeLessThan(free.food);
    expect(under.labels.filter((l) => l.startsWith('An enemy army holds the tile')).length).toBe(setup.held.length);
    expect(under.starve).not.toBeNull();

    // The tile card names it on the held tile.
    await page.evaluate((hex) => {
      const s = (window as unknown as GameWindow).legatum.getState();
      s.set({ techOpen: false });
      s.select(null);
      s.jumpTo(hex);
    }, setup.cityPos);
    await page.waitForTimeout(700);
    // A held tile below the city, clear of its banner.
    const target = setup.held.reduce((a, b) => (b.r > a.r || (b.r === a.r && b.q < a.q) ? b : a));
    await expect.poll(async () => {
      const at = await page.evaluate(({ hex, view }) => {
        const w = window as unknown as GameWindow;
        const p = view === '3d' ? w.legatumMap3d.project(hex)! : w.legatumMap.toScreen(hex);
        const b = document.querySelector(view === '3d' ? '.map3d-canvas' : '[data-game-map]')!.getBoundingClientRect();
        return { x: Math.round(p.x + b.left), y: Math.round(p.y + b.top) };
      }, { hex: target, view });
      await page.mouse.move(at.x + 3, at.y + 3);
      await page.mouse.move(at.x, at.y, { steps: 3 });
      return page.getByTestId('tile-enemy-held').isVisible();
    }, { intervals: [500, 1000, 1000, 2000] }).toBe(true);
    await expect(page.getByTestId('tile-enemy-held')).toHaveText('An enemy army holds the tile: it yields nothing.');
    await shot(page, size, (view === '3d' ? '3d-' : '') + 'tile-card');

    // The city: the starvation countdown, with the cause in its Growth breakdown.
    await page.evaluate((id) => {
      const s = (window as unknown as GameWindow).legatum.getState();
      s.set({ techOpen: false });
      s.select({ kind: 'city', id }, true);
    }, setup.own);
    const panel = page.locator('.city-panel');
    await expect(panel.getByTestId('city-growth')).toContainText(`−1 population in ${under.starve}`);
    await shot(page, size, (view === '3d' ? '3d-' : '') + 'city-starving');
    await panel.getByTestId('city-growth').click();
    await expect(page.locator('.bd-note', { hasText: 'drive it off to get their Food back' })).toBeVisible();
    await expect(page.locator('.bd-note', { hasText: `An enemy army holds ${setup.held.length} of its tiles` })).toBeVisible();
    await shot(page, size, (view === '3d' ? '3d-' : '') + 'city-starving-why');
  });
});
