// Peace terms (core `diplomacy::PeaceTerms`, rules 63): the editor on a Peace offer and on a
// counter-offer. Each term is one row; the forecast underneath says how an AI empire would answer
// today and why (pillar 5). The core validates every term again when the order is queued.
import { useMemo, useState, type ReactNode } from 'react';
import { useGame } from '../store';
import { empireTitle } from '../empires';
import { REPARATIONS_MAX, TRUCE_CHOICES, capturedBetween, isPlain, termLines, truceTurns } from '../peaceTerms';
import { offerGoldForecast } from '../deals';
import type { Order, PeaceForecast, PeaceTerms, PlayerView, TeamId } from '../types';
import { Amounts } from './Amounts';

/** The short rule, shown under the editor. */
export const PEACE_TERMS_RULE = 'Nothing changes hands unless they accept. They can decline, or answer once with terms of their own.';

type Side = 'pay' | 'ask';
interface Draft { city: string; back: boolean; truce: number; repSide: Side; rep: string }

const whole = (text: string) => (text.trim() === '' ? 0 : Number(text));

function draftOf(t: PeaceTerms | undefined): Draft {
  return {
    city: t?.city != null ? String(t.city) : '', back: !!t?.return_captured, truce: truceTurns(t),
    repSide: (t?.reparations ?? 0) < 0 ? 'ask' : 'pay', rep: t?.reparations ? String(Math.abs(t.reparations)) : '',
  };
}

/** The draft as core terms, written from the viewer's side (the viewer proposes). */
export function termsOf(d: Draft): PeaceTerms {
  const rep = whole(d.rep);
  return {
    ...(d.city !== '' ? { city: Number(d.city) } : {}),
    ...(d.back ? { return_captured: true } : {}),
    ...(d.truce !== TRUCE_CHOICES[0] ? { truce: d.truce } : {}),
    ...(rep ? { reparations: d.repSide === 'pay' ? rep : -rep } : {}),
  };
}

/** Why the draft cannot be sent, in the player's words; null when it can. */
export function draftError(view: PlayerView, d: Draft): string | null {
  const rep = whole(d.rep);
  if (!Number.isInteger(rep) || rep < 0 || rep > REPARATIONS_MAX) return `Gold each turn is a whole number up to ${REPARATIONS_MAX}.`;
  const income = view.economy.gold_per_turn.total;
  if (d.repSide === 'pay' && rep > Math.max(0, income)) return `Gold each turn comes out of your income: ${income} Gold a turn.`;
  return null;
}

const ANSWER: Record<PeaceForecast['answer'], string> = { Accept: 'would accept this today', Refuse: 'would decline this today', Counter: 'would answer with other terms today' };

/**
 * The editor. `to` is the team the terms go to; `start` the terms it opens on (a counter-offer
 * opens on the offer it answers, seen from this side). `gold` is the deal's Gold from the viewer's
 * side (the `DealGold` field): an offer shares the empire's field above it; a counter-offer has
 * its own, passed as `goldField` and shown as the first row. `send` queues the order.
 */
export function PeaceTermsEditor({ view, to, start, gold, goldField, orders = [], sendLabel, send, onlyWithTerms = false }: {
  view: PlayerView; to: TeamId; start?: PeaceTerms; gold: number; goldField?: ReactNode; orders?: Order[]; sendLabel: string; send: (terms: PeaceTerms) => void; onlyWithTerms?: boolean;
}) {
  const client = useGame((s) => s.client);
  const [d, setD] = useState<Draft>(() => draftOf(start));
  const set = (patch: Partial<Draft>) => setD((old) => ({ ...old, ...patch }));
  const them = empireTitle(view, to);
  const teamOf = (p: number) => view.players.find((x) => x.id === p)?.team;
  const mine = view.cities.filter((c) => c.owner === view.me && !c.is_capital);
  const theirs = view.cities.filter((c) => teamOf(c.owner) === to && !c.is_capital);
  const captured = capturedBetween(view, view.team, to);
  const error = offerGoldForecast(view, gold, orders)?.error ?? draftError(view, d);
  const terms = termsOf(d);
  const target = view.players.find((p) => p.team === to && p.alive)?.id;
  const key = JSON.stringify(terms);
  const forecast = useMemo((): PeaceForecast | null => {
    if (error || target === undefined || !client?.peaceForecast) return null;
    try { return client.peaceForecast(view.me, target, gold, JSON.parse(key)); } catch { return null; }
    // The forecast reads the game the client holds; a new turn brings a new view.
  }, [client, error, target, gold, key, view.me, view.turn]);
  const lines = termLines(view, terms, view.team, to, gold);
  return <div className="peace-terms" data-testid="peace-terms">
    {goldField}
    {(mine.length > 0 || theirs.length > 0) && <div className="peace-term">
      <label htmlFor={`pt-city-${to}`}>City</label>
      <select id={`pt-city-${to}`} value={d.city} onChange={(e) => set({ city: e.target.value })}>
        <option value="">No city</option>
        {mine.length > 0 && <optgroup label="You hand over">{mine.map((c) => <option key={c.id} value={c.id}>{c.name} (population {c.population})</option>)}</optgroup>}
        {theirs.length > 0 && <optgroup label={`${them} hands over`}>{theirs.map((c) => <option key={c.id} value={c.id}>{c.name} (population {c.population})</option>)}</optgroup>}
      </select>
    </div>}
    {captured.length > 0 && <label className="peace-term peace-term-check">
      <input type="checkbox" checked={d.back} onChange={(e) => set({ back: e.target.checked })} />
      <span>Both sides give back captured cities ({captured.map((c) => c.name).join(', ')})</span>
    </label>}
    <div className="peace-term" role="group" aria-label="Truce length">
      <span className="peace-term-label">Truce</span>
      <span className="peace-truce">{TRUCE_CHOICES.map((n) => <button key={n} type="button" className="btn peace-truce-pick" aria-pressed={d.truce === n} onClick={() => set({ truce: n })}>{n} turns</button>)}</span>
    </div>
    <div className="peace-term">
      <label htmlFor={`pt-rep-${to}`}>Gold each turn</label>
      <select aria-label="Who pays Gold each turn" value={d.repSide} onChange={(e) => set({ repSide: e.target.value as Side })}>
        <option value="pay">You pay</option><option value="ask">{them} pays</option>
      </select>
      <input id={`pt-rep-${to}`} type="number" inputMode="numeric" min={0} max={REPARATIONS_MAX} step={1} placeholder="0" value={d.rep} onChange={(e) => set({ rep: e.target.value })} />
    </div>
    {error
      ? <p className="order-error" role="alert" data-testid="peace-terms-error">{error}</p>
      : lines.length > 0 && <ul className="peace-terms-summary" data-testid="peace-terms-summary">{lines.map((l) => <li key={l.text} className={l.tone}><Amounts text={l.text} /></li>)}</ul>}
    {forecast && <div className={`peace-forecast ${forecast.answer.toLowerCase()}`} role="status" data-testid="peace-forecast">
      <strong>{them} {ANSWER[forecast.answer]}.</strong>
      {forecast.reasons.length > 0 && <ul>{forecast.reasons.map((r) => <li key={r}><Amounts text={r} /></li>)}</ul>}
      {forecast.answer === 'Counter' && forecast.counter && <p className="muted">Their terms: {termLines(view, forecast.counter, to, view.team, forecast.counter_gold ?? 0).map((l) => l.text).join('. ') || 'a plain Peace'}.</p>}
      <p className="muted">They answer next turn; the war may look different to them by then.</p>
    </div>}
    <button className="btn primary" type="button" disabled={!!error || (onlyWithTerms && isPlain(terms) && !gold)} onClick={() => send(terms)} data-testid="peace-terms-send">{sendLabel}</button>
    <p className="muted peace-terms-rule">{PEACE_TERMS_RULE}</p>
  </div>;
}
