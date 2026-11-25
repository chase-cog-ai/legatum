use super::*;
use crate::game::spawn_unit;
use crate::hex::Hex;
use crate::rules::cities::tests::test_game;
use crate::types::{Faction, TeamId, TechId, UnitKind};
use crate::view::player_view;
use crate::{resolve_turn, validate_orders};

const P0: PlayerId = PlayerId(0);
const P1: PlayerId = PlayerId(1);
const IRON: Resource = Resource::Iron;

fn at(col: i32, row: i32) -> Hex {
    Hex::from_offset(col, row)
}

/// P0 owns an Iron deposit (2 supply) and P1 knows Iron but has none. Both have Gold.
fn game() -> GameState {
    let mut s = test_game(&[
        (Faction::Byzantium, 0),
        (Faction::Sparta, 1),
        (Faction::India, 2),
    ]);
    crate::rules::test_support::city(&mut s, 0, at(5, 5), 3);
    crate::rules::test_support::city(&mut s, 1, at(16, 2), 3);
    crate::rules::test_support::city(&mut s, 2, at(3, 16), 3);
    s.setup.deals = true;
    s.map.get_mut(at(6, 5)).unwrap().resource = Some(IRON);
    for p in &mut s.players {
        p.techs.extend([TechId::BronzeWorking, TechId::IronWorking]);
        p.gold = 100;
    }
    let teams = [TeamId(0), TeamId(1), TeamId(2)];
    let mut d = crate::diplomacy::DiplomacyState::new(teams.into_iter());
    d.meet_all(&teams);
    s.diplomacy = Some(d);
    assert_eq!(resources::spare(&s, P0, IRON, None), 2);
    assert_eq!(resources::supplied(&s, P1, IRON), 0);
    s
}

fn sell(to: PlayerId, amount: i32, gold: i32) -> Order {
    Order::OfferSupply {
        to,
        selling: true,
        resource: IRON,
        amount,
        gold,
    }
}

fn step(s: &GameState, orders: &[(PlayerId, Order)]) -> (GameState, Vec<Event>) {
    let mut map: BTreeMap<PlayerId, Vec<Order>> = BTreeMap::new();
    for (p, o) in orders {
        assert_eq!(validate_orders(s, *p, std::slice::from_ref(o)), vec![]);
        map.entry(*p).or_default().push(o.clone());
    }
    resolve_turn(s, &map)
}

/// P0 sells P1 `amount` Iron for `gold` a turn: offered, then accepted a turn later.
fn trading(amount: i32, gold: i32) -> GameState {
    let (s, _) = step(&game(), &[(P0, sell(P1, amount, gold))]);
    let offer = s.supply_trades.offers[0].id;
    let (s, events) = step(
        &s,
        &[(
            P1,
            Order::AnswerSupplyOffer {
                offer,
                accept: true,
                note: None,
            },
        )],
    );
    assert!(
        events
            .iter()
            .any(|e| matches!(e, Event::SupplyTradeStarted { .. }))
    );
    s
}

fn ended(events: &[Event]) -> Vec<&str> {
    events
        .iter()
        .filter_map(|e| match e {
            Event::SupplyTradeEnded { reason, .. } => Some(reason.as_str()),
            _ => None,
        })
        .collect()
}

#[test]
fn an_accepted_offer_moves_supply_and_gold_every_turn() {
    let s = trading(2, 7);
    assert_eq!(resources::supplied(&s, P0, IRON), 0);
    assert_eq!(resources::spare(&s, P0, IRON, None), 0);
    assert_eq!(resources::supplied(&s, P1, IRON), 2);
    // The Gold is an income line on both sides, so every forecast already includes it.
    let line = |p| {
        crate::economy::player_income(&s, p)
            .gold
            .parts
            .into_iter()
            .find(|(l, _)| l.contains("Iron"))
    };
    assert_eq!(line(P0), Some(("2 Iron sold to Sparta".into(), 7)));
    assert_eq!(line(P1), Some(("2 Iron bought from Byzantium".into(), -7)));
    let mut without = s.clone();
    without.supply_trades.trades.clear();
    let (after, _) = resolve_turn(&s, &BTreeMap::new());
    let (control, _) = resolve_turn(&without, &BTreeMap::new());
    assert_eq!(after.player(P0).gold, control.player(P0).gold + 7);
    assert_eq!(after.player(P1).gold, control.player(P1).gold - 7);
    // The supply chip explains where the supply went and came from.
    let supplied = |p| {
        player_view(&s, p)
            .supply
            .into_iter()
            .find(|v| v.resource == IRON)
            .unwrap()
            .supplied
    };
    assert!(supplied(P0).parts.contains(&("Sold to Sparta".into(), -2)));
    assert_eq!(
        supplied(P1).parts,
        vec![("Bought from Byzantium".into(), 2)]
    );
    // Only the two empires see the trade.
    assert_eq!(player_view(&s, P1).supply_trade.trades.len(), 1);
    assert!(player_view(&s, PlayerId(2)).supply_trade.trades.is_empty());
}

#[test]
fn bought_supply_keeps_a_unit_in_supply_and_ending_the_trade_idles_it() {
    let mut s = trading(1, 5);
    let pike = spawn_unit(&mut s, P1, UnitKind::Pikeman, at(16, 3));
    let (s, _) = resolve_turn(&s, &BTreeMap::new());
    assert_eq!(resources::unit_unsupplied(&s, &s.units[&pike]), None);
    let trade = s.supply_trades.trades[0].id;
    let (s, events) = step(&s, &[(P0, Order::EndSupplyTrade { trade })]);
    assert_eq!(ended(&events), vec!["Byzantium ended it"]);
    assert_eq!(resources::unit_unsupplied(&s, &s.units[&pike]), Some(IRON));
    assert_eq!(resources::spare(&s, P0, IRON, None), 2);
}

#[test]
fn war_cuts_the_trade_off_before_the_first_fight() {
    let mut s = trading(1, 5);
    let pike = spawn_unit(&mut s, P1, UnitKind::Pikeman, at(16, 3));
    let (s, _) = resolve_turn(&s, &BTreeMap::new());
    assert_eq!(resources::unit_unsupplied(&s, &s.units[&pike]), None);
    let (after, events) = step(
        &s,
        &[(
            P0,
            Order::DeclareWar {
                against: TeamId(1),
                breach: false,
            },
        )],
    );
    assert_eq!(ended(&events), vec!["War cut it off"]);
    // The cut comes before the shortage report of the same turn: the unit fought short.
    let cut = events
        .iter()
        .position(|e| matches!(e, Event::SupplyTradeEnded { .. }))
        .unwrap();
    let short = events
        .iter()
        .position(|e| matches!(e, Event::SupplyShortage { player: P1, .. }))
        .unwrap();
    assert!(cut < short);
    assert!(after.supply_trades.trades.is_empty());
    assert_eq!(
        resources::unit_unsupplied(&after, &after.units[&pike]),
        Some(IRON)
    );
    // No Gold moved on the turn war began.
    let mut without = s.clone();
    without.supply_trades.trades.clear();
    let (control, _) = step(
        &without,
        &[(
            P0,
            Order::DeclareWar {
                against: TeamId(1),
                breach: false,
            },
        )],
    );
    assert_eq!(after.player(P0).gold, control.player(P0).gold);
    // And enemies cannot trade.
    assert!(
        validate_orders(&after, P0, &[sell(P1, 1, 5)])[0]
            .message
            .contains("at War")
    );
    assert!(
        player_view(&after, P0).supply_trade.partners[0]
            .blocked
            .is_some()
    );
}

#[test]
fn a_seller_cannot_sell_supply_it_uses_or_sell_the_same_supply_twice() {
    let mut s = game();
    spawn_unit(&mut s, P0, UnitKind::Pikeman, at(5, 6));
    assert_eq!(
        validate_orders(&s, P0, &[sell(P1, 2, 5)])[0].message,
        "you have 1 Iron to spare"
    );
    // Two offers of the same 2 Iron to two empires: both may be sent, only one can start.
    let s = game();
    let (s, _) = step(&s, &[(P0, sell(P1, 2, 5)), (P0, sell(PlayerId(2), 2, 5))]);
    let accept = |i: usize| Order::AnswerSupplyOffer {
        offer: s.supply_trades.offers[i].id,
        accept: true,
        note: None,
    };
    let (after, events) = step(&s, &[(P1, accept(0)), (PlayerId(2), accept(1))]);
    assert_eq!(after.supply_trades.trades.len(), 1);
    assert_eq!(after.supply_trades.trades[0].buyer, P1);
    assert!(events.iter().any(|e| matches!(
        e,
        Event::SupplyOfferDeclined { reason: Some(r), .. } if r == "Byzantium has 0 Iron to spare"
    )));
    assert_eq!(resources::supplied(&after, P0, IRON), 0);
}

#[test]
fn a_buyer_needs_the_technology_and_the_gold() {
    let mut s = game();
    s.player_mut(P1).techs.remove(&TechId::IronWorking);
    assert_eq!(
        validate_orders(&s, P0, &[sell(P1, 1, 5)])[0].message,
        "Sparta cannot use Iron yet"
    );
    let mut s = game();
    s.player_mut(P1).gold = 3;
    let buy = Order::OfferSupply {
        to: P0,
        selling: false,
        resource: IRON,
        amount: 1,
        gold: 5,
    };
    assert_eq!(
        validate_orders(&s, P1, &[buy])[0].message,
        "you have 3 Gold"
    );
    for bad in [sell(P1, 0, 5), sell(P1, MAX_AMOUNT + 1, 5), sell(P1, 1, -1)] {
        assert_eq!(validate_orders(&game(), P0, &[bad]).len(), 1);
    }
    assert_eq!(validate_orders(&game(), P0, &[sell(P0, 1, 5)]).len(), 1);
}

#[test]
fn a_buyer_who_cannot_pay_loses_the_trade_not_a_unit() {
    let mut s = trading(2, 60);
    s.player_mut(P1).gold = 0;
    let units = s.units.values().filter(|u| u.owner == P1).count();
    let (after, events) = resolve_turn(&s, &BTreeMap::new());
    assert_eq!(ended(&events), vec!["the buyer could not pay"]);
    assert!(after.player(P1).gold >= 0);
    assert_eq!(
        after.units.values().filter(|u| u.owner == P1).count(),
        units
    );
    assert_eq!(resources::supplied(&after, P0, IRON), 2);
}

#[test]
fn a_seller_that_loses_its_deposit_stops_selling() {
    let mut s = trading(2, 5);
    let iron = at(6, 5);
    s.map.get_mut(iron).unwrap().owner = None;
    s.map.get_mut(iron).unwrap().city = None;
    let (after, events) = resolve_turn(&s, &BTreeMap::new());
    assert_eq!(
        ended(&events),
        vec!["the seller lost the deposits behind it"]
    );
    assert_eq!(resources::supplied(&after, P1, IRON), 0);
    assert_eq!(resources::supplied(&after, P0, IRON), 0);
}

#[test]
fn offers_can_be_declined_lapse_and_are_replaced_by_a_newer_one() {
    let (s, events) = step(&game(), &[(P0, sell(P1, 1, 5))]);
    assert!(
        events
            .iter()
            .any(|e| matches!(e, Event::SupplyOffered { .. }))
    );
    // Both sides see the offer; a third empire does not.
    assert_eq!(player_view(&s, P1).supply_trade.offers.len(), 1);
    assert!(player_view(&s, PlayerId(2)).supply_trade.offers.is_empty());
    let offer = s.supply_trades.offers[0].id;
    let (declined, events) = step(
        &s,
        &[(
            P1,
            Order::AnswerSupplyOffer {
                offer,
                accept: false,
                note: None,
            },
        )],
    );
    assert!(declined.supply_trades.offers.is_empty());
    assert!(
        events
            .iter()
            .any(|e| matches!(e, Event::SupplyOfferDeclined { reason: None, .. }))
    );
    // Only the empire it was made to can answer it.
    assert_eq!(
        validate_orders(
            &s,
            PlayerId(2),
            &[Order::AnswerSupplyOffer {
                offer,
                accept: true,
                note: None,
            }]
        )
        .len(),
        1
    );
    // A newer offer for the same resource replaces the old one.
    let (replaced, _) = step(&s, &[(P0, sell(P1, 2, 9))]);
    assert_eq!(replaced.supply_trades.offers.len(), 1);
    assert_eq!(replaced.supply_trades.offers[0].gold, 9);
    // Unanswered, it lapses.
    let mut lapsing = s.clone();
    for _ in 0..OFFER_TURNS {
        assert_eq!(lapsing.supply_trades.offers.len(), 1);
        lapsing = resolve_turn(&lapsing, &BTreeMap::new()).0;
    }
    assert!(lapsing.supply_trades.offers.is_empty());
}

#[test]
fn a_game_without_supply_trades_serializes_as_before() {
    let s = game();
    let json = serde_json::to_string(&s).unwrap();
    assert!(!json.contains("supply_trades"));
    let view = serde_json::to_string(&player_view(&trading(1, 5), P0)).unwrap();
    assert!(view.contains("supply_trade"));
}

#[test]
fn a_game_without_deals_has_no_supply_trades() {
    let mut s = game();
    s.setup.deals = false;
    let order = Order::OfferSupply {
        to: P1,
        selling: true,
        resource: IRON,
        amount: 1,
        gold: 5,
    };
    assert_eq!(
        order_blocker(&s, P0, &order).as_deref(),
        Some(crate::diplomacy::DEALS_OFF)
    );
    assert!(view(&s, P0).is_empty());
}
