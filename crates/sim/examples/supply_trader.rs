//! Exploiter seat `supply_trader`: can a player farm the AI through supply trades (rules 62)?
//!
//! Eight Hard seats. Each seed is played twice: `plain` (seat 0 is plain Hard) and `trader` (seat 0 is
//! Hard that also tries every abusive supply offer each turn). The trader keeps Hard's own orders and
//! adds, for each empire it may trade with, one offer chosen by the turn number:
//!   lowball   buy all its spare supply for 1 Gold a turn;
//!   strip     buy all its spare supply (leaving it none) at the top price, 500 Gold a turn;
//!   gouge     sell it 1 supply for 500 Gold a turn;
//!   dump      sell it supply it is not short of for 1 Gold a turn;
//!   fair      buy 1 supply at the AI's own asking price (the control: this one should be accepted).
//! It accepts every offer made to it and never ends a trade.
//!
//!   cargo run --release -p legatum-sim --example supply_trader -- --games 16 --seed 6000001
//!
//! Prints one JSON summary: per kind, offers sent and accepted; every trade the trader got with its
//! price as a percent of the AI's fair price; seat 0's win share and VP in both arms; and the census of
//! trades between the AI empires themselves.
use legatum_ai::supply_trade::{PEACE_ASK_PERCENT, fair_price};
use legatum_core::supply_trade::{MAX_AMOUNT, MAX_GOLD};
use legatum_core::{
    AiDifficulty, Event, Faction, GameSetup, Order, PlayerId, player_view, scoring,
};
use legatum_wasm::session::{BrainKind, Session};
use rayon::prelude::*;
use serde_json::{Value, json};
use std::collections::BTreeMap;

const SEAT: PlayerId = PlayerId(0);
const KINDS: [&str; 5] = ["lowball", "strip", "gouge", "dump", "fair"];

fn arg(args: &[String], key: &str, default: &str) -> String {
    args.windows(2)
        .find(|w| w[0] == key)
        .map_or_else(|| default.into(), |w| w[1].clone())
}

fn setup(seed: u64) -> GameSetup {
    let mut setup = GameSetup::all_ai(seed);
    setup.deals = true;
    let all: Vec<_> = Faction::ALL.into_iter().collect();
    let omitted = (seed % all.len() as u64) as usize;
    let mut roster: Vec<_> = all
        .into_iter()
        .enumerate()
        .filter_map(|(i, f)| (i != omitted).then_some(f))
        .collect();
    roster.rotate_right((seed % 8) as usize);
    for (p, faction) in setup.players.iter_mut().zip(roster) {
        p.faction = faction;
        p.difficulty = AiDifficulty::Hard;
    }
    setup.turn_limit = None;
    setup
}

fn play(seed: u64, trader: bool) -> Value {
    let mut session = Session::new(&setup(seed), |_| BrainKind::Scripted);
    // Offer id -> kind, for the offers the trader sent.
    let mut kind_of: BTreeMap<u32, &str> = BTreeMap::new();
    let mut pending: Vec<(&str, PlayerId)> = Vec::new();
    let mut sent: BTreeMap<&str, u32> = BTreeMap::new();
    let mut accepted: BTreeMap<&str, u32> = BTreeMap::new();
    let mut got = Vec::new();
    let (mut ai_trades, mut ai_declines, mut gold_in, mut gold_out) = (0u32, 0u32, 0i64, 0i64);
    let mut ends: BTreeMap<String, u32> = BTreeMap::new();
    while !session.state.finished && session.state.turn <= 1200 {
        let mut orders = session.ai_orders();
        if trader && session.state.player(SEAT).alive {
            let mine = orders.entry(SEAT).or_default();
            mine.retain(|o| {
                !matches!(
                    o,
                    Order::OfferSupply { .. }
                        | Order::AnswerSupplyOffer { .. }
                        | Order::EndSupplyTrade { .. }
                )
            });
            let v = player_view(&session.state, SEAT);
            for o in v.supply_trade.offers.iter().filter(|o| o.to == SEAT) {
                mine.push(Order::AnswerSupplyOffer {
                    offer: o.id,
                    accept: true,
                    note: None,
                });
            }
            pending.clear();
            let kind = KINDS[(session.state.turn as usize / 2) % KINDS.len()];
            for p in v
                .supply_trade
                .partners
                .iter()
                .filter(|p| p.blocked.is_none())
            {
                if v.supply_trade.offers.iter().any(|o| o.to == p.player) {
                    continue;
                }
                let theirs = p.spare.iter().find(|(_, n)| *n > 0).copied();
                let ours = v
                    .supply
                    .iter()
                    .find(|s| s.spare > 0 && p.uses.contains(&s.resource))
                    .map(|s| s.resource);
                let offer = match (kind, theirs, ours) {
                    ("lowball", Some((r, n)), _) => Some((false, r, n.min(MAX_AMOUNT), 1)),
                    ("strip", Some((r, n)), _) => Some((false, r, n.min(MAX_AMOUNT), MAX_GOLD)),
                    ("fair", Some((r, _)), _) => {
                        Some((false, r, 1, fair_price(r) * PEACE_ASK_PERCENT / 100))
                    }
                    ("gouge", _, Some(r)) => Some((true, r, 1, MAX_GOLD)),
                    ("dump", _, Some(r)) => Some((true, r, 1, 1)),
                    _ => None,
                };
                let Some((selling, resource, amount, gold)) = offer else {
                    continue;
                };
                let order = Order::OfferSupply {
                    to: p.player,
                    selling,
                    resource,
                    amount,
                    gold: if selling {
                        gold
                    } else {
                        gold.min(v.economy.gold.max(0))
                    },
                };
                if legatum_core::validate_orders(&session.state, SEAT, std::slice::from_ref(&order))
                    .is_empty()
                {
                    mine.push(order);
                    pending.push((kind, p.player));
                    *sent.entry(kind).or_default() += 1;
                }
            }
        }
        for t in &session.state.supply_trades.trades {
            if t.seller == SEAT {
                gold_in += t.gold as i64;
            } else if t.buyer == SEAT {
                gold_out += t.gold as i64;
            }
        }
        for e in session.step(&orders) {
            match e {
                Event::SupplyOffered { offer } if offer.from == SEAT && trader => {
                    if let Some((k, _)) = pending.iter().find(|(_, to)| *to == offer.to) {
                        kind_of.insert(offer.id, k);
                    }
                }
                Event::SupplyTradeStarted { trade } => {
                    if trader && (trade.seller == SEAT || trade.buyer == SEAT) {
                        let k = kind_of.get(&trade.id).copied().unwrap_or("theirs");
                        *accepted.entry(k).or_default() += 1;
                        got.push(json!({
                            "turn": session.state.turn, "kind": k, "selling": trade.seller == SEAT,
                            "resource": trade.resource, "amount": trade.amount, "gold": trade.gold,
                            "percent_of_fair": 100 * trade.gold / (fair_price(trade.resource) * trade.amount),
                        }));
                    } else {
                        ai_trades += 1;
                    }
                }
                Event::SupplyOfferDeclined { offer, .. } if offer.from != SEAT => ai_declines += 1,
                Event::SupplyTradeEnded { reason, .. } => {
                    let key = if reason.ends_with("ended it") {
                        "ended by a side".into()
                    } else {
                        reason
                    };
                    *ends.entry(key).or_default() += 1;
                }
                _ => {}
            }
        }
    }
    let state = &session.state;
    json!({
        "seed": seed, "arm": if trader { "trader" } else { "plain" }, "turns": state.turn,
        "won": state.winner == Some(state.player(SEAT).team), "alive": state.player(SEAT).alive,
        "vp": scoring::vp_total(state, SEAT), "sent": sent, "accepted": accepted, "got": got,
        "gold_in": gold_in, "gold_out": gold_out,
        "ai_trades": ai_trades, "ai_declines": ai_declines, "ends": ends,
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let games: u64 = arg(&args, "--games", "8").parse().expect("--games");
    let seed: u64 = arg(&args, "--seed", "6000001").parse().expect("--seed");
    let rows: Vec<Value> = (0..games * 2)
        .into_par_iter()
        .map(|i| play(seed + i / 2, i % 2 == 1))
        .collect();
    let arm = |name: &str| rows.iter().filter(|r| r["arm"] == name).collect::<Vec<_>>();
    let sum = |rs: &[&Value], k: &str| rs.iter().map(|r| r[k].as_i64().unwrap_or(0)).sum::<i64>();
    let wins = |rs: &[&Value]| rs.iter().filter(|r| r["won"] == true).count();
    let (plain, trader) = (arm("plain"), arm("trader"));
    let mut sent: BTreeMap<String, i64> = BTreeMap::new();
    let mut accepted: BTreeMap<String, i64> = BTreeMap::new();
    for r in &trader {
        for (m, k) in [(&mut sent, "sent"), (&mut accepted, "accepted")] {
            for (kind, n) in r[k].as_object().into_iter().flatten() {
                *m.entry(kind.clone()).or_default() += n.as_i64().unwrap_or(0);
            }
        }
    }
    let got: Vec<&Value> = trader
        .iter()
        .flat_map(|r| r["got"].as_array().into_iter().flatten())
        .collect();
    let mut ends: BTreeMap<String, i64> = BTreeMap::new();
    for r in &rows {
        for (k, n) in r["ends"].as_object().into_iter().flatten() {
            *ends.entry(k.clone()).or_default() += n.as_i64().unwrap_or(0);
        }
    }
    let summary = json!({
        "games_per_arm": games, "first_seed": seed,
        "seat0": {
            "plain": {"wins": wins(&plain), "vp": sum(&plain, "vp")},
            "trader": {"wins": wins(&trader), "vp": sum(&trader, "vp"),
                       "gold_in": sum(&trader, "gold_in"), "gold_out": sum(&trader, "gold_out")},
        },
        "offers_sent": sent, "offers_accepted": accepted,
        "lowest_percent_of_fair_bought": got.iter().filter(|g| g["selling"] == false).map(|g| g["percent_of_fair"].as_i64().unwrap_or(0)).min(),
        "highest_percent_of_fair_sold": got.iter().filter(|g| g["selling"] == true).map(|g| g["percent_of_fair"].as_i64().unwrap_or(0)).max(),
        "trades_got": got.len(),
        "between_ai": {"trades": sum(&plain, "ai_trades"), "declines": sum(&plain, "ai_declines"), "turns": sum(&plain, "turns")},
        "trade_ends": ends,
    });
    println!("{}", serde_json::to_string_pretty(&summary).expect("json"));
}
