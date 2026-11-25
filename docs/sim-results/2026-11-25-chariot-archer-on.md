# Simulation: chariot-archer-on (2026-11-25)

1024 games, first seed 9311003, brain `scripted`, turn limit off, safety cap 1200, 4 threads, started 2026-11-25T02:42:21Z. Wall time 1870.3s.

Source checkout at run start: `5a8c28c5180efd5a64b53f57589b9cb48c35d4e6`; dirty: no. Map 48×32, 8 initial seats. `first_setup` and exact binary argv are recorded in JSON. Ordinary games use seed + i (wrapping); the strategy tournament schedule is specified below. All games use a deterministic shuffle of the full faction pool, taking eight distinct factions, with any Proxy rotation described below.

Command argv: `["./target/release/legatum-sim","--players","8","--factions","Egypt,Rome,Sparta,Aztecs,Mongols,Celts,Athens,India","--rotate-factions","--difficulty","hard","--games","1024","--seed","9311003","--threads","4","--label","chariot-archer-on"]`

## Overview

| Metric | Value |
| --- | --- |
| Games (non-panic) | 1024 of 1024 |
| Game length, turns (mean / median / min-max) | 223.9 / 220 / 143-410 |
| Win reason mix | VP hold 98.7%, Capitals 1.3% |
| Median end turn | 220 |
| Ending shares of played games (capitals / VP hold / turn limit / other) | 1.3% / 98.7% / 0.0% / 0.0% |
| Games reaching the World War era (any empire) | 1019 of 1024 (99.5%) |
| VP hold length | 10 turn ends (the reaching turn counts) |
| Holds started / broken / completed / unfinished | 1511 / 396 / 1015 / 100 |
| Hold break rate | 26.2% of started; 28.1% of resolved (broken / (broken + completed)) |
| Hold wins | 1011 |
| First reach to win, turns (mean / median / min-max) | 11.1 / 9 / 9-85 |
| Final hold start to win, turns | 9.0 / 9 / 9-9 |
| Comebacks (winner broke an earlier hold of its own) | 121 |
| Wins by a team another team beat to its target | 62 |

Game length by how the game ended (turns resolved; a stall is a safety-cap stop, never a win):

| Ended by | Games | Length mean / median / min-max |
| --- | --- | --- |
| VP hold | 1011 | 223.3 / 220 / 143-410 |
| Capitals | 13 | 272.3 / 269 / 181-392 |

Winner share by faction (wins of that faction's seats / all winning seats):

| Faction | Wins | Share of wins |
| --- | --- | --- |
| India | 100 | 9.8% |
| Celts | 146 | 14.3% |
| Mongols | 89 | 8.7% |
| Sparta | 109 | 10.6% |
| Athens | 168 | 16.4% |
| Aztecs | 136 | 13.3% |
| Egypt | 228 | 22.3% |
| Rome | 48 | 4.7% |

## Outcomes

| Outcome | Games | Share |
| --- | --- | --- |
| VP hold | 1011 | 98.7% |
| Conquest | 0 | 0.0% |
| Capitals | 13 | 1.3% |
| Turn limit | 0 | 0.0% |
| Draw | 0 | 0.0% |
| Stalled (safety cap) | 0 | 0.0% |
| Panic | 0 | 0.0% |

## Win rate by faction

Per seat: wins / seats of that faction. With 8 single-empire teams, an even game is 12.5% per seat.

| Faction | Seats | Wins | Win rate |
| --- | --- | --- | --- |
| India | 1024 | 100 | 9.8% |
| Celts | 1024 | 146 | 14.3% |
| Mongols | 1024 | 89 | 8.7% |
| Sparta | 1024 | 109 | 10.6% |
| Athens | 1024 | 168 | 16.4% |
| Aztecs | 1024 | 136 | 13.3% |
| Egypt | 1024 | 228 | 22.3% |
| Rome | 1024 | 48 | 4.7% |

## Win rate by difficulty

All initial seats in non-panicked games, including eliminated empires and cap stops. Determinism replays are not additional samples.

| Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- |
| Hard | 8192 | 1024 | 12.5% |

## Win rate by faction and difficulty

| Faction | Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- | --- |
| India | Hard | 1024 | 100 | 9.8% |
| Celts | Hard | 1024 | 146 | 14.3% |
| Mongols | Hard | 1024 | 89 | 8.7% |
| Sparta | Hard | 1024 | 109 | 10.6% |
| Athens | Hard | 1024 | 168 | 16.4% |
| Aztecs | Hard | 1024 | 136 | 13.3% |
| Egypt | Hard | 1024 | 228 | 22.3% |
| Rome | Hard | 1024 | 48 | 4.7% |

## Faction goal progress

Per seat: how often the faction's permanent goal was reached before the game ended, and the turn its VP was first awarded (over the seats that reached it). Seats are all initial empires of non-panicked games with telemetry, including eliminated empires and cap stops.

| Faction | Seats | Reached | Goal rate | Min | P10 | Median turn | P90 | Max |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| India | 1024 | 507 | 49.5% | 58 | 84 | 122 | 200 | 276 |
| Celts | 1024 | 612 | 59.8% | 34 | 179 | 210 | 210 | 216 |
| Mongols | 1024 | 146 | 14.3% | 139 | 167 | 200 | 240 | 324 |
| Sparta | 1024 | 113 | 11.0% | 137 | 158 | 202 | 252 | 392 |
| Athens | 1024 | 286 | 27.9% | 138 | 166 | 225 | 255 | 255 |
| Aztecs | 1024 | 294 | 28.7% | 137 | 175 | 225 | 233 | 285 |
| Egypt | 1024 | 408 | 39.8% | 124 | 181 | 198 | 232 | 315 |
| Rome | 1024 | 30 | 2.9% | 132 | 155 | 186 | 227 | 257 |

## Goal cities (Rome's Colonia, Egypt's tomb)

Per seat of the faction: its goal city stood (median first turn), the seat scored it, held it at the end, a rival held it at the end, it changed hands. Rome: seats with a level-3 melee unit while the Colonia was open (median first turn) and the share of those seat-turns with no legal Colonia site on the map. Egypt: seats that started the work (median first start), work turns and pauses per starting seat.

| Faction | Seats | Stood | Turn | Scored | Held at end | Rival holds | Changed hands | Level 3 | Turn | No site | Started | Turn | Work turns | Pauses |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Egypt | 1024 | 350 | 200 | 350 | 264 | 55 | 69 | 0 | - | - | 587 | 186 | 11.5 | 0.6 |
| Rome | 1024 | 0 | - | 0 | 0 | 0 | 0 | 0 | - | - | 0 | - | - | - |

## Distributions

| Metric | Min | P10 | Median | P90 | Max |
| --- | --- | --- | --- | --- | --- |
| Observed game length (turns, including cap stops) | 143 | 185 | 220 | 268 | 410 |
| Winning team VP | 3 | 10 | 11 | 13 | 18 |
| First Soldier completed (turn, per empire) | 5 | 8 | 25 | 47 | 212 |
| First Settler completed (turn, per empire) | 5 | 12 | 14 | 42 | 192 |
| First city founded (turn, per empire) | 1 | 1 | 1 | 1 | 1 |
| Second city founded (turn, per empire) | 8 | 16 | 22 | 58 | 229 |

Safety-cap stops are censored observations, not completed game lengths.

Openings: 8192 observed empires in 1024 first plays; 0 games without opening telemetry excluded. Turn distributions include only empires that reached the milestone. Never means not reached by the end of observed play, including eliminated empires and safety-cap stops. Per-empire turns are in JSON (`games[].telemetry.openings`, keyed by player ID).

| Opening milestone | Reached empires | Never empires |
| --- | --- | --- |
| First Soldier completed | 8068 | 124 |
| First Settler completed | 7128 | 1064 |
| First city founded | 8192 | 0 |
| Second city founded | 7859 | 333 |

Final VP per empire (all seats):

| VP | Empires |
| --- | --- |
| 0 | 2015 |
| 1 | 1381 |
| 2 | 999 |
| 3 | 793 |
| 4 | 510 |
| 5 | 440 |
| 6 | 358 |
| 7 | 248 |
| 8 | 172 |
| 9 | 154 |
| 10 | 461 |
| 11 | 336 |
| 12 | 164 |
| 13 | 75 |
| 14 | 52 |
| 15 | 19 |
| 16 | 10 |
| 17 | 3 |
| 18 | 2 |

## End state

- Average game length: 223.9 turns
- Average cities per empire: 4.44
- Average techs per empire: 30.07
- Average eliminations per game: 2.44

## Strategy diversity: observed behavior (classifier v2)

1024 finished games; 8192 player seats; 1024 winning seats. Excluded 0 unfinished/panicked games and 0 games with missing labels. Winner entropy: **0.4003** (normalized over 6 categories, including zero-win categories).

Thresholds, biases and denominators: [strategy-diversity.md](../strategy-diversity.md).

| Archetype | Players | Wins | Win rate | All-player frequency | Winner frequency |
| --- | ---: | ---: | ---: | ---: | ---: |
| Rush | 3136 | 118 | 3.8% | 38.3% | 11.5% |
| Wide | 2921 | 807 | 27.6% | 35.7% | 78.8% |
| Science | 0 | 0 | n/a | 0.0% | 0.0% |
| Turtle/Wonder | 715 | 74 | 10.3% | 8.7% | 7.2% |
| Economic | 204 | 25 | 12.3% | 2.5% | 2.4% |
| Generalist | 1216 | 0 | 0.0% | 14.8% | 0.0% |

### Matchup matrix

Cell = strict ahead % / tied % (co-occurrence games). Each game has equal weight; all cross-team seat pairs within it share that weight. Rank: winning team first, then surviving empires by VP, then eliminated empires by later elimination turn. Equal ranks tie; no ID tiebreak. This is a multiplayer association, not a duel win rate.

| Row vs column | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Rush | — | 16.0% / 3.2% (1003) | n/a / n/a (0) | 17.1% / 7.5% (532) | 21.8% / 5.7% (189) | 74.5% / 3.1% (780) |
| Wide | 80.8% / 3.2% (1003) | — | n/a / n/a (0) | 57.1% / 5.5% (532) | 58.0% / 6.0% (185) | 93.7% / 1.3% (781) |
| Science | n/a / n/a (0) | n/a / n/a (0) | — | n/a / n/a (0) | n/a / n/a (0) | n/a / n/a (0) |
| Turtle/Wonder | 75.3% / 7.5% (532) | 37.4% / 5.5% (532) | n/a / n/a (0) | — | 44.6% / 9.5% (77) | 93.1% / 3.9% (400) |
| Economic | 72.5% / 5.7% (189) | 36.0% / 6.0% (185) | n/a / n/a (0) | 45.9% / 9.5% (77) | — | 91.2% / 4.5% (132) |
| Generalist | 22.5% / 3.1% (780) | 5.0% / 1.3% (781) | n/a / n/a (0) | 3.0% / 3.9% (400) | 4.2% / 4.5% (132) | — |

### Faction wins by archetype

Cell = wins / seats; win rate in JSON.

| Faction | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| India | 4 / 281 | 96 / 521 | 0 / 0 | 0 / 15 | 0 / 6 | 0 / 201 |
| Celts | 3 / 155 | 137 / 539 | 0 / 0 | 6 / 125 | 0 / 18 | 0 / 187 |
| Mongols | 31 / 660 | 55 / 205 | 0 / 0 | 0 / 0 | 3 / 18 | 0 / 141 |
| Sparta | 20 / 539 | 58 / 188 | 0 / 0 | 14 / 112 | 17 / 84 | 0 / 101 |
| Athens | 3 / 115 | 145 / 466 | 0 / 0 | 19 / 154 | 1 / 16 | 0 / 273 |
| Aztecs | 6 / 188 | 118 / 519 | 0 / 0 | 9 / 89 | 3 / 49 | 0 / 179 |
| Egypt | 29 / 406 | 178 / 419 | 0 / 0 | 21 / 143 | 0 / 1 | 0 / 55 |
| Rome | 22 / 792 | 20 / 64 | 0 / 0 | 5 / 77 | 1 / 12 | 0 / 79 |

Guardrail: **Fail**. Entropy minimum 0.80; at least 20 games per strategy pair. Dominant by observed score: none. Without an observed counter (all pairs covered): none. Sampled but never won: Generalist. Unobserved: Science. Under-sampled pairs: 5.

- insufficient evidence: 5 under-sampled matchups; 1024 winning seats

- winner entropy below 0.80

## Battle and capture telemetry

1024 observed first plays; 0 records without telemetry excluded.

| Event | Total / observed games | Mean per game | Games with event / observed games |
| --- | --- | --- | --- |
| War declarations | 15552/1024 | 15.188 | 1024/1024 (100.0%) |
| Peace offers | 93191/1024 | 91.007 | 1024/1024 (100.0%) |
| Peace agreements | 12616/1024 | 12.320 | 1024/1024 (100.0%) |
| Battles | 1199188/1024 | 1171.082 | 1024/1024 (100.0%) |
| City captures | 22617/1024 | 22.087 | 1024/1024 (100.0%) |
| Settler captures | 2920/1024 | 2.852 | 940/1024 (91.8%) |
| Eliminations | 2497/1024 | 2.438 | 987/1024 (96.4%) |

## Terminal VP sources

1024 finished games. Columns are separate populations; totals across columns must not be added. Every cell is VP / population observations.

| Source | All players | All teams | Winning teams |
| --- | --- | --- | --- |
| Held Great Works | 7052/8192 | 7052/8192 | 3196/1024 |
| Faction goals | 3864/8192 | 3864/8192 | 1518/1024 |
| Era objectives | 15385/8192 | 15385/8192 | 4971/1024 |
| Capitals taken | 2852/8192 | 2852/8192 | 1769/1024 |
| Total VP | 29153/8192 | 29153/8192 | 11454/1024 |

Reconciliation failures: 0 player observations, 0 team observations.

## Turn-50 team leaders

| Cohort | Eventual leader-set wins / eligible games | Draws (included) |
| --- | --- | --- |
| Unique leader | 95/603 (15.8%) | 0 |
| Tied leaders (winner in set) | 344/421 (81.7%) | 0 |
| Any leader set | 439/1024 (42.9%) | 0 |

Excluded: 0 games ended before turn 50, 0 unfinished games, 0 snapshots without teams, 0 records without telemetry.

## Era pacing

Resolved turn each empire entered each era. `Winner vs median` is the winning team's seats' entry turn minus the median entry of the empires that entered it in the same finished game (negative: winners got there first).

| Era | Entered / seats | Fastest | p10 | Median | p90 | Slowest | Winner vs median (mean, n) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Classical | 7928/8192 | 6 | 20 | 28 | 33 | 74 | -2.7 (1024) |
| Medieval | 7672/8192 | 35 | 58 | 63 | 69 | 126 | -2.6 (1024) |
| Renaissance | 7579/8192 | 65 | 84 | 92 | 104 | 244 | -5.2 (1024) |
| Industrial | 7184/8192 | 88 | 110 | 122 | 143 | 256 | -8.5 (1024) |
| Machine Age | 6260/8192 | 114 | 138 | 153 | 176 | 256 | -11.3 (1024) |
| World War | 4843/8192 | 133 | 161 | 179 | 204 | 297 | -12.5 (1014) |
| Modern | 3014/8192 | 150 | 182 | 205 | 232 | 336 | -10.1 (897) |
| Future | 1338/8192 | 170 | 202 | 230 | 256 | 323 | -5.2 (558) |

Research idle turns (techs left, no research set): 3, of which stalled (two or more in a row): 0.

## Tech pace against the calendar

Median resolved turn each technology was learned, the display year of that turn (`calendar::calendar_label`) and the year the technology really arrived (n/a when no single year is established). Empires that never learned it are left out.

| Era | Technology | Learned / seats | Median turn | Median year | Real year |
| --- | --- | --- | --- | --- | --- |
| Ancient | Irrigation | 8190/8192 | 8 | 3276 BC | 6000 BC |
| Ancient | Bronze Working | 7119/8192 | 19 | 2138 BC | 3300 BC |
| Ancient | Masonry | 8192/8192 | 12 | 2863 BC | 3000 BC |
| Ancient | Currency | 8007/8192 | 26 | 1414 BC | 600 BC |
| Ancient | Writing | 8007/8192 | 25 | 1518 BC | 3200 BC |
| Classical | Mathematics | 7727/8192 | 44 | 301 BC | 300 BC |
| Classical | Engineering | 7703/8192 | 51 | 50 AD | n/a |
| Classical | Iron Working | 7779/8192 | 38 | 601 BC | 1200 BC |
| Classical | Code of Laws | 7682/8192 | 58 | 400 AD | 1754 BC |
| Classical | Philosophy | 7674/8192 | 63 | 590 AD | 500 BC |
| Medieval | Feudalism | 7647/8192 | 76 | 980 AD | n/a |
| Medieval | Machinery | 7654/8192 | 71 | 830 AD | n/a |
| Medieval | Chivalry | 7633/8192 | 82 | 1160 AD | n/a |
| Medieval | Education | 7606/8192 | 88 | 1340 AD | 1088 AD |
| Medieval | Crop Rotation | 7585/8192 | 91 | 1410 AD | n/a |
| Renaissance | Gunpowder | 7535/8192 | 99 | 1490 AD | n/a |
| Renaissance | Metallurgy | 7415/8192 | 110 | 1600 AD | n/a |
| Renaissance | Military Engineering | 7461/8192 | 107 | 1570 AD | n/a |
| Renaissance | Banking | 7272/8192 | 119 | 1690 AD | 1397 AD |
| Renaissance | Printing Press | 7211/8192 | 120 | 1700 AD | 1440 AD |
| Industrial | Rifling | 6610/8192 | 143 | 1816 AD | n/a |
| Industrial | Military Science | 6883/8192 | 135 | 1787 AD | n/a |
| Industrial | Ballistics | 6680/8192 | 145 | 1824 AD | n/a |
| Industrial | Industrialization | 7026/8192 | 131 | 1772 AD | 1769 AD |
| Industrial | Railways | 6265/8192 | 153 | 1854 AD | 1825 AD |
| Machine Age | Replaceable Parts | 5594/8192 | 166 | 1886 AD | 1800 AD |
| Machine Age | Combustion | 5916/8192 | 161 | 1881 AD | 1876 AD |
| Machine Age | Steel | 5595/8192 | 165 | 1885 AD | 1856 AD |
| Machine Age | Flight | 5134/8192 | 174 | 1895 AD | 1903 AD |
| Machine Age | Electricity | 4899/8192 | 179 | 1901 AD | 1879 AD |
| World War | Mass Mobilisation | 3886/8192 | 192 | 1916 AD | n/a |
| World War | Rocketry | 3633/8192 | 196 | 1922 AD | 1926 AD |
| World War | Armoured Warfare | 3424/8192 | 199 | 1926 AD | 1916 AD |
| World War | Air Power | 3234/8192 | 202 | 1931 AD | n/a |
| World War | Electronics | 4428/8192 | 185 | 1908 AD | n/a |
| Modern | Mechanised Warfare | 2004/8192 | 216 | 1952 AD | n/a |
| Modern | Composite Armour | 1766/8192 | 220 | 1962 AD | n/a |
| Modern | Guided Missiles | 1630/8192 | 225 | 1974 AD | n/a |
| Modern | Jet Engines | 1598/8192 | 228 | 1981 AD | 1939 AD |
| Modern | Green Revolution | 2461/8192 | 211 | 1944 AD | n/a |
| Future | Autonomous Systems | 797/8192 | 244 | 2016 AD | n/a |
| Future | Robotics | 585/8192 | 250 | 2026 AD | n/a |
| Future | Fusion Power | 634/8192 | 253 | 2029 AD | n/a |
| Future | Stealth Technology | 996/8192 | 238 | 2005 AD | n/a |
| Future | Recursive AI | 525/8192 | 256 | 2032 AD | n/a |

## Metric definitions

One Event::Battle, including Scout encounters, ranged shots and meeting engagements; not a war, siege or submitted attack order.

City captures use the existing GameRecord.city_captures count of Event::CityCaptured (recaptures count again). Settler captures count Event::SettlerCaptured separately; neither is inferred from ownership or unit losses.

Count the existing GameRecord.eliminations entries, one per Event::PlayerEliminated; telemetry.elimination_count is their length. Never infer from final alive flags.

Final-state VP: scored World Age cards and Celtic-sacked rival capitals are banked; held rival capitals count for their holder; Great Works and faction goals use their held core VpSource entries for the current holder. Player sources reconcile to core vp_total; team sources sum each member once and reconcile to core team_vp. Player, team and winning-team populations are separate views, never added together. Eliminated members retain banked VP, as in core scoring.

After resolution AND scoring of turn 50, when GameState.turn is 51, before turn-51 orders. Rank all initial TEAMS by core team_vp (sum every member once), matching the probe's all-seat population. Equal VP preserves the entire leader set: no population or ID tie-break. Games ending on turn 50 are eligible; games ending earlier are excluded. Turn-75 replay comeback data is independent.

Per initial empire, record the resolved turn (turn 1 is the first turn) of its first Event::ProductionCompleted for Soldier and Settler, and its second Event::CityFounded. Starting units and captured units do not count as production; purchases count only when production actually completes. City captures do not count as founding, and losing a city does not reset the founding count. Null means never reached by the end of observed play. Distributions pool reached empires only, using the report's existing percentile convention; reached and never counts include eliminated empires and safety-cap stops. Cap stops are censored, not proof a milestone could never happen. Only first plays count; panics and legacy records without opening telemetry are excluded, not never observations.

A team wins by VP only after standing at its VP target at data::VP_HOLD_TURNS consecutive turn ends (10 Standard); the turn it reaches the target is the first held turn. Started = Event::VictoryHoldStarted; broken = Event::VictoryHoldBroken (dropped below the target or was eliminated); completed = the hold reached the full count (a VP win, or a draw when several teams complete with identical standings); unfinished = still running when the game ended otherwise (conquest, turn limit, safety cap). started = broken + completed + unfinished. Break rate = broken / started, and broken / (broken + completed) over resolved holds. First reach to win = win turn minus the first turn the winner ever stood at its target (includes broken holds); final hold start to win is always turns_needed - 1. A comeback is a hold win by a team that broke an earlier hold of its own; wins not first to reach are hold wins where another team first stood at its target strictly earlier. Only games with games[].telemetry.victory_holds count; panics and older reports are excluded, not zero. Outcome `vp` means a completed hold: a winner at or above its target at the turn limit without a completed hold is `turn_limit`.

Event totals and incidence use first plays with telemetry, including runner-stopped games; panics and legacy records without telemetry are excluded, not zeros. Terminal score populations use finished games only. Snowball cohorts use finished games with a turn-50 snapshot and at least one team; unique and tied leaders are separate. Draws stay in each eligible denominator and count as no leader win. Unfinished games have no eventual result and are excluded. Empty denominators are reported as 0/0 (no rate).

Definitions reuse [balance-probe-1](../analysis/balance-probe-1.md) and [the probe's definitions](../../tools/analysis/README.md); this report is a fresh simulation sample, not a rerun of those campaigns.

## Promotions

1024 games with promotion telemetry. Promotions are UnitPromoted events by the level reached; final units are military units alive when the game ended.

| Level | Promotions / game | Final units / game |
| --- | --- | --- |
| 0 | 0.00 | 87.20 |
| 1 | 73.81 | 16.83 |
| 2 | 21.37 | 5.28 |
| 3 | 5.04 | 1.44 |
| 4 | 0.88 | 0.30 |
| 5 | 0.17 | 0.07 |

## Per-game events

Each row is one observed first play; unavailable telemetry is shown as n/a. Full per-player/team score sources and turn-50 standings are in JSON.

| Game | Seed | Resolved turns | Battles | City captures | Settler captures | Eliminations |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 9311003 | 219 | 1024 | 17 | 3 | 2 |
| 1 | 9311004 | 222 | 1019 | 13 | 3 | 1 |
| 2 | 9311005 | 216 | 1071 | 17 | 3 | 2 |
| 3 | 9311006 | 206 | 1166 | 13 | 4 | 1 |
| 4 | 9311007 | 235 | 1225 | 31 | 2 | 1 |
| 5 | 9311008 | 173 | 806 | 21 | 2 | 3 |
| 6 | 9311009 | 177 | 787 | 19 | 2 | 1 |
| 7 | 9311010 | 226 | 1382 | 26 | 1 | 4 |
| 8 | 9311011 | 199 | 762 | 24 | 0 | 1 |
| 9 | 9311012 | 241 | 1190 | 39 | 5 | 4 |
| 10 | 9311013 | 258 | 1302 | 22 | 1 | 2 |
| 11 | 9311014 | 250 | 1853 | 34 | 2 | 2 |
| 12 | 9311015 | 201 | 1023 | 22 | 3 | 1 |
| 13 | 9311016 | 184 | 923 | 19 | 3 | 3 |
| 14 | 9311017 | 187 | 837 | 20 | 3 | 4 |
| 15 | 9311018 | 216 | 1204 | 18 | 2 | 5 |
| 16 | 9311019 | 240 | 1281 | 24 | 6 | 3 |
| 17 | 9311020 | 163 | 564 | 13 | 4 | 2 |
| 18 | 9311021 | 178 | 841 | 22 | 3 | 2 |
| 19 | 9311022 | 190 | 880 | 14 | 4 | 2 |
| 20 | 9311023 | 193 | 889 | 26 | 1 | 3 |
| 21 | 9311024 | 174 | 523 | 14 | 2 | 2 |
| 22 | 9311025 | 225 | 941 | 12 | 1 | 1 |
| 23 | 9311026 | 187 | 1019 | 28 | 0 | 2 |
| 24 | 9311027 | 226 | 1384 | 33 | 3 | 3 |
| 25 | 9311028 | 204 | 1017 | 20 | 2 | 3 |
| 26 | 9311029 | 185 | 856 | 14 | 4 | 2 |
| 27 | 9311030 | 189 | 713 | 23 | 4 | 3 |
| 28 | 9311031 | 185 | 750 | 24 | 1 | 2 |
| 29 | 9311032 | 233 | 939 | 30 | 7 | 3 |
| 30 | 9311033 | 276 | 1224 | 28 | 2 | 2 |
| 31 | 9311034 | 238 | 1546 | 24 | 3 | 1 |
| 32 | 9311035 | 222 | 1318 | 27 | 3 | 2 |
| 33 | 9311036 | 227 | 1514 | 13 | 3 | 2 |
| 34 | 9311037 | 203 | 865 | 10 | 3 | 3 |
| 35 | 9311038 | 210 | 932 | 20 | 3 | 4 |
| 36 | 9311039 | 198 | 1240 | 12 | 2 | 0 |
| 37 | 9311040 | 236 | 1200 | 23 | 2 | 2 |
| 38 | 9311041 | 240 | 1073 | 22 | 8 | 3 |
| 39 | 9311042 | 233 | 1196 | 19 | 8 | 4 |
| 40 | 9311043 | 229 | 1470 | 29 | 1 | 4 |
| 41 | 9311044 | 210 | 1008 | 24 | 2 | 2 |
| 42 | 9311045 | 269 | 1166 | 24 | 5 | 1 |
| 43 | 9311046 | 226 | 1298 | 31 | 2 | 3 |
| 44 | 9311047 | 184 | 493 | 12 | 3 | 3 |
| 45 | 9311048 | 216 | 1082 | 23 | 3 | 1 |
| 46 | 9311049 | 214 | 1302 | 29 | 2 | 2 |
| 47 | 9311050 | 216 | 1243 | 26 | 2 | 1 |
| 48 | 9311051 | 226 | 1238 | 35 | 1 | 3 |
| 49 | 9311052 | 237 | 1482 | 32 | 0 | 3 |
| 50 | 9311053 | 234 | 1188 | 21 | 1 | 1 |
| 51 | 9311054 | 236 | 1133 | 28 | 5 | 2 |
| 52 | 9311055 | 245 | 1169 | 23 | 6 | 3 |
| 53 | 9311056 | 232 | 1327 | 24 | 6 | 1 |
| 54 | 9311057 | 166 | 724 | 17 | 1 | 2 |
| 55 | 9311058 | 224 | 1464 | 20 | 4 | 2 |
| 56 | 9311059 | 273 | 1321 | 22 | 3 | 3 |
| 57 | 9311060 | 221 | 1112 | 28 | 3 | 2 |
| 58 | 9311061 | 221 | 1356 | 29 | 2 | 3 |
| 59 | 9311062 | 209 | 932 | 23 | 4 | 2 |
| 60 | 9311063 | 182 | 643 | 15 | 2 | 1 |
| 61 | 9311064 | 205 | 1051 | 23 | 3 | 4 |
| 62 | 9311065 | 224 | 1057 | 15 | 3 | 4 |
| 63 | 9311066 | 200 | 1064 | 27 | 0 | 4 |
| 64 | 9311067 | 245 | 1645 | 24 | 0 | 3 |
| 65 | 9311068 | 221 | 1197 | 12 | 2 | 3 |
| 66 | 9311069 | 287 | 1242 | 26 | 4 | 0 |
| 67 | 9311070 | 191 | 784 | 31 | 2 | 3 |
| 68 | 9311071 | 215 | 1161 | 22 | 1 | 2 |
| 69 | 9311072 | 226 | 1190 | 30 | 0 | 4 |
| 70 | 9311073 | 214 | 826 | 20 | 1 | 3 |
| 71 | 9311074 | 204 | 1052 | 19 | 3 | 1 |
| 72 | 9311075 | 212 | 1078 | 18 | 5 | 4 |
| 73 | 9311076 | 262 | 1586 | 22 | 0 | 0 |
| 74 | 9311077 | 211 | 1517 | 27 | 6 | 1 |
| 75 | 9311078 | 218 | 996 | 22 | 3 | 4 |
| 76 | 9311079 | 207 | 1083 | 20 | 0 | 2 |
| 77 | 9311080 | 254 | 1450 | 43 | 1 | 4 |
| 78 | 9311081 | 237 | 1426 | 19 | 0 | 1 |
| 79 | 9311082 | 218 | 1114 | 31 | 4 | 3 |
| 80 | 9311083 | 224 | 1190 | 29 | 5 | 3 |
| 81 | 9311084 | 246 | 1516 | 19 | 1 | 2 |
| 82 | 9311085 | 195 | 899 | 16 | 4 | 1 |
| 83 | 9311086 | 252 | 1398 | 11 | 4 | 4 |
| 84 | 9311087 | 194 | 1175 | 26 | 2 | 1 |
| 85 | 9311088 | 205 | 1038 | 27 | 0 | 2 |
| 86 | 9311089 | 234 | 1349 | 29 | 4 | 4 |
| 87 | 9311090 | 170 | 817 | 18 | 1 | 3 |
| 88 | 9311091 | 325 | 2527 | 24 | 3 | 1 |
| 89 | 9311092 | 199 | 787 | 9 | 2 | 3 |
| 90 | 9311093 | 153 | 422 | 13 | 3 | 2 |
| 91 | 9311094 | 209 | 1019 | 36 | 4 | 3 |
| 92 | 9311095 | 242 | 1364 | 15 | 3 | 5 |
| 93 | 9311096 | 204 | 1047 | 19 | 1 | 4 |
| 94 | 9311097 | 247 | 1338 | 29 | 5 | 4 |
| 95 | 9311098 | 174 | 832 | 17 | 1 | 1 |
| 96 | 9311099 | 234 | 1264 | 20 | 0 | 3 |
| 97 | 9311100 | 243 | 1487 | 19 | 7 | 2 |
| 98 | 9311101 | 215 | 1155 | 24 | 6 | 2 |
| 99 | 9311102 | 192 | 1120 | 28 | 2 | 1 |
| 100 | 9311103 | 221 | 1105 | 31 | 7 | 2 |
| 101 | 9311104 | 238 | 1222 | 20 | 6 | 0 |
| 102 | 9311105 | 202 | 966 | 19 | 4 | 1 |
| 103 | 9311106 | 218 | 1082 | 15 | 3 | 2 |
| 104 | 9311107 | 209 | 1313 | 25 | 2 | 2 |
| 105 | 9311108 | 157 | 613 | 11 | 3 | 1 |
| 106 | 9311109 | 222 | 1053 | 20 | 1 | 2 |
| 107 | 9311110 | 223 | 1036 | 24 | 8 | 2 |
| 108 | 9311111 | 242 | 1231 | 26 | 5 | 2 |
| 109 | 9311112 | 201 | 1078 | 22 | 4 | 3 |
| 110 | 9311113 | 244 | 964 | 7 | 1 | 1 |
| 111 | 9311114 | 245 | 1351 | 20 | 8 | 3 |
| 112 | 9311115 | 208 | 1032 | 24 | 0 | 3 |
| 113 | 9311116 | 267 | 1718 | 39 | 8 | 3 |
| 114 | 9311117 | 308 | 1475 | 29 | 4 | 2 |
| 115 | 9311118 | 171 | 992 | 21 | 1 | 2 |
| 116 | 9311119 | 192 | 880 | 20 | 5 | 2 |
| 117 | 9311120 | 210 | 1084 | 15 | 4 | 1 |
| 118 | 9311121 | 222 | 1127 | 26 | 4 | 2 |
| 119 | 9311122 | 231 | 1028 | 21 | 7 | 3 |
| 120 | 9311123 | 251 | 1807 | 18 | 1 | 4 |
| 121 | 9311124 | 143 | 495 | 12 | 3 | 2 |
| 122 | 9311125 | 195 | 951 | 8 | 5 | 1 |
| 123 | 9311126 | 227 | 1022 | 22 | 3 | 3 |
| 124 | 9311127 | 241 | 1391 | 18 | 1 | 1 |
| 125 | 9311128 | 188 | 811 | 21 | 1 | 0 |
| 126 | 9311129 | 175 | 798 | 14 | 3 | 2 |
| 127 | 9311130 | 264 | 1931 | 30 | 2 | 3 |
| 128 | 9311131 | 220 | 1249 | 23 | 0 | 3 |
| 129 | 9311132 | 185 | 793 | 17 | 4 | 1 |
| 130 | 9311133 | 258 | 1512 | 30 | 4 | 3 |
| 131 | 9311134 | 250 | 1401 | 27 | 5 | 3 |
| 132 | 9311135 | 185 | 864 | 17 | 5 | 3 |
| 133 | 9311136 | 222 | 1202 | 19 | 4 | 2 |
| 134 | 9311137 | 202 | 941 | 19 | 4 | 3 |
| 135 | 9311138 | 249 | 1219 | 15 | 6 | 4 |
| 136 | 9311139 | 159 | 560 | 10 | 1 | 3 |
| 137 | 9311140 | 157 | 629 | 13 | 0 | 1 |
| 138 | 9311141 | 230 | 1090 | 12 | 2 | 3 |
| 139 | 9311142 | 229 | 1043 | 27 | 3 | 2 |
| 140 | 9311143 | 268 | 1359 | 23 | 2 | 4 |
| 141 | 9311144 | 237 | 1225 | 24 | 4 | 3 |
| 142 | 9311145 | 212 | 838 | 5 | 6 | 1 |
| 143 | 9311146 | 192 | 787 | 11 | 6 | 2 |
| 144 | 9311147 | 263 | 1721 | 23 | 5 | 4 |
| 145 | 9311148 | 206 | 1130 | 15 | 2 | 2 |
| 146 | 9311149 | 221 | 957 | 20 | 2 | 3 |
| 147 | 9311150 | 242 | 1379 | 17 | 3 | 2 |
| 148 | 9311151 | 410 | 2927 | 42 | 3 | 3 |
| 149 | 9311152 | 198 | 998 | 26 | 3 | 3 |
| 150 | 9311153 | 281 | 1561 | 35 | 4 | 4 |
| 151 | 9311154 | 228 | 1503 | 38 | 2 | 3 |
| 152 | 9311155 | 186 | 1004 | 19 | 2 | 1 |
| 153 | 9311156 | 206 | 871 | 14 | 6 | 1 |
| 154 | 9311157 | 251 | 1389 | 28 | 4 | 3 |
| 155 | 9311158 | 207 | 1021 | 11 | 2 | 2 |
| 156 | 9311159 | 210 | 1309 | 19 | 1 | 1 |
| 157 | 9311160 | 235 | 1082 | 19 | 4 | 3 |
| 158 | 9311161 | 238 | 1531 | 25 | 7 | 3 |
| 159 | 9311162 | 225 | 913 | 25 | 2 | 1 |
| 160 | 9311163 | 235 | 1515 | 24 | 1 | 3 |
| 161 | 9311164 | 198 | 880 | 21 | 4 | 2 |
| 162 | 9311165 | 191 | 796 | 20 | 2 | 2 |
| 163 | 9311166 | 258 | 1365 | 17 | 2 | 3 |
| 164 | 9311167 | 222 | 1127 | 26 | 2 | 3 |
| 165 | 9311168 | 193 | 899 | 20 | 1 | 1 |
| 166 | 9311169 | 218 | 1058 | 20 | 2 | 2 |
| 167 | 9311170 | 264 | 1164 | 29 | 1 | 2 |
| 168 | 9311171 | 295 | 1617 | 34 | 3 | 3 |
| 169 | 9311172 | 278 | 1006 | 15 | 2 | 3 |
| 170 | 9311173 | 188 | 1221 | 23 | 3 | 1 |
| 171 | 9311174 | 214 | 970 | 16 | 0 | 2 |
| 172 | 9311175 | 171 | 835 | 19 | 9 | 1 |
| 173 | 9311176 | 178 | 733 | 13 | 1 | 2 |
| 174 | 9311177 | 270 | 1439 | 38 | 1 | 5 |
| 175 | 9311178 | 204 | 998 | 32 | 3 | 1 |
| 176 | 9311179 | 222 | 900 | 20 | 6 | 4 |
| 177 | 9311180 | 201 | 799 | 18 | 5 | 1 |
| 178 | 9311181 | 168 | 608 | 12 | 2 | 1 |
| 179 | 9311182 | 208 | 1143 | 28 | 2 | 2 |
| 180 | 9311183 | 201 | 1016 | 19 | 1 | 1 |
| 181 | 9311184 | 177 | 819 | 17 | 1 | 1 |
| 182 | 9311185 | 248 | 1206 | 21 | 3 | 3 |
| 183 | 9311186 | 186 | 933 | 17 | 3 | 2 |
| 184 | 9311187 | 347 | 1970 | 41 | 2 | 4 |
| 185 | 9311188 | 220 | 1071 | 18 | 3 | 3 |
| 186 | 9311189 | 194 | 998 | 19 | 2 | 2 |
| 187 | 9311190 | 156 | 730 | 15 | 3 | 0 |
| 188 | 9311191 | 285 | 1743 | 39 | 5 | 1 |
| 189 | 9311192 | 196 | 922 | 22 | 4 | 1 |
| 190 | 9311193 | 242 | 1289 | 29 | 2 | 3 |
| 191 | 9311194 | 215 | 1014 | 23 | 0 | 0 |
| 192 | 9311195 | 236 | 1609 | 27 | 2 | 3 |
| 193 | 9311196 | 278 | 2309 | 22 | 4 | 3 |
| 194 | 9311197 | 264 | 1428 | 39 | 7 | 4 |
| 195 | 9311198 | 225 | 1279 | 27 | 0 | 2 |
| 196 | 9311199 | 208 | 1308 | 25 | 0 | 3 |
| 197 | 9311200 | 187 | 998 | 16 | 1 | 2 |
| 198 | 9311201 | 236 | 1463 | 31 | 5 | 4 |
| 199 | 9311202 | 196 | 832 | 16 | 2 | 3 |
| 200 | 9311203 | 247 | 1487 | 32 | 5 | 1 |
| 201 | 9311204 | 299 | 1741 | 23 | 4 | 2 |
| 202 | 9311205 | 257 | 1482 | 44 | 0 | 2 |
| 203 | 9311206 | 392 | 2559 | 29 | 0 | 3 |
| 204 | 9311207 | 269 | 1221 | 14 | 1 | 1 |
| 205 | 9311208 | 281 | 1853 | 18 | 1 | 3 |
| 206 | 9311209 | 169 | 752 | 19 | 1 | 4 |
| 207 | 9311210 | 234 | 1245 | 34 | 3 | 3 |
| 208 | 9311211 | 222 | 1244 | 17 | 2 | 2 |
| 209 | 9311212 | 267 | 1061 | 11 | 2 | 2 |
| 210 | 9311213 | 197 | 936 | 23 | 0 | 2 |
| 211 | 9311214 | 247 | 1624 | 28 | 2 | 2 |
| 212 | 9311215 | 306 | 1646 | 46 | 3 | 5 |
| 213 | 9311216 | 252 | 1462 | 32 | 10 | 3 |
| 214 | 9311217 | 239 | 1123 | 25 | 2 | 4 |
| 215 | 9311218 | 335 | 2031 | 45 | 7 | 4 |
| 216 | 9311219 | 241 | 1001 | 19 | 2 | 4 |
| 217 | 9311220 | 217 | 1354 | 19 | 1 | 0 |
| 218 | 9311221 | 211 | 997 | 15 | 2 | 2 |
| 219 | 9311222 | 168 | 862 | 12 | 2 | 1 |
| 220 | 9311223 | 260 | 1467 | 18 | 2 | 2 |
| 221 | 9311224 | 234 | 1134 | 30 | 6 | 3 |
| 222 | 9311225 | 190 | 1019 | 19 | 2 | 1 |
| 223 | 9311226 | 186 | 778 | 18 | 2 | 1 |
| 224 | 9311227 | 237 | 1455 | 19 | 2 | 4 |
| 225 | 9311228 | 247 | 1234 | 33 | 0 | 3 |
| 226 | 9311229 | 214 | 930 | 24 | 2 | 1 |
| 227 | 9311230 | 187 | 878 | 19 | 5 | 2 |
| 228 | 9311231 | 230 | 1157 | 25 | 2 | 2 |
| 229 | 9311232 | 212 | 1111 | 12 | 3 | 4 |
| 230 | 9311233 | 189 | 1152 | 16 | 2 | 3 |
| 231 | 9311234 | 284 | 1467 | 54 | 1 | 4 |
| 232 | 9311235 | 210 | 1379 | 24 | 1 | 3 |
| 233 | 9311236 | 262 | 1375 | 24 | 6 | 3 |
| 234 | 9311237 | 259 | 1704 | 20 | 7 | 5 |
| 235 | 9311238 | 210 | 948 | 34 | 4 | 2 |
| 236 | 9311239 | 195 | 826 | 10 | 2 | 1 |
| 237 | 9311240 | 189 | 854 | 18 | 4 | 2 |
| 238 | 9311241 | 238 | 1241 | 21 | 8 | 2 |
| 239 | 9311242 | 206 | 1002 | 25 | 3 | 4 |
| 240 | 9311243 | 268 | 1490 | 23 | 3 | 3 |
| 241 | 9311244 | 206 | 908 | 25 | 1 | 3 |
| 242 | 9311245 | 241 | 1165 | 29 | 2 | 2 |
| 243 | 9311246 | 249 | 1351 | 39 | 3 | 4 |
| 244 | 9311247 | 222 | 1126 | 20 | 3 | 6 |
| 245 | 9311248 | 243 | 856 | 13 | 5 | 2 |
| 246 | 9311249 | 180 | 799 | 17 | 3 | 2 |
| 247 | 9311250 | 218 | 851 | 24 | 1 | 4 |
| 248 | 9311251 | 197 | 899 | 19 | 3 | 5 |
| 249 | 9311252 | 215 | 1050 | 24 | 3 | 2 |
| 250 | 9311253 | 187 | 914 | 20 | 0 | 1 |
| 251 | 9311254 | 240 | 1413 | 40 | 2 | 4 |
| 252 | 9311255 | 239 | 1155 | 23 | 3 | 4 |
| 253 | 9311256 | 239 | 1201 | 29 | 6 | 2 |
| 254 | 9311257 | 161 | 586 | 18 | 2 | 4 |
| 255 | 9311258 | 224 | 1157 | 20 | 8 | 2 |
| 256 | 9311259 | 241 | 1485 | 11 | 3 | 2 |
| 257 | 9311260 | 241 | 1479 | 29 | 0 | 0 |
| 258 | 9311261 | 237 | 1381 | 43 | 2 | 4 |
| 259 | 9311262 | 208 | 1213 | 22 | 3 | 2 |
| 260 | 9311263 | 186 | 715 | 16 | 3 | 2 |
| 261 | 9311264 | 175 | 916 | 14 | 1 | 2 |
| 262 | 9311265 | 241 | 1409 | 21 | 1 | 5 |
| 263 | 9311266 | 255 | 1052 | 26 | 3 | 2 |
| 264 | 9311267 | 269 | 1589 | 21 | 2 | 4 |
| 265 | 9311268 | 231 | 731 | 10 | 1 | 3 |
| 266 | 9311269 | 201 | 861 | 12 | 1 | 3 |
| 267 | 9311270 | 229 | 1199 | 20 | 2 | 2 |
| 268 | 9311271 | 281 | 1506 | 28 | 3 | 3 |
| 269 | 9311272 | 183 | 510 | 12 | 0 | 3 |
| 270 | 9311273 | 192 | 935 | 21 | 4 | 3 |
| 271 | 9311274 | 212 | 1034 | 18 | 6 | 3 |
| 272 | 9311275 | 186 | 802 | 14 | 3 | 1 |
| 273 | 9311276 | 200 | 740 | 23 | 3 | 3 |
| 274 | 9311277 | 218 | 1267 | 24 | 3 | 4 |
| 275 | 9311278 | 179 | 735 | 14 | 4 | 2 |
| 276 | 9311279 | 256 | 1403 | 21 | 1 | 3 |
| 277 | 9311280 | 200 | 744 | 17 | 5 | 2 |
| 278 | 9311281 | 224 | 1206 | 26 | 0 | 2 |
| 279 | 9311282 | 247 | 1511 | 26 | 0 | 2 |
| 280 | 9311283 | 193 | 1162 | 35 | 3 | 4 |
| 281 | 9311284 | 203 | 765 | 11 | 4 | 3 |
| 282 | 9311285 | 227 | 1298 | 32 | 1 | 2 |
| 283 | 9311286 | 222 | 856 | 13 | 2 | 3 |
| 284 | 9311287 | 184 | 963 | 20 | 2 | 1 |
| 285 | 9311288 | 233 | 1286 | 24 | 5 | 2 |
| 286 | 9311289 | 207 | 1042 | 13 | 6 | 4 |
| 287 | 9311290 | 195 | 885 | 15 | 5 | 1 |
| 288 | 9311291 | 182 | 686 | 21 | 0 | 2 |
| 289 | 9311292 | 278 | 1705 | 27 | 2 | 2 |
| 290 | 9311293 | 270 | 1736 | 20 | 10 | 2 |
| 291 | 9311294 | 247 | 1242 | 18 | 1 | 2 |
| 292 | 9311295 | 212 | 920 | 13 | 1 | 3 |
| 293 | 9311296 | 226 | 856 | 15 | 1 | 3 |
| 294 | 9311297 | 272 | 1855 | 27 | 3 | 1 |
| 295 | 9311298 | 192 | 922 | 19 | 2 | 2 |
| 296 | 9311299 | 247 | 1325 | 11 | 5 | 1 |
| 297 | 9311300 | 222 | 1076 | 25 | 5 | 0 |
| 298 | 9311301 | 251 | 1388 | 22 | 2 | 0 |
| 299 | 9311302 | 198 | 972 | 24 | 2 | 2 |
| 300 | 9311303 | 235 | 961 | 11 | 5 | 2 |
| 301 | 9311304 | 258 | 1362 | 30 | 5 | 1 |
| 302 | 9311305 | 194 | 909 | 19 | 4 | 2 |
| 303 | 9311306 | 265 | 1356 | 24 | 2 | 4 |
| 304 | 9311307 | 210 | 951 | 20 | 1 | 2 |
| 305 | 9311308 | 242 | 1337 | 24 | 4 | 3 |
| 306 | 9311309 | 237 | 1163 | 32 | 5 | 1 |
| 307 | 9311310 | 227 | 1453 | 21 | 1 | 3 |
| 308 | 9311311 | 211 | 994 | 15 | 6 | 4 |
| 309 | 9311312 | 181 | 894 | 14 | 1 | 2 |
| 310 | 9311313 | 251 | 1512 | 24 | 1 | 2 |
| 311 | 9311314 | 283 | 1605 | 23 | 7 | 3 |
| 312 | 9311315 | 200 | 1082 | 17 | 2 | 2 |
| 313 | 9311316 | 237 | 1262 | 23 | 6 | 1 |
| 314 | 9311317 | 253 | 1272 | 17 | 6 | 3 |
| 315 | 9311318 | 205 | 1134 | 20 | 1 | 0 |
| 316 | 9311319 | 211 | 1045 | 10 | 3 | 3 |
| 317 | 9311320 | 246 | 1248 | 22 | 2 | 3 |
| 318 | 9311321 | 243 | 1420 | 35 | 2 | 4 |
| 319 | 9311322 | 237 | 1223 | 9 | 3 | 4 |
| 320 | 9311323 | 205 | 950 | 16 | 5 | 1 |
| 321 | 9311324 | 183 | 735 | 7 | 2 | 2 |
| 322 | 9311325 | 266 | 1627 | 29 | 1 | 2 |
| 323 | 9311326 | 211 | 1195 | 25 | 0 | 3 |
| 324 | 9311327 | 252 | 1316 | 29 | 0 | 2 |
| 325 | 9311328 | 225 | 1246 | 24 | 3 | 3 |
| 326 | 9311329 | 226 | 1169 | 26 | 2 | 3 |
| 327 | 9311330 | 195 | 864 | 13 | 1 | 3 |
| 328 | 9311331 | 286 | 1526 | 25 | 1 | 2 |
| 329 | 9311332 | 177 | 890 | 15 | 1 | 5 |
| 330 | 9311333 | 153 | 609 | 17 | 1 | 2 |
| 331 | 9311334 | 248 | 1294 | 21 | 8 | 4 |
| 332 | 9311335 | 225 | 1328 | 25 | 4 | 1 |
| 333 | 9311336 | 226 | 1089 | 18 | 7 | 3 |
| 334 | 9311337 | 204 | 1148 | 21 | 3 | 4 |
| 335 | 9311338 | 194 | 991 | 21 | 9 | 2 |
| 336 | 9311339 | 300 | 1757 | 34 | 4 | 3 |
| 337 | 9311340 | 272 | 1732 | 23 | 2 | 1 |
| 338 | 9311341 | 227 | 1406 | 17 | 8 | 2 |
| 339 | 9311342 | 270 | 1460 | 18 | 5 | 1 |
| 340 | 9311343 | 228 | 912 | 21 | 1 | 2 |
| 341 | 9311344 | 202 | 1014 | 21 | 4 | 2 |
| 342 | 9311345 | 200 | 1116 | 20 | 1 | 4 |
| 343 | 9311346 | 237 | 1059 | 11 | 2 | 2 |
| 344 | 9311347 | 207 | 1245 | 27 | 4 | 2 |
| 345 | 9311348 | 187 | 880 | 16 | 0 | 2 |
| 346 | 9311349 | 213 | 1060 | 9 | 3 | 1 |
| 347 | 9311350 | 214 | 947 | 15 | 1 | 2 |
| 348 | 9311351 | 181 | 678 | 18 | 3 | 2 |
| 349 | 9311352 | 226 | 1062 | 21 | 4 | 2 |
| 350 | 9311353 | 210 | 1010 | 18 | 4 | 4 |
| 351 | 9311354 | 163 | 569 | 17 | 2 | 2 |
| 352 | 9311355 | 197 | 1018 | 26 | 0 | 2 |
| 353 | 9311356 | 259 | 1462 | 39 | 2 | 1 |
| 354 | 9311357 | 219 | 1357 | 33 | 2 | 4 |
| 355 | 9311358 | 281 | 1553 | 14 | 5 | 4 |
| 356 | 9311359 | 187 | 998 | 14 | 6 | 1 |
| 357 | 9311360 | 191 | 992 | 19 | 0 | 2 |
| 358 | 9311361 | 211 | 999 | 26 | 6 | 3 |
| 359 | 9311362 | 216 | 1174 | 22 | 3 | 3 |
| 360 | 9311363 | 178 | 826 | 19 | 4 | 2 |
| 361 | 9311364 | 343 | 2682 | 29 | 1 | 2 |
| 362 | 9311365 | 194 | 841 | 24 | 1 | 3 |
| 363 | 9311366 | 200 | 805 | 16 | 2 | 4 |
| 364 | 9311367 | 203 | 927 | 15 | 4 | 0 |
| 365 | 9311368 | 213 | 1124 | 30 | 4 | 1 |
| 366 | 9311369 | 204 | 1124 | 27 | 2 | 3 |
| 367 | 9311370 | 290 | 1398 | 28 | 7 | 4 |
| 368 | 9311371 | 194 | 987 | 10 | 6 | 2 |
| 369 | 9311372 | 217 | 1094 | 31 | 0 | 4 |
| 370 | 9311373 | 325 | 1848 | 34 | 1 | 1 |
| 371 | 9311374 | 198 | 790 | 18 | 1 | 3 |
| 372 | 9311375 | 223 | 971 | 11 | 2 | 1 |
| 373 | 9311376 | 307 | 1871 | 40 | 7 | 3 |
| 374 | 9311377 | 257 | 1595 | 25 | 2 | 1 |
| 375 | 9311378 | 186 | 807 | 21 | 3 | 3 |
| 376 | 9311379 | 184 | 953 | 12 | 4 | 2 |
| 377 | 9311380 | 227 | 1291 | 22 | 4 | 1 |
| 378 | 9311381 | 248 | 1377 | 24 | 7 | 1 |
| 379 | 9311382 | 222 | 927 | 13 | 4 | 3 |
| 380 | 9311383 | 268 | 1686 | 21 | 4 | 4 |
| 381 | 9311384 | 206 | 1176 | 21 | 5 | 2 |
| 382 | 9311385 | 191 | 628 | 7 | 1 | 1 |
| 383 | 9311386 | 203 | 1127 | 22 | 4 | 2 |
| 384 | 9311387 | 212 | 1137 | 25 | 5 | 3 |
| 385 | 9311388 | 171 | 755 | 11 | 2 | 0 |
| 386 | 9311389 | 243 | 1314 | 11 | 1 | 3 |
| 387 | 9311390 | 259 | 1358 | 27 | 2 | 3 |
| 388 | 9311391 | 224 | 1252 | 23 | 2 | 3 |
| 389 | 9311392 | 212 | 1093 | 17 | 1 | 1 |
| 390 | 9311393 | 210 | 1132 | 19 | 1 | 2 |
| 391 | 9311394 | 227 | 1224 | 22 | 2 | 2 |
| 392 | 9311395 | 233 | 1102 | 27 | 2 | 4 |
| 393 | 9311396 | 255 | 1221 | 13 | 2 | 1 |
| 394 | 9311397 | 205 | 915 | 17 | 1 | 1 |
| 395 | 9311398 | 183 | 816 | 19 | 2 | 2 |
| 396 | 9311399 | 206 | 967 | 33 | 5 | 5 |
| 397 | 9311400 | 206 | 1050 | 13 | 7 | 1 |
| 398 | 9311401 | 251 | 1633 | 13 | 2 | 4 |
| 399 | 9311402 | 229 | 1277 | 19 | 2 | 4 |
| 400 | 9311403 | 230 | 1086 | 24 | 2 | 0 |
| 401 | 9311404 | 241 | 1441 | 21 | 1 | 3 |
| 402 | 9311405 | 226 | 1057 | 24 | 3 | 4 |
| 403 | 9311406 | 216 | 946 | 24 | 7 | 5 |
| 404 | 9311407 | 229 | 1168 | 30 | 1 | 2 |
| 405 | 9311408 | 247 | 1585 | 13 | 1 | 1 |
| 406 | 9311409 | 262 | 1168 | 29 | 7 | 3 |
| 407 | 9311410 | 185 | 892 | 12 | 2 | 1 |
| 408 | 9311411 | 284 | 1671 | 27 | 0 | 3 |
| 409 | 9311412 | 185 | 943 | 16 | 2 | 1 |
| 410 | 9311413 | 216 | 1145 | 24 | 5 | 1 |
| 411 | 9311414 | 173 | 823 | 18 | 3 | 2 |
| 412 | 9311415 | 248 | 1335 | 27 | 0 | 3 |
| 413 | 9311416 | 153 | 698 | 14 | 0 | 2 |
| 414 | 9311417 | 223 | 1121 | 28 | 3 | 3 |
| 415 | 9311418 | 228 | 1397 | 25 | 3 | 3 |
| 416 | 9311419 | 179 | 650 | 19 | 1 | 2 |
| 417 | 9311420 | 262 | 1407 | 31 | 5 | 2 |
| 418 | 9311421 | 315 | 1994 | 23 | 4 | 4 |
| 419 | 9311422 | 202 | 1018 | 33 | 1 | 2 |
| 420 | 9311423 | 180 | 1030 | 23 | 2 | 4 |
| 421 | 9311424 | 210 | 847 | 15 | 4 | 2 |
| 422 | 9311425 | 244 | 1354 | 13 | 2 | 2 |
| 423 | 9311426 | 191 | 911 | 17 | 1 | 1 |
| 424 | 9311427 | 226 | 1035 | 16 | 0 | 4 |
| 425 | 9311428 | 218 | 1066 | 21 | 2 | 2 |
| 426 | 9311429 | 226 | 1238 | 12 | 1 | 3 |
| 427 | 9311430 | 226 | 953 | 16 | 5 | 2 |
| 428 | 9311431 | 207 | 876 | 16 | 2 | 4 |
| 429 | 9311432 | 227 | 1406 | 11 | 1 | 2 |
| 430 | 9311433 | 222 | 1463 | 31 | 3 | 3 |
| 431 | 9311434 | 270 | 1431 | 35 | 2 | 2 |
| 432 | 9311435 | 241 | 1314 | 34 | 3 | 1 |
| 433 | 9311436 | 208 | 982 | 17 | 7 | 1 |
| 434 | 9311437 | 189 | 886 | 14 | 8 | 3 |
| 435 | 9311438 | 250 | 1335 | 35 | 7 | 1 |
| 436 | 9311439 | 253 | 1994 | 24 | 4 | 1 |
| 437 | 9311440 | 177 | 790 | 8 | 2 | 1 |
| 438 | 9311441 | 195 | 775 | 14 | 1 | 3 |
| 439 | 9311442 | 220 | 929 | 17 | 3 | 2 |
| 440 | 9311443 | 281 | 1645 | 38 | 6 | 2 |
| 441 | 9311444 | 284 | 1795 | 17 | 2 | 2 |
| 442 | 9311445 | 256 | 1815 | 27 | 3 | 1 |
| 443 | 9311446 | 204 | 1006 | 18 | 2 | 3 |
| 444 | 9311447 | 231 | 1197 | 28 | 2 | 4 |
| 445 | 9311448 | 238 | 1236 | 16 | 2 | 1 |
| 446 | 9311449 | 205 | 1083 | 24 | 5 | 1 |
| 447 | 9311450 | 182 | 812 | 22 | 4 | 2 |
| 448 | 9311451 | 204 | 713 | 16 | 1 | 1 |
| 449 | 9311452 | 208 | 1258 | 15 | 1 | 2 |
| 450 | 9311453 | 219 | 1227 | 14 | 4 | 2 |
| 451 | 9311454 | 231 | 1356 | 25 | 5 | 0 |
| 452 | 9311455 | 247 | 1274 | 26 | 0 | 2 |
| 453 | 9311456 | 272 | 1632 | 28 | 0 | 3 |
| 454 | 9311457 | 186 | 982 | 21 | 2 | 1 |
| 455 | 9311458 | 201 | 1008 | 14 | 6 | 2 |
| 456 | 9311459 | 194 | 849 | 14 | 3 | 2 |
| 457 | 9311460 | 280 | 1356 | 29 | 3 | 3 |
| 458 | 9311461 | 192 | 1063 | 20 | 2 | 3 |
| 459 | 9311462 | 227 | 1379 | 16 | 2 | 2 |
| 460 | 9311463 | 180 | 1025 | 14 | 4 | 1 |
| 461 | 9311464 | 313 | 1702 | 30 | 2 | 4 |
| 462 | 9311465 | 203 | 1031 | 22 | 1 | 2 |
| 463 | 9311466 | 217 | 1260 | 23 | 1 | 2 |
| 464 | 9311467 | 190 | 813 | 14 | 2 | 2 |
| 465 | 9311468 | 265 | 1698 | 38 | 4 | 1 |
| 466 | 9311469 | 179 | 847 | 18 | 3 | 3 |
| 467 | 9311470 | 253 | 1178 | 27 | 3 | 4 |
| 468 | 9311471 | 243 | 1347 | 35 | 4 | 2 |
| 469 | 9311472 | 261 | 1735 | 23 | 1 | 1 |
| 470 | 9311473 | 219 | 992 | 20 | 2 | 0 |
| 471 | 9311474 | 315 | 1982 | 53 | 2 | 2 |
| 472 | 9311475 | 244 | 1616 | 24 | 5 | 3 |
| 473 | 9311476 | 231 | 1116 | 23 | 3 | 5 |
| 474 | 9311477 | 189 | 859 | 17 | 3 | 3 |
| 475 | 9311478 | 212 | 1108 | 32 | 3 | 4 |
| 476 | 9311479 | 232 | 1180 | 34 | 2 | 3 |
| 477 | 9311480 | 232 | 1118 | 40 | 3 | 2 |
| 478 | 9311481 | 232 | 1153 | 22 | 1 | 2 |
| 479 | 9311482 | 222 | 1380 | 24 | 1 | 3 |
| 480 | 9311483 | 256 | 1656 | 37 | 1 | 2 |
| 481 | 9311484 | 195 | 821 | 13 | 4 | 2 |
| 482 | 9311485 | 242 | 1315 | 22 | 4 | 3 |
| 483 | 9311486 | 213 | 949 | 22 | 1 | 1 |
| 484 | 9311487 | 212 | 802 | 19 | 3 | 2 |
| 485 | 9311488 | 301 | 1857 | 30 | 4 | 2 |
| 486 | 9311489 | 188 | 1023 | 23 | 2 | 3 |
| 487 | 9311490 | 187 | 852 | 15 | 1 | 2 |
| 488 | 9311491 | 255 | 1658 | 20 | 1 | 4 |
| 489 | 9311492 | 264 | 1413 | 31 | 2 | 3 |
| 490 | 9311493 | 238 | 1344 | 17 | 3 | 3 |
| 491 | 9311494 | 198 | 929 | 25 | 4 | 2 |
| 492 | 9311495 | 230 | 1130 | 25 | 5 | 4 |
| 493 | 9311496 | 219 | 960 | 22 | 5 | 4 |
| 494 | 9311497 | 268 | 2098 | 27 | 4 | 2 |
| 495 | 9311498 | 198 | 1036 | 26 | 4 | 3 |
| 496 | 9311499 | 235 | 1188 | 15 | 2 | 1 |
| 497 | 9311500 | 224 | 1312 | 30 | 0 | 1 |
| 498 | 9311501 | 224 | 1032 | 14 | 3 | 2 |
| 499 | 9311502 | 170 | 781 | 11 | 1 | 2 |
| 500 | 9311503 | 215 | 1129 | 17 | 5 | 3 |
| 501 | 9311504 | 236 | 1219 | 13 | 2 | 2 |
| 502 | 9311505 | 233 | 1278 | 37 | 4 | 2 |
| 503 | 9311506 | 164 | 621 | 12 | 0 | 3 |
| 504 | 9311507 | 190 | 861 | 22 | 1 | 3 |
| 505 | 9311508 | 214 | 1041 | 25 | 3 | 2 |
| 506 | 9311509 | 253 | 1382 | 14 | 5 | 3 |
| 507 | 9311510 | 207 | 1099 | 22 | 4 | 3 |
| 508 | 9311511 | 233 | 1086 | 18 | 2 | 2 |
| 509 | 9311512 | 315 | 1823 | 28 | 3 | 2 |
| 510 | 9311513 | 245 | 1394 | 17 | 1 | 1 |
| 511 | 9311514 | 203 | 1028 | 26 | 3 | 1 |
| 512 | 9311515 | 200 | 924 | 16 | 5 | 3 |
| 513 | 9311516 | 193 | 917 | 32 | 3 | 4 |
| 514 | 9311517 | 189 | 778 | 6 | 2 | 2 |
| 515 | 9311518 | 206 | 1106 | 17 | 0 | 0 |
| 516 | 9311519 | 235 | 1002 | 17 | 5 | 4 |
| 517 | 9311520 | 234 | 1206 | 20 | 5 | 3 |
| 518 | 9311521 | 251 | 944 | 14 | 5 | 3 |
| 519 | 9311522 | 239 | 1296 | 21 | 0 | 4 |
| 520 | 9311523 | 190 | 824 | 18 | 0 | 1 |
| 521 | 9311524 | 224 | 1127 | 24 | 0 | 2 |
| 522 | 9311525 | 228 | 1059 | 24 | 3 | 3 |
| 523 | 9311526 | 188 | 828 | 18 | 9 | 3 |
| 524 | 9311527 | 316 | 1850 | 24 | 10 | 3 |
| 525 | 9311528 | 189 | 974 | 22 | 1 | 3 |
| 526 | 9311529 | 225 | 1280 | 30 | 1 | 2 |
| 527 | 9311530 | 261 | 1512 | 29 | 2 | 3 |
| 528 | 9311531 | 239 | 1734 | 21 | 1 | 1 |
| 529 | 9311532 | 226 | 1444 | 31 | 0 | 1 |
| 530 | 9311533 | 258 | 1738 | 30 | 1 | 2 |
| 531 | 9311534 | 285 | 1539 | 29 | 0 | 3 |
| 532 | 9311535 | 247 | 1570 | 18 | 2 | 4 |
| 533 | 9311536 | 269 | 1747 | 26 | 3 | 2 |
| 534 | 9311537 | 255 | 1167 | 16 | 4 | 4 |
| 535 | 9311538 | 240 | 1201 | 21 | 2 | 2 |
| 536 | 9311539 | 153 | 612 | 16 | 3 | 1 |
| 537 | 9311540 | 228 | 1414 | 33 | 1 | 0 |
| 538 | 9311541 | 187 | 924 | 15 | 4 | 2 |
| 539 | 9311542 | 192 | 1021 | 22 | 7 | 1 |
| 540 | 9311543 | 213 | 1078 | 14 | 6 | 2 |
| 541 | 9311544 | 281 | 2368 | 35 | 1 | 2 |
| 542 | 9311545 | 209 | 1028 | 19 | 1 | 2 |
| 543 | 9311546 | 257 | 1651 | 25 | 2 | 2 |
| 544 | 9311547 | 213 | 849 | 17 | 6 | 2 |
| 545 | 9311548 | 208 | 1017 | 19 | 3 | 2 |
| 546 | 9311549 | 230 | 1123 | 26 | 4 | 2 |
| 547 | 9311550 | 272 | 1726 | 34 | 2 | 4 |
| 548 | 9311551 | 278 | 1375 | 21 | 3 | 3 |
| 549 | 9311552 | 202 | 1255 | 15 | 3 | 2 |
| 550 | 9311553 | 157 | 791 | 21 | 1 | 1 |
| 551 | 9311554 | 223 | 1267 | 22 | 1 | 2 |
| 552 | 9311555 | 188 | 661 | 15 | 1 | 2 |
| 553 | 9311556 | 209 | 1162 | 23 | 5 | 2 |
| 554 | 9311557 | 197 | 905 | 26 | 3 | 5 |
| 555 | 9311558 | 197 | 1084 | 26 | 1 | 4 |
| 556 | 9311559 | 347 | 2124 | 33 | 5 | 2 |
| 557 | 9311560 | 222 | 1107 | 13 | 4 | 3 |
| 558 | 9311561 | 251 | 1515 | 28 | 7 | 3 |
| 559 | 9311562 | 204 | 1074 | 17 | 2 | 3 |
| 560 | 9311563 | 247 | 951 | 25 | 4 | 5 |
| 561 | 9311564 | 254 | 1457 | 19 | 5 | 3 |
| 562 | 9311565 | 200 | 632 | 21 | 1 | 3 |
| 563 | 9311566 | 229 | 1152 | 14 | 4 | 0 |
| 564 | 9311567 | 163 | 952 | 16 | 1 | 2 |
| 565 | 9311568 | 178 | 675 | 17 | 0 | 5 |
| 566 | 9311569 | 237 | 1282 | 38 | 1 | 2 |
| 567 | 9311570 | 210 | 978 | 16 | 5 | 3 |
| 568 | 9311571 | 219 | 1156 | 16 | 1 | 2 |
| 569 | 9311572 | 217 | 1185 | 17 | 3 | 2 |
| 570 | 9311573 | 283 | 1481 | 25 | 2 | 2 |
| 571 | 9311574 | 249 | 1410 | 29 | 10 | 4 |
| 572 | 9311575 | 221 | 1444 | 30 | 1 | 1 |
| 573 | 9311576 | 199 | 995 | 19 | 2 | 2 |
| 574 | 9311577 | 262 | 1391 | 28 | 2 | 4 |
| 575 | 9311578 | 199 | 913 | 19 | 2 | 3 |
| 576 | 9311579 | 218 | 1111 | 18 | 2 | 4 |
| 577 | 9311580 | 217 | 1090 | 16 | 5 | 2 |
| 578 | 9311581 | 251 | 1304 | 22 | 8 | 4 |
| 579 | 9311582 | 224 | 1298 | 18 | 4 | 1 |
| 580 | 9311583 | 187 | 891 | 13 | 0 | 1 |
| 581 | 9311584 | 211 | 1010 | 19 | 4 | 4 |
| 582 | 9311585 | 239 | 1521 | 27 | 4 | 2 |
| 583 | 9311586 | 191 | 984 | 19 | 7 | 4 |
| 584 | 9311587 | 183 | 934 | 14 | 3 | 1 |
| 585 | 9311588 | 268 | 1560 | 40 | 4 | 4 |
| 586 | 9311589 | 222 | 1576 | 30 | 3 | 5 |
| 587 | 9311590 | 200 | 913 | 14 | 2 | 2 |
| 588 | 9311591 | 203 | 944 | 13 | 4 | 4 |
| 589 | 9311592 | 368 | 2278 | 31 | 1 | 1 |
| 590 | 9311593 | 218 | 1168 | 22 | 1 | 2 |
| 591 | 9311594 | 219 | 1178 | 23 | 1 | 2 |
| 592 | 9311595 | 172 | 796 | 17 | 2 | 2 |
| 593 | 9311596 | 197 | 1097 | 20 | 0 | 2 |
| 594 | 9311597 | 205 | 781 | 17 | 2 | 3 |
| 595 | 9311598 | 202 | 970 | 20 | 3 | 2 |
| 596 | 9311599 | 245 | 1372 | 27 | 4 | 4 |
| 597 | 9311600 | 264 | 1170 | 25 | 5 | 3 |
| 598 | 9311601 | 184 | 839 | 14 | 1 | 2 |
| 599 | 9311602 | 218 | 1155 | 13 | 0 | 2 |
| 600 | 9311603 | 237 | 1075 | 21 | 7 | 5 |
| 601 | 9311604 | 233 | 1278 | 36 | 5 | 1 |
| 602 | 9311605 | 226 | 920 | 10 | 4 | 1 |
| 603 | 9311606 | 258 | 1475 | 30 | 3 | 2 |
| 604 | 9311607 | 233 | 1512 | 29 | 2 | 4 |
| 605 | 9311608 | 234 | 1302 | 24 | 4 | 3 |
| 606 | 9311609 | 209 | 974 | 18 | 3 | 2 |
| 607 | 9311610 | 299 | 1593 | 26 | 3 | 4 |
| 608 | 9311611 | 243 | 1509 | 32 | 2 | 3 |
| 609 | 9311612 | 243 | 1369 | 14 | 2 | 2 |
| 610 | 9311613 | 214 | 1024 | 16 | 5 | 3 |
| 611 | 9311614 | 183 | 985 | 15 | 3 | 0 |
| 612 | 9311615 | 219 | 931 | 22 | 2 | 3 |
| 613 | 9311616 | 201 | 1029 | 19 | 4 | 3 |
| 614 | 9311617 | 208 | 1063 | 26 | 1 | 3 |
| 615 | 9311618 | 213 | 822 | 12 | 4 | 2 |
| 616 | 9311619 | 259 | 1366 | 32 | 1 | 4 |
| 617 | 9311620 | 216 | 922 | 19 | 1 | 1 |
| 618 | 9311621 | 250 | 1585 | 29 | 2 | 2 |
| 619 | 9311622 | 157 | 738 | 20 | 3 | 2 |
| 620 | 9311623 | 229 | 1496 | 23 | 3 | 3 |
| 621 | 9311624 | 197 | 1102 | 35 | 2 | 2 |
| 622 | 9311625 | 200 | 931 | 24 | 5 | 3 |
| 623 | 9311626 | 225 | 981 | 15 | 14 | 1 |
| 624 | 9311627 | 202 | 952 | 15 | 3 | 2 |
| 625 | 9311628 | 320 | 1533 | 16 | 5 | 3 |
| 626 | 9311629 | 183 | 804 | 19 | 0 | 3 |
| 627 | 9311630 | 219 | 1104 | 36 | 2 | 3 |
| 628 | 9311631 | 229 | 1273 | 16 | 2 | 0 |
| 629 | 9311632 | 182 | 724 | 18 | 2 | 2 |
| 630 | 9311633 | 213 | 1026 | 22 | 0 | 2 |
| 631 | 9311634 | 274 | 1744 | 18 | 8 | 1 |
| 632 | 9311635 | 177 | 952 | 8 | 2 | 1 |
| 633 | 9311636 | 228 | 1229 | 18 | 3 | 4 |
| 634 | 9311637 | 194 | 1121 | 29 | 2 | 4 |
| 635 | 9311638 | 254 | 1437 | 20 | 2 | 3 |
| 636 | 9311639 | 289 | 1668 | 18 | 8 | 2 |
| 637 | 9311640 | 220 | 1157 | 21 | 3 | 5 |
| 638 | 9311641 | 212 | 1326 | 31 | 2 | 4 |
| 639 | 9311642 | 214 | 914 | 23 | 2 | 3 |
| 640 | 9311643 | 192 | 957 | 23 | 1 | 4 |
| 641 | 9311644 | 181 | 884 | 16 | 0 | 1 |
| 642 | 9311645 | 215 | 924 | 22 | 2 | 3 |
| 643 | 9311646 | 217 | 998 | 23 | 7 | 4 |
| 644 | 9311647 | 197 | 917 | 15 | 4 | 1 |
| 645 | 9311648 | 201 | 1184 | 20 | 4 | 0 |
| 646 | 9311649 | 289 | 1661 | 20 | 3 | 3 |
| 647 | 9311650 | 238 | 1108 | 23 | 1 | 2 |
| 648 | 9311651 | 265 | 1366 | 23 | 3 | 3 |
| 649 | 9311652 | 223 | 1103 | 19 | 6 | 2 |
| 650 | 9311653 | 187 | 1354 | 14 | 2 | 1 |
| 651 | 9311654 | 214 | 1108 | 18 | 3 | 4 |
| 652 | 9311655 | 328 | 1749 | 30 | 4 | 4 |
| 653 | 9311656 | 246 | 1259 | 24 | 1 | 4 |
| 654 | 9311657 | 211 | 1365 | 32 | 2 | 2 |
| 655 | 9311658 | 234 | 1222 | 21 | 5 | 3 |
| 656 | 9311659 | 271 | 1560 | 37 | 0 | 4 |
| 657 | 9311660 | 196 | 808 | 17 | 6 | 3 |
| 658 | 9311661 | 222 | 1180 | 24 | 6 | 2 |
| 659 | 9311662 | 236 | 1402 | 30 | 4 | 4 |
| 660 | 9311663 | 276 | 1269 | 25 | 4 | 5 |
| 661 | 9311664 | 230 | 1193 | 31 | 0 | 4 |
| 662 | 9311665 | 256 | 1331 | 29 | 1 | 3 |
| 663 | 9311666 | 267 | 1530 | 13 | 6 | 5 |
| 664 | 9311667 | 207 | 1111 | 21 | 2 | 2 |
| 665 | 9311668 | 203 | 1235 | 24 | 2 | 3 |
| 666 | 9311669 | 255 | 1529 | 24 | 3 | 3 |
| 667 | 9311670 | 298 | 1781 | 22 | 3 | 6 |
| 668 | 9311671 | 213 | 1014 | 23 | 2 | 3 |
| 669 | 9311672 | 209 | 1123 | 28 | 1 | 4 |
| 670 | 9311673 | 210 | 1106 | 25 | 1 | 2 |
| 671 | 9311674 | 203 | 1064 | 23 | 1 | 2 |
| 672 | 9311675 | 193 | 849 | 8 | 4 | 4 |
| 673 | 9311676 | 233 | 1254 | 23 | 2 | 3 |
| 674 | 9311677 | 227 | 1193 | 22 | 5 | 2 |
| 675 | 9311678 | 218 | 943 | 30 | 4 | 0 |
| 676 | 9311679 | 262 | 1020 | 14 | 8 | 2 |
| 677 | 9311680 | 220 | 1132 | 27 | 5 | 4 |
| 678 | 9311681 | 192 | 900 | 22 | 1 | 3 |
| 679 | 9311682 | 227 | 1090 | 22 | 5 | 1 |
| 680 | 9311683 | 191 | 1033 | 17 | 5 | 0 |
| 681 | 9311684 | 237 | 1409 | 28 | 1 | 3 |
| 682 | 9311685 | 227 | 1182 | 18 | 5 | 1 |
| 683 | 9311686 | 196 | 942 | 26 | 4 | 3 |
| 684 | 9311687 | 185 | 720 | 14 | 1 | 3 |
| 685 | 9311688 | 203 | 949 | 31 | 1 | 4 |
| 686 | 9311689 | 218 | 959 | 22 | 1 | 2 |
| 687 | 9311690 | 214 | 968 | 17 | 2 | 3 |
| 688 | 9311691 | 233 | 1408 | 14 | 1 | 4 |
| 689 | 9311692 | 225 | 1050 | 22 | 1 | 4 |
| 690 | 9311693 | 211 | 1187 | 11 | 4 | 1 |
| 691 | 9311694 | 165 | 661 | 16 | 6 | 0 |
| 692 | 9311695 | 197 | 1105 | 29 | 2 | 2 |
| 693 | 9311696 | 200 | 884 | 18 | 1 | 2 |
| 694 | 9311697 | 265 | 1753 | 21 | 4 | 3 |
| 695 | 9311698 | 233 | 1397 | 24 | 2 | 4 |
| 696 | 9311699 | 214 | 1290 | 23 | 2 | 3 |
| 697 | 9311700 | 187 | 661 | 16 | 2 | 1 |
| 698 | 9311701 | 185 | 769 | 17 | 3 | 2 |
| 699 | 9311702 | 225 | 979 | 19 | 1 | 1 |
| 700 | 9311703 | 230 | 1304 | 27 | 2 | 3 |
| 701 | 9311704 | 294 | 1544 | 38 | 8 | 4 |
| 702 | 9311705 | 254 | 1615 | 21 | 1 | 3 |
| 703 | 9311706 | 170 | 701 | 10 | 5 | 0 |
| 704 | 9311707 | 194 | 946 | 17 | 5 | 0 |
| 705 | 9311708 | 192 | 899 | 11 | 3 | 4 |
| 706 | 9311709 | 290 | 2010 | 27 | 0 | 2 |
| 707 | 9311710 | 226 | 1217 | 20 | 2 | 4 |
| 708 | 9311711 | 236 | 1309 | 35 | 2 | 2 |
| 709 | 9311712 | 248 | 1363 | 27 | 4 | 3 |
| 710 | 9311713 | 222 | 1258 | 17 | 2 | 4 |
| 711 | 9311714 | 194 | 997 | 26 | 4 | 1 |
| 712 | 9311715 | 264 | 1194 | 17 | 1 | 2 |
| 713 | 9311716 | 267 | 1735 | 30 | 3 | 2 |
| 714 | 9311717 | 181 | 761 | 17 | 1 | 4 |
| 715 | 9311718 | 213 | 1272 | 31 | 4 | 5 |
| 716 | 9311719 | 220 | 1085 | 20 | 2 | 2 |
| 717 | 9311720 | 177 | 708 | 11 | 3 | 3 |
| 718 | 9311721 | 257 | 1265 | 26 | 1 | 4 |
| 719 | 9311722 | 205 | 1119 | 20 | 1 | 5 |
| 720 | 9311723 | 234 | 1263 | 27 | 5 | 3 |
| 721 | 9311724 | 188 | 1078 | 35 | 5 | 1 |
| 722 | 9311725 | 233 | 1406 | 33 | 3 | 4 |
| 723 | 9311726 | 198 | 1011 | 18 | 3 | 3 |
| 724 | 9311727 | 270 | 1190 | 20 | 0 | 5 |
| 725 | 9311728 | 222 | 1142 | 32 | 1 | 3 |
| 726 | 9311729 | 253 | 1165 | 27 | 5 | 3 |
| 727 | 9311730 | 234 | 1383 | 31 | 8 | 2 |
| 728 | 9311731 | 210 | 1293 | 23 | 5 | 3 |
| 729 | 9311732 | 247 | 1273 | 22 | 4 | 4 |
| 730 | 9311733 | 302 | 2206 | 19 | 0 | 2 |
| 731 | 9311734 | 277 | 1533 | 33 | 5 | 4 |
| 732 | 9311735 | 203 | 1005 | 19 | 3 | 3 |
| 733 | 9311736 | 248 | 1152 | 13 | 3 | 4 |
| 734 | 9311737 | 207 | 1051 | 14 | 1 | 3 |
| 735 | 9311738 | 180 | 888 | 16 | 2 | 3 |
| 736 | 9311739 | 269 | 1214 | 27 | 3 | 3 |
| 737 | 9311740 | 192 | 946 | 15 | 4 | 2 |
| 738 | 9311741 | 215 | 1051 | 21 | 4 | 4 |
| 739 | 9311742 | 214 | 871 | 17 | 2 | 2 |
| 740 | 9311743 | 207 | 1274 | 20 | 2 | 2 |
| 741 | 9311744 | 209 | 1003 | 15 | 2 | 2 |
| 742 | 9311745 | 199 | 1133 | 29 | 0 | 2 |
| 743 | 9311746 | 238 | 1267 | 18 | 3 | 4 |
| 744 | 9311747 | 182 | 1133 | 16 | 3 | 3 |
| 745 | 9311748 | 219 | 1193 | 22 | 4 | 1 |
| 746 | 9311749 | 246 | 1535 | 36 | 2 | 1 |
| 747 | 9311750 | 214 | 971 | 12 | 4 | 1 |
| 748 | 9311751 | 204 | 1096 | 22 | 2 | 1 |
| 749 | 9311752 | 197 | 1067 | 21 | 1 | 1 |
| 750 | 9311753 | 177 | 670 | 15 | 1 | 4 |
| 751 | 9311754 | 205 | 645 | 11 | 2 | 1 |
| 752 | 9311755 | 200 | 770 | 23 | 0 | 1 |
| 753 | 9311756 | 222 | 1148 | 13 | 2 | 4 |
| 754 | 9311757 | 200 | 1111 | 15 | 3 | 2 |
| 755 | 9311758 | 209 | 1172 | 24 | 2 | 3 |
| 756 | 9311759 | 187 | 792 | 16 | 4 | 2 |
| 757 | 9311760 | 196 | 1188 | 24 | 1 | 4 |
| 758 | 9311761 | 176 | 894 | 23 | 2 | 3 |
| 759 | 9311762 | 228 | 1362 | 31 | 5 | 2 |
| 760 | 9311763 | 209 | 1114 | 35 | 1 | 3 |
| 761 | 9311764 | 189 | 952 | 14 | 3 | 1 |
| 762 | 9311765 | 208 | 1201 | 23 | 1 | 3 |
| 763 | 9311766 | 158 | 720 | 15 | 3 | 2 |
| 764 | 9311767 | 189 | 1097 | 18 | 3 | 1 |
| 765 | 9311768 | 229 | 1340 | 20 | 4 | 3 |
| 766 | 9311769 | 328 | 1963 | 56 | 1 | 4 |
| 767 | 9311770 | 193 | 960 | 17 | 1 | 3 |
| 768 | 9311771 | 193 | 695 | 12 | 2 | 2 |
| 769 | 9311772 | 266 | 1396 | 28 | 1 | 2 |
| 770 | 9311773 | 229 | 1043 | 15 | 4 | 4 |
| 771 | 9311774 | 181 | 936 | 28 | 2 | 4 |
| 772 | 9311775 | 230 | 1233 | 35 | 5 | 4 |
| 773 | 9311776 | 234 | 1421 | 24 | 5 | 3 |
| 774 | 9311777 | 250 | 1713 | 24 | 4 | 6 |
| 775 | 9311778 | 217 | 1052 | 19 | 3 | 1 |
| 776 | 9311779 | 226 | 1342 | 26 | 4 | 3 |
| 777 | 9311780 | 201 | 930 | 28 | 7 | 2 |
| 778 | 9311781 | 206 | 989 | 8 | 1 | 3 |
| 779 | 9311782 | 198 | 948 | 22 | 1 | 3 |
| 780 | 9311783 | 268 | 1789 | 20 | 3 | 1 |
| 781 | 9311784 | 228 | 1171 | 17 | 4 | 3 |
| 782 | 9311785 | 189 | 942 | 9 | 0 | 2 |
| 783 | 9311786 | 151 | 565 | 16 | 0 | 3 |
| 784 | 9311787 | 166 | 663 | 14 | 3 | 3 |
| 785 | 9311788 | 219 | 1301 | 24 | 7 | 3 |
| 786 | 9311789 | 186 | 1066 | 24 | 1 | 3 |
| 787 | 9311790 | 159 | 816 | 21 | 2 | 2 |
| 788 | 9311791 | 233 | 1664 | 29 | 2 | 3 |
| 789 | 9311792 | 238 | 1415 | 19 | 1 | 3 |
| 790 | 9311793 | 266 | 1719 | 25 | 1 | 2 |
| 791 | 9311794 | 220 | 1069 | 17 | 4 | 4 |
| 792 | 9311795 | 214 | 988 | 23 | 7 | 1 |
| 793 | 9311796 | 215 | 1045 | 21 | 3 | 1 |
| 794 | 9311797 | 245 | 1263 | 14 | 8 | 2 |
| 795 | 9311798 | 212 | 1210 | 26 | 1 | 2 |
| 796 | 9311799 | 167 | 688 | 17 | 1 | 3 |
| 797 | 9311800 | 244 | 1349 | 25 | 0 | 6 |
| 798 | 9311801 | 240 | 1411 | 20 | 5 | 2 |
| 799 | 9311802 | 210 | 1078 | 26 | 4 | 2 |
| 800 | 9311803 | 210 | 1250 | 27 | 6 | 4 |
| 801 | 9311804 | 232 | 1351 | 35 | 1 | 3 |
| 802 | 9311805 | 251 | 1190 | 20 | 2 | 3 |
| 803 | 9311806 | 232 | 899 | 18 | 1 | 1 |
| 804 | 9311807 | 205 | 1117 | 10 | 0 | 3 |
| 805 | 9311808 | 244 | 1342 | 14 | 2 | 2 |
| 806 | 9311809 | 254 | 1465 | 30 | 2 | 3 |
| 807 | 9311810 | 269 | 1705 | 27 | 1 | 1 |
| 808 | 9311811 | 204 | 1176 | 16 | 3 | 3 |
| 809 | 9311812 | 226 | 1268 | 23 | 3 | 2 |
| 810 | 9311813 | 218 | 884 | 28 | 3 | 2 |
| 811 | 9311814 | 253 | 1245 | 18 | 4 | 2 |
| 812 | 9311815 | 204 | 1042 | 24 | 2 | 1 |
| 813 | 9311816 | 235 | 1357 | 36 | 1 | 2 |
| 814 | 9311817 | 209 | 1095 | 16 | 3 | 4 |
| 815 | 9311818 | 200 | 772 | 14 | 2 | 2 |
| 816 | 9311819 | 285 | 1487 | 23 | 6 | 4 |
| 817 | 9311820 | 244 | 1005 | 17 | 7 | 2 |
| 818 | 9311821 | 223 | 1122 | 29 | 5 | 4 |
| 819 | 9311822 | 215 | 1249 | 28 | 4 | 2 |
| 820 | 9311823 | 218 | 979 | 22 | 0 | 3 |
| 821 | 9311824 | 213 | 1002 | 23 | 1 | 3 |
| 822 | 9311825 | 182 | 926 | 15 | 4 | 3 |
| 823 | 9311826 | 234 | 1257 | 17 | 1 | 3 |
| 824 | 9311827 | 244 | 1128 | 22 | 6 | 3 |
| 825 | 9311828 | 164 | 615 | 21 | 3 | 3 |
| 826 | 9311829 | 215 | 774 | 24 | 5 | 3 |
| 827 | 9311830 | 214 | 1230 | 20 | 1 | 1 |
| 828 | 9311831 | 251 | 1396 | 32 | 0 | 0 |
| 829 | 9311832 | 273 | 1516 | 33 | 6 | 2 |
| 830 | 9311833 | 184 | 861 | 17 | 5 | 3 |
| 831 | 9311834 | 225 | 1126 | 26 | 3 | 3 |
| 832 | 9311835 | 186 | 995 | 23 | 10 | 1 |
| 833 | 9311836 | 240 | 1453 | 37 | 3 | 3 |
| 834 | 9311837 | 194 | 1025 | 19 | 3 | 1 |
| 835 | 9311838 | 191 | 1031 | 13 | 4 | 1 |
| 836 | 9311839 | 194 | 711 | 17 | 1 | 1 |
| 837 | 9311840 | 248 | 1543 | 29 | 2 | 3 |
| 838 | 9311841 | 242 | 1309 | 36 | 1 | 3 |
| 839 | 9311842 | 207 | 1092 | 29 | 0 | 2 |
| 840 | 9311843 | 208 | 1068 | 31 | 3 | 4 |
| 841 | 9311844 | 252 | 1203 | 24 | 4 | 2 |
| 842 | 9311845 | 229 | 1147 | 23 | 3 | 3 |
| 843 | 9311846 | 212 | 1138 | 11 | 2 | 1 |
| 844 | 9311847 | 173 | 984 | 19 | 0 | 1 |
| 845 | 9311848 | 201 | 1123 | 21 | 3 | 1 |
| 846 | 9311849 | 249 | 1238 | 11 | 1 | 2 |
| 847 | 9311850 | 223 | 1170 | 15 | 1 | 3 |
| 848 | 9311851 | 220 | 1052 | 28 | 4 | 3 |
| 849 | 9311852 | 277 | 1773 | 25 | 4 | 2 |
| 850 | 9311853 | 240 | 1060 | 9 | 1 | 2 |
| 851 | 9311854 | 152 | 533 | 9 | 1 | 2 |
| 852 | 9311855 | 235 | 1247 | 24 | 1 | 1 |
| 853 | 9311856 | 196 | 1201 | 24 | 2 | 5 |
| 854 | 9311857 | 229 | 1223 | 36 | 1 | 3 |
| 855 | 9311858 | 230 | 1143 | 27 | 2 | 4 |
| 856 | 9311859 | 211 | 949 | 17 | 3 | 2 |
| 857 | 9311860 | 255 | 1626 | 25 | 6 | 2 |
| 858 | 9311861 | 397 | 2820 | 39 | 5 | 3 |
| 859 | 9311862 | 273 | 1415 | 27 | 1 | 1 |
| 860 | 9311863 | 207 | 1073 | 21 | 2 | 1 |
| 861 | 9311864 | 227 | 1157 | 21 | 1 | 0 |
| 862 | 9311865 | 245 | 1428 | 23 | 2 | 2 |
| 863 | 9311866 | 315 | 2112 | 45 | 1 | 2 |
| 864 | 9311867 | 235 | 1315 | 31 | 5 | 5 |
| 865 | 9311868 | 251 | 1152 | 25 | 4 | 3 |
| 866 | 9311869 | 203 | 977 | 23 | 1 | 2 |
| 867 | 9311870 | 183 | 675 | 16 | 3 | 4 |
| 868 | 9311871 | 228 | 1036 | 13 | 2 | 3 |
| 869 | 9311872 | 266 | 1379 | 26 | 4 | 3 |
| 870 | 9311873 | 312 | 1635 | 43 | 8 | 6 |
| 871 | 9311874 | 246 | 1223 | 29 | 4 | 1 |
| 872 | 9311875 | 213 | 1022 | 19 | 2 | 2 |
| 873 | 9311876 | 215 | 1075 | 27 | 3 | 3 |
| 874 | 9311877 | 219 | 897 | 18 | 1 | 3 |
| 875 | 9311878 | 232 | 1377 | 14 | 3 | 3 |
| 876 | 9311879 | 216 | 947 | 25 | 4 | 3 |
| 877 | 9311880 | 217 | 1423 | 17 | 2 | 4 |
| 878 | 9311881 | 223 | 1208 | 15 | 1 | 0 |
| 879 | 9311882 | 214 | 1073 | 21 | 1 | 1 |
| 880 | 9311883 | 170 | 599 | 14 | 2 | 0 |
| 881 | 9311884 | 215 | 1076 | 31 | 2 | 3 |
| 882 | 9311885 | 241 | 1277 | 26 | 4 | 4 |
| 883 | 9311886 | 312 | 1853 | 26 | 7 | 4 |
| 884 | 9311887 | 326 | 2061 | 39 | 1 | 4 |
| 885 | 9311888 | 223 | 1001 | 23 | 2 | 3 |
| 886 | 9311889 | 191 | 1023 | 13 | 2 | 2 |
| 887 | 9311890 | 194 | 668 | 15 | 2 | 2 |
| 888 | 9311891 | 188 | 971 | 12 | 1 | 2 |
| 889 | 9311892 | 191 | 1024 | 12 | 2 | 2 |
| 890 | 9311893 | 205 | 1090 | 16 | 6 | 2 |
| 891 | 9311894 | 216 | 1335 | 16 | 2 | 0 |
| 892 | 9311895 | 237 | 1316 | 26 | 1 | 3 |
| 893 | 9311896 | 233 | 1144 | 29 | 4 | 2 |
| 894 | 9311897 | 226 | 1066 | 21 | 1 | 3 |
| 895 | 9311898 | 269 | 1415 | 28 | 2 | 3 |
| 896 | 9311899 | 179 | 733 | 13 | 1 | 2 |
| 897 | 9311900 | 255 | 1264 | 16 | 2 | 1 |
| 898 | 9311901 | 193 | 884 | 23 | 3 | 3 |
| 899 | 9311902 | 174 | 745 | 8 | 4 | 1 |
| 900 | 9311903 | 210 | 1054 | 24 | 6 | 1 |
| 901 | 9311904 | 291 | 1462 | 19 | 1 | 5 |
| 902 | 9311905 | 195 | 945 | 23 | 4 | 1 |
| 903 | 9311906 | 219 | 1085 | 26 | 3 | 3 |
| 904 | 9311907 | 222 | 1193 | 37 | 3 | 4 |
| 905 | 9311908 | 218 | 1158 | 18 | 6 | 3 |
| 906 | 9311909 | 198 | 812 | 23 | 4 | 3 |
| 907 | 9311910 | 209 | 1163 | 25 | 3 | 3 |
| 908 | 9311911 | 193 | 1044 | 18 | 4 | 2 |
| 909 | 9311912 | 158 | 916 | 19 | 1 | 2 |
| 910 | 9311913 | 253 | 1440 | 21 | 1 | 3 |
| 911 | 9311914 | 278 | 1273 | 17 | 3 | 3 |
| 912 | 9311915 | 258 | 1372 | 29 | 5 | 1 |
| 913 | 9311916 | 237 | 1296 | 19 | 1 | 3 |
| 914 | 9311917 | 229 | 1289 | 30 | 3 | 4 |
| 915 | 9311918 | 224 | 1235 | 26 | 3 | 1 |
| 916 | 9311919 | 177 | 719 | 18 | 4 | 1 |
| 917 | 9311920 | 204 | 1048 | 22 | 1 | 2 |
| 918 | 9311921 | 227 | 1400 | 19 | 4 | 1 |
| 919 | 9311922 | 235 | 1448 | 13 | 4 | 3 |
| 920 | 9311923 | 203 | 1055 | 11 | 1 | 2 |
| 921 | 9311924 | 250 | 1191 | 20 | 3 | 2 |
| 922 | 9311925 | 186 | 850 | 18 | 0 | 2 |
| 923 | 9311926 | 278 | 1546 | 34 | 0 | 4 |
| 924 | 9311927 | 220 | 932 | 25 | 1 | 2 |
| 925 | 9311928 | 213 | 1435 | 28 | 0 | 3 |
| 926 | 9311929 | 181 | 794 | 22 | 1 | 1 |
| 927 | 9311930 | 229 | 1401 | 35 | 8 | 5 |
| 928 | 9311931 | 221 | 886 | 17 | 0 | 4 |
| 929 | 9311932 | 241 | 1011 | 18 | 2 | 2 |
| 930 | 9311933 | 231 | 1474 | 20 | 5 | 0 |
| 931 | 9311934 | 237 | 1463 | 24 | 4 | 4 |
| 932 | 9311935 | 277 | 1820 | 43 | 3 | 3 |
| 933 | 9311936 | 193 | 1100 | 24 | 1 | 3 |
| 934 | 9311937 | 207 | 961 | 16 | 4 | 1 |
| 935 | 9311938 | 163 | 690 | 14 | 0 | 3 |
| 936 | 9311939 | 224 | 1292 | 18 | 3 | 3 |
| 937 | 9311940 | 173 | 869 | 21 | 0 | 1 |
| 938 | 9311941 | 165 | 677 | 12 | 5 | 2 |
| 939 | 9311942 | 207 | 1087 | 26 | 3 | 3 |
| 940 | 9311943 | 209 | 1138 | 20 | 2 | 2 |
| 941 | 9311944 | 279 | 1450 | 24 | 0 | 4 |
| 942 | 9311945 | 275 | 1774 | 30 | 5 | 2 |
| 943 | 9311946 | 280 | 1692 | 45 | 2 | 4 |
| 944 | 9311947 | 205 | 1129 | 18 | 3 | 4 |
| 945 | 9311948 | 217 | 918 | 29 | 0 | 3 |
| 946 | 9311949 | 238 | 1222 | 27 | 1 | 1 |
| 947 | 9311950 | 193 | 632 | 17 | 4 | 2 |
| 948 | 9311951 | 262 | 1314 | 11 | 2 | 1 |
| 949 | 9311952 | 200 | 879 | 18 | 1 | 1 |
| 950 | 9311953 | 200 | 973 | 16 | 1 | 2 |
| 951 | 9311954 | 217 | 1048 | 29 | 1 | 2 |
| 952 | 9311955 | 251 | 1512 | 23 | 1 | 4 |
| 953 | 9311956 | 233 | 1233 | 23 | 3 | 2 |
| 954 | 9311957 | 217 | 1171 | 18 | 7 | 1 |
| 955 | 9311958 | 218 | 1194 | 27 | 6 | 1 |
| 956 | 9311959 | 228 | 1234 | 35 | 7 | 3 |
| 957 | 9311960 | 275 | 1376 | 29 | 2 | 3 |
| 958 | 9311961 | 231 | 1077 | 22 | 1 | 3 |
| 959 | 9311962 | 260 | 1436 | 26 | 1 | 2 |
| 960 | 9311963 | 265 | 1583 | 31 | 1 | 2 |
| 961 | 9311964 | 280 | 1620 | 23 | 5 | 3 |
| 962 | 9311965 | 254 | 1522 | 25 | 4 | 4 |
| 963 | 9311966 | 216 | 1172 | 24 | 6 | 4 |
| 964 | 9311967 | 227 | 1175 | 21 | 2 | 2 |
| 965 | 9311968 | 241 | 1517 | 35 | 4 | 4 |
| 966 | 9311969 | 274 | 1598 | 30 | 1 | 1 |
| 967 | 9311970 | 201 | 1132 | 23 | 5 | 3 |
| 968 | 9311971 | 179 | 1092 | 24 | 0 | 1 |
| 969 | 9311972 | 203 | 1175 | 21 | 4 | 2 |
| 970 | 9311973 | 254 | 1524 | 32 | 1 | 3 |
| 971 | 9311974 | 223 | 1559 | 17 | 1 | 1 |
| 972 | 9311975 | 210 | 776 | 28 | 0 | 2 |
| 973 | 9311976 | 275 | 1358 | 34 | 2 | 1 |
| 974 | 9311977 | 245 | 1039 | 17 | 4 | 2 |
| 975 | 9311978 | 258 | 1182 | 19 | 4 | 1 |
| 976 | 9311979 | 178 | 893 | 10 | 2 | 2 |
| 977 | 9311980 | 266 | 1526 | 28 | 1 | 3 |
| 978 | 9311981 | 214 | 1286 | 16 | 4 | 1 |
| 979 | 9311982 | 195 | 782 | 17 | 4 | 2 |
| 980 | 9311983 | 213 | 1227 | 31 | 2 | 2 |
| 981 | 9311984 | 329 | 1417 | 22 | 3 | 3 |
| 982 | 9311985 | 192 | 895 | 18 | 0 | 0 |
| 983 | 9311986 | 247 | 1208 | 26 | 6 | 3 |
| 984 | 9311987 | 262 | 1306 | 41 | 2 | 4 |
| 985 | 9311988 | 269 | 1444 | 42 | 4 | 6 |
| 986 | 9311989 | 207 | 864 | 15 | 2 | 3 |
| 987 | 9311990 | 248 | 1150 | 19 | 2 | 5 |
| 988 | 9311991 | 242 | 1231 | 29 | 5 | 1 |
| 989 | 9311992 | 223 | 1202 | 39 | 2 | 5 |
| 990 | 9311993 | 262 | 1638 | 16 | 3 | 3 |
| 991 | 9311994 | 237 | 1212 | 21 | 1 | 3 |
| 992 | 9311995 | 285 | 1443 | 25 | 1 | 1 |
| 993 | 9311996 | 319 | 2285 | 21 | 4 | 1 |
| 994 | 9311997 | 288 | 1531 | 21 | 6 | 4 |
| 995 | 9311998 | 169 | 708 | 14 | 1 | 1 |
| 996 | 9311999 | 224 | 1261 | 27 | 2 | 3 |
| 997 | 9312000 | 218 | 868 | 22 | 2 | 0 |
| 998 | 9312001 | 240 | 1249 | 18 | 2 | 1 |
| 999 | 9312002 | 223 | 1033 | 17 | 2 | 2 |
| 1000 | 9312003 | 229 | 996 | 19 | 5 | 2 |
| 1001 | 9312004 | 243 | 1261 | 24 | 1 | 4 |
| 1002 | 9312005 | 215 | 1334 | 23 | 5 | 2 |
| 1003 | 9312006 | 199 | 1036 | 17 | 3 | 1 |
| 1004 | 9312007 | 232 | 1330 | 20 | 2 | 3 |
| 1005 | 9312008 | 201 | 907 | 27 | 2 | 1 |
| 1006 | 9312009 | 225 | 1137 | 19 | 6 | 2 |
| 1007 | 9312010 | 176 | 835 | 22 | 1 | 2 |
| 1008 | 9312011 | 189 | 735 | 22 | 2 | 4 |
| 1009 | 9312012 | 209 | 1183 | 15 | 0 | 2 |
| 1010 | 9312013 | 226 | 1393 | 25 | 5 | 3 |
| 1011 | 9312014 | 216 | 1154 | 18 | 2 | 2 |
| 1012 | 9312015 | 168 | 993 | 26 | 4 | 1 |
| 1013 | 9312016 | 193 | 1034 | 21 | 0 | 2 |
| 1014 | 9312017 | 237 | 1430 | 36 | 7 | 3 |
| 1015 | 9312018 | 216 | 1089 | 12 | 5 | 0 |
| 1016 | 9312019 | 210 | 992 | 18 | 3 | 4 |
| 1017 | 9312020 | 201 | 1078 | 15 | 0 | 2 |
| 1018 | 9312021 | 213 | 1032 | 17 | 3 | 1 |
| 1019 | 9312022 | 317 | 2113 | 38 | 2 | 5 |
| 1020 | 9312023 | 197 | 859 | 32 | 4 | 3 |
| 1021 | 9312024 | 234 | 1383 | 23 | 1 | 2 |
| 1022 | 9312025 | 235 | 1213 | 7 | 10 | 1 |
| 1023 | 9312026 | 176 | 677 | 16 | 3 | 3 |

## Throughput

- Turns simulated: 229283 (first play of each game)
- Aggregate: 122.6 turns/s over 1870.3s wall
- Per game thread: 30.7 turns/s

## Determinism

Not checked (run with `--determinism`).

## Panics

None.
