//! Supply sold by the turn: an empire with spare strategic supply lets another empire use it
//! for Gold each turn. The supply moves in `resources::supplied` (the seller's falls, the
//! buyer's rises) and the Gold in `economy::player_income`, so every count, forecast and
//! breakdown already includes it. War between the two empires ends the trade at once.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::events::Event;
use crate::orders::{Order, OrderError};
use crate::resources::{self, Resource};
use crate::state::GameState;
use crate::types::PlayerId;

/// Turns an offer can be answered before it lapses.
pub const OFFER_TURNS: u32 = 3;
/// Most supply one trade moves.
pub const MAX_AMOUNT: i32 = 6;
/// Most Gold per turn one trade may ask.
pub const MAX_GOLD: i32 = 500;

/// A running sale: `amount` of the seller's supply counts as the buyer's, and `gold` moves from
/// buyer to seller with each turn's income.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupplyTrade {
    pub id: u32,
    pub seller: PlayerId,
    pub buyer: PlayerId,
    pub resource: Resource,
    pub amount: i32,
    /// Gold per turn for the whole amount.
    pub gold: i32,
    /// The turn it was agreed; supply and Gold move from the turn after.
    pub since: u32,
}

/// An offer waiting for `to`'s answer. `from` sells when `selling`, and buys otherwise.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupplyOffer {
    pub id: u32,
    pub from: PlayerId,
    pub to: PlayerId,
    pub selling: bool,
    pub resource: Resource,
    pub amount: i32,
    pub gold: i32,
    /// The turn it was sent.
    pub turn: u32,
}

impl SupplyOffer {
    pub fn seller(&self) -> PlayerId {
        if self.selling { self.from } else { self.to }
    }

    pub fn buyer(&self) -> PlayerId {
        if self.selling { self.to } else { self.from }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupplyTrades {
    pub next_id: u32,
    /// Running trades, oldest first.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trades: Vec<SupplyTrade>,
    /// Open offers, oldest first.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub offers: Vec<SupplyOffer>,
}

impl SupplyTrades {
    pub fn is_empty(&self) -> bool {
        self.next_id == 0 && self.trades.is_empty() && self.offers.is_empty()
    }
}

/// Supply `p` receives from running trades, minus what it sells.
pub fn net(state: &GameState, p: PlayerId, resource: Resource) -> i32 {
    state
        .supply_trades
        .trades
        .iter()
        .filter(|t| t.resource == resource)
        .map(|t| {
            if t.buyer == p {
                t.amount
            } else if t.seller == p {
                -t.amount
            } else {
                0
            }
        })
        .sum()
}

fn name_of(state: &GameState, p: PlayerId) -> String {
    crate::scenario::empire_name(state, p)
}

/// Gold lines for `p`'s income: one per running trade, signed.
pub fn gold_lines(state: &GameState, p: PlayerId) -> Vec<(String, i32)> {
    state
        .supply_trades
        .trades
        .iter()
        .filter_map(|t| {
            let r = resources::name(t.resource);
            if t.seller == p {
                Some((
                    format!("{} {r} sold to {}", t.amount, name_of(state, t.buyer)),
                    t.gold,
                ))
            } else if t.buyer == p {
                Some((
                    format!("{} {r} bought from {}", t.amount, name_of(state, t.seller)),
                    -t.gold,
                ))
            } else {
                None
            }
        })
        .collect()
}

/// Supply lines for `p`'s supply breakdown: one per running trade in `resource`, signed.
pub fn supply_lines(state: &GameState, p: PlayerId, resource: Resource) -> Vec<(String, i32)> {
    state
        .supply_trades
        .trades
        .iter()
        .filter(|t| t.resource == resource)
        .filter_map(|t| {
            if t.seller == p {
                Some((format!("Sold to {}", name_of(state, t.buyer)), -t.amount))
            } else if t.buyer == p {
                Some((
                    format!("Bought from {}", name_of(state, t.seller)),
                    t.amount,
                ))
            } else {
                None
            }
        })
        .collect()
}

/// Why `a` and `b` cannot trade supply at all, whatever the terms.
pub fn pair_blocker(state: &GameState, a: PlayerId, b: PlayerId) -> Option<String> {
    if a == b {
        return Some("you cannot trade with yourself".into());
    }
    if b.is_barbarian() || (b.0 as usize) >= state.players.len() {
        return Some("no such empire".into());
    }
    if !state.player(b).alive {
        return Some(format!("{} has been eliminated", name_of(state, b)));
    }
    let (ta, tb) = (state.team_of(a), state.team_of(b));
    if state.diplomacy.as_ref().is_some_and(|d| !d.met(ta, tb)) {
        return Some(crate::diplomacy::NOT_MET.into());
    }
    if state.teams_at_war(ta, tb) {
        return Some(format!("you are at War with {}", name_of(state, b)));
    }
    None
}

/// Why this seller cannot supply, or this buyer cannot use and pay for, the terms right now.
fn terms_blocker(
    state: &GameState,
    me: PlayerId,
    seller: PlayerId,
    buyer: PlayerId,
    resource: Resource,
    amount: i32,
    gold: i32,
) -> Option<String> {
    let r = resources::name(resource);
    if !resources::is_strategic(resource) {
        return Some(format!("{r} is not a strategic supply"));
    }
    if !(1..=MAX_AMOUNT).contains(&amount) {
        return Some(format!("a trade moves 1 to {MAX_AMOUNT} supply"));
    }
    if !(0..=MAX_GOLD).contains(&gold) {
        return Some(format!("a trade pays 0 to {MAX_GOLD} Gold a turn"));
    }
    let spare = resources::spare(state, seller, resource, None);
    if spare < amount {
        return Some(if seller == me {
            format!("you have {} {r} to spare", spare.max(0))
        } else {
            format!(
                "{} has {} {r} to spare",
                name_of(state, seller),
                spare.max(0)
            )
        });
    }
    if !resources::known_to(resource, &state.player(buyer).techs) {
        let tech = resources::stats(resource)
            .revealed_by
            .map_or("", crate::data::tech_name);
        return Some(if buyer == me {
            format!("you need {tech} to use {r}")
        } else {
            format!("{} cannot use {r} yet", name_of(state, buyer))
        });
    }
    if state.player(buyer).gold < gold {
        return Some(if buyer == me {
            format!("you have {} Gold", state.player(buyer).gold.max(0))
        } else {
            format!("{} cannot pay {gold} Gold", name_of(state, buyer))
        });
    }
    None
}

/// Why `player` may not give this supply trade order now (`None` for any other order).
pub fn order_blocker(state: &GameState, player: PlayerId, order: &Order) -> Option<String> {
    match order {
        Order::OfferSupply {
            to,
            selling,
            resource,
            amount,
            gold,
        } => {
            let (seller, buyer) = if *selling {
                (player, *to)
            } else {
                (*to, player)
            };
            pair_blocker(state, player, *to)
                .or_else(|| terms_blocker(state, player, seller, buyer, *resource, *amount, *gold))
        }
        Order::AnswerSupplyOffer { offer, accept, .. } => {
            let Some(o) = state
                .supply_trades
                .offers
                .iter()
                .find(|o| o.id == *offer && o.to == player)
            else {
                return Some("that offer is no longer open".into());
            };
            if !*accept {
                return None;
            }
            pair_blocker(state, player, o.from).or_else(|| {
                terms_blocker(
                    state,
                    player,
                    o.seller(),
                    o.buyer(),
                    o.resource,
                    o.amount,
                    o.gold,
                )
            })
        }
        Order::EndSupplyTrade { trade } => state
            .supply_trades
            .trades
            .iter()
            .all(|t| t.id != *trade || (t.seller != player && t.buyer != player))
            .then(|| "that trade is not running".into()),
        _ => None,
    }
}

/// One offer per empire and resource, one answer per offer and one ending per trade each turn.
fn same_slot(a: &Order, b: &Order) -> bool {
    match (a, b) {
        (
            Order::OfferSupply {
                to: ta,
                resource: ra,
                ..
            },
            Order::OfferSupply {
                to: tb,
                resource: rb,
                ..
            },
        ) => ta == tb && ra == rb,
        (Order::AnswerSupplyOffer { offer: a, .. }, Order::AnswerSupplyOffer { offer: b, .. }) => {
            a == b
        }
        (Order::EndSupplyTrade { trade: a }, Order::EndSupplyTrade { trade: b }) => a == b,
        _ => false,
    }
}

pub fn validate_orders(state: &GameState, player: PlayerId, orders: &[Order]) -> Vec<OrderError> {
    let mut errors = Vec::new();
    for (index, order) in orders.iter().enumerate() {
        if let Some(message) = order_blocker(state, player, order) {
            errors.push(OrderError { index, message });
        } else if orders[..index].iter().any(|o| same_slot(o, order)) {
            errors.push(OrderError {
                index,
                message: "you already gave this supply trade order this turn".into(),
            });
        }
    }
    errors
}

fn end_where(
    state: &mut GameState,
    events: &mut Vec<Event>,
    reason: &str,
    mut ends: impl FnMut(&GameState, &SupplyTrade) -> bool,
) -> bool {
    let (ended, kept): (Vec<_>, Vec<_>) = std::mem::take(&mut state.supply_trades.trades)
        .into_iter()
        .partition(|t| ends(state, t));
    state.supply_trades.trades = kept;
    let any = !ended.is_empty();
    for trade in ended {
        events.push(Event::SupplyTradeEnded {
            trade,
            reason: reason.into(),
        });
    }
    any
}

/// End every trade between empires now at War or with an eliminated side, and drop their
/// offers. Returns whether a trade ended (supply must then be counted again).
pub fn end_broken(state: &mut GameState, events: &mut Vec<Event>) -> bool {
    if state.supply_trades.trades.is_empty() && state.supply_trades.offers.is_empty() {
        return false;
    }
    let dead = |s: &GameState, a: PlayerId, b: PlayerId| !s.player(a).alive || !s.player(b).alive;
    let mut any = end_where(state, events, "War cut it off", |s, t| {
        !dead(s, t.seller, t.buyer) && s.at_war(t.seller, t.buyer)
    });
    any |= end_where(state, events, "an empire was eliminated", |s, t| {
        dead(s, t.seller, t.buyer)
    });
    let offers = std::mem::take(&mut state.supply_trades.offers);
    state.supply_trades.offers = offers
        .into_iter()
        .filter(|o| !dead(state, o.from, o.to) && !state.at_war(o.from, o.to))
        .collect();
    any
}

/// A seller whose deposits no longer cover what it sells stops its newest sales first.
fn end_undeliverable(state: &mut GameState, events: &mut Vec<Event>) -> bool {
    let mut any = false;
    loop {
        let short = state.supply_trades.trades.iter().rev().find(|t| {
            let sold: i32 = state
                .supply_trades
                .trades
                .iter()
                .filter(|o| o.seller == t.seller && o.resource == t.resource)
                .map(|o| o.amount)
                .sum();
            resources::deposits_supply(state, t.seller, t.resource) < sold
        });
        let Some(id) = short.map(|t| t.id) else {
            return any;
        };
        any |= end_where(
            state,
            events,
            "the seller lost the deposits behind it",
            |_, t| t.id == id,
        );
    }
}

/// Before income: a buyer whose treasury and income cannot cover its purchases stops its newest
/// ones first, so a trade never bankrupts an empire.
pub fn end_unpaid(state: &mut GameState, events: &mut Vec<Event>) {
    loop {
        let unpaid = state.supply_trades.trades.iter().rev().find(|t| {
            state.player(t.buyer).gold + crate::economy::player_income(state, t.buyer).gold.total
                < 0
        });
        let Some(id) = unpaid.map(|t| t.id) else {
            return;
        };
        end_where(state, events, "the buyer could not pay", |_, t| t.id == id);
    }
}

/// End of turn, after the diplomatic boundary: endings, broken trades, answers, lapsed offers,
/// then new offers. Returns whether supply changed hands (supply must then be counted again).
pub fn resolve(
    state: &mut GameState,
    orders: &BTreeMap<PlayerId, &[Order]>,
    events: &mut Vec<Event>,
) -> bool {
    let mut changed = false;
    for (player, os) in orders {
        for order in os.iter() {
            if let Order::EndSupplyTrade { trade } = order {
                let other = state
                    .supply_trades
                    .trades
                    .iter()
                    .find(|t| t.id == *trade)
                    .filter(|t| t.seller == *player || t.buyer == *player)
                    .map(|_| format!("{} ended it", name_of(state, *player)));
                if let Some(reason) = other {
                    changed |= end_where(state, events, &reason, |_, t| t.id == *trade);
                }
            }
        }
    }
    changed |= end_broken(state, events);
    changed |= end_undeliverable(state, events);
    for (player, os) in orders {
        if !state.player(*player).alive {
            continue;
        }
        for order in os.iter() {
            let Order::AnswerSupplyOffer {
                offer,
                accept,
                note,
            } = order
            else {
                continue;
            };
            let Some(at) = state
                .supply_trades
                .offers
                .iter()
                .position(|o| o.id == *offer && o.to == *player)
            else {
                continue;
            };
            let blocked = accept
                .then(|| order_blocker(state, *player, order))
                .flatten();
            let o = state.supply_trades.offers.remove(at);
            if !*accept {
                events.push(Event::SupplyOfferDeclined {
                    offer: o,
                    reason: note.clone(),
                });
            } else if let Some(reason) = blocked {
                events.push(Event::SupplyOfferDeclined {
                    offer: o,
                    reason: Some(reason),
                });
            } else {
                let trade = SupplyTrade {
                    id: o.id,
                    seller: o.seller(),
                    buyer: o.buyer(),
                    resource: o.resource,
                    amount: o.amount,
                    gold: o.gold,
                    since: state.turn,
                };
                state.supply_trades.trades.push(trade.clone());
                events.push(Event::SupplyTradeStarted { trade });
                changed = true;
            }
        }
    }
    let turn = state.turn;
    let (lapsed, open): (Vec<_>, Vec<_>) = std::mem::take(&mut state.supply_trades.offers)
        .into_iter()
        .partition(|o| turn >= o.turn + OFFER_TURNS);
    state.supply_trades.offers = open;
    for offer in lapsed {
        events.push(Event::SupplyOfferDeclined {
            offer,
            reason: Some("no answer".into()),
        });
    }
    for (player, os) in orders {
        if !state.player(*player).alive {
            continue;
        }
        for (i, order) in os.iter().enumerate() {
            let Order::OfferSupply {
                to,
                selling,
                resource,
                amount,
                gold,
            } = order
            else {
                continue;
            };
            if os[..i].iter().any(|o| same_slot(o, order)) {
                continue;
            }
            if let Some(reason) = order_blocker(state, *player, order) {
                events.push(Event::OrderFailed {
                    player: *player,
                    reason: format!("Supply offer not sent: {reason}"),
                });
                continue;
            }
            state
                .supply_trades
                .offers
                .retain(|o| !(o.from == *player && o.to == *to && o.resource == *resource));
            let offer = SupplyOffer {
                id: state.supply_trades.next_id,
                from: *player,
                to: *to,
                selling: *selling,
                resource: *resource,
                amount: *amount,
                gold: *gold,
                turn,
            };
            state.supply_trades.next_id += 1;
            state.supply_trades.offers.push(offer.clone());
            events.push(Event::SupplyOffered { offer });
        }
    }
    changed
}

/// What another empire can trade, as anyone who has met it sees.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartnerView {
    pub player: PlayerId,
    /// Why no supply trade with this empire is possible now ("you are at War with Rome").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked: Option<String>,
    /// Supply it has to spare, per strategic resource it has any of.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spare: Vec<(Resource, i32)>,
    /// Strategic resources it knows, so it can use supply of them.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uses: Vec<Resource>,
}

/// One player's supply trades: its running trades, the offers it sent or must answer, and what
/// each empire it has met could trade.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupplyTradeView {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trades: Vec<SupplyTrade>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub offers: Vec<SupplyOffer>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub partners: Vec<PartnerView>,
}

impl SupplyTradeView {
    pub fn is_empty(&self) -> bool {
        self.trades.is_empty() && self.offers.is_empty() && self.partners.is_empty()
    }
}

pub fn view(state: &GameState, me: PlayerId) -> SupplyTradeView {
    let mine = |a: PlayerId, b: PlayerId| a == me || b == me;
    let partners = state
        .players
        .iter()
        .filter(|p| p.alive && p.id != me)
        .filter(|p| {
            state
                .diplomacy
                .as_ref()
                .is_none_or(|d| d.met(state.team_of(me), state.team_of(p.id)))
        })
        .map(|p| {
            let blocked = pair_blocker(state, me, p.id);
            let open = blocked.is_none();
            PartnerView {
                player: p.id,
                blocked,
                spare: Resource::STRATEGIC
                    .into_iter()
                    .filter(|_| open)
                    .map(|r| (r, resources::spare(state, p.id, r, None)))
                    .filter(|(_, n)| *n > 0)
                    .collect(),
                uses: Resource::STRATEGIC
                    .into_iter()
                    .filter(|r| open && resources::known_to(*r, &p.techs))
                    .collect(),
            }
        })
        .collect();
    SupplyTradeView {
        trades: state
            .supply_trades
            .trades
            .iter()
            .filter(|t| mine(t.seller, t.buyer))
            .cloned()
            .collect(),
        offers: state
            .supply_trades
            .offers
            .iter()
            .filter(|o| mine(o.from, o.to))
            .cloned()
            .collect(),
        partners,
    }
}

#[cfg(test)]
mod tests;
