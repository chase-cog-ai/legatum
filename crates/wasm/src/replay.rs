use crate::{
    guard,
    history::{Recording, replay_turn},
    js,
    session::Session,
    to_json,
};
use legatum_core::{
    Event, GameState, PlayerId, PlayerView, TeamId, ViewDetail,
    diplomacy::RelationState,
    event_visibility::{VisibleEvent, event_visibility_view, visible_events},
    player_view_detail, public_view, scoring,
};
use serde::Serialize;
use wasm_bindgen::prelude::*;

const MAX_CHECKPOINTS: usize = 16;

pub(crate) fn available(session: &Session, player: PlayerId) -> bool {
    session.state.finished
        && session.state.players.iter().filter(|p| p.is_human).count() == 1
        && session
            .state
            .players
            .iter()
            .any(|p| p.id == player && p.is_human)
        && session
            .history
            .as_ref()
            .is_some_and(|h| !h.tape.turns.is_empty())
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum Moment {
    Founded {
        city: String,
        player: Option<PlayerId>,
    },
    Captured {
        city: String,
        player: Option<PlayerId>,
    },
    War {
        a: Option<TeamId>,
        b: Option<TeamId>,
    },
    DeclaredWar {
        from: Option<TeamId>,
        against: Option<TeamId>,
    },
    Eliminated {
        player: Option<PlayerId>,
    },
    Scored {
        gained: u32,
        total: u32,
    },
    Hold {
        team: Option<TeamId>,
    },
    Won {
        team: Option<TeamId>,
    },
    Draw,
}

#[derive(Serialize)]
struct Milestone {
    turn: u32,
    moment: Moment,
}

fn met_team(view: &PlayerView, team: TeamId) -> Option<TeamId> {
    (team == view.team
        || view.diplomacy.met.contains(&team)
        || view
            .diplomacy
            .relations
            .iter()
            .any(|r| r.between(view.team, team) && r.state == RelationState::War))
    .then_some(team)
}

fn met_player(view: &PlayerView, player: PlayerId) -> Option<PlayerId> {
    if player.is_barbarian() {
        return Some(player);
    }
    met_team(view, view.players.iter().find(|p| p.id == player)?.team).map(|_| player)
}

#[derive(Serialize)]
struct ReplayInfo<'a> {
    first: u32,
    last: u32,
    partial: bool,
    milestones: &'a [Milestone],
}

#[wasm_bindgen]
pub struct GameReplay {
    record: Recording,
    viewer: PlayerId,
    checkpoints: Vec<(usize, GameState)>,
    current: Option<(usize, GameState)>,
    milestones: Vec<Milestone>,
}

#[wasm_bindgen]
impl GameReplay {
    #[wasm_bindgen(constructor)]
    pub fn new(recording: &str, player: u8) -> Result<GameReplay, JsError> {
        js(guard(|| {
            let record = serde_json::from_str(recording).map_err(|e| e.to_string())?;
            Self::open(record, PlayerId(player))
        }))
    }

    pub fn info(&self) -> Result<String, JsError> {
        js(to_json(&ReplayInfo {
            first: self.first(),
            last: self.first() + self.record.metadata.turn_count,
            partial: !self.record.metadata.complete_history,
            milestones: &self.milestones,
        }))
    }

    pub fn frame(&mut self, turn: u32) -> Result<String, JsError> {
        js(guard(|| to_json(&self.seek(turn)?)))
    }
}

impl GameReplay {
    fn open(record: Recording, viewer: PlayerId) -> Result<Self, String> {
        let mut state = record.replay_start()?;
        if record.tape.turns.is_empty()
            || !matches!(
                record.metadata.outcome.as_str(),
                "vp" | "draw" | "elimination" | "turn-limit"
            )
            || state.players.iter().filter(|p| p.is_human).count() != 1
            || !state.players.iter().any(|p| p.id == viewer && p.is_human)
        {
            return Err("No finished solo recording".into());
        }
        let spacing = record.tape.turns.len().div_ceil(MAX_CHECKPOINTS - 1).max(1);
        let mut checkpoints = vec![(0, state.clone())];
        let mut milestones = Vec::new();
        let mut vp = scoring::vp_total(&state, viewer);
        for (index, tape_turn) in record.tape.turns.iter().enumerate() {
            let (next, events) = replay_turn(&state, tape_turn)?;
            let total = scoring::vp_total(&next, viewer);
            if total > vp {
                milestones.push(Milestone {
                    turn: tape_turn.turn,
                    moment: Moment::Scored {
                        gained: total - vp,
                        total,
                    },
                });
            }
            vp = total;
            let notable: Vec<_> = events
                .into_iter()
                .filter(|event| {
                    matches!(
                        event,
                        Event::CityFounded { .. }
                            | Event::CityCaptured { .. }
                            | Event::WarDeclared { .. }
                            | Event::PlayerEliminated { .. }
                            | Event::VictoryHoldStarted { .. }
                            | Event::GameWon { .. }
                            | Event::GameDrawn { .. }
                    )
                })
                .collect();
            if !notable.is_empty() {
                let before = event_visibility_view(&state, viewer);
                let after = event_visibility_view(&next, viewer);
                milestones.extend(
                    visible_events(&notable, &before, &after)
                        .into_iter()
                        .filter_map(|event| {
                            let moment = match event {
                                VisibleEvent::Complete(Event::CityFounded {
                                    name, owner, ..
                                }) => Moment::Founded {
                                    city: name,
                                    player: met_player(&after, owner),
                                },
                                VisibleEvent::Complete(Event::CityCaptured {
                                    city, to, ..
                                }) => Moment::Captured {
                                    city: after
                                        .cities
                                        .iter()
                                        .chain(&before.cities)
                                        .find(|c| c.id == city)?
                                        .name
                                        .clone(),
                                    player: met_player(&after, to),
                                },
                                VisibleEvent::Complete(Event::WarDeclared { relation }) => {
                                    match relation.declarers.as_slice() {
                                        [team] => Moment::DeclaredWar {
                                            from: met_team(&after, *team),
                                            against: met_team(
                                                &after,
                                                if *team == relation.a {
                                                    relation.b
                                                } else {
                                                    relation.a
                                                },
                                            ),
                                        },
                                        _ => Moment::War {
                                            a: met_team(&after, relation.a),
                                            b: met_team(&after, relation.b),
                                        },
                                    }
                                }
                                VisibleEvent::Complete(Event::PlayerEliminated { player }) => {
                                    Moment::Eliminated {
                                        player: met_player(&after, player),
                                    }
                                }
                                VisibleEvent::Complete(Event::VictoryHoldStarted {
                                    team, ..
                                }) => Moment::Hold {
                                    team: met_team(&after, team),
                                },
                                VisibleEvent::Complete(Event::GameWon { team, .. }) => {
                                    Moment::Won {
                                        team: met_team(&after, team),
                                    }
                                }
                                VisibleEvent::Complete(Event::GameDrawn { .. }) => Moment::Draw,
                                _ => return None,
                            };
                            Some(Milestone {
                                turn: tape_turn.turn,
                                moment,
                            })
                        }),
                );
            }
            state = next;
            if (index + 1).is_multiple_of(spacing) {
                checkpoints.push((index + 1, state.clone()));
            }
        }
        if !state.finished {
            return Err("Recording ends before the game finishes".into());
        }
        Ok(Self {
            record,
            viewer,
            checkpoints,
            current: None,
            milestones,
        })
    }

    fn first(&self) -> u32 {
        self.record.metadata.first_turn - 1
    }

    fn seek(&mut self, turn: u32) -> Result<PlayerView, String> {
        let target = turn
            .checked_sub(self.first())
            .ok_or("Turn is outside the replay")? as usize;
        if target > self.record.tape.turns.len() {
            return Err("Turn is outside the replay".into());
        }
        let checkpoint =
            &self.checkpoints[self.checkpoints.partition_point(|(i, _)| *i <= target) - 1];
        let (index, mut state) = match self.current.take() {
            Some((index, state)) if index >= checkpoint.0 && index <= target => (index, state),
            _ => checkpoint.clone(),
        };
        for t in &self.record.tape.turns[index..target] {
            state = replay_turn(&state, t)?.0;
        }
        let view = player_view_detail(&state, self.viewer, &public_view(&state), ViewDetail::Map);
        self.current = Some((target, state));
        Ok(view)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{BrainKind, OrderMap};
    use legatum_core::{Faction, GameSetup, Order};

    fn play(partial: bool) -> (Session, Vec<GameState>) {
        let mut setup = GameSetup::standard(42, [Faction::India, Faction::Sparta]);
        setup.players.truncate(2);
        setup.players[1].is_human = false;
        setup.players[1].team = legatum_core::TeamId(1);
        setup.turn_limit = Some(20);
        let mut session = Session::new(&setup, |_| BrainKind::Scripted);
        if partial {
            for _ in 0..3 {
                session.step(&OrderMap::new());
            }
            session.history = Some(Recording::new(&session.state));
        }
        let mut states = vec![session.state.clone()];
        while !session.state.finished {
            let orders: OrderMap = if session.state.turn == 4 {
                OrderMap::from([(
                    PlayerId(0),
                    vec![Order::DeclareWar {
                        against: legatum_core::TeamId(1),
                        breach: false,
                    }],
                )])
            } else {
                OrderMap::new()
            };
            session.step(&orders);
            states.push(session.state.clone());
        }
        (session, states)
    }

    #[test]
    fn seeks_both_ways_with_original_fog_and_a_partial_anchor() {
        for partial in [false, true] {
            let (session, states) = play(partial);
            assert!(available(&session, PlayerId(0)));
            let mut replay = GameReplay::open(session.history.unwrap(), PlayerId(0)).unwrap();
            assert_eq!(replay.record.metadata.complete_history, !partial);
            assert!(replay.checkpoints.len() <= MAX_CHECKPOINTS);
            let first = replay.first();
            assert_eq!(first, if partial { 3 } else { 0 });
            let opening = replay.seek(first).unwrap();
            assert_eq!(met_player(&opening, PlayerId(1)), None);
            assert_eq!(met_team(&opening, TeamId(1)), None);
            for i in [0, states.len() - 1, 6, 7, 0] {
                let view = replay.seek(first + i as u32).unwrap();
                let state = &states[i];
                assert_eq!(replay.current.as_ref().unwrap().1.hash(), state.hash());
                let live = legatum_core::player_view(state, PlayerId(0));
                assert_eq!(to_json(&view.tiles).unwrap(), to_json(&live.tiles).unwrap());
                assert_eq!(to_json(&view.units).unwrap(), to_json(&live.units).unwrap());
                assert!(view.units.len() < state.units.len());
            }
            assert!(replay.seek(first + states.len() as u32).is_err());
            assert!(replay.milestones.iter().any(|m| matches!(
                m.moment,
                Moment::DeclaredWar {
                    from: Some(TeamId(0)),
                    against: Some(TeamId(1))
                }
            )));
            assert!(matches!(
                replay.milestones.last().unwrap().moment,
                Moment::Won { .. } | Moment::Draw
            ));
        }
    }

    #[test]
    fn refuses_live_empty_divergent_and_wrong_seat_recordings() {
        let (mut session, _) = play(false);
        let mut recording = session.history.take().unwrap();
        assert!(GameReplay::open(recording.clone(), PlayerId(1)).is_err());
        recording.tape.turns[2].hash = "123".into();
        assert_eq!(
            GameReplay::open(recording, PlayerId(0)).err().unwrap(),
            "first divergent turn: 3"
        );

        let mut live = Session::new(&session.state.setup, |_| BrainKind::Scripted);
        assert!(!available(&live, PlayerId(0)));
        assert!(GameReplay::open(live.history.clone().unwrap(), PlayerId(0)).is_err());
        live.step(&OrderMap::new());
        assert!(!available(&live, PlayerId(0)));
        assert!(GameReplay::open(live.history.unwrap(), PlayerId(0)).is_err());
    }
}
