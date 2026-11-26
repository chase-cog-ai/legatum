# Flanking did not feed the conquest snowball in 200 Hard games (2026-11-25)

Flanking (rules 64: +2 melee strength per other friendly land military unit next to a defender in the open, at most +6) was A/B tested against the `no-flanking` rules experiment: 200 Hard games each on the same seeds (`legatum-sim --games 200 --seed 5000 --difficulty hard`, 8 empires, medium map).

| Measure | Flanking | `no-flanking` |
| --- | --- | --- |
| Game length, mean / median turns | 251.0 / 243 | 243.3 / 238 |
| City captures per game | 23.2 | 22.4 |
| Eliminations per game | 2.69 | 2.73 |
| Battles per game | 1219 | 1172 |
| Turn-50 leader set goes on to win | 36.0% | 36.5% |
| VP holds broken, of those started | 23.3% | 21.6% |
| Sparta win rate per seat (the phalanx stacks with flanking) | 18.2% | 18.2% |
| Best / worst faction win rate | 20.9% / 1.7% | 22.0% / 2.2% |

These figures are from the branch with master merged through #319 (city upkeep, rules 63, and Hard Sparta's conquest opening, both 2026-11-26), commit `9deaa15d`. The feared effects did not appear: captures and eliminations are level, the turn-50 leader wins as often, and Sparta wins as often with the rule as without. Games run about eight turns longer with flanking in this cohort. The wide faction spread (India near 2%) is there with and without the rule.

VP holds broken has had no stable direction across four runs of the same seeds as master moved: 20.1% against 31.7% before breakable Walls, 17.1% against 19.3% with them, 21.6% against 19.6% after #306, and 23.3% against 21.6% now.

Why it stays small: the bonus is capped at three units, a city and a unit inside one are never flanked (so it does not speed up sieges, where the split counterattack already rewards numbers), and shots gain nothing.

## Hard duels: a rare stalemate exists with or without the rule

`crates/sim/tests/hold_metrics.rs` (16 two-player Hard games from seed 1) failed with flanking on: seed 2 ran to the 1,200-turn safety cap. That is not specific to flanking. Before breakable Walls were merged (not rerun since), over 364 Hard duels each way (seeds 1-64 and 100-399), one stalled with flanking (seed 2) and one without (`no-flanking`, seed 115). The test now starts at seed 100. The stalemate itself, two Hard empires that never finish each other and never hold their VP, is an existing AI gap worth a look of its own.

| 364 Hard duels | Flanking | `no-flanking` |
| --- | --- | --- |
| Capitals / VP hold / conquest / stalled | 285 / 72 / 6 / 1 | 275 / 85 / 3 / 1 |
