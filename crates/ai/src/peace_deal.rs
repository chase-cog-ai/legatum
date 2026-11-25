//! Pricing the terms a Peace offer carries (`legatum_core::diplomacy::PeaceTerms`).
//!
//! Everything is priced in Gold from our side and added to what the Peace itself is worth
//! (`treaty_score`, ten Gold a point). The plan's refusals stay refusals: no terms buy off
//! a war the AI will not end (`wants_peace`). The rules that keep a player from milking it:
//!
//! - It gives nothing (Gold, reparations, a city it founded) to the side that declared
//!   this war on it, so nobody farms it by declaring.
//! - It gives only while that enemy really endangers a city, never just for liking Peace.
//! - A city about to fall to someone else is worth nothing; reparations count for a
//!   quarter of their face value, nothing from a known betrayer.
//! - A city it gives up costs double what the same city is worth to receive.
use crate::diplomacy::{TREATY_GOLD_PER_POINT, strength};
use crate::scripted::Planner;
use legatum_core::diplomacy::{
    PEACE_GOLD_MAX, PEACE_REPARATIONS_MAX, PEACE_TRUCE_TURNS, PeaceTerms, RelationState,
};
use legatum_core::view::CityView;
use legatum_core::{PlayerId, TeamId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PeaceAnswer {
    Accept,
    Refuse,
    /// Refuse, and send these terms back (written from our side).
    Counter(PeaceTerms),
}

/// The answer to a Peace offer with the lines that priced it, in Gold (positive: good for us).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeaceVerdict {
    pub answer: PeaceAnswer,
    pub lines: Vec<(String, i32)>,
    /// A line that no price outweighs, when one decided it.
    pub never: Option<String>,
}

impl PeaceVerdict {
    pub fn total(&self) -> i32 {
        self.lines.iter().map(|(_, v)| *v).sum()
    }
    /// Plain sentences for the player (`DecisionExplanation::reasons`).
    pub fn reasons(&self) -> Vec<String> {
        let mut out: Vec<String> = self.never.iter().cloned().collect();
        out.extend(
            self.lines
                .iter()
                .filter(|(_, v)| *v != 0)
                .map(|(label, v)| format!("{label}: {v:+} Gold")),
        );
        out
    }
}

/// What a city is worth in Gold, to give or to receive.
fn city_value(c: &CityView) -> i32 {
    150 + 60 * c.population + 100 * i32::from(c.walls) + 300 * c.great_works.len() as i32
}

impl Planner<'_> {
    /// Army advantage over `other` in percent, -50..=50 (the Peace score's own measure).
    fn peace_advantage(&self, other: TeamId) -> i32 {
        let v = self.view();
        (strength(v, v.team) * 100 / strength(v, other).max(1) - 100).clamp(-50, 50)
    }

    /// `other` declared the current war on us and we did not declare back.
    fn declared_on_us(&self, other: TeamId) -> bool {
        let v = self.view();
        v.diplomacy.relations.iter().any(|r| {
            r.between(v.team, other)
                && r.state == RelationState::War
                && r.declarers.contains(&other)
                && !r.declarers.contains(&v.team)
        })
    }

    /// A city someone else is about to take: half its HP gone, or an army at war with its
    /// owner (not ours) standing within two hexes.
    fn about_to_fall(&self, c: &CityView) -> bool {
        let v = self.view();
        c.hp * 2 <= c.max_hp
            || v.units.iter().any(|u| {
                u.military
                    && u.owner != v.me
                    && v.team_of(u.owner) != v.team
                    && v.team_of(u.owner) != v.team_of(c.owner)
                    && !u.owner.is_barbarian()
                    && v.at_war(u.owner, c.owner)
                    && u.pos.distance(c.pos) <= 2
            })
    }

    fn receive_value(&self, c: &CityView) -> (i32, &'static str) {
        let v = self.view();
        if c.is_capital {
            return (0, "a capital cannot be handed over");
        }
        if v.faction_of(v.me) == legatum_core::Faction::Celts && c.founder != Some(v.me) {
            return (0, "the Celts keep no city they did not found");
        }
        if self.about_to_fall(c) {
            return (0, "about to fall");
        }
        let near = self
            .ctx
            .own_cities
            .iter()
            .map(|o| o.pos.distance(c.pos))
            .min()
            .unwrap_or(i32::MAX);
        if near > 12 {
            (city_value(c) / 2, "too far to hold well")
        } else {
            (city_value(c), "")
        }
    }

    /// Our answer to a Peace `proposer` (of team `other`) offers on `terms`, written from the
    /// proposer's side. `base` is the plain Peace's score, `None` when the plan refuses any
    /// Peace with them now. `counter`: the offer already answers one of ours, so it can only
    /// be accepted or refused.
    pub fn peace_verdict(
        &self,
        proposer: PlayerId,
        other: TeamId,
        terms: &PeaceTerms,
        base: Option<i32>,
        counter: bool,
    ) -> PeaceVerdict {
        let mut verdict = self.price_peace(proposer, other, terms, base);
        if verdict.answer != PeaceAnswer::Refuse || counter || base.is_none() {
            return verdict;
        }
        // Keep what they give, drop what we would give, and ask Gold for what is still short.
        let v = self.view();
        let ours = |c: &legatum_core::CityId| {
            v.cities
                .iter()
                .any(|city| city.id == *c && city.owner == v.me)
        };
        let mut kept = PeaceTerms {
            gold: terms.gold.max(0),
            city: terms.city.filter(|c| !ours(c)),
            return_captured: terms.return_captured
                && self.returned_value(other).is_some_and(|(net, _)| net >= 0),
            truce: if self.truce_value(other, terms.truce_turns()) >= 0 {
                terms.truce
            } else {
                0
            },
            // A betrayer's promise is not worth keeping in the deal.
            reparations: if v.diplomacy.betrayals.iter().any(|b| b.by == other) {
                0
            } else {
                terms.reparations.max(0)
            },
        };
        let short = -self.price_peace(proposer, other, &kept, base).total();
        if short > 0 {
            // Round the ask up to a round figure.
            kept.gold += (short + 9) / 10 * 10;
        }
        if kept.gold > PEACE_GOLD_MAX || kept == *terms {
            return verdict;
        }
        let again = self.price_peace(proposer, other, &kept, base);
        if again.answer == PeaceAnswer::Accept {
            verdict.answer = PeaceAnswer::Counter(kept.mirrored());
        }
        verdict
    }

    /// Net Gold value to us of both sides returning captured cities, with how many we give.
    fn returned_value(&self, other: TeamId) -> Option<(i32, usize)> {
        let v = self.view();
        let mut net = 0;
        let mut given = 0;
        let mut any = false;
        for c in v.cities.iter().filter(|c| !c.is_capital) {
            let Some(back) = c.occupied_from else {
                continue;
            };
            let (holder, home) = (v.team_of(c.owner), v.team_of(back));
            if holder == v.team && home == other {
                net -= city_value(c) * 3 / 2;
                given += 1;
                any = true;
            } else if holder == other && home == v.team {
                net += self.receive_value(c).0;
                any = true;
            }
        }
        any.then_some((net, given))
    }

    /// A longer truce costs the stronger army the turns it could have pressed, and is worth
    /// half as much to the weaker one.
    fn truce_value(&self, other: TeamId, turns: u32) -> i32 {
        let extra = turns.saturating_sub(PEACE_TRUCE_TURNS) as i32;
        let advantage = self.peace_advantage(other);
        let points = advantage * extra / 40;
        -if points > 0 { points } else { points / 2 } * TREATY_GOLD_PER_POINT
    }

    fn price_peace(
        &self,
        proposer: PlayerId,
        other: TeamId,
        terms: &PeaceTerms,
        base: Option<i32>,
    ) -> PeaceVerdict {
        let v = self.view();
        let mut lines = Vec::new();
        let mut never = None;
        let Some(score) = base else {
            return PeaceVerdict {
                answer: PeaceAnswer::Refuse,
                lines,
                never: Some("We will not make Peace now, on any terms".into()),
            };
        };
        let danger = self.endangered_by(other) || self.collapsing();
        let attacked = self.declared_on_us(other);
        // Liking the Peace pays for concessions only while they endanger us.
        let peace = if score >= 0 && !danger {
            0
        } else {
            score.min(30) * TREATY_GOLD_PER_POINT
        };
        lines.push(("The Peace itself".to_string(), peace));
        let give = |what: String, never: &mut Option<String>| {
            if attacked {
                never.get_or_insert(format!("We give {what} to no one who declared war on us"));
            } else if !danger {
                never.get_or_insert(format!("We are in no danger that is worth {what}"));
            }
        };
        if terms.gold != 0 {
            lines.push(("Gold".into(), terms.gold));
            if terms.gold < 0 {
                give("Gold".into(), &mut never);
                if v.economy.gold / 2 < -terms.gold {
                    never.get_or_insert("That is more than half our treasury".into());
                }
            }
        }
        if terms.reparations != 0 {
            let turns = terms.truce_turns() as i32;
            if terms.reparations > 0 {
                let betrayer = v.diplomacy.betrayals.iter().any(|b| b.by == other);
                let worth = if betrayer {
                    0
                } else {
                    terms.reparations.min(PEACE_REPARATIONS_MAX) * turns / 4
                };
                lines.push((
                    if betrayer {
                        "Reparations from a betrayer".into()
                    } else {
                        "Reparations (they may stop paying)".into()
                    },
                    worth,
                ));
            } else {
                lines.push(("Reparations we pay".into(), terms.reparations * turns));
                give("reparations".into(), &mut never);
                if v.economy.gold_per_turn.total < -terms.reparations {
                    never.get_or_insert("Our income cannot carry those reparations".into());
                }
            }
        }
        if let Some(id) = terms.city {
            match v.cities.iter().find(|c| c.id == id) {
                Some(c) if c.owner == v.me => {
                    lines.push((format!("We give up {}", c.name), -2 * city_value(c)));
                    give(c.name.clone(), &mut never);
                }
                Some(c) if c.owner == proposer => {
                    let (worth, why) = self.receive_value(c);
                    let label = if why.is_empty() {
                        format!("We receive {}", c.name)
                    } else {
                        format!("We receive {} ({why})", c.name)
                    };
                    lines.push((label, worth));
                }
                _ => {
                    never.get_or_insert("That city is not theirs or ours to hand over".into());
                }
            }
        }
        if terms.return_captured
            && let Some((net, given)) = self.returned_value(other)
        {
            lines.push(("Captured cities go home".into(), net));
            if given > 0 && net < 0 && !danger {
                // Handing back conquests is a concession like any other, but the side that
                // declared may still be made to give back what it took.
                never.get_or_insert("We are in no danger that is worth our conquests".into());
            }
        }
        if terms.truce_turns() != PEACE_TRUCE_TURNS {
            lines.push((
                format!("A truce of {} turns", terms.truce_turns()),
                self.truce_value(other, terms.truce_turns()),
            ));
        }
        let total: i32 = lines.iter().map(|(_, v)| *v).sum();
        PeaceVerdict {
            answer: if never.is_none() && total >= 0 {
                PeaceAnswer::Accept
            } else {
                PeaceAnswer::Refuse
            },
            lines,
            never,
        }
    }

    /// Gene `peace_terms`: the Gold we add to our own Peace offer. Only for a war we declared
    /// that now endangers a city of ours; a quarter of the treasury, 200 at most.
    pub(crate) fn peace_sweetener(&self, to: TeamId) -> i32 {
        let v = self.view();
        if self.decisions.peace_terms == 0
            || self.declared_on_us(to)
            || !(self.endangered_by(to) || self.collapsing())
        {
            return 0;
        }
        let gold = (v.economy.gold / 4).min(20 * TREATY_GOLD_PER_POINT) / 10 * 10;
        if gold >= 20 { gold } else { 0 }
    }
}
