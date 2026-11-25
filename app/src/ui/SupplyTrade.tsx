// Supply sold by the turn (core `supply_trade`, rules 61): an empire with strategic supply to spare
// sells it to another for Gold each turn. The diplomacy panel lists the offers to answer and the
// running trades, and each empire's row has the offer form with the forecast of both sides
// (pillar 5). The core decides what is legal; this file only shows the numbers its view carries.
import { useState } from 'react';
import { useGame } from '../store';
import { RULES } from '../data';
import { empireOfPlayer } from '../empires';
import { withThe } from '../resources';
import type { Order, PlayerId, PlayerView, Resource, SupplyOffer, SupplyTrade } from '../types';
import { Amount } from './Amounts';
import type { AmountUnit } from './amountParts';

/** The short rule, shown under the form. */
export const SUPPLY_TRADE_RULE = 'The seller’s spare supply counts as the buyer’s, for Gold each turn. War between you ends the trade, and either of you can end it at any time.';

const MAX_AMOUNT: number = RULES.supplyTrade.maxAmount;
const MAX_GOLD: number = RULES.supplyTrade.maxGold;

const them = (view: PlayerView, p: PlayerId) => withThe(empireOfPlayer(view, p));
const cap = (s: string) => s.charAt(0).toUpperCase() + s.slice(1);
const spareOf = (view: PlayerView, r: Resource) => view.supply?.find((s) => s.resource === r)?.spare ?? 0;
const edit = (fn: (os: Order[]) => Order[]) => useGame.getState().editOrders(fn);

function Terms({ amount, resource, gold }: { amount: number; resource: Resource; gold: number }) {
  return <><Amount n={amount} unit={resource as AmountUnit} /> for <Amount n={gold} unit="Gold" /> a turn</>;
}

/** Offers to answer, offers sent and running trades. Nothing renders while there are none. */
export function SupplyDeals({ view }: { view: PlayerView }) {
  const orders = useGame((s) => s.orders[s.seat] ?? []);
  const st = view.supply_trade;
  const offers = st?.offers ?? [], trades = st?.trades ?? [];
  if (!offers.length && !trades.length) return null;
  const incoming = offers.filter((o) => o.to === view.me), sent = offers.filter((o) => o.from === view.me);
  return <section className="supply-deals" aria-label="Supply trades" data-testid="supply-deals">
    <h4 className="diplomacy-section">Supply trades</h4>
    {incoming.map((o) => <IncomingOffer key={o.id} o={o} view={view} orders={orders} />)}
    {sent.map((o) => <article className="diplomacy-item supply-deal" key={o.id} data-testid="supply-sent">
      <strong>You offered {them(view, o.to)} {o.selling ? 'to sell' : 'to buy'} <Terms {...o} /></strong>
      <p className="muted">Awaiting their reply by turn {o.turn + RULES.supplyTrade.offerTurns}.</p>
    </article>)}
    {trades.map((t) => <RunningTrade key={t.id} t={t} view={view} orders={orders} />)}
  </section>;
}

function IncomingOffer({ o, view, orders }: { o: SupplyOffer; view: PlayerView; orders: Order[] }) {
  const answer = orders.find((x): x is Extract<Order, { type: 'AnswerSupplyOffer' }> => x.type === 'AnswerSupplyOffer' && x.offer === o.id);
  const reply = (accept: boolean) => edit((os) => [...os.filter((x) => !(x.type === 'AnswerSupplyOffer' && x.offer === o.id)), { type: 'AnswerSupplyOffer', offer: o.id, accept }]);
  const undo = () => edit((os) => os.filter((x) => !(x.type === 'AnswerSupplyOffer' && x.offer === o.id)));
  const from = them(view, o.from), spare = spareOf(view, o.resource);
  // `selling` is the sender's side: they sell to us, or they buy from us.
  const short = !o.selling && spare < o.amount, poor = o.selling && view.economy.gold < o.gold;
  return <article className="diplomacy-item supply-deal" data-testid="supply-offer">
    <strong>{cap(from)} {o.selling ? <>offers to sell you <Terms {...o} /></> : <>asks to buy <Terms {...o} /> from you</>}</strong>
    <p data-testid="supply-offer-forecast">{o.selling
      ? <>If you accept: <Amount sign="+" n={o.amount} unit={o.resource as AmountUnit} /> supply, <Amount sign="−" n={o.gold} unit="Gold" /> each turn.</>
      : <>If you accept: <Amount sign="+" n={o.gold} unit="Gold" /> each turn. You keep <Amount n={Math.max(0, spare - o.amount)} unit={o.resource as AmountUnit} /> to spare.</>}</p>
    {short && <p className="order-error">You have <Amount n={Math.max(0, spare)} unit={o.resource as AmountUnit} /> to spare, so you cannot accept.</p>}
    {poor && <p className="order-error">You have <Amount n={Math.max(0, view.economy.gold)} unit="Gold" />, so you cannot accept.</p>}
    {answer
      ? <p data-testid="supply-answer-queued">Queued: {answer.accept ? 'accept' : 'decline'}. <button className="link" onClick={undo}>Undo</button></p>
      : <div className="diplomacy-actions-row supply-actions">
          <button className="btn primary" disabled={short || poor || view.finished} onClick={() => reply(true)} data-testid="supply-accept">Accept</button>
          <button className="btn quiet" disabled={view.finished} onClick={() => reply(false)} data-testid="supply-decline">Decline</button>
        </div>}
    <p className="muted">The offer lapses after turn {o.turn + RULES.supplyTrade.offerTurns}.</p>
  </article>;
}

function RunningTrade({ t, view, orders }: { t: SupplyTrade; view: PlayerView; orders: Order[] }) {
  const selling = t.seller === view.me;
  const ending = orders.some((x) => x.type === 'EndSupplyTrade' && x.trade === t.id);
  const end = () => edit((os) => [...os, { type: 'EndSupplyTrade', trade: t.id }]);
  const undo = () => edit((os) => os.filter((x) => !(x.type === 'EndSupplyTrade' && x.trade === t.id)));
  return <article className="diplomacy-item supply-deal" data-testid="supply-trade">
    <strong>{selling ? <>You sell {them(view, t.buyer)} <Terms {...t} /></> : <>You buy <Terms {...t} /> from {them(view, t.seller)}</>}</strong>
    <p className="muted">Since turn {t.since}. War between you ends it.</p>
    {ending
      ? <p data-testid="supply-end-queued">Queued: end this trade. <button className="link" onClick={undo}>Undo</button></p>
      : <button className="btn quiet" disabled={view.finished} onClick={end} data-testid="supply-end">End trade</button>}
  </article>;
}

/** The offer form on one empire's row: sell it your spare supply, or buy what it has to spare. */
export function TradeSupply({ view, to }: { view: PlayerView; to: PlayerId }) {
  const orders = useGame((s) => s.orders[s.seat] ?? []);
  const errors = useGame((s) => s.errors[s.seat] ?? []);
  const partner = view.supply_trade?.partners?.find((p) => p.player === to);
  const theirSpare = new Map(partner?.spare ?? []);
  const uses = partner?.uses ?? [];
  // What can change hands: our spare supply they can use, and their spare supply we can use.
  const sellable = (view.supply ?? []).filter((s) => s.revealed && s.spare > 0 && uses.includes(s.resource)).map((s) => s.resource);
  const buyable = (view.supply ?? []).filter((s) => s.revealed && (theirSpare.get(s.resource) ?? 0) > 0).map((s) => s.resource);
  const [selling, setSelling] = useState(sellable.length > 0 || buyable.length === 0);
  const choices = selling ? sellable : buyable;
  const [picked, setPicked] = useState<Resource | null>(null);
  const resource = picked && choices.includes(picked) ? picked : choices[0] ?? null;
  const [amountText, setAmount] = useState('1');
  const [goldText, setGold] = useState('10');
  if (!partner || view.finished) return null;
  const name = them(view, to);
  const queued = orders.find((o): o is Extract<Order, { type: 'OfferSupply' }> => o.type === 'OfferSupply' && o.to === to);
  const refused = queued && errors.find((e) => orders[e.index] === queued)?.message;
  const open = (view.supply_trade?.offers ?? []).some((o) => (o.from === view.me && o.to === to) || (o.from === to && o.to === view.me));
  const head = <h5 className="supply-trade-head">Trade supply</h5>;
  if (partner.blocked) return <div className="supply-trade" data-testid="trade-supply">{head}<p className="diplomacy-blocked-reason">No supply trade: {partner.blocked}.</p></div>;
  if (queued) return <div className="supply-trade" data-testid="trade-supply">{head}
    <p data-testid="supply-offer-queued">Queued: offer {name} {queued.selling ? 'to sell' : 'to buy'} <Terms {...queued} />. <button className="link" onClick={() => edit((os) => os.filter((o) => o !== queued))}>Undo</button></p>
    {refused && <p className="order-error" role="alert" data-testid="supply-offer-refused">This offer will not be sent: {refused}.</p>}
  </div>;
  if (!sellable.length && !buyable.length) return <div className="supply-trade" data-testid="trade-supply">{head}
    <p className="muted">Nothing to trade now: neither of you has supply to spare that the other can use.</p>
  </div>;
  const max = Math.min(MAX_AMOUNT, resource ? (selling ? spareOf(view, resource) : theirSpare.get(resource) ?? 0) : 0);
  const amount = amountText.trim() === '' ? NaN : Number(amountText), gold = goldText.trim() === '' ? NaN : Number(goldText);
  const amountOk = Number.isInteger(amount) && amount >= 1 && amount <= max;
  const goldOk = Number.isInteger(gold) && gold >= 0 && gold <= MAX_GOLD && (selling || gold <= view.economy.gold);
  const valid = !!resource && amountOk && goldOk;
  const offer = () => { if (valid) edit((os) => [...os, { type: 'OfferSupply', to, selling, resource: resource!, amount, gold }]); };
  return <div className="supply-trade" data-testid="trade-supply">{head}
    <div className="supply-trade-sides" role="group" aria-label="Sell or buy">
      <button className={`btn${selling ? ' primary' : ' quiet'}`} aria-pressed={selling} disabled={!sellable.length} onClick={() => setSelling(true)} data-testid="supply-side-sell">Sell</button>
      <button className={`btn${!selling ? ' primary' : ' quiet'}`} aria-pressed={!selling} disabled={!buyable.length} onClick={() => setSelling(false)} data-testid="supply-side-buy">Buy</button>
    </div>
    <div className="gift-control supply-trade-form">
      <label>Supply <select value={resource ?? ''} onChange={(e) => setPicked(e.target.value as Resource)} aria-label="Supply to trade">
        {choices.map((r) => <option key={r} value={r}>{r}</option>)}
      </select></label>
      <label>Amount <input type="number" inputMode="numeric" min={1} max={Math.max(1, max)} step={1} value={amountText} aria-label="Supply amount" onChange={(e) => setAmount(e.target.value)} /></label>
      <label>Gold a turn <input type="number" inputMode="numeric" min={0} max={MAX_GOLD} step={1} value={goldText} aria-label="Gold each turn" onChange={(e) => setGold(e.target.value)} /></label>
      <button className="btn" disabled={!valid || open} onClick={offer} data-testid="supply-offer-send">{selling ? 'Offer to sell' : 'Offer to buy'}</button>
      {open
        ? <p className="gift-forecast muted" data-testid="supply-forecast">An offer between you is already open. Wait for its answer.</p>
        : valid
          ? <p className="gift-forecast" data-testid="supply-forecast">{selling
              ? <>If {name} accept{/s$/.test(empireOfPlayer(view, to)) ? '' : 's'}: <Amount sign="+" n={gold} unit="Gold" /> each turn. You keep <Amount n={spareOf(view, resource!) - amount} unit={resource as AmountUnit} /> to spare.</>
              : <>If {name} accept{/s$/.test(empireOfPlayer(view, to)) ? '' : 's'}: <Amount sign="+" n={amount} unit={resource as AmountUnit} /> supply, <Amount sign="−" n={gold} unit="Gold" /> each turn.</>}</p>
          : <p className="order-error gift-forecast" role="alert" data-testid="supply-forecast">{!amountOk
              ? <>{selling ? 'You have' : `${cap(name)} ${/s$/.test(empireOfPlayer(view, to)) ? 'have' : 'has'}`} <Amount n={max} unit={resource as AmountUnit} /> to spare. Trade between 1 and {max}.</>
              : !selling && Number.isInteger(gold) && gold > view.economy.gold
                ? <>You have <Amount n={Math.max(0, view.economy.gold)} unit="Gold" />. Offer no more than that a turn.</>
                : <>Ask between 0 and <Amount n={MAX_GOLD} unit="Gold" /> a turn.</>}</p>}
    </div>
    <p className="muted supply-trade-rule">{SUPPLY_TRADE_RULE}</p>
  </div>;
}
