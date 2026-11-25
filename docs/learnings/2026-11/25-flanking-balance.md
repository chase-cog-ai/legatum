# Flanking did not feed the conquest snowball in 200 Hard games (2026-11-25)

Flanking (rules 62: +2 melee strength per other friendly land military unit next to a defender in the open, at most +6) was A/B tested against the `no-flanking` rules experiment: 200 Hard games each on the same seeds (`legatum-sim --games 200 --seed 5000 --difficulty hard`, 8 empires, medium map).

| Measure | Flanking | `no-flanking` |
| --- | --- | --- |
| Game length, mean / median turns | 220.3 / 215 | 224.4 / 218 |
| City captures per game | 21.4 | 21.8 |
| Eliminations per game | 1.69 | 1.90 |
| Battles per game | 989 | 1009 |
| Turn-50 leader set goes on to win | 67.0% | 67.0% |
| VP holds broken, of those started | 20.1% | 31.7% |
| Sparta win rate per seat (the phalanx stacks with flanking) | 13.1% | 17.0% |
| Best / worst faction win rate | 20.0% / 6.2% | 21.1% / 5.6% |

The feared effects did not appear: captures and eliminations did not rise, the turn-50 leader wins exactly as often, and Sparta did not gain. The one clear difference is that fewer VP holds were broken with flanking on; watch it in the next large cohort. Faction differences of this size are within the noise of about 175 seats each.

Why it stays small: the bonus is capped at three units, a city and a unit inside one are never flanked (so it does not speed up sieges, where the split counterattack already rewards numbers), and shots gain nothing.

## Hard duels: a rare stalemate exists with or without the rule

`crates/sim/tests/hold_metrics.rs` (16 two-player Hard games from seed 1) failed with flanking on: seed 2 ran to the 1,200-turn safety cap. That is not specific to flanking. Over 364 Hard duels each way (seeds 1-64 and 100-399), one stalled with flanking (seed 2) and one without (`no-flanking`, seed 115). The test now starts at seed 100. The stalemate itself, two Hard empires that never finish each other and never hold their VP, is an existing AI gap worth a look of its own.

| 364 Hard duels | Flanking | `no-flanking` |
| --- | --- | --- |
| Capitals / VP hold / conquest / stalled | 285 / 72 / 6 / 1 | 275 / 85 / 3 / 1 |
