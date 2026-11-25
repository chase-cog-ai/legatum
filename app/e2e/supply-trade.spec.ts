// Supply sold by the turn (rules 60), played against the real Hard AI in the WASM engine. The game is the
// playtest 8.9.2 save at turn 203 (committed for the race panel): the human Switzerland has 5 Coal to spare, India
// (an ally) has Coal to spare, and Byzantium and Germany can use Coal. The player haggles: a lowball bid and an
// overpriced sale are turned down with the AI's price, the sale at that price is taken, the supply and the Gold
// move every turn, and ending the sale gives the Coal back.
// LEGATUM_SHOTS=<dir> also writes the screenshots used in the pull request.
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import { fileURLToPath } from 'node:url';
import type { Page } from '@playwright/test';
import type { useGame } from '../src/store';
import type { Event } from '../src/types';
import { test, expect } from './fixtures';

type GameWindow = Window & { legatum: typeof useGame };
const save = () => gunzipSync(readFileSync(fileURLToPath(new URL('./saves/p892-race-panel-t203.current.legatum.gz', import.meta.url)))).toString('utf8');
const shots = process.env.LEGATUM_SHOTS;

async function open(page: Page) {
  await page.addInitScript((save) => {
    if (!location.protocol.startsWith('http') || localStorage.getItem('legatum.e2e.seeded')) return;
    localStorage.setItem('legatum.e2e.seeded', '1');
    localStorage.setItem('legatum.autosave.v1', save);
    localStorage.setItem('legatum.first-game-hints.v2', JSON.stringify({ enabled: false, seen: [] }));
  }, save());
  await page.goto('/?client=wasm');
  await page.getByRole('button', { name: /^Continue/ }).click();
  await expect(page.getByTestId('turn-number')).toHaveText('203', { timeout: 90_000 });
  await expect(page.locator('.loading-screen')).toHaveCount(0);
}

const game = (page: Page) => page.evaluate(() => {
  const s = (window as unknown as GameWindow).legatum.getState();
  const v = s.views![s.seat];
  return {
    turn: v.turn, trades: v.supply_trade?.trades ?? [], offers: v.supply_trade?.offers ?? [],
    coal: v.supply!.find((x) => x.resource === 'Coal')!, income: v.economy.gold_per_turn.parts,
    events: ((s.report?.events?.[s.seat] ?? []) as Event[]).filter((e) => e.type.startsWith('SupplyOffer') || e.type.startsWith('SupplyTrade')),
    lines: (s.report?.lines?.[s.seat] ?? []).map((l) => [l.text, ...(l.details ?? [])].join(' | ')),
  };
});

const panel = (page: Page) => page.getByRole('complementary', { name: 'Diplomacy', exact: true });

async function openDiplomacy(page: Page) {
  for (let i = 0; i < 8 && !(await panel(page).isVisible()); i++) {
    await page.keyboard.press('Escape');
    const button = page.getByRole('button', { name: /^Diplomacy( \(\d+\))?$/ }).first();
    if (await button.isVisible()) await button.click({ timeout: 2_000 }).catch(() => undefined);
    await page.waitForTimeout(300);
  }
  await expect(panel(page)).toBeVisible();
}

async function fillOffer(page: Page, empire: string, side: 'sell' | 'buy', resource: string, amount: number, gold: number) {
  await openDiplomacy(page);
  await panel(page).locator('.empire-pick', { hasText: empire }).first().click();
  const form = panel(page).getByTestId('trade-supply');
  await form.getByTestId(`supply-side-${side}`).click();
  await form.getByLabel('Supply to trade').selectOption(resource);
  await form.getByLabel('Supply amount').fill(String(amount));
  await form.getByLabel('Gold each turn').fill(String(gold));
  return form;
}

async function endTurn(page: Page) {
  const turn = (await game(page)).turn;
  await page.keyboard.press('Escape');
  await page.locator('.seatbar').getByRole('button', { name: /^(Ready anyway|End turn|Ready\b)/ }).click();
  await page.getByRole('dialog', { name: 'End turn with decisions pending?' }).getByRole('button', { name: 'End turn anyway', exact: true })
    .click({ timeout: 3_000 }).catch(() => undefined);
  await page.waitForFunction((t) => {
    const s = (window as unknown as GameWindow).legatum.getState();
    return s.views![s.seat].turn > t && !s.busy;
  }, turn, { timeout: 120_000 });
  await page.waitForTimeout(500);
}

const shot = async (page: Page, name: string) => { if (shots) await page.screenshot({ path: `${shots}/${name}.png` }); };

test('the player haggles supply with the Hard AI: bad offers are refused with a price, fair ones run every turn', async ({ page }) => {
  test.setTimeout(420_000);
  await page.setViewportSize({ width: 1440, height: 900 });
  await open(page);
  expect((await game(page)).coal.spare).toBe(5);

  // An offer the buyer cannot pay for is refused by the engine before it is sent, in its own words.
  let form = await fillOffer(page, 'Germany', 'sell', 'Coal', 1, 500);
  await form.getByTestId('supply-offer-send').click();
  await expect(form.getByTestId('supply-offer-refused')).toContainText('Germany cannot pay 500 Gold');
  await form.getByRole('button', { name: 'Undo' }).click();

  // A lowball bid for India's Coal and an overpriced sale to Byzantium.
  form = await fillOffer(page, 'India', 'buy', 'Coal', 1, 1);
  await expect(form.getByTestId('supply-forecast')).toContainText('If India accepts');
  await form.getByTestId('supply-offer-send').click();
  form = await fillOffer(page, 'Byzantium', 'sell', 'Coal', 2, 40);
  await expect(form.getByTestId('supply-forecast')).toContainText('You keep 3 Coal to spare');
  await form.scrollIntoViewIfNeeded();
  await shot(page, '1-offer-form');
  await form.getByTestId('supply-offer-send').click();
  await expect(form.getByTestId('supply-offer-queued')).toBeVisible();
  await shot(page, '1b-offer-queued');

  await endTurn(page);
  await endTurn(page);
  let g = await game(page);
  expect(g.trades).toEqual([]);
  const said = g.lines.filter((l) => /turned down your offer/.test(l));
  expect(said.find((l) => l.includes('India'))).toMatch(/They said: we want (\d+) Gold a turn for it/);
  expect(said.find((l) => l.includes('Byzantium'))).toMatch(/They said: we would pay (\d+) Gold a turn for it/);
  const indiaAsk = Number(/we want (\d+) Gold/.exec(said.find((l) => l.includes('India'))!)![1]);
  const byzantiumBid = Number(/we would pay (\d+) Gold/.exec(said.find((l) => l.includes('Byzantium'))!)![1]);
  // The AI names a real price: never less than the fair price for what it sells, never more than twice it for what it buys.
  expect(indiaAsk).toBeGreaterThanOrEqual(8);
  expect(byzantiumBid).toBeLessThanOrEqual(32);
  expect(byzantiumBid).toBeLessThan(40);
  if (shots) {
    await page.keyboard.press('Escape');
    await page.keyboard.press('l');
    await page.getByRole('complementary', { name: 'Turn report' }).getByRole('button', { name: /Byzantium turned down your offer/ }).click({ timeout: 3_000 }).catch(() => undefined);
  }
  await shot(page, '2-declined-with-price');

  // The sale at the AI's own price is taken.
  form = await fillOffer(page, 'Byzantium', 'sell', 'Coal', 2, byzantiumBid);
  await form.scrollIntoViewIfNeeded();
  await shot(page, '2b-offer-at-their-price');
  await form.getByTestId('supply-offer-send').click();
  await endTurn(page);
  await endTurn(page);
  g = await game(page);
  expect(g.trades.map((t) => [t.resource, t.amount, t.gold])).toEqual([['Coal', 2, byzantiumBid]]);
  // Supply is counted for both sides at once: 6 from our mines, 2 sold.
  expect(g.coal.supplied.total).toBe(4);
  expect(g.coal.supplied.parts).toEqual(expect.arrayContaining([['Sold to Byzantium', -2]]));
  // The Gold moves with income, each turn.
  expect(g.income).toEqual(expect.arrayContaining([['2 Coal sold to Byzantium', byzantiumBid]]));
  await openDiplomacy(page);
  const deals = panel(page).getByTestId('supply-deals');
  await expect(deals.getByTestId('supply-trade')).toHaveCount(1);
  await deals.scrollIntoViewIfNeeded();
  await shot(page, '3-trades-running');

  // Ending the sale gives the Coal back next turn.
  await deals.getByTestId('supply-trade').filter({ hasText: 'Byzantium' }).getByTestId('supply-end').click();
  await expect(deals.getByTestId('supply-end-queued')).toBeVisible();
  await endTurn(page);
  g = await game(page);
  expect(g.trades).toEqual([]);
  expect(g.coal.supplied.total).toBe(6);
  expect(g.lines.some((l) => /Your sale of 2 Coal to Byzantium ended/.test(l))).toBe(true);
});
