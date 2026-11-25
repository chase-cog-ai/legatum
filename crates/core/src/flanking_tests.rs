//! Flanking: a melee attack on a unit gains strength for each other land military unit of the
//! attacker's team next to the defender, and the forecast shows the same line as the fight.
use crate::combat;
use crate::rules::battle::{FightKind, Target, flankers, plan_fight};
use crate::rules::test_support::{city, game, hex, unit};
use crate::{forecast_attack, player_view, *};

const TARGET: (i32, i32) = (5, 4);

/// Two-team war with a third player allied to the attacker: attacker (P0) west of the defender
/// (P1), and the defender's other five neighbours free for flankers.
fn field() -> (GameState, UnitId, UnitId, Vec<Hex>) {
    let mut s = game(&[0, 1, 0]);
    let target = hex(TARGET.0, TARGET.1);
    let around = target.neighbors();
    let a = unit(&mut s, 0, UnitKind::Spearman, around[0]);
    let d = unit(&mut s, 1, UnitKind::Spearman, target);
    (s, a, d, around[1..].to_vec())
}

fn flanking_line(s: &GameState, a: UnitId, d: UnitId, kind: FightKind) -> Option<(String, i32)> {
    plan_fight(s, 1, a, Target::Unit(d), kind)
        .attacker_strength
        .parts
        .into_iter()
        .find(|(l, _)| l.starts_with("Flanking"))
}

#[test]
fn a_lone_attacker_gets_nothing() {
    let (s, a, d, _) = field();
    assert_eq!(flankers(&s, &s.units[&a], s.units[&d].pos), 0);
    assert_eq!(flanking_line(&s, a, d, FightKind::Melee), None);
}

#[test]
fn each_flanker_adds_strength_up_to_three() {
    let (mut s, a, d, free) = field();
    let base = plan_fight(&s, 1, a, Target::Unit(d), FightKind::Melee)
        .attacker_strength
        .total;
    for (i, h) in free.iter().enumerate() {
        unit(&mut s, 0, UnitKind::Spearman, *h);
        let n = (i + 1).min(usize::from(data::FLANKING_MAX_UNITS));
        let bonus = n as i32 * data::FLANKING_STRENGTH;
        assert_eq!(
            flanking_line(&s, a, d, FightKind::Melee),
            Some((combat::FLANKING_LABELS[n - 1].to_string(), bonus)),
            "{} flankers",
            i + 1
        );
        let fight = plan_fight(&s, 1, a, Target::Unit(d), FightKind::Melee);
        assert_eq!(fight.attacker_strength.total, base + bonus);
    }
    assert_eq!(combat::flanking_bonus(5), 6, "capped at three units");
}

#[test]
fn a_teammates_unit_flanks_and_an_enemy_or_unarmed_unit_does_not() {
    let (mut s, a, d, free) = field();
    unit(&mut s, 1, UnitKind::Spearman, free[0]);
    unit(&mut s, 0, UnitKind::Settler, free[1]);
    unit(&mut s, 0, UnitKind::Scout, free[2]);
    assert_eq!(flanking_line(&s, a, d, FightKind::Melee), None);
    unit(&mut s, 2, UnitKind::Archer, free[3]);
    assert_eq!(
        flanking_line(&s, a, d, FightKind::Melee),
        Some((
            combat::FLANKING_LABELS[0].to_string(),
            data::FLANKING_STRENGTH
        ))
    );
}

#[test]
fn shots_and_meeting_engagements_are_not_flanked() {
    let (mut s, a, d, free) = field();
    unit(&mut s, 0, UnitKind::Spearman, free[0]);
    let archer = unit(&mut s, 0, UnitKind::Archer, free[1]);
    assert_eq!(flanking_line(&s, archer, d, FightKind::Ranged), None);
    assert_eq!(flanking_line(&s, a, d, FightKind::Meeting), None);
    assert!(flanking_line(&s, archer, d, FightKind::Melee).is_some());
}

#[test]
fn a_unit_in_a_city_cannot_be_flanked() {
    let (mut s, a, d, free) = field();
    unit(&mut s, 0, UnitKind::Spearman, free[0]);
    let c = city(&mut s, 1, hex(TARGET.0, TARGET.1), 1);
    assert_eq!(flankers(&s, &s.units[&a], s.units[&d].pos), 0);
    s.cities.get_mut(&c).unwrap().hp = 0;
    assert_eq!(flanking_line(&s, a, d, FightKind::Melee), None);
    let f = forecast_attack(&player_view(&s, PlayerId(0)), a, hex(TARGET.0, TARGET.1)).unwrap();
    assert!(
        f.attacker_strength
            .parts
            .iter()
            .all(|(l, _)| !l.starts_with("Flanking"))
    );
}

#[test]
fn the_forecast_shows_the_same_flanking_as_the_fight() {
    let (mut s, a, d, free) = field();
    unit(&mut s, 0, UnitKind::Spearman, free[0]);
    unit(&mut s, 2, UnitKind::Horseman, free[2]);
    let alone = {
        let (s, a, _, _) = field();
        forecast_attack(&player_view(&s, PlayerId(0)), a, hex(TARGET.0, TARGET.1)).unwrap()
    };
    let f = forecast_attack(&player_view(&s, PlayerId(0)), a, hex(TARGET.0, TARGET.1)).unwrap();
    let fight = plan_fight(&s, 1, a, Target::Unit(d), FightKind::Melee);
    assert_eq!(f.attacker_strength.parts, fight.attacker_strength.parts);
    assert_eq!(f.defender_strength.parts, fight.defender_strength.parts);
    assert_eq!(
        f.attacker_strength.total,
        alone.attacker_strength.total + 2 * data::FLANKING_STRENGTH
    );
    assert!(
        f.expected_damage_to_defender > alone.expected_damage_to_defender,
        "it hits harder"
    );
    assert!(
        f.expected_damage_to_attacker < alone.expected_damage_to_attacker,
        "and is hit less"
    );
}

#[test]
fn the_no_flanking_experiment_switches_it_off_in_the_fight_and_the_forecast() {
    let (mut s, a, d, free) = field();
    unit(&mut s, 0, UnitKind::Spearman, free[0]);
    s.setup.rule_experiments.no_flanking = true;
    assert_eq!(flanking_line(&s, a, d, FightKind::Melee), None);
    let f = forecast_attack(&player_view(&s, PlayerId(0)), a, hex(TARGET.0, TARGET.1)).unwrap();
    let fight = plan_fight(&s, 1, a, Target::Unit(d), FightKind::Melee);
    assert_eq!(f.attacker_strength.parts, fight.attacker_strength.parts);
}
