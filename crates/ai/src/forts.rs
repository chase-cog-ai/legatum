//! Forts in the AI (`legatum_core::rules::forts`). The AI reads the rule, never a shortcut: a
//! military unit that is holding ground on our own land, outside a city, with an enemy army in
//! sight nearby, digs a Fort where it stands when the treasury can spare the Gold. The Fort
//! then defends it (+`data::FORT_DEFENSE`) and stops enemy units that step next to it.
//! One Fort a turn, never beside another Fort, and only with a unit that already stood still
//! last turn and has no other order: digging never takes a unit off a march or an attack.

use crate::scripted::Planner;
use legatum_core::{Order, UnitId};

/// An enemy military unit within this many tiles makes a holding unit dig.
const FORT_THREAT_RADIUS: i32 = 5;
/// No new Fort within this many tiles of a Fort we already know.
const FORT_SPACING: i32 = 2;

impl Planner<'_> {
    /// The unit that digs a Fort this turn, if any (see the module doc).
    fn fort_digger(&self) -> Option<UnitId> {
        let v = self.view();
        let keep = self.ordinary_gold_reserve();
        let busy = |id: UnitId| {
            self.orders.iter().any(|o| {
                legatum_core::rules::units::order_unit(o) == Some(id)
                    && !matches!(o, Order::SkipUnit { .. } | Order::SleepUnit { .. })
            })
        };
        let fort_near = |at: legatum_core::Hex| {
            at.within_iter(FORT_SPACING)
                .any(|h| v.tile(h).is_some_and(|t| t.fort))
        };
        self.ctx
            .own_units
            .iter()
            .copied()
            .filter(|u| u.fortify > 0 && u.goto.is_none())
            .filter(|u| {
                u.fort_offer
                    .as_ref()
                    .is_some_and(|o| o.blocker.is_none() && self.gold_left - o.gold >= keep)
            })
            .filter(|u| !busy(u.id))
            .filter(|u| !fort_near(u.pos))
            .filter_map(|u| {
                let near = self
                    .ctx
                    .threat_units
                    .iter()
                    .filter(|e| e.military)
                    .map(|e| e.pos.distance(u.pos))
                    .min()?;
                (near <= FORT_THREAT_RADIUS).then_some((near, u.id))
            })
            .min()
            .map(|(_, id)| id)
    }

    /// Dig at most one Fort this turn with a unit that is holding ground near the enemy.
    pub(crate) fn plan_forts(&mut self) {
        let Some(id) = self.fort_digger() else {
            return;
        };
        if !self.affordable_beyond_runway(legatum_core::data::FORT_GOLD) {
            return;
        }
        self.release_unit_order(id);
        if self.unit_order(id, Order::BuildFort { unit: id }) {
            self.gold_left -= legatum_core::data::FORT_GOLD;
        }
    }
}
