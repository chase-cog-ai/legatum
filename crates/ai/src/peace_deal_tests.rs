//! Gene `peace_terms` (`peace_deal.rs`): what the AI signs, refuses and sends back when a Peace
//! offer carries terms, played from the seat of a human trying to milk it.
use crate::fixture::{ENEMY, ME, ViewBuilder, off};
use crate::peace_deal::PeaceAnswer;
use legatum_core::diplomacy::{Betrayal, PeaceTerms, Relation, RelationState};
use legatum_core::{CityId, PlayerId, PlayerView, TeamId, UnitKind};

const THIRD: PlayerId = PlayerId(3);

/// Turn 80 at War with the enemy since turn 60, 400 Gold. `declarer` started it. `pressed`
/// puts an enemy army beside our border city, which nothing defends.
fn war_view(declarer: PlayerId, pressed: bool) -> PlayerView {
    let mut b = ViewBuilder::new(40, 20)
        .gold(400)
        .city(1, ME, off(4, 8), 3)
        .city(2, ME, off(10, 8), 4)
        .city(10, ENEMY, off(30, 8), 3)
        .city(11, ENEMY, off(18, 8), 5)
        .city(20, THIRD, off(20, 16), 3);
    if pressed {
        for i in 0..4 {
            b = b.unit(
                50 + i,
                ENEMY,
                UnitKind::Spearman,
                off(11 + i as i32 % 2, 7 + i as i32 / 2),
            );
        }
    } else {
        b = b.unit(1, ME, UnitKind::Spearman, off(10, 8));
    }
    let mut v = b.build();
    v.turn = 80;
    v.diplomacy.legacy = false;
    let (me, enemy) = (v.team, v.team_of(ENEMY));
    v.diplomacy.relations = vec![Relation {
        a: me,
        b: enemy,
        state: RelationState::War,
        effective_turn: 60,
        guarantee_until: 0,
        version: 1,
        security: None,
        war_id: Some(1),
        declarers: vec![v.team_of(declarer)],
        reparations: None,
        truce_released: None,
    }];
    v.cities[0].is_capital = true;
    crate::fixture::set_armies(&mut v, &[(ME, 200), (ENEMY, 200), (THIRD, 200)]);
    v
}

fn answer(v: &PlayerView, terms: PeaceTerms, base: Option<i32>) -> PeaceAnswer {
    crate::fixture::planner(v, 7)
        .peace_verdict(ENEMY, v.team_of(ENEMY), &terms, base, false)
        .answer
}
fn gold(gold: i32) -> PeaceTerms {
    PeaceTerms {
        gold,
        ..Default::default()
    }
}
fn enemy(v: &PlayerView) -> TeamId {
    v.team_of(ENEMY)
}

#[test]
fn gold_buys_a_peace_the_score_narrowly_refuses_and_the_counter_names_the_price() {
    let v = war_view(ME, false);
    // Ten points short: 100 Gold.
    assert_eq!(answer(&v, gold(100), Some(-10)), PeaceAnswer::Accept);
    assert_eq!(
        answer(&v, gold(40), Some(-10)),
        PeaceAnswer::Counter(gold(-100)),
        "the counter asks what is short, written from our side"
    );
    assert_eq!(
        answer(&v, gold(0), Some(-10)),
        PeaceAnswer::Counter(gold(-100))
    );
}

#[test]
fn no_terms_buy_off_a_war_the_plan_will_not_end() {
    let v = war_view(ME, false);
    let rich = PeaceTerms {
        gold: 9_999,
        city: Some(CityId(11)),
        reparations: 99,
        ..Default::default()
    };
    assert_eq!(answer(&v, rich, None), PeaceAnswer::Refuse);
}

#[test]
fn liking_peace_is_no_reason_to_pay_for_it() {
    // Exploit: ask an AI that would sign anyway for Gold on top.
    let v = war_view(ME, false);
    assert_eq!(answer(&v, gold(0), Some(25)), PeaceAnswer::Accept);
    assert_eq!(
        answer(&v, gold(-1), Some(25)),
        PeaceAnswer::Counter(gold(0)),
        "it sends the plain Peace back"
    );
    let city = PeaceTerms {
        city: Some(CityId(2)),
        ..Default::default()
    };
    assert_eq!(answer(&v, city, Some(25)), PeaceAnswer::Counter(gold(0)));
}

#[test]
fn declaring_war_on_the_ai_extorts_nothing() {
    // Exploit: declare, press a city, demand Gold or the city for Peace; repeat each truce.
    let v = war_view(ENEMY, true);
    assert!(crate::fixture::planner(&v, 7).endangered_by(enemy(&v)));
    assert_eq!(answer(&v, gold(0), Some(25)), PeaceAnswer::Accept);
    for terms in [
        gold(-10),
        PeaceTerms {
            reparations: -1,
            ..Default::default()
        },
        PeaceTerms {
            city: Some(CityId(2)),
            ..Default::default()
        },
    ] {
        assert_eq!(answer(&v, terms, Some(25)), PeaceAnswer::Counter(gold(0)));
    }
}

#[test]
fn a_war_the_ai_started_and_is_losing_is_worth_gold_up_to_half_its_treasury() {
    let v = war_view(ME, true);
    // 25 points of wanting Peace: 250 Gold, but 400 in the treasury caps it at 200.
    assert_eq!(answer(&v, gold(-200), Some(25)), PeaceAnswer::Accept);
    assert_eq!(
        answer(&v, gold(-210), Some(25)),
        PeaceAnswer::Counter(gold(0))
    );
    // The eagerness that pays is capped at 30 points however desperate the score.
    let mut rich = v.clone();
    rich.economy.gold = 5_000;
    assert_eq!(answer(&rich, gold(-300), Some(90)), PeaceAnswer::Accept);
    assert_ne!(answer(&rich, gold(-310), Some(90)), PeaceAnswer::Accept);
    // Its own offer carries Gold then, and none when it was the one attacked.
    assert_eq!(
        crate::fixture::planner(&v, 7).peace_sweetener(enemy(&v)),
        100
    );
    let attacked = war_view(ENEMY, true);
    assert_eq!(
        crate::fixture::planner(&attacked, 7).peace_sweetener(enemy(&attacked)),
        0
    );
}

#[test]
fn a_city_about_to_fall_to_someone_else_is_worth_nothing() {
    // Exploit: sell the AI a city a third army is about to take.
    let sale = PeaceTerms {
        city: Some(CityId(11)),
        ..Default::default()
    };
    let v = war_view(ME, false);
    // A sound city of 5 is worth 450 Gold: it outweighs a Peace 30 points short.
    assert_eq!(answer(&v, sale.clone(), Some(-30)), PeaceAnswer::Accept);
    let mut b = ViewBuilder::new(40, 20)
        .gold(400)
        .city(1, ME, off(4, 8), 3)
        .city(2, ME, off(10, 8), 4)
        .city(10, ENEMY, off(30, 8), 3)
        .city(11, ENEMY, off(18, 8), 5)
        .unit(1, ME, UnitKind::Spearman, off(10, 8));
    b = b.unit(90, THIRD, UnitKind::Spearman, off(19, 8));
    let mut besieged = b.build();
    besieged.turn = 80;
    besieged.diplomacy = v.diplomacy.clone();
    let (third, enemy) = (besieged.team_of(THIRD), besieged.team_of(ENEMY));
    besieged.diplomacy.relations.push(Relation {
        a: third.min(enemy),
        b: third.max(enemy),
        war_id: Some(2),
        declarers: vec![third],
        ..v.diplomacy.relations[0].clone()
    });
    assert_ne!(
        answer(&besieged, sale.clone(), Some(-30)),
        PeaceAnswer::Accept
    );
    let mut damaged = v.clone();
    let c = damaged
        .cities
        .iter_mut()
        .find(|c| c.id == CityId(11))
        .unwrap();
    c.hp = c.max_hp / 2;
    assert_ne!(answer(&damaged, sale, Some(-30)), PeaceAnswer::Accept);
}

#[test]
fn promised_reparations_count_for_a_quarter_and_nothing_from_a_betrayer() {
    // Exploit: promise 99 Gold a turn, sign, stop paying.
    let v = war_view(ME, false);
    let promise = |reparations| PeaceTerms {
        reparations,
        ..Default::default()
    };
    // 40 a turn for 10 turns is 400 on paper, 100 to the AI.
    assert_eq!(answer(&v, promise(40), Some(-10)), PeaceAnswer::Accept);
    assert_ne!(answer(&v, promise(39), Some(-11)), PeaceAnswer::Accept);
    let mut burned = v.clone();
    burned.diplomacy.betrayals.push(Betrayal {
        by: enemy(&v),
        against: v.team,
        turn: 70,
        until: 110,
    });
    assert_eq!(
        answer(&burned, promise(99), Some(-10)),
        PeaceAnswer::Counter(gold(-100)),
        "a betrayer's promise is worth nothing: Gold up front or no Peace"
    );
}

#[test]
fn a_long_truce_costs_the_stronger_army_and_a_counter_offer_is_never_countered() {
    let mut v = war_view(ME, false);
    crate::fixture::set_armies(&mut v, &[(ME, 300), (ENEMY, 200), (THIRD, 200)]);
    let long = PeaceTerms {
        truce: 30,
        ..Default::default()
    };
    // Half again their army: 50 x 20 extra turns / 40 = 25 points.
    assert_eq!(
        answer(&v, long.clone(), Some(0)),
        PeaceAnswer::Counter(gold(0))
    );
    let paid = PeaceTerms {
        gold: 250,
        ..long.clone()
    };
    assert_eq!(answer(&v, paid, Some(0)), PeaceAnswer::Accept);
    let p = crate::fixture::planner(&v, 7);
    assert_eq!(
        p.peace_verdict(ENEMY, enemy(&v), &long, Some(0), true)
            .answer,
        PeaceAnswer::Refuse
    );
}

#[test]
fn the_ai_offers_back_what_it_took_when_its_own_war_turns_on_it() {
    let mut v = war_view(ME, true);
    // We hold a city captured from the enemy.
    v.cities[1].occupied_from = Some(ENEMY);
    let offer = crate::fixture::planner(&v, 7).peace_offer(enemy(&v));
    assert_eq!((offer.gold, offer.return_captured), (100, true));
    // Attacked, it offers a plain Peace: nothing is extorted from its own offers either.
    let attacked = war_view(ENEMY, true);
    assert_eq!(
        crate::fixture::planner(&attacked, 7).peace_offer(enemy(&attacked)),
        PeaceTerms::default()
    );
}
