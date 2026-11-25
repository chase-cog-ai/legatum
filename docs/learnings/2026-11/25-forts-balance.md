# Forts leave game length and conquest pace unchanged in all-AI games (2026-11-25)

256 paired eight-empire all-Hard games with barbarians (`legatum-sim --players 8 --difficulty hard --barbarians true --games 256 --seed 7100`), before (037d1ee2) and after Forts (rules 61):

| | Before | With Forts |
| --- | --- | --- |
| Game length, mean / median | 221.7 / 217 | 221.1 / 217 |
| City captures per game | 21.3 | 20.8 |
| Eliminations per game | 1.68 | 1.55 |
| War declarations per game | 14.7 | 14.6 |
| Battles per game | 997 | 991 |
| VP hold break rate (of resolved) | 32.6% | 32.2% |
| Longest game | 499 turns | 390 turns |

The Hard AI issued 5,662 `BuildFort` orders, in all 256 games (about 22 a game across eight empires), under the rule in `legatum_ai::forts`: one a turn, by a unit already holding its ground on own land with an enemy army within 5 tiles, and only above the Gold reserve. Captures fell 2.3% and eliminations 7%, a small defender's edge that did not lengthen games. Faction win shares moved within what 256 games can resolve (about 28 wins a faction, so a swing of 10 wins is noise); the faction spread was not measured at the 2,000-game size Pillar 8 asks for.

Not measured: a human against the AI's Forts over a full game, and whether a human can wall a chokepoint the AI then fails to route around. The AI does not plan for the stop when it marches; it loses the one turn and walks on.
