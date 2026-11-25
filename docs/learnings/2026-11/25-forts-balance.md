# Forts barely move game length and conquest pace in all-AI games (2026-11-25)

256 paired eight-empire all-Hard games with barbarians (`legatum-sim --players 8 --difficulty hard --barbarians true --games 256 --seed 7100`), before (upstream master 1f97d0e6, rules 62, breakable Walls) and after Forts (a977abfc, rules 63):

| | Before | With Forts |
| --- | --- | --- |
| Game length, mean / median | 226.6 / 220 | 228.3 / 224 |
| City captures per game | 23.1 | 22.4 |
| Eliminations per game | 2.04 | 1.96 |
| War declarations per game | 15.3 | 15.3 |
| Battles per game | 1,060 | 1,072 |
| VP hold break rate (of resolved) | 29.6% | 26.5% |
| Longest game | 430 turns | 347 turns |

Captures fell 3.2% and eliminations 4%, a small defender's edge; the mean game ran 1.7 turns (0.8%) longer and the longest game was shorter. Faction win shares moved within what 256 games can resolve (about 28 wins a faction, so a swing of 10 wins is noise); the faction spread was not measured at the 2,000-game size Pillar 8 asks for.

An earlier pair on rules 61 against 62 (before breakable Walls) read 221.7 against 221.1 turns and 21.3 against 20.8 captures; in it the Hard AI issued 5,662 `BuildFort` orders, in all 256 games (about 22 a game across eight empires), under the rule in `legatum_ai::forts`: one a turn, by a unit already holding its ground on own land with an enemy army within 5 tiles, and only above the Gold reserve. The order count was not recounted on rules 63.

Not measured: a human against the AI's Forts over a full game, and whether a human can wall a chokepoint the AI then fails to route around. The AI does not plan for the stop when it marches; it loses the one turn and walks on.
