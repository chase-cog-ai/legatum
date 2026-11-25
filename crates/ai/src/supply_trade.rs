//! Supply trades (`legatum_core::supply_trade`): what the AI asks and pays for strategic supply,
//! which offers it accepts, and when it ends a trade.
//!
//! The rules keep a human from farming it: it sells only supply it will not miss, never below
//! its asking price and never to an empire it has reason to fear; it buys only to end a
//! shortage it already has, at a capped price it can pay from income; and it drops a trade the
//! turn it stops making sense.

use legatum_core::resources::Resource;
use legatum_core::supply_trade::{SupplyOffer, SupplyTrade};
use legatum_core::{Order, PlayerId, PlayerView, TeamId};

/// Gold a turn one supply is worth between friends. Later resources feed stronger units.
pub fn fair_price(resource: Resource) -> i32 {
    match resource {
        Resource::Iron => 4,
        Resource::Niter => 6,
        Resource::Coal => 8,
        Resource::Oil => 10,
        _ => 12,
    }
}

/// Supply the seller keeps spare for itself after any sale.
pub const KEEP_SPARE: i32 = 1;
/// A team this far to its VP target is a leader nobody should arm.
pub const LEADER_PROGRESS: u32 = 60;
/// Percent of the fair price asked of an empire at Peace (an ally pays the fair price).
pub const PEACE_ASK_PERCENT: i32 = 150;
/// Percent of the fair price the AI pays at most, and what it bids.
pub const MAX_BID_PERCENT: i32 = 200;
pub const BID_PERCENT: i32 = 150;
/// The share of its Gold income the AI spends on supply at most (one part in this many).
pub const INCOME_SHARE: i32 = 4;

fn progress(v: &PlayerView, team: TeamId) -> u32 {
    v.standings
        .iter()
        .find(|s| s.team == team)
        .map_or(0, |s| 100 * s.vp / s.target.max(1))
}

fn allied(v: &PlayerView, other: TeamId) -> bool {
    other == v.team
        || v.diplomacy.relations.iter().any(|r| {
            r.between(v.team, other)
                && r.state == legatum_core::diplomacy::RelationState::Alliance
        })
}

fn spare(v: &PlayerView, resource: Resource) -> i32 {
    v.supply
        .iter()
        .find(|s| s.resource == resource)
        .map_or(0, |s| s.spare)
}

/// Users of `resource` that are short of supply now.
fn short(v: &PlayerView, resource: Resource) -> i32 {
    v.supply
        .iter()
        .find(|s| s.resource == resource)
        .map_or(0, |s| s.idle.len() as i32)
}

/// Why we will not sell `buyer` supply at any price.
pub fn sale_refusal(v: &PlayerView, target: Option<TeamId>, buyer: PlayerId) -> Option<String> {
    let team = v.team_of(buyer);
    if team == v.team {
        return None;
    }
    if target == Some(team) {
        return Some("we will not arm a rival we mean to stop".into());
    }
    if progress(v, team) >= LEADER_PROGRESS && progress(v, team) >= progress(v, v.team) {
        return Some("we will not arm an empire this close to winning".into());
    }
    if !allied(v, team)
        && crate::diplomacy::strength(v, team) > crate::diplomacy::strength(v, v.team)
    {
        return Some("we will not arm an army larger than ours".into());
    }
    None
}

/// The least Gold a turn we take for `amount` of `resource` from `buyer`.
pub fn ask(v: &PlayerView, buyer: PlayerId, resource: Resource, amount: i32) -> i32 {
    let percent = if allied(v, v.team_of(buyer)) {
        100
    } else {
        PEACE_ASK_PERCENT
    };
    (fair_price(resource) * amount * percent + 99) / 100
}

/// Gold a turn we can still commit to buying supply.
fn budget(v: &PlayerView) -> i32 {
    // Running purchases are already inside the income total.
    v.economy.gold_per_turn.total.max(0) / INCOME_SHARE
}

/// The most Gold a turn we pay for `amount` of `resource`, or 0 when we do not want it.
pub fn bid_limit(v: &PlayerView, resource: Resource, amount: i32) -> i32 {
    if amount < 1 || amount > short(v, resource) {
        return 0;
    }
    (fair_price(resource) * amount * MAX_BID_PERCENT / 100)
        .min(budget(v))
        .min(v.economy.gold)
}

fn answer(v: &PlayerView, target: Option<TeamId>, o: &SupplyOffer) -> (bool, Option<String>) {
    let r = legatum_core::resources::name(o.resource);
    if o.seller() == v.me {
        if let Some(why) = sale_refusal(v, target, o.buyer()) {
            return (false, Some(why));
        }
        let left = spare(v, o.resource) - o.amount;
        if left < KEEP_SPARE {
            let most = (spare(v, o.resource) - KEEP_SPARE).max(0);
            return (false, Some(format!("we can spare {most} {r}")));
        }
        let ask = ask(v, o.buyer(), o.resource, o.amount);
        if o.gold < ask {
            return (false, Some(format!("we want {ask} Gold a turn for it")));
        }
        (true, None)
    } else {
        if target == Some(v.team_of(o.seller())) {
            return (false, Some("we will not depend on a rival we mean to stop".into()));
        }
        let need = short(v, o.resource);
        if need == 0 {
            return (false, Some(format!("we have no use for more {r} now")));
        }
        if o.amount > need {
            return (false, Some(format!("we only need {need} {r}")));
        }
        let limit = bid_limit(v, o.resource, o.amount);
        if o.gold > limit {
            return (false, Some(format!("we would pay {limit} Gold a turn for it")));
        }
        (true, None)
    }
}

/// Whether a running trade no longer makes sense for us.
fn should_end(v: &PlayerView, target: Option<TeamId>, t: &SupplyTrade) -> bool {
    if t.seller == v.me {
        // Our own users are short, or the buyer became someone we would not sell to.
        short(v, t.resource) > 0
            || spare(v, t.resource) < 0
            || sale_refusal(v, target, t.buyer).is_some()
    } else {
        // We can do without it, or it costs more than we earn.
        spare(v, t.resource) >= t.amount || v.economy.gold_per_turn.total < 0
    }
}

/// This turn's supply trade orders: endings, answers to open offers, and a bid for supply we are
/// short of from whoever has it to spare.
pub fn plan(v: &PlayerView, target: Option<TeamId>) -> Vec<Order> {
    let mut orders = Vec::new();
    let st = &v.supply_trade;
    for t in st.trades.iter().filter(|t| should_end(v, target, t)) {
        orders.push(Order::EndSupplyTrade { trade: t.id });
    }
    // Answers share this turn's supply and Gold: each accepted offer uses some up.
    let mut sold = [0; Resource::ALL.len()];
    let mut bought = [0; Resource::ALL.len()];
    let mut spent = 0;
    for o in st.offers.iter().filter(|o| o.to == v.me) {
        let i = o.resource as usize;
        let (mut accept, mut note) = answer(v, target, o);
        if accept && o.seller() == v.me {
            if spare(v, o.resource) - sold[i] - o.amount < KEEP_SPARE {
                (accept, note) = (false, Some("we promised that supply elsewhere".into()));
            } else {
                sold[i] += o.amount;
            }
        } else if accept {
            if bought[i] + o.amount > short(v, o.resource) || spent + o.gold > budget(v) {
                (accept, note) = (false, Some("we found that supply elsewhere".into()));
            } else {
                bought[i] += o.amount;
                spent += o.gold;
            }
        }
        orders.push(Order::AnswerSupplyOffer {
            offer: o.id,
            accept,
            note,
        });
    }
    for resource in Resource::STRATEGIC {
        let i = resource as usize;
        let need = short(v, resource) - bought[i];
        if need < 1 || st.offers.iter().any(|o| o.resource == resource) {
            continue;
        }
        // The partner with the most to spare, allies first; ties go to the lowest id.
        let seller = st
            .partners
            .iter()
            .filter(|p| p.blocked.is_none() && target != Some(v.team_of(p.player)))
            .filter_map(|p| {
                let n = p.spare.iter().find(|(r, _)| *r == resource)?.1;
                (n > KEEP_SPARE).then_some((
                    allied(v, v.team_of(p.player)),
                    n,
                    std::cmp::Reverse(p.player),
                ))
            })
            .max();
        let Some((_, has, std::cmp::Reverse(seller))) = seller else {
            continue;
        };
        let amount = need
            .min(has - KEEP_SPARE)
            .min(legatum_core::supply_trade::MAX_AMOUNT);
        let gold = fair_price(resource) * amount * BID_PERCENT / 100;
        if gold > bid_limit(v, resource, amount) - spent {
            continue;
        }
        spent += gold;
        orders.push(Order::OfferSupply {
            to: seller,
            selling: false,
            resource,
            amount,
            gold,
        });
    }
    orders
}

#[cfg(test)]
mod tests;
