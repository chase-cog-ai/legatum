//! Siege starves the land: a land military unit at War with a tile's owner denies the yield of
//! the tile it stands on (`economy::besieges`), exactly as a barbarian unit does.
use crate::diplomacy::RelationState;
use crate::economy::{city_totals, city_yields, worked_tiles};
use crate::events::Event;
use crate::rules::resolve_turn;
use crate::rules::test_support::*;
use crate::state::GameState;
use crate::types::*;
use crate::view::player_view;
use std::collections::BTreeMap;

const HOME: (i32, i32) = (5, 5);

/// P0's city of `pop` at `HOME`, P1 at War with it (the `game` fixture's default).
fn besieged(pop: i32) -> (GameState, CityId) {
    let mut s = game(&[0, 1]);
    let id = city(&mut s, 0, hex(HOME.0, HOME.1), pop);
    (s, id)
}

fn worked_tile(s: &GameState, id: CityId) -> crate::hex::Hex {
    let c = &s.cities[&id];
    *worked_tiles(s, c).iter().find(|h| **h != c.pos).unwrap()
}

fn peace(s: &mut GameState) {
    for relation in &mut s.diplomacy.as_mut().unwrap().relations {
        relation.state = RelationState::Peace;
    }
}

fn labels(s: &GameState, id: CityId) -> Vec<String> {
    city_yields(s, &s.cities[&id])
        .sources
        .into_iter()
        .map(|src| src.label)
        .collect()
}

#[test]
fn an_enemy_army_on_a_worked_tile_removes_its_yields_and_says_so() {
    let (mut s, id) = besieged(1);
    let tile = worked_tile(&s, id);
    let before = city_totals(&s, &s.cities[&id]);
    unit(&mut s, 1, UnitKind::Spearman, tile);
    assert!(!worked_tiles(&s, &s.cities[&id]).contains(&tile));
    let after = city_totals(&s, &s.cities[&id]);
    assert!(after.food < before.food || after.production < before.production);
    let labels = labels(&s, id);
    assert!(
        labels.contains(&format!(
            "An enemy army holds the tile: it yields nothing at {},{}",
            tile.q, tile.r
        )),
        "{labels:?}"
    );
    assert!(!labels.iter().any(|l| l.starts_with("Barbarians hold")));
}

#[test]
fn the_yield_returns_when_the_army_leaves_or_dies() {
    let (mut s, id) = besieged(1);
    let tile = worked_tile(&s, id);
    let before = city_totals(&s, &s.cities[&id]);
    let u = unit(&mut s, 1, UnitKind::Spearman, tile);
    assert_ne!(city_totals(&s, &s.cities[&id]), before);
    // Stepped off the city's land.
    s.units.get_mut(&u).unwrap().pos = hex(HOME.0 + 4, HOME.1);
    assert_eq!(city_totals(&s, &s.cities[&id]), before);
    s.units.get_mut(&u).unwrap().pos = tile;
    assert_ne!(city_totals(&s, &s.cities[&id]), before);
    s.units.remove(&u);
    assert_eq!(city_totals(&s, &s.cities[&id]), before);
}

#[test]
fn peace_returns_the_yield_with_the_army_still_on_the_tile() {
    let (mut s, id) = besieged(1);
    let tile = worked_tile(&s, id);
    let before = city_totals(&s, &s.cities[&id]);
    unit(&mut s, 1, UnitKind::Spearman, tile);
    assert_ne!(city_totals(&s, &s.cities[&id]), before);
    peace(&mut s);
    assert_eq!(city_totals(&s, &s.cities[&id]), before);
    assert!(!labels(&s, id).iter().any(|l| l.contains("holds the tile")));
}

#[test]
fn civilians_scouts_and_the_owners_own_army_deny_nothing() {
    for (owner, kind) in [
        (1, UnitKind::Settler),
        (1, UnitKind::Scout),
        (0, UnitKind::Spearman),
    ] {
        let (mut s, id) = besieged(1);
        let tile = worked_tile(&s, id);
        let before = city_totals(&s, &s.cities[&id]);
        unit(&mut s, owner, kind, tile);
        assert_eq!(city_totals(&s, &s.cities[&id]), before, "P{owner} {kind:?}");
    }
}

#[test]
fn a_teammates_army_denies_nothing() {
    let mut s = game(&[0, 0]);
    let id = city(&mut s, 0, hex(HOME.0, HOME.1), 1);
    let tile = worked_tile(&s, id);
    let before = city_totals(&s, &s.cities[&id]);
    unit(&mut s, 1, UnitKind::Spearman, tile);
    assert_eq!(city_totals(&s, &s.cities[&id]), before);
}

#[test]
fn the_view_marks_the_unit_that_holds_a_tile_for_both_sides() {
    let (mut s, id) = besieged(1);
    let tile = worked_tile(&s, id);
    let u = unit(&mut s, 1, UnitKind::Spearman, tile);
    let off = unit(&mut s, 1, UnitKind::Spearman, hex(HOME.0 + 4, HOME.1));
    for me in [0, 1] {
        let v = player_view(&s, PlayerId(me));
        let held = |id| v.units.iter().find(|x| x.id == id).map(|x| x.holds_tile);
        assert_eq!(held(u), Some(true), "seen by P{me}");
        if me == 1 {
            assert_eq!(held(off), Some(false), "unowned land is not held");
        }
    }
    peace(&mut s);
    let v = player_view(&s, PlayerId(0));
    assert!(!v.units.iter().any(|x| x.holds_tile));
}

/// Armies on every tile the city owns but its centre: the city view counts down to starvation,
/// and the turn it reaches zero the city loses a population.
#[test]
fn a_city_cut_off_from_its_land_shows_the_starvation_countdown_and_then_shrinks() {
    let (mut s, id) = besieged(6);
    // The besieger needs a city of its own, or it is eliminated and its armies removed.
    city(&mut s, 1, hex(10, 1), 1);
    s.player_mut(PlayerId(1)).gold = 1_000;
    let home = s.cities[&id].pos;
    let fed = player_view(&s, PlayerId(0));
    let detail = |v: &crate::view::PlayerView| {
        v.cities
            .iter()
            .find(|c| c.id == id)
            .and_then(|c| c.detail.clone())
            .unwrap()
    };
    assert_eq!(detail(&fed).turns_to_starve, None, "fed before the siege");
    for h in home.within(1) {
        if h != home {
            let u = unit(&mut s, 1, UnitKind::Spearman, h);
            s.units.get_mut(&u).unwrap().fortify = 1;
        }
    }
    let turns = detail(&player_view(&s, PlayerId(0)))
        .turns_to_starve
        .expect("a besieged city with a Food deficit counts down");
    assert!(turns >= 1);
    let mut starved_after = None;
    for n in 1..=turns + 1 {
        let (next, events) = resolve_turn(&s, &BTreeMap::new());
        s = next;
        if events
            .iter()
            .any(|e| matches!(e, Event::CityStarved { city, .. } if *city == id))
        {
            starved_after = Some(n);
            break;
        }
    }
    assert_eq!(starved_after, Some(turns), "the countdown is exact");
    assert_eq!(s.cities[&id].population, 5);
}
