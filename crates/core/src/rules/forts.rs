//! Forts (roadmap item "units building Forts (Feudalism)", built 2026-11-25).
//!
//! - **Build.** `Order::BuildFort`: a land military unit (not a Scout) standing on a tile of
//!   its own empire, once its owner knows Feudalism, digs a Fort there for `data::FORT_GOLD`.
//!   Not on a city, wonder or landmark tile, and not where a Fort already stands. It is the
//!   unit's order for the turn. The Fort is permanent and stays when the tile changes hands.
//! - **Defence.** A military unit standing in a Fort defends with +`data::FORT_DEFENSE`
//!   (`combat::add_fort`, the line "Fort"), on top of terrain and fortification. Whoever
//!   stands in it has it: a Fort taken by the enemy shelters the enemy.
//! - **Blocks the road.** A unit that steps from outside a Fort's reach onto a tile next to it
//!   stops there for the turn, when a military unit it is at War with holds the Fort
//!   (`blocker`, applied in `movement::movement_round`): it cannot walk past in one turn. A
//!   unit that starts its turn beside the Fort moves freely, attacking the Fort is never
//!   stopped, and an empty Fort stops nobody.

use crate::data;
use crate::hex::Hex;
use crate::state::GameState;
use crate::types::*;

/// Why `unit` cannot dig a Fort now, or its Gold price. `gold` is the treasury left.
pub fn check(state: &GameState, player: PlayerId, unit: UnitId, gold: i32) -> Result<i32, String> {
    let u = state.units.get(&unit).ok_or("no such unit")?;
    let name = data::unit_name_for(state.player(player).faction, u.kind);
    if !data::can_build_fort(u.kind) {
        return Err(format!("{name} cannot build a Fort"));
    }
    if !state.player(player).techs.contains(&data::FORT_TECH) {
        return Err(format!(
            "Forts need {}",
            data::tech_name(data::FORT_TECH)
        ));
    }
    let tile = state.map.get(u.pos).ok_or("no such tile")?;
    if tile.fort {
        return Err("There is already a Fort here".into());
    }
    if tile.owner != Some(player) {
        return Err("A Fort is built only in your own territory".into());
    }
    if state.city_at(u.pos).is_some() {
        return Err("A Fort cannot be built in a city".into());
    }
    if tile.wonder.is_some() || tile.landmark.is_some() {
        return Err("A Fort cannot be built on a wonder".into());
    }
    if gold < data::FORT_GOLD {
        return Err(format!(
            "A Fort costs {} Gold, you have {gold}",
            data::FORT_GOLD
        ));
    }
    Ok(data::FORT_GOLD)
}

/// The Fort that stops a unit of `mover` stepping from `from` onto `to`: a Fort tile next to
/// `to`, but not next to `from`, held by a military unit `mover` is at War with. A unit already
/// beside a Fort moves on freely, so a Fort costs a passer one turn, not one per tile. The
/// lowest neighbour direction wins, so the answer never depends on unit order.
pub fn blocker(state: &GameState, mover: PlayerId, from: Hex, to: Hex) -> Option<Hex> {
    to.neighbors().into_iter().find(|h| {
        from.distance(*h) > 1
            && state.map.get(*h).is_some_and(|t| t.fort)
            && state.units.values().any(|v| {
                v.pos == *h && super::is_military(v.kind) && super::hostile(state, mover, v.owner)
            })
    })
}

#[cfg(test)]
mod tests {
    use super::super::test_support::*;
    use super::*;
    use crate::events::Event;
    use crate::orders::Order;
    use crate::state::GameState;

    const FORT: (i32, i32) = (5, 3);

    /// A corridor along row 4 between mountains, with one open tile above it (the Fort tile)
    /// owned by player 1, and a Horseman of player 0 at its west end bound for its east end.
    fn corridor() -> (GameState, UnitId) {
        let mut s = game(&[0, 1]);
        for c in 0..12 {
            for r in [3, 5] {
                if (c, r) != FORT {
                    set_terrain(&mut s, hex(c, r), Terrain::Mountains);
                }
            }
        }
        s.map.get_mut(hex(FORT.0, FORT.1)).unwrap().owner = Some(PlayerId(1));
        let rider = unit(&mut s, 0, UnitKind::Horseman, hex(2, 4));
        goto(&mut s, rider, hex(9, 4));
        (s, rider)
    }

    fn fort(s: &mut GameState) -> UnitId {
        s.map.get_mut(hex(FORT.0, FORT.1)).unwrap().fort = true;
        unit(s, 1, UnitKind::Spearman, hex(FORT.0, FORT.1))
    }

    fn stopped(events: &[Event], unit: UnitId) -> Option<(Hex, Hex)> {
        events.iter().find_map(|e| match e {
            Event::StoppedByFort {
                unit: u, at, fort, ..
            } if *u == unit => Some((*at, *fort)),
            _ => None,
        })
    }

    #[test]
    fn a_unit_stops_next_to_an_enemy_fort_and_walks_on_next_turn() {
        let (mut s, rider) = corridor();
        let (free, events) = resolve_units(&s, &no_orders());
        let free_pos = free.units[&rider].pos;
        assert!(stopped(&events, rider).is_none());

        fort(&mut s);
        let (next, events) = resolve_units(&s, &no_orders());
        let u = &next.units[&rider];
        assert_eq!(u.pos.distance(hex(FORT.0, FORT.1)), 1, "stopped beside it");
        assert!(
            u.pos.distance(hex(2, 4)) < free_pos.distance(hex(2, 4)),
            "without the Fort it walks further in the turn"
        );
        assert_eq!(u.moves_left, 0);
        assert_eq!(u.goto, Some(hex(9, 4)), "it keeps its destination");
        assert_eq!(stopped(&events, rider), Some((u.pos, hex(FORT.0, FORT.1))));

        // Next turn it starts beside the Fort and leaves: the stop is on arrival only.
        let (later, _) = resolve_units(&next, &no_orders());
        assert!(later.units[&rider].pos.distance(hex(FORT.0, FORT.1)) > 1);
    }

    #[test]
    fn an_empty_fort_and_a_fort_at_peace_stop_nobody() {
        let (mut s, rider) = corridor();
        s.map.get_mut(hex(FORT.0, FORT.1)).unwrap().fort = true;
        let (_, events) = resolve_units(&s, &no_orders());
        assert!(stopped(&events, rider).is_none(), "nobody holds it");

        fort(&mut s);
        for relation in &mut s.diplomacy.as_mut().unwrap().relations {
            relation.state = crate::diplomacy::RelationState::Peace;
        }
        // At Peace the rider may not be allowed through closed borders at all; only the
        // stop is asserted.
        let (_, events) = resolve_units(&s, &no_orders());
        assert!(stopped(&events, rider).is_none(), "not at War");
    }

    #[test]
    fn the_holders_own_units_pass_their_fort() {
        let (mut s, _) = corridor();
        fort(&mut s);
        let friend = unit(&mut s, 1, UnitKind::Horseman, hex(1, 4));
        s.units.retain(|_, u| u.owner == PlayerId(1));
        goto(&mut s, friend, hex(9, 4));
        let (_, events) = resolve_units(&s, &no_orders());
        assert!(stopped(&events, friend).is_none());
    }

    #[test]
    fn building_a_fort_costs_gold_and_the_units_turn() {
        let mut s = game(&[0, 1]);
        let at = hex(4, 4);
        let digger = unit(&mut s, 0, UnitKind::Spearman, at);
        let order = Order::BuildFort { unit: digger };
        let reason = |s: &GameState| {
            crate::validate_orders(s, PlayerId(0), std::slice::from_ref(&order))
                .first()
                .map(|e| e.message.clone())
        };
        s.players[0].gold = data::FORT_GOLD;
        assert_eq!(reason(&s).as_deref(), Some("Forts need Feudalism"));
        s.players[0].techs.insert(data::FORT_TECH);
        assert_eq!(
            reason(&s).as_deref(),
            Some("A Fort is built only in your own territory")
        );
        s.map.get_mut(at).unwrap().owner = Some(PlayerId(0));
        s.players[0].gold = data::FORT_GOLD - 1;
        assert!(reason(&s).is_some_and(|r| r.starts_with("A Fort costs 100 Gold")));
        s.players[0].gold = data::FORT_GOLD + 7;
        assert_eq!(reason(&s), None);

        let (next, events) = resolve_units(&s, &orders(0, vec![order.clone()]));
        assert!(next.map.get(at).unwrap().fort);
        assert_eq!(next.players[0].gold, 7);
        assert!(events.iter().any(|e| matches!(
            e,
            Event::FortBuilt { unit, at: h, gold, .. }
                if *unit == digger && *h == at && *gold == data::FORT_GOLD
        )));
        assert_eq!(
            reason(&next).as_deref(),
            Some("There is already a Fort here")
        );

        // Scouts and civilians do not dig.
        let scout = unit(&mut s, 0, UnitKind::Scout, at);
        assert!(check(&s, PlayerId(0), scout, 1000).is_err());
    }

    #[test]
    fn a_unit_in_a_fort_defends_with_the_fort_line() {
        let mut s = game(&[0, 1]);
        let attacker = unit(&mut s, 0, UnitKind::Spearman, hex(4, 4));
        let holder = unit(&mut s, 1, UnitKind::Spearman, hex(5, 4));
        let defence = |s: &GameState| {
            let view = crate::view::player_view(s, PlayerId(0));
            crate::forecast::forecast_attack(&view, attacker, hex(5, 4))
                .expect("forecast")
                .defender_strength
        };
        let open = defence(&s);
        s.map.get_mut(hex(5, 4)).unwrap().fort = true;
        let held = defence(&s);
        assert_eq!(held.total, open.total + data::FORT_DEFENSE);
        assert!(
            held.parts
                .iter()
                .any(|(l, v)| l == crate::combat::FORT_LABEL && *v == data::FORT_DEFENSE)
        );
        // Resolution agrees with the forecast.
        let fight = super::super::battle::plan_fight(
            &s,
            1,
            attacker,
            super::super::battle::Target::Unit(holder),
            super::super::battle::FightKind::Melee,
        );
        assert_eq!(fight.defender_strength.total, held.total);
    }
}
