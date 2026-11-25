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

#[cfg(test)]
mod tests {
    use crate::fixture::{ENEMY, ME, ViewBuilder, off, planner};
    use legatum_core::view::FortOffer;
    use legatum_core::{Order, PlayerView, TechId, UnitId, UnitKind};

    /// Our Spearman (unit 1) has held a tile of our land for a turn and could dig a Fort; an
    /// enemy Spearman stands `gap` tiles east of it.
    fn staged(gap: i32, gold: i32) -> PlayerView {
        let mut view = ViewBuilder::new(30, 16)
            .known_techs(&[TechId::Feudalism])
            .gold(gold)
            .city(1, ME, off(3, 8), 3)
            .owner(off(8, 8), ME)
            .unit(1, ME, UnitKind::Spearman, off(8, 8))
            .unit(20, ENEMY, UnitKind::Spearman, off(8 + gap, 8))
            .build();
        let u = view.units.iter_mut().find(|u| u.id == UnitId(1)).unwrap();
        u.fortify = 3;
        u.fort_offer = Some(FortOffer {
            gold: legatum_core::data::FORT_GOLD,
            blocker: None,
        });
        view
    }

    fn digs(view: &PlayerView) -> bool {
        let mut p = planner(view, 7);
        p.plan_forts();
        p.orders
            .iter()
            .any(|o| matches!(o, Order::BuildFort { unit } if *unit == UnitId(1)))
    }

    #[test]
    fn a_holding_unit_near_an_enemy_army_digs_a_fort() {
        assert!(digs(&staged(4, 2000)));
    }

    #[test]
    fn no_fort_without_a_threat_the_gold_a_rested_unit_or_room() {
        // No enemy army within reach of the rule.
        assert!(!digs(&staged(9, 2000)));
        // The treasury cannot spare the Gold.
        assert!(!digs(&staged(4, legatum_core::data::FORT_GOLD - 1)));
        // A unit that moved last turn is on its way somewhere.
        let mut moved = staged(4, 2000);
        moved
            .units
            .iter_mut()
            .find(|u| u.id == UnitId(1))
            .unwrap()
            .fortify = 0;
        assert!(!digs(&moved));
        // The rule says no (a Fort already here, foreign land, a city).
        let mut blocked = staged(4, 2000);
        blocked
            .units
            .iter_mut()
            .find(|u| u.id == UnitId(1))
            .unwrap()
            .fort_offer = Some(FortOffer {
            gold: legatum_core::data::FORT_GOLD,
            blocker: Some("A Fort already stands here".into()),
        });
        assert!(!digs(&blocked));
        // A Fort two tiles away already holds this ground.
        let mut crowded = staged(4, 2000);
        let i = crowded.index(off(6, 8)).unwrap();
        crowded.tiles[i].fort = true;
        assert!(!digs(&crowded));
    }
}
