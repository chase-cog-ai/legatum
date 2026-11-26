# Forts make all-AI games about 1% longer and do not change conquest pace (2026-11-25)

2,048 paired eight-empire all-Hard games with barbarians (`legatum-sim --players 8 --difficulty hard --barbarians true --games 2048 --seed 7100`), before (upstream master 7aadfc04, rules 62, after Hard learned to finish a combat unit under way) and after Forts (351d65a4, rules 63):

| | Before | With Forts |
| --- | --- | --- |
| Game length, mean / median | 227.5 / 223 | 229.9 / 224 |
| City captures per game | 22.6 | 22.7 |
| Eliminations per game | 2.26 | 2.25 |
| Battles per game | 1,171 | 1,182 |
| VP hold break rate | 24.8% | 25.5% |
| Games of 350 turns or more | 22 | 28 |
| Games of 400 turns or more | 6 | 7 |
| Games that ran to the turn-500 limit | 2 | 1 |
| Best / worst faction win rate | 2.91 | 2.80 |

Game for game on the same seed, Forts make a game 2.4 turns longer on average (standard error 1.0), about 1%; captures differ by +0.13 a game (standard error 0.19), which is no measurable change. Games that run to the turn-500 limit are not a Fort effect: master has two in these 2,048 (seeds 7793 and 7838), the Fort branch one (seed 7148).

Seed 7148 is the game that reached the limit in the first 256-game pair, where master ends it by VP on turn 285. Nobody ever reached 10 VP in it: the lead changed hands seven times, four wonders and six rival capitals were taken and lost again, and Germany reached 9 VP only on turn 488. Forts were a small part of it: 56 built, 59 moves stopped out of about 33,000, and a Fort bonus in 154 of 3,167 battles. Any change to the rules sends this seed to a different game: with Hard never building a Fort it is master's game exactly (turn 285, Aztecs); with only the stop rule off it ends on turn 255 (Aztecs), with only the defence bonus off on turn 299 (Switzerland), and with both off, so a Fort does nothing, on turn 319 (Celts). A single seed that diverges says nothing about a rule; the tail has to be counted over the whole set.

The first 256 games of this set, read alone, were 225.8 against 227.9 turns, 22.3 against 22.7 captures and a longest game of 338 against 500 turns.

The pair before that (master 1f97d0e6 against a977abfc, before the Hard production change) read 226.6 / 220 against 228.3 / 224 turns, 23.1 against 22.4 captures, 2.04 against 1.96 eliminations and a longest game of 430 against 347 turns.

An earlier pair on rules 61 against 62 (before breakable Walls) read 221.7 against 221.1 turns and 21.3 against 20.8 captures; in it the Hard AI issued 5,662 `BuildFort` orders, in all 256 games (about 22 a game across eight empires), under the rule in `legatum_ai::forts`: one a turn, by a unit already holding its ground on own land with an enemy army within 5 tiles, and only above the Gold reserve. The order count was not recounted on rules 63.

Not measured: a human against the AI's Forts over a full game, and whether a human can wall a chokepoint the AI then fails to route around. The AI does not plan for the stop when it marches; it loses the one turn and walks on.
