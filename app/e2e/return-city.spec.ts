import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import { fileURLToPath } from 'node:url';
import type { useGame } from '../src/store';
import { test, expect } from './fixtures';
import { endPlanning } from './turnActions';

type GameWindow = Window & { legatum: typeof useGame };
const SAKAI = 18, INDIA = 2, SPARTA = 7, CROSSBOWMAN = 82;

// Turn 117 of a recorded game: Sparta holds Sakai, which India founded. The position is edited so
// the player has just taken Sakai from Sparta and garrisons it, at Peace with India.
function save(): string {
  const file = JSON.parse(gunzipSync(readFileSync(fileURLToPath(new URL('./saves/sidebar-mid-t117.current.legatum.gz', import.meta.url)))).toString('utf8'));
  const game = JSON.parse(file.game);
  const list = <T,>(x: T[] | Record<string, T>): T[] => (Array.isArray(x) ? x : Object.values(x));
  const city = list<any>(game.cities).find((c) => c.id === SAKAI);
  expect([city.owner, city.founder]).toEqual([SPARTA, INDIA]);
  Object.assign(city, { owner: 0, occupied_from: SPARTA, resistance: 0, producing: null, production_queue: [] });
  for (const tile of game.map.tiles) if (tile.city === SAKAI) tile.owner = 0;
  for (const unit of list<any>(game.units)) {
    if (unit.owner === SPARTA && unit.pos.q === city.pos.q && unit.pos.r === city.pos.r) unit.pos = { q: 20, r: 17 };
    if (unit.id === CROSSBOWMAN) Object.assign(unit, { pos: { ...city.pos }, standing_order: null, fortify: 0 });
  }
  return JSON.stringify({ ...file, game: JSON.stringify(game), unseen: {} });
}

test('a city taken from a third empire goes back to its founder for 20 Gold per population', async ({ page }) => {
  test.setTimeout(180_000);
  await page.addInitScript((save) => {
    if (!location.protocol.startsWith('http') || localStorage.getItem('legatum.autosave.v1')) return;
    localStorage.setItem('legatum.autosave.v1', save);
    localStorage.setItem('legatum.first-game-hints.v2', JSON.stringify({ enabled: false, seen: [] }));
  }, save());
  await page.goto('/?client=wasm');
  await page.getByRole('button', { name: /^Continue/ }).click();
  await expect(page.getByTestId('turn-number')).toHaveText('117', { timeout: 90_000 });
  await expect(page.locator('.loading-screen')).toHaveCount(0);
  const seen = () => page.evaluate(([id, unit]) => {
    const s = (window as unknown as GameWindow).legatum.getState();
    const v = s.views![s.seat]!;
    const c = v.cities.find((x) => x.id === id)!;
    const u = v.units.find((x) => x.id === unit)!;
    return { owner: c.owner, population: c.population, gold: v.players.find((p) => p.id === v.me)!.gold ?? 0, guardInCity: u.pos.q === c.pos.q && u.pos.r === c.pos.r };
  }, [SAKAI, CROSSBOWMAN]);
  const before = await seen();
  expect(before).toMatchObject({ owner: 0, population: 5, guardInCity: true });

  await page.evaluate((id) => (window as unknown as GameWindow).legatum.getState().select({ kind: 'city', id }, true), SAKAI);
  const block = page.getByTestId('city-return');
  await expect(block).toContainText('Return to India');
  await expect(block).toContainText('gain 100 Gold, twice a Raze');
  await block.getByRole('button', { name: 'Return city' }).click();
  const confirm = page.getByTestId('return-confirm');
  await expect(confirm).toContainText('India gets the city whole');
  await expect(confirm).toContainText('You gain 100 Gold.');
  await confirm.getByRole('button', { name: 'Confirm Return' }).click();
  await expect(block.getByRole('button', { name: /Return ordered/ })).toBeVisible();
  await expect(page.getByTestId('city-raze').getByRole('button')).toBeDisabled();

  await page.keyboard.press('Escape');
  await endPlanning(page);
  await expect(page.getByTestId('turn-number')).toHaveText('118', { timeout: 120_000 });
  const after = await seen();
  // India runs Sakai again at its full population, and the garrison stands outside.
  expect(after).toMatchObject({ owner: INDIA, population: 5, guardInCity: false });
  expect(after.gold).toBeGreaterThanOrEqual(before.gold + 100);
  await page.evaluate(() => (window as unknown as GameWindow).legatum.getState().set({ reportOpen: true }));
  await expect(page.getByRole('complementary', { name: 'Turn report' })).toContainText(/You returned Sakai to India and gained\s*100/);
});
