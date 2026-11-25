# Flanking did not feed the conquest snowball in 200 Hard games (2026-11-25)

Flanking (rules 63: +2 melee strength per other friendly land military unit next to a defender in the open, at most +6) was A/B tested against the `no-flanking` rules experiment: 200 Hard games each on the same seeds (`legatum-sim --games 200 --seed 5000 --difficulty hard`, 8 empires, medium map).

| Measure | Flanking | `no-flanking` |
| --- | --- | --- |
| Game length, mean / median turns | 224.5 / 219 | 222.3 / 217 |
| City captures per game | 22.5 | 22.5 |
| Eliminations per game | 1.95 | 1.94 |
| Battles per game | 1037 | 1034 |
| Turn-50 leader set goes on to win | 65.0% | 66.5% |
| VP holds broken, of those started | 17.1% | 19.3% |
| Sparta win rate per seat (the phalanx stacks with flanking) | 9.1% | 12.5% |
| Best / worst faction win rate | 20.2% / 5.6% | 20.8% / 6.4% |

These figures are from the branch with breakable Walls (rules 62) merged, commit `8dd4896e`. The feared effects did not appear: captures and eliminations are level, the turn-50 leader wins about as often, and Sparta did not gain. Faction differences of this size are within the noise of about 175 seats each.

An earlier run of the same seeds before breakable Walls showed fewer VP holds broken with flanking on (20.1% against 31.7%); with breakable Walls the gap is 17.1% against 19.3%, so that difference did not hold up.

Why it stays small: the bonus is capped at three units, a city and a unit inside one are never flanked (so it does not speed up sieges, where the split counterattack already rewards numbers), and shots gain nothing.

## Hard duels: a rare stalemate exists with or without the rule

`crates/sim/tests/hold_metrics.rs` (16 two-player Hard games from seed 1) failed with flanking on: seed 2 ran to the 1,200-turn safety cap. That is not specific to flanking. Before breakable Walls were merged (not rerun since), over 364 Hard duels each way (seeds 1-64 and 100-399), one stalled with flanking (seed 2) and one without (`no-flanking`, seed 115). The test now starts at seed 100. The stalemate itself, two Hard empires that never finish each other and never hold their VP, is an existing AI gap worth a look of its own.

| 364 Hard duels | Flanking | `no-flanking` |
| --- | --- | --- |
| Capitals / VP hold / conquest / stalled | 285 / 72 / 6 / 1 | 275 / 85 / 3 / 1 |
