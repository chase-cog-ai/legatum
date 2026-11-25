//! The post-game "what the AI was planning" record: each AI seat's own plan, campaign and
//! threat logs (`legatum_ai::OpponentMemory`, written as the seat played and kept in saves),
//! with team and city ids resolved against the final state. Nothing is inferred from orders.
//! Readable only once the game is finished: mid-game it would leak rival intent.

use crate::session::Session;
use legatum_ai::CampaignEnd;
use legatum_ai::plan::{PlanEvent, PlanKind, PlanReason};
use legatum_ai::threat::ThreatEvent;
use legatum_core::diplomacy::TreatyKind;
use legatum_core::{CityId, GameState, TeamId};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CityRef {
    pub id: u32,
    /// None only for a city razed before its name was kept (an older save).
    pub name: Option<String>,
    /// Who holds it at the end of the game; None when it was razed.
    pub owner: Option<u8>,
}

/// The seat changed what it was working towards (Hard seats only; the plan layer).
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PlanStep {
    pub turn: u32,
    /// economy, peace, campaign, contain, snipe, field_strike or hold.
    pub plan: &'static str,
    /// The empires the plan is aimed at (the members of its team).
    pub against: Vec<u8>,
    pub city: Option<CityRef>,
    pub reason: PlanReason,
    /// The seat's own estimate of its chance to win at the switch (0..=10,000).
    pub chance: i32,
}

/// A march on a city started (`end` None) or ended.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CampaignStep {
    pub turn: u32,
    pub city: CityRef,
    pub end: Option<CampaignEnd>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ThreatStep {
    /// `rival` became the empire this seat feared most.
    TopThreat { turn: u32, rival: u8 },
    /// Walls ordered in a city exposed to `rival`.
    Walls { turn: u32, city: CityRef, rival: u8 },
    /// A treaty sought or accepted with others because of `rival`.
    Treaty {
        turn: u32,
        with: Vec<u8>,
        treaty: TreatyKind,
        accepted: bool,
        rival: u8,
    },
    /// War declared on the feared rival.
    SecondFront {
        turn: u32,
        against: Vec<u8>,
        rival: u8,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct SeatTimeline {
    pub player: u8,
    pub plans: Vec<PlanStep>,
    pub campaigns: Vec<CampaignStep>,
    pub threats: Vec<ThreatStep>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct AiTimeline {
    pub seats: Vec<SeatTimeline>,
}

fn city_ref(session: &Session, id: CityId) -> CityRef {
    let city = session.state.cities.get(&id);
    CityRef {
        id: id.0,
        name: city
            .map(|c| c.name.clone())
            .or_else(|| session.razed_city_names.get(&id).cloned()),
        owner: city.map(|c| c.owner.0),
    }
}

fn members(state: &GameState, team: TeamId) -> Vec<u8> {
    state
        .players
        .iter()
        .filter(|p| p.team == team)
        .map(|p| p.id.0)
        .collect()
}

fn plan_step(session: &Session, event: &PlanEvent) -> Option<PlanStep> {
    let PlanEvent::Switch {
        turn,
        to,
        chance,
        reason,
        ..
    } = event
    else {
        return None;
    };
    let (plan, team, city) = match to {
        PlanKind::Economy => ("economy", None, None),
        PlanKind::Peace { with } => ("peace", Some(*with), None),
        PlanKind::Campaign { against, target } => ("campaign", Some(*against), Some(*target)),
        PlanKind::Contain { against } => ("contain", Some(*against), None),
        PlanKind::Snipe { against, .. } => ("snipe", Some(*against), None),
        PlanKind::FieldStrike { against, .. } => ("field_strike", Some(*against), None),
        PlanKind::Hold { city } => ("hold", None, Some(*city)),
    };
    Some(PlanStep {
        turn: *turn,
        plan,
        against: team.map(|t| members(&session.state, t)).unwrap_or_default(),
        city: city.map(|c| city_ref(session, c)),
        reason: *reason,
        chance: *chance,
    })
}

fn threat_step(session: &Session, event: &ThreatEvent) -> ThreatStep {
    match event {
        ThreatEvent::TopThreat { turn, rival, .. } => ThreatStep::TopThreat {
            turn: *turn,
            rival: rival.0,
        },
        ThreatEvent::Walls { turn, city, rival } => ThreatStep::Walls {
            turn: *turn,
            city: city_ref(session, *city),
            rival: rival.0,
        },
        ThreatEvent::Treaty {
            turn,
            with,
            kind,
            rival,
            accepted,
        } => ThreatStep::Treaty {
            turn: *turn,
            with: members(&session.state, *with),
            treaty: *kind,
            accepted: *accepted,
            rival: rival.0,
        },
        ThreatEvent::SecondFront {
            turn,
            against,
            rival,
            ..
        } => ThreatStep::SecondFront {
            turn: *turn,
            against: members(&session.state, *against),
            rival: rival.0,
        },
    }
}

/// Every AI seat's planning record, in seat order. Refused until the game is finished.
pub fn ai_timeline(session: &Session) -> Result<AiTimeline, String> {
    let state = &session.state;
    if !state.finished {
        return Err("Rival plans are shown once the game is over".into());
    }
    let seats = state
        .players
        .iter()
        .filter(|p| !p.is_human)
        .filter_map(|p| {
            let memory = session.brain(p.id)?.opponent_memory()?;
            Some(SeatTimeline {
                player: p.id.0,
                plans: memory
                    .campaigns
                    .plan
                    .events
                    .iter()
                    .filter_map(|e| plan_step(session, e))
                    .collect(),
                campaigns: memory
                    .campaigns
                    .events
                    .iter()
                    .map(|e| CampaignStep {
                        turn: e.turn,
                        city: city_ref(session, e.target),
                        end: e.end,
                    })
                    .collect(),
                threats: memory
                    .threat_log
                    .iter()
                    .map(|e| threat_step(session, e))
                    .collect(),
            })
        })
        .collect();
    Ok(AiTimeline { seats })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Game;
    use legatum_ai::Brain;
    use legatum_core::{Faction, GameSetup};
    use serde_json::Value;

    fn played(turns: u32) -> Game {
        let mut setup = GameSetup::standard(7, [Faction::Athens, Faction::Byzantium]);
        for p in &mut setup.players {
            p.difficulty = legatum_core::AiDifficulty::Hard;
        }
        let mut game =
            Game::from_setup_json(&crate::json::setup_to_value(&setup).to_string()).unwrap();
        for _ in 0..turns {
            game.end_turn_json("{}").unwrap();
        }
        game
    }

    #[test]
    fn rival_plans_stay_hidden_until_the_game_is_over() {
        let mut game = played(3);
        assert!(game.ai_timeline_json().is_err());
        game.session.state.finished = true;
        assert!(game.ai_timeline_json().is_ok());
    }

    #[test]
    fn the_timeline_is_the_seats_own_logs_and_survives_a_save() {
        let mut game = played(120);
        let reloaded = Game::from_save_json(&game.session.save_json()).unwrap();
        game.session.state.finished = true;
        let timeline = ai_timeline(&game.session).unwrap();

        let state = &game.session.state;
        let ais: Vec<u8> = state
            .players
            .iter()
            .filter(|p| !p.is_human)
            .map(|p| p.id.0)
            .collect();
        assert_eq!(
            timeline.seats.iter().map(|s| s.player).collect::<Vec<_>>(),
            ais
        );
        for seat in &timeline.seats {
            let brain = game
                .session
                .brain(legatum_core::PlayerId(seat.player))
                .unwrap();
            let switches = brain
                .plan_events()
                .iter()
                .filter(|e| matches!(e, PlanEvent::Switch { .. }))
                .count();
            assert_eq!(seat.plans.len(), switches);
            assert_eq!(seat.campaigns.len(), brain.campaign_events().len());
            assert_eq!(seat.threats.len(), brain.threat_log().len());
            assert!(seat.plans.windows(2).all(|w| w[0].turn <= w[1].turn));
            assert!(seat.plans.iter().all(|p| !p.against.contains(&seat.player)));
        }
        assert!(
            timeline
                .seats
                .iter()
                .any(|s| !s.plans.is_empty() || !s.threats.is_empty()),
            "120 turns of play leave something to show"
        );

        // A razed target keeps its name, through a save too.
        let targets: Vec<&CityRef> = timeline
            .seats
            .iter()
            .flat_map(|s| s.campaigns.iter().map(|c| &c.city))
            .collect();
        assert!(targets.iter().any(|c| c.owner.is_none()));
        assert!(targets.iter().all(|c| c.name.is_some()));

        let mut reloaded = reloaded;
        reloaded.session.state.finished = true;
        assert_eq!(ai_timeline(&reloaded.session).unwrap(), timeline);

        let json: Value = serde_json::from_str(&game.ai_timeline_json().unwrap()).unwrap();
        assert_eq!(json["seats"].as_array().unwrap().len(), ais.len());
    }

    /// Regenerates `app/src/test/fixtures/postgame-rival-plans.current.json`: a finished solo
    /// game against seven Hard seats that keeps the AI memory the report's Rival plans tab
    /// reads (the recorded end states beside it had theirs stripped). The human seat is played
    /// by the Hard script from its own view, so the rivals have a real opponent to plan against.
    /// `LEGATUM_WRITE_FIXTURE=1 cargo test -p legatum-wasm --release --lib rival_plans_fixture -- --ignored`
    #[test]
    #[ignore = "writes a test fixture; run by hand after a rules change that breaks the old one"]
    fn write_rival_plans_fixture() {
        let mut setup = GameSetup::all_ai(42);
        setup.players[0].is_human = true;
        setup.players[0].name = "Player 1".into();
        for p in &mut setup.players[1..] {
            p.difficulty = legatum_core::AiDifficulty::Hard;
        }
        let mut game =
            Game::from_setup_json(&crate::json::setup_to_value(&setup).to_string()).unwrap();
        let me = legatum_core::PlayerId(0);
        let mut stand_in = legatum_ai::ScriptedBrain::with_difficulty(
            game.session.state.player(me).faction,
            42,
            me,
            legatum_core::AiDifficulty::Hard,
        );
        while !game.session.state.finished {
            let orders: std::collections::BTreeMap<
                legatum_core::PlayerId,
                Vec<legatum_core::Order>,
            > = std::collections::BTreeMap::from([(
                me,
                stand_in.plan(&legatum_core::player_view(&game.session.state, me)),
            )]);
            game.end_turn_json(&serde_json::to_string(&orders).unwrap())
                .unwrap();
        }
        let timeline = ai_timeline(&game.session).unwrap();
        assert!(timeline.seats.iter().any(|s| !s.campaigns.is_empty()));

        let mut save: serde_json::Value = serde_json::from_str(&game.session.save_json()).unwrap();
        save.as_object_mut().unwrap().remove("archive");
        eprintln!(
            "finished on turn {}, winner {:?}",
            game.session.state.turn, game.session.state.winner
        );
        if std::env::var_os("LEGATUM_WRITE_FIXTURE").is_some() {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../app/src/test/fixtures/postgame-rival-plans.current.json");
            std::fs::write(path, format!("{save}\n")).unwrap();
        }
    }
}
