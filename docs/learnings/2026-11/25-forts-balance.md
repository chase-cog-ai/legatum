# Forts barely move game length and conquest pace in all-AI games (2026-11-25)

256 paired eight-empire all-Hard games with barbarians (`legatum-sim --players 8 --difficulty hard --barbarians true --games 256 --seed 7100`), before (upstream master 7aadfc04, rules 62, after Hard learned to finish a combat unit under way) and after Forts (351d65a4, rules 63):

| | Before | With Forts |
| --- | --- | --- |
| Game length, mean / median | 225.8 / 226 | 227.9 / 226 |
| City captures per game | 22.3 | 22.7 |
| Eliminations per game | 2.24 | 2.29 |
| War declarations per game | 15.6 | 15.9 |
| Battles per game | 1,163 | 1,173 |
| VP hold break rate (of resolved) | 21.6% | 24.1% |
| Longest game | 338 turns | 500 turns (the turn limit, one game) |

Every difference is within 2% except the hold break rate, and the signs are not the ones the earlier pairs showed (captures up 1.7% here, down 3.2% before), so at 256 games Forts have no measurable effect on conquest pace. The mean game ran 2.1 turns (0.9%) longer, 1.1 turns of it from one game: seed 7148 ran to the 500-turn limit with no winner (the leader on 9 of 10 VP), where without Forts it ended by VP on turn 285. That game had 56 city captures and three eliminations after turn 450, so it was not a frozen front; why nobody took a tenth VP was not investigated, and no other game in any of the pairs (768 Fort games) passed turn 347. Faction win shares moved within what 256 games can resolve (about 28 wins a faction, so a swing of 10 wins is noise); the faction spread was not measured at the 2,000-game size Pillar 8 asks for.

The pair before that (master 1f97d0e6 against a977abfc, before the Hard production change) read 226.6 / 220 against 228.3 / 224 turns, 23.1 against 22.4 captures, 2.04 against 1.96 eliminations and a longest game of 430 against 347 turns.

An earlier pair on rules 61 against 62 (before breakable Walls) read 221.7 against 221.1 turns and 21.3 against 20.8 captures; in it the Hard AI issued 5,662 `BuildFort` orders, in all 256 games (about 22 a game across eight empires), under the rule in `legatum_ai::forts`: one a turn, by a unit already holding its ground on own land with an enemy army within 5 tiles, and only above the Gold reserve. The order count was not recounted on rules 63.

Not measured: a human against the AI's Forts over a full game, and whether a human can wall a chokepoint the AI then fails to route around. The AI does not plan for the stop when it marches; it loses the one turn and walks on.
