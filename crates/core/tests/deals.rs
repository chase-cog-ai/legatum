//! Deals (rules 61, `GameSetup::deals`): Gold in Peace, Alliance and Open Borders offers, Gold
//! gifts to met empires not at War, and Open Borders as a third treaty.
use legatum_core::diplomacy::{Agreement, DEALS_OFF, ProposalId, RelationState, TreatyKind};
use legatum_core::game::{blank_game, spawn_unit};
use legatum_core::state::Map;
use legatum_core::*;

fn h(c: i32, r: i32) -> Hex {
    Hex::from_offset(c, r)
}

/// One empire per team, all met, each with 500 Gold; `deals` sets the switch.
fn game(players: usize, deals: bool) -> GameState {
    let mut setup = GameSetup::all_ai(19);
    setup.width = 16;
    setup.height = 12;
    setup.players.truncate(players);
    setup.deals = deals;
    let mut s = blank_game(&setup, Map::new(16, 12, Terrain::Grassland));
    for i in 0..players {
        // Settlers keep every empire alive; Switzerland has no passage or attrition rule.
        s.player_mut(PlayerId(i as u8)).faction = Faction::Switzerland;
        spawn_unit(
            &mut s,
            PlayerId(i as u8),
            UnitKind::Settler,
            h(i as i32 * 3, 11),
        );
    }
    let teams = s.teams();
    s.diplomacy.as_mut().unwrap().meet_all(&teams);
    for p in &mut s.players {
        p.gold = 500;
    }
    s
}

fn step(s: &GameState, orders: &[(u8, Vec<Order>)]) -> (GameState, Vec<Event>) {
    resolve_turn(
        s,
        &orders
            .iter()
            .map(|(p, os)| (PlayerId(*p), os.clone()))
            .collect(),
    )
}

fn propose(to: u8, kind: TreatyKind, gold: i32) -> Order {
    Order::ProposeTreaty {
        to: TeamId(to),
        kind,
        gold,
    }
}

fn accept(id: ProposalId) -> Order {
    Order::RespondToProposal {
        proposal: id,
        accept: true,
        explanation: None,
    }
}

fn war(to: u8) -> Order {
    Order::DeclareWar {
        against: TeamId(to),
        breach: false,
    }
}

fn close_borders(with: u8) -> Order {
    Order::BreakTreaty {
        with: TeamId(with),
        agreement: Agreement::OpenBorders,
    }
}

fn errors(s: &GameState, player: u8, orders: &[Order]) -> Vec<String> {
    validate_orders(s, PlayerId(player), orders)
        .into_iter()
        .map(|e| e.message)
        .collect()
}

fn proposal_id(s: &GameState) -> ProposalId {
    s.diplomacy.as_ref().unwrap().proposals[0].id
}

fn borders_open(s: &GameState, a: u8, b: u8) -> bool {
    s.diplomacy
        .as_ref()
        .unwrap()
        .borders_open(TeamId(a), TeamId(b))
}

/// Make an offer from `from` to `to`, then accept it the next turn.
fn deal(s: &GameState, from: u8, to: u8, kind: TreatyKind, gold: i32) -> (GameState, Vec<Event>) {
    let (s, _) = step(s, &[(from, vec![propose(to, kind, gold)])]);
    let id = proposal_id(&s);
    step(&s, &[(to, vec![accept(id)])])
}

/// Each empire's Gold after `orders`, less its Gold after a turn with none (the same income).
fn gold_moved(s: &GameState, orders: &[(u8, Vec<Order>)]) -> (Vec<i32>, Vec<Event>) {
    let (after, events) = step(s, orders);
    let (control, _) = step(s, &[]);
    let moved = after
        .players
        .iter()
        .zip(&control.players)
        .map(|(a, c)| a.gold - c.gold)
        .collect();
    (moved, events)
}

#[test]
fn with_deals_off_every_deal_is_refused_and_nothing_serializes_differently() {
    let s = game(2, false);
    for order in [
        propose(1, TreatyKind::OpenBorders, 0),
        propose(1, TreatyKind::Alliance, 10),
        propose(1, TreatyKind::Alliance, -10),
        close_borders(1),
    ] {
        assert_eq!(errors(&s, 0, &[order]), vec![DEALS_OFF.to_string()]);
    }
    let gift = Order::GiveGold {
        to: PlayerId(1),
        amount: 10,
    };
    assert!(errors(&s, 0, &[gift])[0].contains("is not on your team"));
    // A plain Alliance offer is unchanged.
    assert!(errors(&s, 0, &[propose(1, TreatyKind::Alliance, 0)]).is_empty());

    // The setup, the state with an offer in it, the offer order and the view carry no new field.
    let setup = serde_json::to_string(&GameSetup::all_ai(19)).unwrap();
    assert!(!setup.contains("deals"));
    let (next, _) = step(&s, &[(0, vec![propose(1, TreatyKind::Alliance, 0)])]);
    let state = serde_json::to_string(&next).unwrap();
    assert!(!state.contains("deals") && !state.contains("open_borders"));
    assert!(!state.contains("\"gold\":0"));
    let order = serde_json::to_string(&propose(1, TreatyKind::Peace, 0)).unwrap();
    assert_eq!(order, r#"{"type":"ProposeTreaty","to":1,"kind":"Peace"}"#);
    let view = player_view(&next, PlayerId(0));
    assert!(!view.diplomacy.deals);
    assert!(!view.diplomacy.actions.iter().any(|a| matches!(
        a.order,
        Order::ProposeTreaty {
            kind: TreatyKind::OpenBorders,
            ..
        } | Order::BreakTreaty {
            agreement: Agreement::OpenBorders,
            ..
        }
    )));
    let json = serde_json::to_string(&view.diplomacy).unwrap();
    assert!(!json.contains("open_borders") && !json.contains("deals"));
}

#[test]
fn the_loser_pays_gold_for_peace_when_the_peace_is_made() {
    let s = game(2, true);
    let (s, _) = step(&s, &[(0, vec![war(1)])]);
    assert!(s.at_war(PlayerId(0), PlayerId(1)));
    assert!(errors(&s, 0, &[propose(1, TreatyKind::Peace, 1000)])[0].contains("at most 999"));
    assert!(errors(&s, 0, &[propose(1, TreatyKind::Peace, 600)])[0].contains("costs 600 Gold"));
    let (s, _) = step(&s, &[(0, vec![propose(1, TreatyKind::Peace, 80)])]);
    let p = &s.diplomacy.as_ref().unwrap().proposals[0];
    assert_eq!(p.gold, 80);
    let view = player_view(&s, PlayerId(1));
    assert_eq!(view.diplomacy.proposals[0].gold, 80);
    let id = p.id;
    let (moved, events) = gold_moved(&s, &[(1, vec![accept(id)])]);
    assert_eq!(moved, vec![-80, 80]);
    let accepted = events
        .iter()
        .position(|e| matches!(e, Event::ProposalAccepted { .. }))
        .unwrap();
    assert!(matches!(
        events[accepted + 1],
        Event::GoldGiven {
            from: PlayerId(0),
            to: PlayerId(1),
            amount: 80
        }
    ));
    let (after, _) = step(&s, &[(1, vec![accept(id)])]);
    assert!(!after.at_war(PlayerId(0), PlayerId(1)));
}

#[test]
fn an_alliance_offer_can_ask_for_gold_and_the_accepter_must_afford_it() {
    let s = game(2, true);
    let (mut s, _) = step(&s, &[(0, vec![propose(1, TreatyKind::Alliance, -40)])]);
    let id = proposal_id(&s);
    s.player_mut(PlayerId(1)).gold = 10;
    assert!(errors(&s, 1, &[accept(id)])[0].contains("costs 40 Gold"));
    let view = player_view(&s, PlayerId(1));
    let action = view
        .diplomacy
        .actions
        .iter()
        .find(|a| a.order == accept(id))
        .unwrap();
    assert_eq!(action.cost, 40);
    s.player_mut(PlayerId(1)).gold = 100;
    assert!(errors(&s, 1, &[accept(id)]).is_empty());
    let (moved, events) = gold_moved(&s, &[(1, vec![accept(id)])]);
    assert_eq!(moved, vec![40, -40]);
    assert!(events.iter().any(|e| matches!(
        e,
        Event::GoldGiven {
            from: PlayerId(1),
            to: PlayerId(0),
            amount: 40
        }
    )));
    let (after, _) = step(&s, &[(1, vec![accept(id)])]);
    assert!(after.allied(PlayerId(0), PlayerId(1)));
}

#[test]
fn a_payer_short_of_gold_at_resolution_ends_the_offer_and_signs_nothing() {
    let s = game(2, true);
    let (mut s, _) = step(&s, &[(0, vec![propose(1, TreatyKind::OpenBorders, 300)])]);
    let id = proposal_id(&s);
    // The offer was affordable when made; the treasury is empty when it would take effect.
    s.player_mut(PlayerId(0)).gold = 0;
    let (after, events) = step(&s, &[(1, vec![accept(id)])]);
    assert!(events.iter().any(|e| matches!(
        e,
        Event::ProposalInvalidated { reason, .. } if reason == "Switzerland could not pay 300 Gold"
    )));
    assert!(!events.iter().any(|e| matches!(
        e,
        Event::GoldGiven { .. } | Event::BordersOpened { .. } | Event::ProposalAccepted { .. }
    )));
    assert!(!borders_open(&after, 0, 1));
    assert!(after.diplomacy.as_ref().unwrap().proposals.is_empty());
}

#[test]
fn open_borders_let_units_in_at_peace_and_end_when_closed_at_war_or_in_an_alliance() {
    let mut s = game(3, true);
    let inside = h(6, 5);
    for tile in inside.within(2) {
        s.map.get_mut(tile).unwrap().owner = Some(PlayerId(1));
    }
    let edge = inside.neighbors()[0];
    let start = edge.neighbors()[0];
    assert_eq!(s.map.get(start).unwrap().owner, Some(PlayerId(1)));
    let outside = h(2, 5);
    let unit = spawn_unit(&mut s, PlayerId(0), UnitKind::Spearman, outside);
    let into =
        |s: &GameState| s.passage_allowed(PlayerId(0), UnitKind::Spearman, Some(PlayerId(1)));
    assert!(!into(&s));

    // Offer and accept Open Borders.
    let view = player_view(&s, PlayerId(0));
    assert!(view.diplomacy.deals);
    let offer = view
        .diplomacy
        .actions
        .iter()
        .find(|a| a.order == propose(1, TreatyKind::OpenBorders, 0))
        .unwrap();
    assert_eq!(offer.blocked_reason, None);
    assert_eq!((offer.cost, offer.upkeep), (0, 0));
    let (s, events) = deal(&s, 0, 1, TreatyKind::OpenBorders, 0);
    assert!(events.iter().any(|e| matches!(
        e,
        Event::BordersOpened {
            a: TeamId(0),
            b: TeamId(1),
            ..
        }
    )));
    assert!(borders_open(&s, 0, 1) && into(&s));
    assert!(!s.shares_vision(PlayerId(0), PlayerId(1)));
    let view = player_view(&s, PlayerId(0));
    assert_eq!(view.diplomacy.open_borders, vec![(TeamId(0), TeamId(1))]);
    assert!(view.passage_allowed(PlayerId(0), UnitKind::Spearman, Some(PlayerId(1))));
    // A third team still may not enter.
    assert!(!s.passage_allowed(PlayerId(2), UnitKind::Spearman, Some(PlayerId(1))));
    assert!(
        errors(&s, 0, &[propose(1, TreatyKind::OpenBorders, 0)])[0].contains("already have Open")
    );

    // A unit walks in at Peace.
    let mut s = s;
    s.units.get_mut(&unit).unwrap().pos = start;
    let order = Order::MoveUnit { unit, to: inside };
    assert!(errors(&s, 0, std::slice::from_ref(&order)).is_empty());
    let (s, _) = step(&s, &[(0, vec![order])]);
    assert_ne!(s.units[&unit].pos, start, "the unit moved into their land");

    // Either side closes them, free, next turn; the unit is not expelled.
    let pos = s.units[&unit].pos;
    assert!(errors(&s, 0, &[close_borders(2)])[0].contains("No Open Borders to end"));
    let view = player_view(&s, PlayerId(1));
    let close = view
        .diplomacy
        .actions
        .iter()
        .find(|a| a.order == close_borders(0))
        .unwrap();
    assert_eq!(
        (close.blocked_reason.as_deref(), close.cost, close.betrayal),
        (None, 0, false)
    );
    let (closed, events) = step(&s, &[(1, vec![close_borders(0)])]);
    assert!(events.iter().any(|e| matches!(
        e,
        Event::BordersClosed { reason, .. } if reason.contains("closed the borders")
    )));
    assert!(!borders_open(&closed, 0, 1) && !into(&closed));
    assert_eq!(closed.units[&unit].pos, pos);
    assert!(closed.diplomacy.as_ref().unwrap().betrayals.is_empty());

    // A surprise war ends them in the same turn.
    let (s, events) = step(&s, &[(1, vec![war(0)])]);
    assert!(events.iter().any(|e| matches!(
        e,
        Event::BordersClosed { reason, .. } if reason.contains("War")
    )));
    assert!(!borders_open(&s, 0, 1));
    assert_eq!(
        s.diplomacy
            .as_ref()
            .unwrap()
            .relation(TeamId(0), TeamId(1))
            .unwrap()
            .state,
        RelationState::War
    );
    assert!(errors(&s, 0, &[propose(1, TreatyKind::OpenBorders, 0)])[0].contains("make peace"));
}

#[test]
fn an_alliance_replaces_open_borders() {
    let s = game(2, true);
    let (s, _) = deal(&s, 0, 1, TreatyKind::OpenBorders, 0);
    assert!(borders_open(&s, 0, 1));
    let (s, events) = deal(&s, 1, 0, TreatyKind::Alliance, 0);
    assert!(s.allied(PlayerId(0), PlayerId(1)));
    assert!(events.iter().any(|e| matches!(
        e,
        Event::BordersClosed { reason, .. } if reason.contains("Alliance")
    )));
    assert!(!borders_open(&s, 0, 1));
    // Allies don't need Open Borders.
    assert!(errors(&s, 0, &[propose(1, TreatyKind::OpenBorders, 0)])[0].contains("allied"));
}

#[test]
fn gold_gifts_reach_a_met_empire_not_at_war() {
    let s = game(3, true);
    let gift = |to: u8| Order::GiveGold {
        to: PlayerId(to),
        amount: 25,
    };
    assert!(errors(&s, 0, &[gift(1)]).is_empty());
    let (moved, events) = gold_moved(&s, &[(0, vec![gift(1)])]);
    // The Swiss giver also earns a little less interest on the Gold it gave away.
    assert!((-26..=-25).contains(&moved[0]), "{moved:?}");
    assert_eq!(moved[1..], [25, 0]);
    assert!(events.iter().any(|e| matches!(
        e,
        Event::GoldGiven {
            from: PlayerId(0),
            to: PlayerId(1),
            amount: 25
        }
    )));
    // The receiver's team sees the gift; a third team does not.
    let (after, events) = step(&s, &[(0, vec![gift(1)])]);
    let seen = |p: u8| {
        use legatum_core::event_visibility::{VisibleEvent, event_visibility_view, visible_events};
        let (before_view, after_view) = (
            event_visibility_view(&s, PlayerId(p)),
            event_visibility_view(&after, PlayerId(p)),
        );
        visible_events(&events, &before_view, &after_view)
            .iter()
            .any(|e| matches!(e, VisibleEvent::Complete(Event::GoldGiven { .. })))
    };
    assert!(seen(0) && seen(1) && !seen(2));

    // At War: refused.
    let (s, _) = step(&s, &[(0, vec![war(2)])]);
    assert!(errors(&s, 0, &[gift(2)])[0].contains("you are at War with"));
    // Not met: refused.
    let mut s = s;
    s.diplomacy.as_mut().unwrap().contacts = Some(Default::default());
    assert!(errors(&s, 0, &[gift(1)])[0].contains("is not on your team"));
}
