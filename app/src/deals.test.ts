// Deals (rules 61): the pure helpers, the order lines and the turn report lines.
import { describe, expect, it } from 'vitest';
import { bordersOpen, clampDealGold, freeGold, goldForViewer, incomingOfferHeadline, offerGoldForecast, sentOfferGold, withDealGold } from './deals';
import { canEnter, endsOffers, sameDiplomaticAction } from './diplomacy';
import { formatEvents, type ReportContext } from './format';
import type { Event, PeaceProposal, PlayerView } from './types';

const view = (gold: number, open: [number, number][] = []) => ({
  team: 0, me: 0, economy: { gold }, players: [{ id: 0, team: 0 }, { id: 1, team: 1 }, { id: 2, team: 2 }],
  courier_halls: [],
  diplomacy: { deals: true, open_borders: open, legacy: false,
    relations: [{ a: 0, b: 1, state: 'Peace' }, { a: 0, b: 2, state: 'Peace' }] },
}) as unknown as PlayerView;

describe('deal Gold', () => {
  it('clamps to whole Gold within 999 either way, and drops 0 from the order', () => {
    expect(clampDealGold(1500)).toBe(999);
    expect(clampDealGold(-1500)).toBe(-999);
    expect(clampDealGold(12.7)).toBe(12);
    expect(clampDealGold(NaN)).toBe(0);
    expect(withDealGold({ type: 'ProposeTreaty', to: 1, kind: 'Peace' }, 40)).toEqual({ type: 'ProposeTreaty', to: 1, kind: 'Peace', gold: 40 });
    expect(withDealGold({ type: 'ProposeTreaty', to: 1, kind: 'Peace', gold: 40 }, 0)).toEqual({ type: 'ProposeTreaty', to: 1, kind: 'Peace' });
  });

  it('forecasts who pays, when, and from which treasury', () => {
    expect(offerGoldForecast(view(120), 0, [])).toBeNull();
    expect(offerGoldForecast(view(120), 40, [])).toEqual({ tone: 'cost', text: 'You pay 40 Gold when they accept. You have 120 Gold.' });
    // Gifts already queued come out of the same treasury.
    expect(offerGoldForecast(view(120), 40, [{ type: 'GiveGold', to: 1, amount: 100 }])).toEqual({
      tone: 'cost', text: 'You pay 40 Gold when they accept. You have 20 Gold.', error: 'You have 20 Gold, so offer at most 20.' });
    // So is Gold promised in another queued offer.
    expect(offerGoldForecast(view(120), 40, [{ type: 'ProposeTreaty', to: 2, kind: 'OpenBorders', gold: 100 }])?.error).toBe('You have 20 Gold, so offer at most 20.');
    expect(freeGold(view(120), [{ type: 'ProposeTreaty', to: 2, kind: 'OpenBorders', gold: -100 }, { type: 'GiveGold', to: 1, amount: 30 }])).toBe(90);
    expect(offerGoldForecast(view(120), -40, [])).toEqual({ tone: 'gain', text: 'They pay you 40 Gold when they accept.' });
  });

  it('names an incoming offer by its Gold, from the reader’s side', () => {
    const p = { from: 1, kind: 'Peace' as const };
    expect(incomingOfferHeadline('Sparta', { ...p, gold: 40 })).toBe('Sparta offers Peace and 40 Gold');
    expect(incomingOfferHeadline('Sparta', { ...p, gold: -40 })).toBe('Sparta asks 40 Gold for Peace');
    expect(incomingOfferHeadline('Sparta', { kind: 'Alliance' })).toBe('Sparta offers an Alliance');
    expect(incomingOfferHeadline('Sparta', { kind: 'OpenBorders', gold: -5 })).toBe('Sparta asks 5 Gold for Open Borders');
    expect(goldForViewer(view(0), { ...p, gold: 40 })).toBe(40);
    expect(goldForViewer(view(0), { from: 0, gold: 40 })).toBe(-40);
    expect(sentOfferGold({ gold: 40 })).toBe('You pay 40 Gold if they accept.');
    expect(sentOfferGold({ gold: -40 })).toBe('You ask them for 40 Gold.');
    expect(sentOfferGold({})).toBeNull();
  });

  it('treats the same treaty with other Gold as the same offer; closing borders ends no offer', () => {
    expect(sameDiplomaticAction({ type: 'ProposeTreaty', to: 1, kind: 'Peace', gold: 40 }, { type: 'ProposeTreaty', to: 1, kind: 'Peace' })).toBe(true);
    expect(sameDiplomaticAction({ type: 'ProposeTreaty', to: 1, kind: 'Peace' }, { type: 'ProposeTreaty', to: 1, kind: 'OpenBorders' })).toBe(false);
    expect(endsOffers({ type: 'BreakTreaty', with: 1, agreement: 'OpenBorders' })).toBe(false);
    expect(endsOffers({ type: 'BreakTreaty', with: 1, agreement: 'Security' })).toBe(true);
  });
});

describe('Open Borders passage', () => {
  it('lets the viewer’s units into the land of an empire it has Open Borders with, not into its cities', () => {
    const v = view(0, [[0, 1]]);
    expect(bordersOpen(v, 1)).toBe(true);
    expect(bordersOpen(v, 2)).toBe(false);
    expect(canEnter(v, 'Spearman', 1)).toBe(true);
    expect(canEnter(v, 'Spearman', 2)).toBe(false);
    expect(canEnter(v, 'Spearman', 1, true)).toBe(false);
  });
});

describe('report lines', () => {
  const ctx = { me: 0, teamId: 0, team: [0], empireName: (t: number) => ['Rome', 'Sparta', 'Egypt'][t],
    playerName: (p: number) => ['Rome', 'Sparta', 'Egypt'][p], cityPos: () => undefined } as unknown as ReportContext;
  const proposal = (gold: number): PeaceProposal => ({ id: 3, from: 1, to: 0, proposer: 1, kind: 'OpenBorders', created_turn: 4, respond_by: 6, relation_version: 0, gold });
  const texts = (events: Event[]) => formatEvents(events, ctx).map((l) => l.text);

  it('a deal payment says what it paid for', () => {
    const lines = texts([
      { type: 'ProposalAccepted', proposal: proposal(-40), effective_turn: 6 },
      { type: 'GoldGiven', from: 0, to: 1, amount: 40 },
      { type: 'BordersOpened', a: 0, b: 1, effective_turn: 6 },
    ]);
    expect(lines.some((t) => /You gave .*Sparta 40 Gold for Open Borders/.test(t))).toBe(true);
    expect(lines, lines.join(' | ')).toContain('Open Borders with Sparta begin.');
    expect(formatEvents([{ type: 'BordersOpened', a: 0, b: 1, effective_turn: 6 }], ctx)[0].treaty).toBe('OpenBorders');
  });

  it('a plain gift says nothing about a deal; a closed border names who closed it', () => {
    const lines = texts([
      { type: 'GoldGiven', from: 1, to: 0, amount: 25 },
      { type: 'BordersClosed', a: 0, b: 1, effective_turn: 9, reason: 'Sparta closed the borders' },
    ]);
    expect(lines.some((t) => /gave you 25 Gold$/.test(t))).toBe(true);
    expect(lines).toContain('Sparta closed the borders.');
    expect(texts([{ type: 'BordersClosed', a: 0, b: 1, effective_turn: 9, reason: 'They went to War' }])).toEqual(['Open Borders with Sparta end: War.']);
  });

  it('an offer that arrives with Gold says so', () => {
    const lines = texts([{ type: 'ProposalCreated', proposal: proposal(-40) }]);
    expect(lines[0]).toContain('they ask 40 Gold');
  });
});
