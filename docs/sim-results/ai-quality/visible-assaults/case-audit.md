# Public-seed defense case audit

Refs #366. Completed inventory; decision-level diagnosis remains blocked by missing trace evidence. No policy change or acceptance claim.

Before inspecting individual decisions, select the first two candidate-arm seeds in ascending order that have a `visible_within_two` episode ending `lost_to_attacker`, and the first two candidate-arm seeds with `losses_without_assault`. Within a seed use the earliest qualifying event, breaking ties by city ID. Retain overlapping seeds if any. Compare the same seed in the baseline arm; do not choose cases by favorable outcomes. Source: the 300-pair frozen comparison in [full-game-results.md](full-game-results.md).

First inspect saved fields and existing runner diagnostics. Do not infer absent Gold, defenders, visibility snapshots or orders from outcome records. A rerun is useful only if existing diagnostics recover the missing decision evidence; otherwise document the evidence gap and precise next step.

Written by ChatGPT (OpenAI; agent: chatgpt).

## Inventory result

**The case inventory is complete; causal diagnosis is not.** The fixed selection is below. No selected case establishes a detection, purchasing, production or combat bug. No new simulations were run: the existing executable would reproduce outcome records without adding the missing late decision evidence.

| Bucket | Seed / measured seat / faction | Candidate record | Paired baseline context |
| --- | --- | --- | --- |
| Short-warning loss | 7102 / 4 / Rome | City 42, attacking team 0; first seen T293, assault starts T293, attributed loss T294 | Baseline finishes T277, before this candidate event. Its 6 total city losses versus candidate 15 are whole-game counts, not matched attacks. |
| Short-warning loss | 7105 / 1 / Rome | City 38, team 3; first seen T125, assault starts T126, attributed loss T132 | Baseline contains the same recorded city/team/first-seen/start/end tuple. Both lose this city; that does not establish identical intervening decisions or state. |
| Unattributed loss | 7110 / 0 / Byzantium | Earliest record T286, city 12, captor team 5 (city 28 also lost at T286) | Baseline has no unattributed-loss record; candidate ends T296, baseline T322. A different outcome is not evidence of a particular policy failure. |
| Unattributed loss | 7149 / 3 / Byzantium | Earliest record T264, city 45, captor team 4 | Baseline has no unattributed-loss record; candidate ends T288, baseline T371. |

Selection uses assault `start` for the earliest short-warning episode and tuple turn for an unattributed loss; city ID breaks ties. [case-selection.json](case-selection.json) contains each selected event and the unchanged complete compact baseline/candidate rows, plus the source file SHA-256. City IDs across diverged games are not generally stable semantic identities. No cross-arm city match is assumed beyond the explicitly equal recorded tuple above.

“Visible within two turns” describes the opening actual attackers' first sighting relative to **assault start**, not capture. For seed 7105 the attack starts one turn after first sighting, but the loss is seven turns after that sighting. The record alone does not establish that a legal defender purchase would have prevented the loss. “Unattributed” means no active assault for the captor team at ownership loss, not necessarily a completely unseen or undefended capture.

## Evidence available and missing

The frozen `rush_ladder::play` builds the measured `PlayerView`, calls the brain and resolves orders, but its serialized result does not retain those views or order lists. `Visibility::observe` retains first sightings internally; `Visibility::report` exports episode metadata rather than attacker positions, identities or combat state. The compact data keeps the full episode records; inspecting the raw runner schema finds no missing diagnostic payload that compacting discarded.

`RL_TRACE` records production-item changes only while `state.turn <= 60`. These selected events begin at T126 or later. `--tape` consumes an existing recording; it is not a recorder for this batch. Aggregate `army_at`/`armies`/`front` fields cannot reconstruct a city decision, and the original non-handover comparison does not collect the handover front telemetry. The AI source exposes no environment-controlled late decision trace; `LEGATUM_ROLLOUT_THREADS` controls search concurrency, not observability. `HILL_TRACE` belongs to the separate recorded-cell `hill_suite` and does not replay this public generated-game comparison.

Missing for all four selected cases:

- The measured seat's pre-order visible units and their distance/reach to the city; whether a war or approach was visible when a defensive decision was possible.
- Treasury, strategic supply, legal unit purchase quotes/blockers, city production/progress and existing defenders.
- The returned `BuyItem`/`SetProduction`/unit orders, their resolution and the ordered capture/raze events needed to explain attribution gaps.

The rules reference states bought units act next turn, purchases share one city slot, and resource/other legality constraints apply. A Gold balance alone would not demonstrate a missed legal defense. The army must be assessed using the actual planning view, not an omniscient post-hoc map.

## Bounded next step

Keep #366 open and policy unchanged. The next diagnostic change should add an **opt-in, output-only trace to the sim example**, with no planner feedback or defaults change. Record the measured seat's exact planning view, returned orders and ordered resolution events for the selected city windows: seed 7102 T290–294; 7105 T122–132; 7110 T283–286; 7149 T261–264. For unattributed cases, include the captor team's unit IDs from resolution only as post-hoc attribution evidence, explicitly separated from the AI's legal view. If this window is insufficient, report that limitation before expanding it.

Replay both arms from T1 on the frozen `1f7db8eed96d513a7b1acc70b8ad72c27458a398` engine inputs with unchanged seeds and configuration. Before interpreting traces, require full outcome/episode equality with the published paired rows, and ensure trace-on versus trace-off produces identical outcomes. Then classify only what the records support: absent visible threat, insufficient/illegal resources, delay before a defender can act, a chosen defense that lost anyway, or an observer attribution limitation. Policy work, if justified, requires fresh disjoint validation seeds.

This PR deliberately stops at the inventory and evidence-gap boundary. It does not claim to have audited actual defender orders or reproduced these games. No human decision is requested; the next task is scoped agent instrumentation and eight bounded game replays after this inventory is reviewed.
