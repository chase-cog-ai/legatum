// Peace terms (core `diplomacy::PeaceTerms`, rules 63, deals only): what a Peace offer carries
// besides the Peace and the deal's Gold (`deals.ts`), and the plain sentences that say it from the viewer's side. The core validates
// and settles every term; nothing here decides a rule.
import { empireOfPlayer, empireTitle } from './empires';
import { RULES } from './data';
import type { PeaceProposal, PeaceTerms, PlayerView, Relation, TeamId } from './types';

export const TRUCE_CHOICES: number[] = RULES.treaty.truceChoices;
export const REPARATIONS_MAX: number = RULES.treaty.reparationsMax;

export const isPlain = (t: PeaceTerms | undefined | null) => !t || !(t.city != null || t.return_captured || t.truce || t.reparations);
export const truceTurns = (t: PeaceTerms | undefined | null) => t?.truce || RULES.treaty.guaranteeTurns;

/** The same deal written from the other side (core `PeaceTerms::mirrored`). */
export const mirrored = (t: PeaceTerms): PeaceTerms => ({ ...t, ...(t.reparations ? { reparations: -t.reparations } : {}) });

export interface TermLine { tone: 'cost' | 'gain' | 'note'; text: string; icon?: string }

/** Cities each side of `a` and `b` would give back: held now, captured from the other (core `captured_between`). */
export function capturedBetween(view: PlayerView, a: TeamId, b: TeamId) {
  const team = (p: number) => view.players.find((x) => x.id === p)?.team;
  return view.cities.filter((c) => c.occupied_from != null && !c.is_capital
    && ((team(c.owner) === a && team(c.occupied_from) === b) || (team(c.owner) === b && team(c.occupied_from) === a)));
}

/**
 * The terms as the viewer's costs and gains. `proposer` is the team the terms are written from;
 * `other` the team that would accept. A viewer on neither team reads them as the proposer's.
 * `gold` is the deal's Gold from the proposer's side, for a draft that states it with the terms.
 */
export function termLines(view: PlayerView, terms: PeaceTerms | undefined | null, proposer: TeamId, other: TeamId, gold = 0): TermLine[] {
  if (isPlain(terms) && !gold) return [];
  const t = terms ?? {};
  const mine = view.team === proposer;
  const them = empireTitle(view, mine ? other : proposer);
  const lines: TermLine[] = [];
  if (gold) {
    const pay = (gold > 0) === mine;
    lines.push({ tone: pay ? 'cost' : 'gain', icon: 'yield-gold', text: pay ? `You pay ${them} ${Math.abs(gold)} Gold when Peace is signed` : `${them} pays you ${Math.abs(gold)} Gold when Peace is signed` });
  }
  if (t.city != null) {
    const c = view.cities.find((x) => x.id === t.city);
    const give = c ? view.players.find((p) => p.id === c.owner)?.team === view.team : false;
    lines.push({ tone: give ? 'cost' : 'gain', icon: 'emblem-captured', text: give ? `You hand ${c?.name ?? 'a city'} over to ${them}` : `${them} hands ${c?.name ?? 'a city'} over to you` });
  }
  if (t.return_captured) {
    for (const c of capturedBetween(view, proposer, other)) {
      const give = view.players.find((p) => p.id === c.owner)?.team === view.team;
      lines.push({ tone: give ? 'cost' : 'gain', icon: 'emblem-captured', text: give ? `You give ${c.name} back to ${empireOfPlayer(view, c.occupied_from!)}` : `${empireOfPlayer(view, c.owner)} gives ${c.name} back to you` });
    }
  }
  if (t.reparations) {
    const pay = (t.reparations > 0) === mine;
    const n = Math.abs(t.reparations), turns = truceTurns(t);
    lines.push({ tone: pay ? 'cost' : 'gain', icon: 'yield-gold', text: pay ? `You pay ${them} ${n} Gold each turn for ${turns} turns` : `${them} pays you ${n} Gold each turn for ${turns} turns` });
  }
  if (t.truce && t.truce !== RULES.treaty.guaranteeTurns) lines.push({ tone: 'note', text: `The truce lasts ${t.truce} turns. Neither side can declare War before it ends.` });
  return lines;
}

/** Peace with `other` frees the viewer's units locked in cities captured from it (core `rules::occupation::lock`). */
export function freedUnitsLine(view: PlayerView, other: TeamId): TermLine | null {
  const n = view.units.filter((u) => u.owner === view.me && u.occupation_lock != null
    && view.players.find((p) => p.id === u.occupation_lock)?.team === other).length;
  return n ? { tone: 'gain', text: `Peace frees ${n} ${n === 1 ? 'unit' : 'units'} holding captured cities` } : null;
}

/** The terms of an offer, for whoever reads it. */
export const offerTermLines = (view: PlayerView, p: PeaceProposal) => termLines(view, p.terms, p.from, p.to);

/** What a signed Peace still holds: the truce's end and any reparations (the treaty list). */
export function liveTerms(view: PlayerView, r: Relation): string[] {
  if (r.state !== 'Peace') return [];
  const out: string[] = [];
  if (r.guarantee_until > view.turn) out.push(`Truce through turn ${r.guarantee_until - 1}`);
  const rep = r.reparations;
  if (rep && rep.until > view.turn) {
    const left = rep.until - view.turn;
    const from = rep.from === view.me ? 'You pay' : `${empireOfPlayer(view, rep.from)} pays`;
    const to = rep.to === view.me ? 'you' : empireOfPlayer(view, rep.to);
    out.push(`${from} ${to} ${rep.gold} Gold each turn, ${left} more ${left === 1 ? 'turn' : 'turns'}`);
  }
  if (r.truce_released != null) out.push(`${empireTitle(view, r.truce_released)} may declare War: the reparations stopped`);
  return out;
}
