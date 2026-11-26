# Flanking did not feed the conquest snowball in 200 Hard games (2026-11-25)

Flanking (rules 64: +2 melee strength per other friendly land military unit next to a defender in the open, at most +6) was A/B tested against the `no-flanking` rules experiment: 200 Hard games each on the same seeds (`legatum-sim --games 200 --seed 5000 --difficulty hard`, 8 empires, medium map).

| Measure | Flanking | `no-flanking` |
| --- | --- | --- |
| Game length, mean / median turns | 224.9 / 220 | 228.9 / 222 |
| City captures per game | 22.1 | 23.1 |
| Eliminations per game | 2.13 | 2.33 |
| Battles per game | 1157 | 1175 |
| Turn-50 leader set goes on to win | 65.0% | 70.0% |
| VP holds broken, of those started | 21.6% | 19.6% |
| Sparta win rate per seat (the phalanx stacks with flanking) | 11.4% | 13.6% |
| Best / worst faction win rate | 20.9% / 6.2% | 17.1% / 8.7% |

These figures are from the branch with master merged through #306 (Hard finishing a combat unit under way, 2026-11-26), commit `8a6de0eb`. The feared effects did not appear: captures and eliminations are slightly lower with flanking, the turn-50 leader wins no more often, and Sparta did not gain. Faction differences of this size are within the noise of about 175 seats each.

VP holds broken has had no stable direction across three runs of the same seeds as master moved: 20.1% against 31.7% before breakable Walls, 17.1% against 19.3% with them, and 21.6% against 19.6% now.

Why it stays small: the bonus is capped at three units, a city and a unit inside one are never flanked (so it does not speed up sieges, where the split counterattack already rewards numbers), and shots gain nothing.

## Hard duels: a rare stalemate exists with or without the rule

`crates/sim/tests/hold_metrics.rs` (16 two-player Hard games from seed 1) failed with flanking on: seed 2 ran to the 1,200-turn safety cap. That is not specific to flanking. Before breakable Walls were merged (not rerun since), over 364 Hard duels each way (seeds 1-64 and 100-399), one stalled with flanking (seed 2) and one without (`no-flanking`, seed 115). The test now starts at seed 100. The stalemate itself, two Hard empires that never finish each other and never hold their VP, is an existing AI gap worth a look of its own.

| 364 Hard duels | Flanking | `no-flanking` |
| --- | --- | --- |
| Capitals / VP hold / conquest / stalled | 285 / 72 / 6 / 1 | 275 / 85 / 3 / 1 |
