// Luxury tiles (rules 64): six flavour tiles on the map, each with a painted medallion, a small
// tile bonus and one line of history on the tile card. The real WASM core loads the position.
import { test as base, expect, type Page } from '@playwright/test';
import { createServer } from 'vite';
import { mkdir } from 'node:fs/promises';
import type { useGame } from '../src/store';
import { distance, hexAt, tileIndex } from '../src/hex';
import { LUXURY_RESOURCES, LUXURY_STORY, RESOURCE_INFO } from '../src/resources';
import type { Hex, Resource } from '../src/types';
import { coreFixture } from '../src/test/coreFixture';
import { mapArtFixture } from '../src/test/mapArtFixture';

type MapWindow = Window & { legatum: typeof useGame; legatumMap: { camera: { zoom: number }; redraw(): void; toScreen(h: Hex): { x: number; y: number } } };
const SHOTS = 'test-results/luxury-tiles';

/** The art-review map with one of each luxury on its own terrain, as near the Indian capital as the land allows. */
function luxuryFixture() {
  const setup = mapArtFixture();
  const g = setup.fixture;
  const capital = g.cities.find((c) => c.owner === 0)!.pos;
  const taken: Hex[] = [];
  const placed = LUXURY_RESOURCES.map((luxury) => {
    const at = g.terrain
      .map((terrain, i) => ({ terrain, i, hex: hexAt(i, g.width) }))
      .filter(({ terrain, i, hex }) => RESOURCE_INFO[luxury].terrains.includes(terrain as never)
        && !g.resource?.[i] && !g.ruin[i] && !g.wonder?.[i]
        && !g.cities.some((c) => distance(c.pos, hex) === 0) && !g.units.some((u) => distance(u.pos, hex) === 0)
        && distance(capital, hex) >= 2 && taken.every((t) => distance(t, hex) >= 2))
      .sort((a, b) => distance(capital, a.hex) - distance(capital, b.hex) || a.i - b.i)[0];
    if (!at) throw new Error(`no ${luxury} tile on the fixture map`);
    g.resource![at.i] = luxury;
    taken.push(at.hex);
    return { luxury, hex: at.hex };
  });
  return { game: coreFixture(g), capital, placed, width: g.width, height: g.height };
}

const fixture = luxuryFixture();

const test = base.extend<{}, { devURL: string }>({
  devURL: [async ({}, use) => {
    const server = await createServer({ cacheDir: `${SHOTS}/vite-cache`, server: { host: '127.0.0.1', port: 0 } });
    await server.listen();
    try {
      const address = server.httpServer!.address();
      if (!address || typeof address === 'string') throw new Error('Vite did not bind');
      await use(`http://127.0.0.1:${address.port}`);
    } finally { await server.close(); }
  }, { scope: 'worker' }],
  baseURL: async ({ devURL }, use) => { await use(devURL); },
});

async function load(page: Page, size: { width: number; height: number }) {
  await page.setViewportSize(size);
  await page.addInitScript(({ game }) => {
    localStorage.setItem('legatum.autosave.v1', JSON.stringify({ format: 'legatum-save', version: 1, client: 'wasm', turn: 12, names: ['Ada'], savedAt: new Date().toISOString(), game }));
    localStorage.setItem('legatum.first-game-hints.v2', JSON.stringify({ enabled: false, seen: [] }));
  }, { game: fixture.game });
  await page.goto('/?client=wasm');
  await page.getByRole('button', { name: /^Continue/ }).click();
  await expect(page.getByTestId('turn-number')).toHaveText('12');
}

async function lookAt(page: Page, at: Hex, zoom: number) {
  await page.evaluate(({ at, zoom }) => {
    const w = window as unknown as MapWindow;
    w.legatum.getState().jumpTo(at);
    w.legatumMap.camera.zoom = zoom;
    w.legatumMap.redraw();
  }, { at, zoom });
  await page.waitForTimeout(500);
}

async function hover(page: Page, hex: Hex) {
  const card = page.locator('.tilecard');
  await expect.poll(async () => {
    const at = await page.evaluate((h) => (window as unknown as MapWindow).legatumMap.toScreen(h), hex);
    const box = (await page.locator('canvas[data-game-map]').boundingBox())!;
    await page.mouse.move(box.x + at.x + 3, box.y + at.y + 3);
    await page.mouse.move(box.x + at.x, box.y + at.y, { steps: 3 });
    return card.isVisible();
  }, { intervals: [300, 600, 1000, 2000] }).toBe(true);
  return card;
}

const bonusText = (r: Resource) => (['food', 'production', 'gold', 'science'] as const)
  .filter((k) => RESOURCE_INFO[r].bonus[k]).map((k) => `+${RESOURCE_INFO[r].bonus[k]} ${k[0].toUpperCase()}${k.slice(1)}`).join(', ');

for (const size of [{ name: '1000x600', width: 1000, height: 600 }, { name: '1440x900', width: 1440, height: 900 }] as const) {
  test(`the core's view carries every luxury and each tile card tells its bonus and story at ${size.name}`, async ({ page }) => {
    await load(page, size);
    await mkdir(SHOTS, { recursive: true });
    // The real core accepted the position and shows each luxury to the viewer from the start (no technology hides it).
    const seen = await page.evaluate(({ placed, width, height }) => {
      const view = (window as unknown as MapWindow).legatum.getState().views![0];
      return placed.map(({ hex }) => { const row = hex.r, col = hex.q + Math.floor((row - (row & 1)) / 2); return row >= 0 && row < height ? view.tiles[row * width + col].resource : null; });
    }, { placed: fixture.placed, width: fixture.width, height: fixture.height });
    expect(seen).toEqual(fixture.placed.map((p) => p.luxury));
    for (const { luxury, hex } of fixture.placed) {
      expect(tileIndex(hex, fixture.width, fixture.height)).toBeGreaterThanOrEqual(0);
      await lookAt(page, hex, 1.4);
      const card = await hover(page, hex);
      await expect(card.locator('.tilecard-lead-name')).toHaveText(luxury);
      await expect(card.locator('.tilecard-lead-kind')).toHaveText('Luxury resource');
      await expect(card.locator('.tilecard-benefits')).toContainText(`${bonusText(luxury)} when a citizen works the tile`);
      await expect(card.locator('.tilecard-benefits')).toContainText('A luxury needs no improvement');
      await expect(card.getByTestId('tile-luxury-story')).toHaveText(LUXURY_STORY[luxury]!);
      await expect(card).not.toContainText('Dormant');
      const r = (await card.boundingBox())!;
      expect(r.x).toBeGreaterThanOrEqual(0); expect(r.y).toBeGreaterThanOrEqual(0);
      expect(r.x + r.width).toBeLessThanOrEqual(size.width); expect(r.y + r.height).toBeLessThanOrEqual(size.height);
      await page.screenshot({ path: `${SHOTS}/${size.name}-${luxury.toLowerCase()}.png`, animations: 'disabled' });
      await page.mouse.move(2, 2);
    }
  });
}

test('the Guide lists the six luxuries with their bonus and history', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto('/?client=wasm');
  await page.getByRole('button', { name: /^How to Play/ }).click();
  const guide = page.getByRole('dialog', { name: 'How to Play', exact: true });
  await guide.getByRole('navigation', { name: 'How to Play chapters' }).getByRole('button').filter({ hasText: 'Cities & research' }).click();
  const heading = guide.getByRole('heading', { name: 'Luxuries: rare finds', exact: true });
  await heading.scrollIntoViewIfNeeded();
  await expect(heading).toBeVisible();
  for (const luxury of LUXURY_RESOURCES) {
    const row = guide.getByRole('row').filter({ hasText: LUXURY_STORY[luxury]! });
    await expect(row).toContainText(luxury);
    await expect(row).toContainText(bonusText(luxury));
  }
  await mkdir(SHOTS, { recursive: true });
  await page.screenshot({ path: `${SHOTS}/guide.png`, animations: 'disabled' });
});
