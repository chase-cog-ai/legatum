// @vitest-environment happy-dom
// Supply sold by the turn: an empire's row offers a sale or purchase with the forecast of both
// sides, the inbox answers offers and ends trades, and the report names each step.
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { FixtureClient } from '../test/fixtureEngine';
import { useGame } from '../store';
import { contextFromViews, formatEvents } from '../format';
import { describeOrder } from '../orders';
import type { PlayerView, SupplyOffer, SupplyTrade } from '../types';
import { SupplyDeals, TradeSupply, SUPPLY_TRADE_RULE } from './SupplyTrade';

let container: HTMLDivElement, root: Root, view: PlayerView;
beforeEach(async () => {
  vi.stubGlobal('IS_REACT_ACT_ENVIRONMENT', true);
  container = document.body.appendChild(document.createElement('div')); root = createRoot(container);
  useGame.setState(useGame.getInitialState(), true);
  const client = new FixtureClient();
  await client.newGame(await client.standardSetup('700040', 'India', 'Byzantium', 'Ada', 'Bo'));
  view = structuredClone(client.view(0));
  // Seat 1 plays Sparta and nobody else does, so it is named "Sparta".
  view.players = view.players.map((p) => p.id === 1 ? { ...p, faction: 'Sparta' } : p.faction === 'Sparta' ? { ...p, faction: 'Rome' } : p);
  view.economy = { ...view.economy, gold: 120 };
  // We have 3 Iron to spare and no Coal; Sparta uses Iron and has 2 Coal to spare.
  view.supply = (view.supply ?? []).map((s) => s.resource === 'Iron' ? { ...s, revealed: true, spare: 3 } : s.resource === 'Coal' ? { ...s, revealed: true, spare: 0 } : s);
  view.supply_trade = { partners: [{ player: 1, uses: ['Iron'], spare: [['Coal', 2]] }] };
  useGame.setState({ client, views: [view], humans: [0], orders: [[]], errors: [[]], ready: [false], seat: 0 });
});
afterEach(() => { act(() => root.unmount()); container.remove(); vi.unstubAllGlobals(); });

const q = <T extends Element = HTMLElement>(id: string) => container.querySelector<T>(`[data-testid=${id}]`)!;
const type = (label: string, value: string) => act(() => {
  const el = container.querySelector<HTMLInputElement>(`input[aria-label="${label}"]`)!;
  Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(el, value);
  el.dispatchEvent(new Event('input', { bubbles: true }));
});
const offer: SupplyOffer = { id: 4, from: 1, to: 0, selling: false, resource: 'Iron', amount: 2, gold: 12, turn: 5 };
const trade: SupplyTrade = { id: 6, seller: 0, buyer: 1, resource: 'Iron', amount: 2, gold: 12, since: 6 };

it('offers to sell spare supply with the forecast, and queues the order', () => {
  act(() => root.render(<TradeSupply view={view} to={1} />));
  expect(container.textContent).toContain(SUPPLY_TRADE_RULE);
  type('Supply amount', '2'); type('Gold each turn', '12');
  expect(q('supply-forecast').textContent).toBe('If Sparta accepts: +12 Gold each turn. You keep 1 Iron to spare.');
  act(() => q<HTMLButtonElement>('supply-offer-send').click());
  const orders = useGame.getState().orders[0];
  expect(orders).toEqual([{ type: 'OfferSupply', to: 1, selling: true, resource: 'Iron', amount: 2, gold: 12 }]);
  expect(describeOrder(orders[0], view)).toBe('Offer Sparta 2 Iron for 12 Gold a turn');
  expect(q('supply-offer-queued').textContent).toContain('offer Sparta to sell 2 Iron for 12 Gold a turn');
});

it('refuses more supply than there is to spare, and more Gold than the buyer has', () => {
  act(() => root.render(<TradeSupply view={view} to={1} />));
  type('Supply amount', '4');
  expect(q('supply-forecast').getAttribute('role')).toBe('alert');
  expect(q('supply-forecast').textContent).toBe('You have 3 Iron to spare. Trade between 1 and 3.');
  expect(q<HTMLButtonElement>('supply-offer-send').disabled).toBe(true);
  act(() => q<HTMLButtonElement>('supply-side-buy').click());
  type('Supply amount', '2'); type('Gold each turn', '121');
  expect(q('supply-forecast').textContent).toBe('You have 120 Gold. Offer no more than that a turn.');
  type('Gold each turn', '20');
  expect(q('supply-forecast').textContent).toBe('If Sparta accepts: +2 Coal supply, −20 Gold each turn.');
});

it('says why no trade is possible', () => {
  view.supply_trade = { partners: [{ player: 1, blocked: 'you are at War with Sparta' }] };
  act(() => root.render(<TradeSupply view={view} to={1} />));
  expect(q('trade-supply').textContent).toContain('No supply trade: you are at War with Sparta.');
  expect(container.querySelector('[data-testid=supply-offer-send]')).toBeNull();
});

it('answers an offer and ends a running trade from the inbox', () => {
  view.supply_trade = { offers: [offer], trades: [trade] };
  act(() => root.render(<SupplyDeals view={view} />));
  expect(q('supply-offer').textContent).toContain('Sparta asks to buy 2 Iron for 12 Gold a turn from you');
  expect(q('supply-offer-forecast').textContent).toBe('If you accept: +12 Gold each turn. You keep 1 Iron to spare.');
  act(() => q<HTMLButtonElement>('supply-accept').click());
  expect(q('supply-trade').textContent).toContain('You sell Sparta 2 Iron for 12 Gold a turn');
  act(() => q<HTMLButtonElement>('supply-end').click());
  expect(useGame.getState().orders[0]).toEqual([{ type: 'AnswerSupplyOffer', offer: 4, accept: true }, { type: 'EndSupplyTrade', trade: 6 }]);
  expect(q('supply-answer-queued').textContent).toContain('Queued: accept.');
});

it('cannot accept a sale of supply it no longer has to spare', () => {
  view.supply = view.supply!.map((s) => s.resource === 'Iron' ? { ...s, spare: 1 } : s);
  view.supply_trade = { offers: [offer] };
  act(() => root.render(<SupplyDeals view={view} />));
  expect(q<HTMLButtonElement>('supply-accept').disabled).toBe(true);
  expect(q('supply-offer').textContent).toContain('You have 1 Iron to spare, so you cannot accept.');
});

it('shows nothing in the inbox without offers or trades', () => {
  act(() => root.render(<SupplyDeals view={view} />));
  expect(container.querySelector('[data-testid=supply-deals]')).toBeNull();
});

it('the report names each step for the viewer', () => {
  const ctx = contextFromViews(null, view);
  const text = (e: Parameters<typeof formatEvents>[0][number]) => formatEvents([e], ctx)[0];
  expect(text({ type: 'SupplyOffered', offer }).text).toBe('Sparta asked to buy 2 Iron for 12 Gold a turn from you: answer in Diplomacy');
  const declined = text({ type: 'SupplyOfferDeclined', offer: { ...offer, from: 0, to: 1 }, reason: 'we want 12 Gold a turn for it' });
  expect(declined.text).toBe('Sparta turned down your offer (2 Iron for 12 Gold a turn)');
  expect(declined.details).toEqual(['They said: we want 12 Gold a turn for it.']);
  expect(text({ type: 'SupplyTradeStarted', trade }).text).toBe('You now sell Sparta 2 Iron for 12 Gold a turn');
  expect(text({ type: 'SupplyTradeEnded', trade, reason: 'you are at War' }).text).toBe('Your sale of 2 Iron to Sparta ended: 12 Gold a turn less');
});
