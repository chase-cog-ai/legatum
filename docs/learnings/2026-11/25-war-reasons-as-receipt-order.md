# AI war reasons ride a receipt order, not the declaration (2026-11-25)

An AI declaration now states why (`Order::StateWarReason`, then `Event::WarReasonStated` after its `WarDeclared`). The reason is a separate order rather than a field on `DeclareWar` because `DeclareWar` is compared by value all over (`PlayerView::diplomacy.actions`, `same_action`, the app's order matching, AI tests): a new field would make an AI's declaration differ from the legal action it was picked from. A separate non-diplomatic order leaves every comparison alone and the engine drops it when no declaration stands. State and results are identical with or without it (core test `a_war_reason_changes_nothing_but_its_event`), so `RULES_VERSION` did not move.

The AI adds the reason in one place (`Planner::state_war_reasons`, after `finish_logged`), not at the dozen sites that build a `DeclareWar`, so no path can declare without one.
