//! Luxuries: flavour tiles, like natural wonders. A small bonus on the worked tile, nothing else.
use super::placement::{LUXURY_MIN_START_DISTANCE, LUXURY_SPACING, luxury_target};
use super::*;
use crate::rules::test_support::{city, game as fixture, hex};
use crate::state::GameSetup;
use crate::{Faction, economy, improvements, mapgen};

#[test]
fn every_luxury_is_always_visible_takes_no_improvement_and_supplies_nothing() {
    for luxury in Resource::LUXURY {
        let stats = stats(luxury);
        assert_eq!(stats.class, ResourceClass::Luxury);
        assert!(is_luxury(luxury) && !is_strategic(luxury));
        assert_eq!(stats.revealed_by, None, "{luxury:?} is visible from turn 1");
        assert_eq!(improvements::kind_for(luxury), None);
        assert!(unit_users(luxury).is_empty(), "{luxury:?} feeds no unit");
        let b = stats.bonus;
        assert_eq!(b.food + b.production + b.gold + b.science, 2, "{luxury:?}");
    }
    for other in Resource::ALL {
        assert_eq!(
            improvements::kind_for(other).is_none(),
            is_luxury(other),
            "{other:?}"
        );
    }
}

fn unit_users(resource: Resource) -> Vec<UnitKind> {
    UnitKind::ALL
        .into_iter()
        .filter(|unit| unit_need(*unit) == Some(resource))
        .collect()
}

#[test]
fn a_worked_luxury_adds_its_bonus_without_an_improvement() {
    let mut s = fixture(&[0, 1]);
    let id = city(&mut s, 0, hex(4, 4), 3);
    let before = economy::city_yields(&s, &s.cities[&id]);
    let tile = before.worked[0];
    assert!(!s.map.get(tile).unwrap().improved);
    s.map.get_mut(tile).unwrap().resource = Some(Resource::Silk);
    let with = economy::city_yields(&s, &s.cities[&id]);
    assert!(with.worked.contains(&tile));
    assert_eq!(with.total.gold - before.total.gold, 2);
    assert!(with.sources.iter().any(|x| x.label == "Resources: Silk"));
    // Nothing to build on it, and an order for one is refused.
    assert!(improvements::candidates(&s, &s.cities[&id]).is_empty());
    let refused = improvements::blocker(&s, &s.cities[&id], ImprovementKind::Farm, tile).unwrap();
    assert!(refused.contains("needs no improvement"), "{refused}");
}

#[test]
fn a_luxury_has_no_empire_wide_effect() {
    let mut s = fixture(&[0, 1]);
    let near = city(&mut s, 0, hex(4, 4), 3);
    let far = city(&mut s, 0, hex(12, 4), 3);
    let far_before = economy::city_yields(&s, &s.cities[&far]).total;
    let tile = economy::city_yields(&s, &s.cities[&near]).worked[0];
    s.map.get_mut(tile).unwrap().resource = Some(Resource::Ivory);
    assert_eq!(economy::city_yields(&s, &s.cities[&far]).total, far_before);
    for resource in Resource::ALL {
        assert_eq!(supplied(&s, PlayerId(0), resource), 0, "{resource:?}");
    }
    assert!(deposits(&s, PlayerId(0)).is_empty());
}

#[test]
fn seeded_placement_puts_each_luxury_once_on_its_terrain_away_from_starts() {
    for seed in 0..24 {
        let setup = GameSetup::standard(seed, [Faction::India, Faction::Switzerland]);
        let (map, starts) = mapgen::generate(&setup);
        let (again, _) = mapgen::generate(&setup);
        assert_eq!(map.tiles, again.tiles, "seed {seed}");
        let placed: Vec<(Hex, Resource)> = map
            .hexes()
            .filter_map(|h| Some((h, map.get(h)?.resource.filter(|r| is_luxury(*r))?)))
            .collect();
        assert!(
            (1..=luxury_target(&setup)).contains(&placed.len()),
            "seed {seed}: {}",
            placed.len()
        );
        for &(h, luxury) in &placed {
            let tile = map.get(h).unwrap();
            assert!(
                stats(luxury).terrains.contains(&tile.terrain),
                "seed {seed}"
            );
            assert!(!tile.ruin && tile.wonder.is_none(), "seed {seed}");
            assert!(
                starts
                    .iter()
                    .all(|s| s.distance(h) >= LUXURY_MIN_START_DISTANCE),
                "seed {seed}"
            );
            for &(other, kind) in &placed {
                if other != h {
                    assert_ne!(kind, luxury, "seed {seed}: one of each");
                    assert!(other.distance(h) >= LUXURY_SPACING, "seed {seed}");
                }
            }
            assert!(
                h.neighbors()
                    .iter()
                    .all(|n| map.get(*n).is_none_or(|t| t.resource.is_none())),
                "seed {seed}: next to another deposit"
            );
        }
    }
}

#[test]
fn the_standard_map_has_all_six_and_a_small_map_fewer() {
    let standard = GameSetup::standard(1, [Faction::India, Faction::Switzerland]);
    assert_eq!(luxury_target(&standard), 6);
    let mut small = standard.clone();
    (small.width, small.height) = (40, 26);
    assert_eq!(luxury_target(&small), 4);
}
