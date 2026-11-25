//! Why an AI empire declares war: the reason it states with the declaration
//! (`Order::StateWarReason`), read from the same signals its decision rested on.

use legatum_core::diplomacy::WarReason;
use legatum_core::{Order, PlayerView, TeamId};

use crate::plan::PlanKind;
use crate::scripted::Planner;

/// Share of its VP target at which a rival counts as close to winning.
const NEAR_VICTORY_PERCENT: u32 = 70;

fn progress(v: &PlayerView, team: TeamId) -> u32 {
    v.standings
        .iter()
        .find(|s| s.team == team)
        .map_or(0, |s| 100 * s.vp / s.target.max(1))
}

impl Planner<'_> {
    /// The strongest true reason for a war on `against`, most urgent first: a victory
    /// countdown, the leader to contain, a conqueror at the door, the faction's own goal, a
    /// city the army is ready to take, then plain conquest.
    pub(crate) fn war_reason(&self, against: TeamId) -> WarReason {
        let v = self.view();
        if crate::hold::rival_hold_of(v, against).is_some() {
            WarReason::VictoryHold
        } else if self.diplomatic_target() == Some(against)
            || progress(v, against) >= NEAR_VICTORY_PERCENT
        {
            WarReason::NearVictory
        } else if self.top_threat_team() == Some(against) {
            WarReason::Threat
        } else if self.sparta_expedition_target() == Some(against) {
            WarReason::FactionGoal
        } else if self.strike_against(against).is_some()
            || self.routed_plan().is_some_and(|plan| {
                matches!(
                    plan.kind,
                    PlanKind::Campaign { against: t, .. }
                    | PlanKind::Snipe { against: t, .. }
                    | PlanKind::FieldStrike { against: t, .. } if t == against
                )
            })
        {
            WarReason::Opportunity
        } else {
            WarReason::Conquest
        }
    }

    /// Follow every declaration in `orders` with its stated reason, whichever part of the
    /// planner made it.
    pub(crate) fn state_war_reasons(&self, orders: &mut Vec<Order>) {
        let stated: Vec<Order> = orders
            .iter()
            .filter_map(|order| match *order {
                Order::DeclareWar { against, .. } => Some(Order::StateWarReason {
                    against,
                    reason: self.war_reason(against),
                }),
                _ => None,
            })
            .collect();
        orders.extend(stated);
    }
}
