# Simulation: deals-terms-off (2026-11-25)

1000 games, first seed 9311003, brain `scripted`, turn limit off, safety cap 1200, 5 threads, started 2026-11-25T23:14:29Z. Wall time 1550.5s.

Source checkout at run start: `505937ecd0f1e1b47f8b1c7ed0dd0ac2a4cab5e9`; dirty: no. Map 48×32, 8 initial seats. `first_setup` and exact binary argv are recorded in JSON. Ordinary games use seed + i (wrapping); the strategy tournament schedule is specified below. All games use a deterministic shuffle of the full faction pool, taking eight distinct factions, with any Proxy rotation described below.

Command argv: `["./sim","--players","8","--difficulty","hard","--games","1000","--seed","9311003","--out-dir","out","--threads","5","--deals","--ai-params","off.json","--label","deals-terms-off"]`

## Overview

| Metric | Value |
| --- | --- |
| Games (non-panic) | 1000 of 1000 |
| Game length, turns (mean / median / min-max) | 226.1 / 223 / 137-451 |
| Win reason mix | VP hold 98.3%, Capitals 1.7% |
| Median end turn | 223 |
| Ending shares of played games (capitals / VP hold / turn limit / other) | 1.7% / 98.3% / 0.0% / 0.0% |
| Games reaching the World War era (any empire) | 995 of 1000 (99.5%) |
| VP hold length | 10 turn ends (the reaching turn counts) |
| Holds started / broken / completed / unfinished | 1483 / 397 / 986 / 100 |
| Hold break rate | 26.8% of started; 28.7% of resolved (broken / (broken + completed)) |
| Hold wins | 983 |
| First reach to win, turns (mean / median / min-max) | 11.2 / 9 / 9-100 |
| Final hold start to win, turns | 9.0 / 9 / 9-9 |
| Comebacks (winner broke an earlier hold of its own) | 130 |
| Wins by a team another team beat to its target | 82 |

Game length by how the game ended (turns resolved; a stall is a safety-cap stop, never a win):

| Ended by | Games | Length mean / median / min-max |
| --- | --- | --- |
| VP hold | 983 | 225.3 / 222 / 137-451 |
| Capitals | 17 | 273.2 / 270 / 211-369 |

Winner share by faction (wins of that faction's seats / all winning seats):

| Faction | Wins | Share of wins |
| --- | --- | --- |
| Switzerland | 86 | 8.6% |
| India | 74 | 7.4% |
| Byzantium | 79 | 7.9% |
| Germany | 149 | 14.9% |
| Celts | 117 | 11.7% |
| Mongols | 104 | 10.4% |
| Sparta | 117 | 11.7% |
| Athens | 156 | 15.6% |
| Aztecs | 118 | 11.8% |

## Outcomes

| Outcome | Games | Share |
| --- | --- | --- |
| VP hold | 983 | 98.3% |
| Conquest | 0 | 0.0% |
| Capitals | 17 | 1.7% |
| Turn limit | 0 | 0.0% |
| Draw | 0 | 0.0% |
| Stalled (safety cap) | 0 | 0.0% |
| Panic | 0 | 0.0% |

## Win rate by faction

Per seat: wins / seats of that faction. With 8 single-empire teams, an even game is 12.5% per seat.

| Faction | Seats | Wins | Win rate |
| --- | --- | --- | --- |
| Switzerland | 879 | 86 | 9.8% |
| India | 905 | 74 | 8.2% |
| Byzantium | 885 | 79 | 8.9% |
| Germany | 900 | 149 | 16.6% |
| Celts | 880 | 117 | 13.3% |
| Mongols | 878 | 104 | 11.8% |
| Sparta | 901 | 117 | 13.0% |
| Athens | 900 | 156 | 17.3% |
| Aztecs | 872 | 118 | 13.5% |

## Win rate by difficulty

All initial seats in non-panicked games, including eliminated empires and cap stops. Determinism replays are not additional samples.

| Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- |
| Hard | 8000 | 1000 | 12.5% |

## Win rate by faction and difficulty

| Faction | Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- | --- |
| Switzerland | Hard | 879 | 86 | 9.8% |
| India | Hard | 905 | 74 | 8.2% |
| Byzantium | Hard | 885 | 79 | 8.9% |
| Germany | Hard | 900 | 149 | 16.6% |
| Celts | Hard | 880 | 117 | 13.3% |
| Mongols | Hard | 878 | 104 | 11.8% |
| Sparta | Hard | 901 | 117 | 13.0% |
| Athens | Hard | 900 | 156 | 17.3% |
| Aztecs | Hard | 872 | 118 | 13.5% |

## Faction goal progress

Per seat: how often the faction's permanent goal was reached before the game ended, and the turn its VP was first awarded (over the seats that reached it). Seats are all initial empires of non-panicked games with telemetry, including eliminated empires and cap stops.

| Faction | Seats | Reached | Goal rate | Min | P10 | Median turn | P90 | Max |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Switzerland | 879 | 198 | 22.5% | 110 | 136 | 165 | 235 | 345 |
| India | 905 | 523 | 57.8% | 44 | 89 | 126 | 203 | 379 |
| Byzantium | 885 | 700 | 79.1% | 69 | 83 | 105 | 146 | 281 |
| Germany | 900 | 364 | 40.4% | 84 | 123 | 153 | 200 | 285 |
| Celts | 880 | 565 | 64.2% | 35 | 170 | 210 | 210 | 286 |
| Mongols | 878 | 157 | 17.9% | 105 | 148 | 190 | 231 | 341 |
| Sparta | 901 | 162 | 18.0% | 121 | 154 | 198 | 259 | 315 |
| Athens | 900 | 261 | 29.0% | 106 | 153 | 218 | 255 | 255 |
| Aztecs | 872 | 317 | 36.4% | 134 | 172 | 225 | 235 | 290 |

## Distributions

| Metric | Min | P10 | Median | P90 | Max |
| --- | --- | --- | --- | --- | --- |
| Observed game length (turns, including cap stops) | 137 | 182 | 223 | 271 | 451 |
| Winning team VP | 2 | 10 | 11 | 13 | 17 |
| First Soldier completed (turn, per empire) | 6 | 8 | 36 | 71 | 259 |
| First Settler completed (turn, per empire) | 6 | 12 | 14 | 25 | 165 |
| First city founded (turn, per empire) | 1 | 1 | 1 | 1 | 1 |
| Second city founded (turn, per empire) | 10 | 16 | 19 | 39 | 227 |

Safety-cap stops are censored observations, not completed game lengths.

Openings: 8000 observed empires in 1000 first plays; 0 games without opening telemetry excluded. Turn distributions include only empires that reached the milestone. Never means not reached by the end of observed play, including eliminated empires and safety-cap stops. Per-empire turns are in JSON (`games[].telemetry.openings`, keyed by player ID).

| Opening milestone | Reached empires | Never empires |
| --- | --- | --- |
| First Soldier completed | 7908 | 92 |
| First Settler completed | 7983 | 17 |
| First city founded | 8000 | 0 |
| Second city founded | 7911 | 89 |

Final VP per empire (all seats):

| VP | Empires |
| --- | --- |
| 0 | 1578 |
| 1 | 1455 |
| 2 | 1114 |
| 3 | 756 |
| 4 | 597 |
| 5 | 487 |
| 6 | 355 |
| 7 | 242 |
| 8 | 177 |
| 9 | 146 |
| 10 | 455 |
| 11 | 300 |
| 12 | 170 |
| 13 | 90 |
| 14 | 49 |
| 15 | 20 |
| 16 | 8 |
| 17 | 1 |

## End state

- Average game length: 226.1 turns
- Average cities per empire: 4.62
- Average techs per empire: 32.74
- Average eliminations per game: 2.01

## Strategy diversity: observed behavior (classifier v2)

1000 finished games; 8000 player seats; 1000 winning seats. Excluded 0 unfinished/panicked games and 0 games with missing labels. Winner entropy: **0.3160** (normalized over 6 categories, including zero-win categories).

Thresholds, biases and denominators: [strategy-diversity.md](../strategy-diversity.md).

| Archetype | Players | Wins | Win rate | All-player frequency | Winner frequency |
| --- | ---: | ---: | ---: | ---: | ---: |
| Rush | 1779 | 57 | 3.2% | 22.2% | 5.7% |
| Wide | 3771 | 845 | 22.4% | 47.1% | 84.5% |
| Science | 0 | 0 | n/a | 0.0% | 0.0% |
| Turtle/Wonder | 51 | 5 | 9.8% | 0.6% | 0.5% |
| Economic | 1057 | 90 | 8.5% | 13.2% | 9.0% |
| Generalist | 1342 | 3 | 0.2% | 16.8% | 0.3% |

### Matchup matrix

Cell = strict ahead % / tied % (co-occurrence games). Each game has equal weight; all cross-team seat pairs within it share that weight. Rank: winning team first, then surviving empires by VP, then eliminated empires by later elimination turn. Equal ranks tie; no ID tiebreak. This is a multiplayer association, not a duel win rate.

| Row vs column | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Rush | — | 17.6% / 4.5% (866) | n/a / n/a (0) | 13.3% / 7.7% (42) | 17.3% / 8.1% (565) | 58.2% / 5.8% (671) |
| Wide | 77.9% / 4.5% (866) | — | n/a / n/a (0) | 49.3% / 7.5% (50) | 55.6% / 6.6% (675) | 87.5% / 2.7% (792) |
| Science | n/a / n/a (0) | n/a / n/a (0) | — | n/a / n/a (0) | n/a / n/a (0) | n/a / n/a (0) |
| Turtle/Wonder | 79.0% / 7.7% (42) | 43.2% / 7.5% (50) | n/a / n/a (0) | — | 45.7% / 8.1% (31) | 89.2% / 5.4% (37) |
| Economic | 74.6% / 8.1% (565) | 37.7% / 6.6% (675) | n/a / n/a (0) | 46.2% / 8.1% (31) | — | 84.6% / 6.4% (507) |
| Generalist | 35.9% / 5.8% (671) | 9.8% / 2.7% (792) | n/a / n/a (0) | 5.4% / 5.4% (37) | 8.9% / 6.4% (507) | — |

### Faction wins by archetype

Cell = wins / seats; win rate in JSON.

| Faction | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Switzerland | 1 / 87 | 74 / 442 | 0 / 0 | 0 / 1 | 11 / 197 | 0 / 152 |
| India | 2 / 215 | 72 / 572 | 0 / 0 | 0 / 0 | 0 / 16 | 0 / 102 |
| Byzantium | 0 / 10 | 75 / 483 | 0 / 0 | 0 / 4 | 4 / 130 | 0 / 258 |
| Germany | 3 / 131 | 138 / 480 | 0 / 0 | 0 / 8 | 8 / 84 | 0 / 197 |
| Celts | 1 / 108 | 113 / 530 | 0 / 0 | 0 / 0 | 3 / 117 | 0 / 125 |
| Mongols | 33 / 548 | 66 / 201 | 0 / 0 | 0 / 0 | 5 / 42 | 0 / 87 |
| Sparta | 15 / 448 | 66 / 178 | 0 / 0 | 5 / 36 | 29 / 147 | 2 / 92 |
| Athens | 0 / 94 | 141 / 449 | 0 / 0 | 0 / 1 | 14 / 154 | 1 / 202 |
| Aztecs | 2 / 138 | 100 / 436 | 0 / 0 | 0 / 1 | 16 / 170 | 0 / 127 |

Guardrail: **Fail**. Entropy minimum 0.80; at least 20 games per strategy pair. Dominant by observed score: none. Without an observed counter (all pairs covered): none. Sampled but never won: none. Unobserved: Science. Under-sampled pairs: 5.

- insufficient evidence: 5 under-sampled matchups; 1000 winning seats

- winner entropy below 0.80

## Battle and capture telemetry

1000 observed first plays; 0 records without telemetry excluded.

| Event | Total / observed games | Mean per game | Games with event / observed games |
| --- | --- | --- | --- |
| War declarations | 15149/1000 | 15.149 | 1000/1000 (100.0%) |
| Peace offers | 89745/1000 | 89.745 | 1000/1000 (100.0%) |
| Peace agreements | 12871/1000 | 12.871 | 1000/1000 (100.0%) |
| Battles | 1061148/1000 | 1061.148 | 1000/1000 (100.0%) |
| City captures | 22864/1000 | 22.864 | 1000/1000 (100.0%) |
| Settler captures | 5216/1000 | 5.216 | 984/1000 (98.4%) |
| Eliminations | 2014/1000 | 2.014 | 906/1000 (90.6%) |

## Terminal VP sources

1000 finished games. Columns are separate populations; totals across columns must not be added. Every cell is VP / population observations.

| Source | All players | All teams | Winning teams |
| --- | --- | --- | --- |
| Held Great Works | 7392/8000 | 7392/8000 | 3246/1000 |
| Faction goals | 5130/8000 | 5130/8000 | 2188/1000 |
| Era objectives | 14265/8000 | 14265/8000 | 4130/1000 |
| Capitals taken | 2732/8000 | 2732/8000 | 1611/1000 |
| Total VP | 29519/8000 | 29519/8000 | 11175/1000 |

Reconciliation failures: 0 player observations, 0 team observations.

## Turn-50 team leaders

| Cohort | Eventual leader-set wins / eligible games | Draws (included) |
| --- | --- | --- |
| Unique leader | 72/416 (17.3%) | 0 |
| Tied leaders (winner in set) | 579/584 (99.1%) | 0 |
| Any leader set | 651/1000 (65.1%) | 0 |

Excluded: 0 games ended before turn 50, 0 unfinished games, 0 snapshots without teams, 0 records without telemetry.

## Era pacing

Resolved turn each empire entered each era. `Winner vs median` is the winning team's seats' entry turn minus the median entry of the empires that entered it in the same finished game (negative: winners got there first).

| Era | Entered / seats | Fastest | p10 | Median | p90 | Slowest | Winner vs median (mean, n) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Classical | 7896/8000 | 5 | 20 | 28 | 32 | 59 | -2.5 (1000) |
| Medieval | 7803/8000 | 35 | 56 | 60 | 63 | 127 | -1.7 (1000) |
| Renaissance | 7743/8000 | 63 | 81 | 86 | 93 | 191 | -2.7 (1000) |
| Industrial | 7563/8000 | 87 | 107 | 114 | 126 | 256 | -4.3 (1000) |
| Machine Age | 6983/8000 | 113 | 135 | 145 | 160 | 257 | -6.4 (1000) |
| World War | 5726/8000 | 133 | 160 | 173 | 192 | 294 | -9.0 (994) |
| Modern | 3660/8000 | 153 | 184 | 203 | 226 | 308 | -10.4 (876) |
| Future | 1559/8000 | 178 | 208 | 231 | 256 | 406 | -7.0 (557) |

Research idle turns (techs left, no research set): 4, of which stalled (two or more in a row): 0.

## Tech pace against the calendar

Median resolved turn each technology was learned, the display year of that turn (`calendar::calendar_label`) and the year the technology really arrived (n/a when no single year is established). Empires that never learned it are left out.

| Era | Technology | Learned / seats | Median turn | Median year | Real year |
| --- | --- | --- | --- | --- | --- |
| Ancient | Irrigation | 8000/8000 | 9 | 3173 BC | 6000 BC |
| Ancient | Bronze Working | 7984/8000 | 17 | 2345 BC | 3300 BC |
| Ancient | Masonry | 7998/8000 | 12 | 2863 BC | 3000 BC |
| Ancient | Currency | 7923/8000 | 27 | 1311 BC | 600 BC |
| Ancient | Writing | 7936/8000 | 25 | 1518 BC | 3200 BC |
| Classical | Mathematics | 7827/8000 | 43 | 351 BC | 300 BC |
| Classical | Engineering | 7812/8000 | 49 | 51 BC | n/a |
| Classical | Iron Working | 7845/8000 | 37 | 651 BC | 1200 BC |
| Classical | Code of Laws | 7806/8000 | 57 | 350 AD | 1754 BC |
| Classical | Philosophy | 7804/8000 | 58 | 400 AD | 500 BC |
| Medieval | Feudalism | 7782/8000 | 73 | 890 AD | n/a |
| Medieval | Machinery | 7795/8000 | 67 | 710 AD | n/a |
| Medieval | Chivalry | 6908/8000 | 78 | 1040 AD | n/a |
| Medieval | Education | 7756/8000 | 83 | 1190 AD | 1088 AD |
| Medieval | Crop Rotation | 7743/8000 | 85 | 1250 AD | n/a |
| Renaissance | Gunpowder | 7726/8000 | 92 | 1420 AD | n/a |
| Renaissance | Metallurgy | 7677/8000 | 102 | 1520 AD | n/a |
| Renaissance | Military Engineering | 7683/8000 | 100 | 1500 AD | n/a |
| Renaissance | Banking | 7609/8000 | 111 | 1610 AD | 1397 AD |
| Renaissance | Printing Press | 7567/8000 | 112 | 1620 AD | 1440 AD |
| Industrial | Rifling | 7211/8000 | 135 | 1787 AD | n/a |
| Industrial | Military Science | 7375/8000 | 128 | 1761 AD | n/a |
| Industrial | Ballistics | 7245/8000 | 137 | 1794 AD | n/a |
| Industrial | Industrialization | 7463/8000 | 122 | 1720 AD | 1769 AD |
| Industrial | Railways | 6985/8000 | 145 | 1824 AD | 1825 AD |
| Machine Age | Replaceable Parts | 6473/8000 | 160 | 1880 AD | 1800 AD |
| Machine Age | Combustion | 6726/8000 | 153 | 1854 AD | 1876 AD |
| Machine Age | Steel | 6437/8000 | 158 | 1872 AD | 1856 AD |
| Machine Age | Flight | 6034/8000 | 168 | 1889 AD | 1903 AD |
| Machine Age | Electricity | 5777/8000 | 173 | 1894 AD | 1879 AD |
| World War | Mass Mobilisation | 4763/8000 | 188 | 1911 AD | n/a |
| World War | Rocketry | 4448/8000 | 192 | 1916 AD | 1926 AD |
| World War | Armoured Warfare | 4095/8000 | 197 | 1924 AD | 1916 AD |
| World War | Air Power | 3888/8000 | 200 | 1928 AD | n/a |
| World War | Electronics | 5319/8000 | 180 | 1902 AD | n/a |
| Modern | Mechanised Warfare | 2494/8000 | 217 | 1954 AD | n/a |
| Modern | Composite Armour | 2176/8000 | 220 | 1962 AD | n/a |
| Modern | Guided Missiles | 1943/8000 | 225 | 1974 AD | n/a |
| Modern | Jet Engines | 1879/8000 | 226 | 1976 AD | 1939 AD |
| Modern | Green Revolution | 3071/8000 | 210 | 1942 AD | n/a |
| Future | Autonomous Systems | 925/8000 | 244 | 2016 AD | n/a |
| Future | Robotics | 718/8000 | 249 | 2024 AD | n/a |
| Future | Fusion Power | 693/8000 | 253 | 2029 AD | n/a |
| Future | Stealth Technology | 1137/8000 | 239 | 2007 AD | n/a |
| Future | Recursive AI | 612/8000 | 256 | 2032 AD | n/a |

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

1000 games with promotion telemetry. Promotions are UnitPromoted events by the level reached; final units are military units alive when the game ended.

| Level | Promotions / game | Final units / game |
| --- | --- | --- |
| 0 | 0.00 | 70.70 |
| 1 | 63.58 | 16.37 |
| 2 | 22.09 | 5.67 |
| 3 | 5.88 | 1.74 |
| 4 | 1.14 | 0.38 |
| 5 | 0.23 | 0.10 |

## Per-game events

Each row is one observed first play; unavailable telemetry is shown as n/a. Full per-player/team score sources and turn-50 standings are in JSON.

| Game | Seed | Resolved turns | Battles | City captures | Settler captures | Eliminations |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 9311003 | 216 | 1031 | 21 | 6 | 2 |
| 1 | 9311004 | 246 | 1349 | 26 | 4 | 2 |
| 2 | 9311005 | 201 | 854 | 21 | 6 | 1 |
| 3 | 9311006 | 208 | 1023 | 19 | 10 | 2 |
| 4 | 9311007 | 202 | 678 | 15 | 5 | 0 |
| 5 | 9311008 | 238 | 1102 | 31 | 3 | 2 |
| 6 | 9311009 | 228 | 1209 | 24 | 14 | 1 |
| 7 | 9311010 | 188 | 864 | 15 | 8 | 1 |
| 8 | 9311011 | 235 | 717 | 11 | 5 | 4 |
| 9 | 9311012 | 207 | 969 | 25 | 4 | 3 |
| 10 | 9311013 | 237 | 1214 | 30 | 12 | 5 |
| 11 | 9311014 | 249 | 1681 | 27 | 2 | 3 |
| 12 | 9311015 | 185 | 722 | 16 | 3 | 2 |
| 13 | 9311016 | 222 | 1158 | 21 | 6 | 1 |
| 14 | 9311017 | 261 | 1250 | 28 | 10 | 4 |
| 15 | 9311018 | 185 | 828 | 20 | 11 | 2 |
| 16 | 9311019 | 278 | 1061 | 21 | 12 | 3 |
| 17 | 9311020 | 241 | 1040 | 24 | 11 | 1 |
| 18 | 9311021 | 211 | 1018 | 22 | 7 | 5 |
| 19 | 9311022 | 181 | 758 | 14 | 3 | 0 |
| 20 | 9311023 | 274 | 1457 | 30 | 5 | 2 |
| 21 | 9311024 | 184 | 761 | 11 | 1 | 1 |
| 22 | 9311025 | 218 | 990 | 12 | 11 | 0 |
| 23 | 9311026 | 193 | 981 | 19 | 3 | 2 |
| 24 | 9311027 | 241 | 1056 | 23 | 7 | 2 |
| 25 | 9311028 | 207 | 950 | 29 | 5 | 2 |
| 26 | 9311029 | 175 | 586 | 12 | 6 | 2 |
| 27 | 9311030 | 189 | 680 | 20 | 8 | 2 |
| 28 | 9311031 | 190 | 813 | 18 | 4 | 2 |
| 29 | 9311032 | 240 | 1076 | 30 | 11 | 1 |
| 30 | 9311033 | 254 | 1112 | 27 | 5 | 2 |
| 31 | 9311034 | 241 | 1378 | 27 | 16 | 0 |
| 32 | 9311035 | 213 | 1060 | 26 | 3 | 2 |
| 33 | 9311036 | 231 | 1116 | 20 | 14 | 2 |
| 34 | 9311037 | 244 | 1100 | 32 | 10 | 1 |
| 35 | 9311038 | 216 | 1289 | 27 | 2 | 3 |
| 36 | 9311039 | 318 | 1881 | 37 | 4 | 3 |
| 37 | 9311040 | 201 | 1088 | 9 | 2 | 0 |
| 38 | 9311041 | 197 | 946 | 21 | 7 | 1 |
| 39 | 9311042 | 200 | 785 | 18 | 6 | 2 |
| 40 | 9311043 | 293 | 1579 | 38 | 4 | 4 |
| 41 | 9311044 | 253 | 1235 | 26 | 5 | 2 |
| 42 | 9311045 | 281 | 1530 | 30 | 4 | 3 |
| 43 | 9311046 | 207 | 981 | 12 | 3 | 2 |
| 44 | 9311047 | 178 | 695 | 16 | 5 | 2 |
| 45 | 9311048 | 212 | 813 | 10 | 8 | 1 |
| 46 | 9311049 | 236 | 1329 | 21 | 3 | 2 |
| 47 | 9311050 | 181 | 550 | 10 | 5 | 2 |
| 48 | 9311051 | 284 | 1719 | 46 | 6 | 3 |
| 49 | 9311052 | 232 | 1203 | 35 | 6 | 2 |
| 50 | 9311053 | 211 | 960 | 29 | 7 | 1 |
| 51 | 9311054 | 258 | 1017 | 26 | 15 | 5 |
| 52 | 9311055 | 210 | 1017 | 16 | 6 | 3 |
| 53 | 9311056 | 241 | 1243 | 17 | 1 | 1 |
| 54 | 9311057 | 195 | 798 | 14 | 7 | 0 |
| 55 | 9311058 | 186 | 705 | 11 | 5 | 0 |
| 56 | 9311059 | 182 | 686 | 12 | 5 | 2 |
| 57 | 9311060 | 246 | 1135 | 24 | 7 | 4 |
| 58 | 9311061 | 161 | 579 | 12 | 1 | 1 |
| 59 | 9311062 | 221 | 973 | 26 | 5 | 3 |
| 60 | 9311063 | 206 | 754 | 13 | 2 | 2 |
| 61 | 9311064 | 246 | 890 | 19 | 4 | 4 |
| 62 | 9311065 | 301 | 1613 | 44 | 3 | 5 |
| 63 | 9311066 | 229 | 1224 | 17 | 8 | 1 |
| 64 | 9311067 | 219 | 913 | 20 | 6 | 2 |
| 65 | 9311068 | 194 | 968 | 25 | 6 | 2 |
| 66 | 9311069 | 213 | 907 | 37 | 3 | 2 |
| 67 | 9311070 | 242 | 1101 | 30 | 4 | 1 |
| 68 | 9311071 | 220 | 887 | 27 | 0 | 1 |
| 69 | 9311072 | 246 | 1015 | 22 | 8 | 4 |
| 70 | 9311073 | 178 | 756 | 11 | 6 | 2 |
| 71 | 9311074 | 182 | 849 | 23 | 5 | 2 |
| 72 | 9311075 | 220 | 1036 | 25 | 13 | 2 |
| 73 | 9311076 | 239 | 1091 | 25 | 1 | 1 |
| 74 | 9311077 | 323 | 1939 | 29 | 10 | 2 |
| 75 | 9311078 | 167 | 641 | 17 | 3 | 3 |
| 76 | 9311079 | 272 | 1525 | 31 | 8 | 2 |
| 77 | 9311080 | 205 | 950 | 19 | 7 | 2 |
| 78 | 9311081 | 213 | 983 | 25 | 11 | 2 |
| 79 | 9311082 | 246 | 1119 | 32 | 7 | 2 |
| 80 | 9311083 | 249 | 1148 | 27 | 5 | 1 |
| 81 | 9311084 | 182 | 908 | 16 | 7 | 3 |
| 82 | 9311085 | 160 | 633 | 16 | 8 | 0 |
| 83 | 9311086 | 271 | 1418 | 36 | 4 | 3 |
| 84 | 9311087 | 231 | 1109 | 15 | 3 | 2 |
| 85 | 9311088 | 224 | 976 | 32 | 2 | 3 |
| 86 | 9311089 | 190 | 924 | 32 | 1 | 3 |
| 87 | 9311090 | 210 | 899 | 26 | 10 | 4 |
| 88 | 9311091 | 329 | 1767 | 57 | 2 | 2 |
| 89 | 9311092 | 226 | 987 | 23 | 6 | 0 |
| 90 | 9311093 | 232 | 950 | 23 | 5 | 3 |
| 91 | 9311094 | 276 | 1362 | 33 | 6 | 2 |
| 92 | 9311095 | 220 | 1083 | 16 | 7 | 2 |
| 93 | 9311096 | 217 | 1046 | 38 | 3 | 2 |
| 94 | 9311097 | 155 | 608 | 21 | 1 | 0 |
| 95 | 9311098 | 234 | 1203 | 11 | 6 | 0 |
| 96 | 9311099 | 202 | 989 | 17 | 2 | 2 |
| 97 | 9311100 | 213 | 982 | 19 | 7 | 3 |
| 98 | 9311101 | 181 | 710 | 11 | 4 | 1 |
| 99 | 9311102 | 213 | 974 | 25 | 6 | 1 |
| 100 | 9311103 | 236 | 1092 | 17 | 4 | 1 |
| 101 | 9311104 | 267 | 1495 | 23 | 6 | 4 |
| 102 | 9311105 | 196 | 1105 | 18 | 4 | 0 |
| 103 | 9311106 | 215 | 1085 | 24 | 2 | 3 |
| 104 | 9311107 | 217 | 1098 | 37 | 3 | 3 |
| 105 | 9311108 | 153 | 648 | 14 | 2 | 1 |
| 106 | 9311109 | 212 | 914 | 23 | 4 | 3 |
| 107 | 9311110 | 188 | 741 | 10 | 8 | 0 |
| 108 | 9311111 | 198 | 814 | 19 | 3 | 4 |
| 109 | 9311112 | 215 | 1236 | 25 | 4 | 1 |
| 110 | 9311113 | 192 | 1034 | 24 | 4 | 1 |
| 111 | 9311114 | 261 | 1410 | 35 | 4 | 1 |
| 112 | 9311115 | 219 | 966 | 15 | 2 | 0 |
| 113 | 9311116 | 177 | 804 | 13 | 1 | 2 |
| 114 | 9311117 | 261 | 1202 | 28 | 2 | 3 |
| 115 | 9311118 | 242 | 1213 | 15 | 4 | 2 |
| 116 | 9311119 | 330 | 1541 | 19 | 10 | 4 |
| 117 | 9311120 | 182 | 690 | 23 | 6 | 4 |
| 118 | 9311121 | 244 | 1111 | 21 | 9 | 3 |
| 119 | 9311122 | 210 | 1083 | 19 | 6 | 2 |
| 120 | 9311123 | 237 | 1257 | 21 | 1 | 1 |
| 121 | 9311124 | 250 | 1039 | 16 | 6 | 2 |
| 122 | 9311125 | 229 | 690 | 14 | 3 | 1 |
| 123 | 9311126 | 248 | 844 | 17 | 14 | 1 |
| 124 | 9311127 | 221 | 1090 | 20 | 4 | 1 |
| 125 | 9311128 | 160 | 444 | 14 | 4 | 1 |
| 126 | 9311129 | 416 | 2264 | 49 | 1 | 2 |
| 127 | 9311130 | 146 | 336 | 5 | 6 | 0 |
| 128 | 9311131 | 194 | 941 | 20 | 8 | 1 |
| 129 | 9311132 | 158 | 588 | 9 | 4 | 1 |
| 130 | 9311133 | 171 | 814 | 12 | 4 | 1 |
| 131 | 9311134 | 244 | 1223 | 26 | 8 | 3 |
| 132 | 9311135 | 194 | 830 | 21 | 4 | 2 |
| 133 | 9311136 | 243 | 1240 | 38 | 8 | 5 |
| 134 | 9311137 | 216 | 808 | 22 | 2 | 1 |
| 135 | 9311138 | 263 | 1463 | 36 | 10 | 3 |
| 136 | 9311139 | 239 | 1142 | 27 | 5 | 3 |
| 137 | 9311140 | 188 | 652 | 19 | 8 | 1 |
| 138 | 9311141 | 242 | 1113 | 18 | 4 | 1 |
| 139 | 9311142 | 234 | 1143 | 25 | 8 | 1 |
| 140 | 9311143 | 267 | 1328 | 34 | 2 | 2 |
| 141 | 9311144 | 180 | 867 | 15 | 7 | 0 |
| 142 | 9311145 | 220 | 894 | 11 | 6 | 0 |
| 143 | 9311146 | 194 | 951 | 21 | 7 | 2 |
| 144 | 9311147 | 181 | 755 | 20 | 1 | 1 |
| 145 | 9311148 | 240 | 914 | 35 | 2 | 3 |
| 146 | 9311149 | 192 | 871 | 9 | 4 | 1 |
| 147 | 9311150 | 208 | 1096 | 22 | 5 | 1 |
| 148 | 9311151 | 223 | 952 | 23 | 12 | 2 |
| 149 | 9311152 | 205 | 1030 | 27 | 8 | 4 |
| 150 | 9311153 | 195 | 602 | 10 | 3 | 1 |
| 151 | 9311154 | 243 | 1540 | 21 | 6 | 1 |
| 152 | 9311155 | 276 | 1408 | 34 | 4 | 3 |
| 153 | 9311156 | 211 | 945 | 27 | 4 | 3 |
| 154 | 9311157 | 194 | 987 | 7 | 7 | 2 |
| 155 | 9311158 | 216 | 1048 | 15 | 5 | 2 |
| 156 | 9311159 | 279 | 1437 | 34 | 5 | 3 |
| 157 | 9311160 | 237 | 1311 | 34 | 3 | 2 |
| 158 | 9311161 | 207 | 1039 | 27 | 5 | 2 |
| 159 | 9311162 | 193 | 817 | 16 | 9 | 1 |
| 160 | 9311163 | 211 | 1024 | 29 | 7 | 1 |
| 161 | 9311164 | 236 | 1250 | 36 | 1 | 3 |
| 162 | 9311165 | 203 | 971 | 16 | 6 | 1 |
| 163 | 9311166 | 233 | 1072 | 19 | 3 | 3 |
| 164 | 9311167 | 230 | 754 | 27 | 7 | 2 |
| 165 | 9311168 | 240 | 1229 | 18 | 4 | 3 |
| 166 | 9311169 | 238 | 1200 | 14 | 4 | 1 |
| 167 | 9311170 | 238 | 1095 | 31 | 3 | 4 |
| 168 | 9311171 | 218 | 1214 | 27 | 5 | 2 |
| 169 | 9311172 | 240 | 831 | 15 | 4 | 4 |
| 170 | 9311173 | 297 | 1506 | 25 | 6 | 2 |
| 171 | 9311174 | 214 | 845 | 17 | 6 | 4 |
| 172 | 9311175 | 273 | 1478 | 37 | 9 | 4 |
| 173 | 9311176 | 214 | 1015 | 19 | 2 | 2 |
| 174 | 9311177 | 228 | 976 | 22 | 1 | 2 |
| 175 | 9311178 | 160 | 554 | 12 | 3 | 0 |
| 176 | 9311179 | 270 | 1036 | 28 | 10 | 3 |
| 177 | 9311180 | 264 | 1246 | 27 | 2 | 2 |
| 178 | 9311181 | 209 | 888 | 27 | 0 | 3 |
| 179 | 9311182 | 213 | 833 | 25 | 5 | 3 |
| 180 | 9311183 | 252 | 1338 | 27 | 12 | 4 |
| 181 | 9311184 | 255 | 1108 | 21 | 9 | 4 |
| 182 | 9311185 | 242 | 970 | 38 | 0 | 3 |
| 183 | 9311186 | 219 | 937 | 22 | 5 | 0 |
| 184 | 9311187 | 199 | 789 | 13 | 4 | 3 |
| 185 | 9311188 | 182 | 887 | 13 | 5 | 3 |
| 186 | 9311189 | 198 | 1086 | 16 | 7 | 2 |
| 187 | 9311190 | 194 | 834 | 20 | 5 | 1 |
| 188 | 9311191 | 249 | 1287 | 31 | 6 | 3 |
| 189 | 9311192 | 177 | 571 | 12 | 5 | 1 |
| 190 | 9311193 | 179 | 820 | 16 | 3 | 1 |
| 191 | 9311194 | 295 | 1466 | 21 | 7 | 5 |
| 192 | 9311195 | 174 | 776 | 15 | 4 | 1 |
| 193 | 9311196 | 246 | 1145 | 26 | 2 | 2 |
| 194 | 9311197 | 204 | 1098 | 30 | 7 | 3 |
| 195 | 9311198 | 248 | 1208 | 25 | 19 | 3 |
| 196 | 9311199 | 261 | 1269 | 22 | 4 | 1 |
| 197 | 9311200 | 222 | 1044 | 22 | 6 | 1 |
| 198 | 9311201 | 233 | 1248 | 27 | 7 | 3 |
| 199 | 9311202 | 216 | 1218 | 20 | 8 | 3 |
| 200 | 9311203 | 229 | 1097 | 14 | 8 | 3 |
| 201 | 9311204 | 247 | 1264 | 12 | 5 | 1 |
| 202 | 9311205 | 259 | 1259 | 30 | 2 | 4 |
| 203 | 9311206 | 174 | 579 | 18 | 5 | 1 |
| 204 | 9311207 | 213 | 1261 | 29 | 4 | 1 |
| 205 | 9311208 | 209 | 965 | 25 | 8 | 2 |
| 206 | 9311209 | 188 | 763 | 17 | 3 | 2 |
| 207 | 9311210 | 270 | 1499 | 26 | 3 | 1 |
| 208 | 9311211 | 217 | 1132 | 32 | 4 | 3 |
| 209 | 9311212 | 193 | 602 | 10 | 4 | 1 |
| 210 | 9311213 | 369 | 2189 | 62 | 3 | 2 |
| 211 | 9311214 | 163 | 709 | 12 | 13 | 1 |
| 212 | 9311215 | 177 | 746 | 14 | 9 | 1 |
| 213 | 9311216 | 203 | 865 | 18 | 3 | 0 |
| 214 | 9311217 | 184 | 777 | 11 | 5 | 0 |
| 215 | 9311218 | 222 | 818 | 14 | 5 | 2 |
| 216 | 9311219 | 186 | 713 | 21 | 2 | 1 |
| 217 | 9311220 | 205 | 857 | 28 | 5 | 1 |
| 218 | 9311221 | 224 | 1211 | 26 | 8 | 2 |
| 219 | 9311222 | 299 | 1864 | 27 | 3 | 1 |
| 220 | 9311223 | 232 | 951 | 19 | 10 | 3 |
| 221 | 9311224 | 234 | 1043 | 18 | 7 | 2 |
| 222 | 9311225 | 199 | 762 | 22 | 4 | 2 |
| 223 | 9311226 | 160 | 639 | 13 | 6 | 1 |
| 224 | 9311227 | 201 | 827 | 12 | 7 | 3 |
| 225 | 9311228 | 217 | 1188 | 25 | 6 | 1 |
| 226 | 9311229 | 232 | 1152 | 30 | 6 | 4 |
| 227 | 9311230 | 357 | 2056 | 25 | 4 | 2 |
| 228 | 9311231 | 204 | 893 | 23 | 6 | 2 |
| 229 | 9311232 | 282 | 1245 | 22 | 4 | 3 |
| 230 | 9311233 | 234 | 1227 | 28 | 4 | 3 |
| 231 | 9311234 | 175 | 757 | 30 | 3 | 2 |
| 232 | 9311235 | 219 | 1018 | 24 | 5 | 3 |
| 233 | 9311236 | 233 | 1222 | 20 | 2 | 0 |
| 234 | 9311237 | 301 | 1629 | 28 | 5 | 3 |
| 235 | 9311238 | 184 | 563 | 13 | 2 | 3 |
| 236 | 9311239 | 228 | 932 | 23 | 4 | 0 |
| 237 | 9311240 | 224 | 976 | 23 | 3 | 2 |
| 238 | 9311241 | 233 | 921 | 24 | 7 | 5 |
| 239 | 9311242 | 247 | 1038 | 12 | 9 | 4 |
| 240 | 9311243 | 238 | 921 | 34 | 3 | 4 |
| 241 | 9311244 | 248 | 1123 | 17 | 3 | 3 |
| 242 | 9311245 | 219 | 994 | 19 | 5 | 3 |
| 243 | 9311246 | 219 | 1108 | 32 | 5 | 2 |
| 244 | 9311247 | 211 | 1125 | 29 | 9 | 3 |
| 245 | 9311248 | 224 | 752 | 18 | 8 | 2 |
| 246 | 9311249 | 216 | 814 | 32 | 4 | 1 |
| 247 | 9311250 | 230 | 1100 | 24 | 9 | 2 |
| 248 | 9311251 | 272 | 1480 | 26 | 3 | 2 |
| 249 | 9311252 | 225 | 950 | 19 | 8 | 2 |
| 250 | 9311253 | 198 | 957 | 17 | 4 | 1 |
| 251 | 9311254 | 221 | 1026 | 15 | 2 | 0 |
| 252 | 9311255 | 262 | 1592 | 46 | 8 | 2 |
| 253 | 9311256 | 210 | 1025 | 28 | 1 | 1 |
| 254 | 9311257 | 206 | 953 | 16 | 7 | 2 |
| 255 | 9311258 | 222 | 1033 | 23 | 4 | 1 |
| 256 | 9311259 | 238 | 898 | 15 | 0 | 0 |
| 257 | 9311260 | 246 | 963 | 22 | 8 | 4 |
| 258 | 9311261 | 217 | 1192 | 32 | 2 | 3 |
| 259 | 9311262 | 236 | 1040 | 14 | 9 | 2 |
| 260 | 9311263 | 218 | 1058 | 24 | 3 | 1 |
| 261 | 9311264 | 201 | 758 | 23 | 1 | 1 |
| 262 | 9311265 | 235 | 1078 | 28 | 5 | 4 |
| 263 | 9311266 | 189 | 734 | 13 | 6 | 2 |
| 264 | 9311267 | 278 | 1082 | 22 | 10 | 2 |
| 265 | 9311268 | 231 | 1310 | 19 | 6 | 0 |
| 266 | 9311269 | 247 | 989 | 30 | 5 | 3 |
| 267 | 9311270 | 230 | 1359 | 18 | 8 | 3 |
| 268 | 9311271 | 246 | 882 | 10 | 3 | 3 |
| 269 | 9311272 | 191 | 820 | 20 | 7 | 4 |
| 270 | 9311273 | 230 | 1183 | 28 | 5 | 0 |
| 271 | 9311274 | 272 | 1350 | 19 | 3 | 4 |
| 272 | 9311275 | 243 | 813 | 25 | 3 | 3 |
| 273 | 9311276 | 244 | 1031 | 23 | 4 | 0 |
| 274 | 9311277 | 217 | 1136 | 32 | 7 | 2 |
| 275 | 9311278 | 275 | 1467 | 22 | 8 | 3 |
| 276 | 9311279 | 264 | 1417 | 37 | 3 | 2 |
| 277 | 9311280 | 237 | 1267 | 18 | 5 | 1 |
| 278 | 9311281 | 233 | 1202 | 27 | 4 | 3 |
| 279 | 9311282 | 214 | 808 | 17 | 1 | 2 |
| 280 | 9311283 | 276 | 1425 | 35 | 4 | 1 |
| 281 | 9311284 | 180 | 649 | 22 | 3 | 2 |
| 282 | 9311285 | 208 | 1150 | 17 | 3 | 2 |
| 283 | 9311286 | 229 | 1014 | 17 | 6 | 1 |
| 284 | 9311287 | 203 | 926 | 16 | 4 | 5 |
| 285 | 9311288 | 251 | 1274 | 15 | 4 | 2 |
| 286 | 9311289 | 211 | 1044 | 14 | 8 | 4 |
| 287 | 9311290 | 225 | 1082 | 21 | 2 | 1 |
| 288 | 9311291 | 201 | 883 | 13 | 3 | 0 |
| 289 | 9311292 | 246 | 1346 | 23 | 2 | 4 |
| 290 | 9311293 | 200 | 1024 | 22 | 7 | 1 |
| 291 | 9311294 | 218 | 849 | 18 | 6 | 2 |
| 292 | 9311295 | 202 | 875 | 15 | 6 | 3 |
| 293 | 9311296 | 221 | 958 | 20 | 8 | 2 |
| 294 | 9311297 | 201 | 898 | 25 | 1 | 2 |
| 295 | 9311298 | 183 | 607 | 11 | 4 | 2 |
| 296 | 9311299 | 223 | 1087 | 17 | 8 | 2 |
| 297 | 9311300 | 230 | 974 | 24 | 11 | 1 |
| 298 | 9311301 | 177 | 730 | 15 | 7 | 2 |
| 299 | 9311302 | 251 | 1350 | 28 | 8 | 5 |
| 300 | 9311303 | 253 | 1337 | 29 | 10 | 1 |
| 301 | 9311304 | 228 | 730 | 15 | 3 | 2 |
| 302 | 9311305 | 193 | 705 | 15 | 4 | 1 |
| 303 | 9311306 | 245 | 997 | 24 | 4 | 1 |
| 304 | 9311307 | 265 | 1333 | 32 | 2 | 1 |
| 305 | 9311308 | 219 | 1021 | 20 | 3 | 2 |
| 306 | 9311309 | 209 | 1097 | 22 | 9 | 3 |
| 307 | 9311310 | 223 | 1119 | 14 | 4 | 2 |
| 308 | 9311311 | 287 | 1730 | 35 | 3 | 2 |
| 309 | 9311312 | 215 | 1112 | 17 | 8 | 2 |
| 310 | 9311313 | 199 | 834 | 24 | 5 | 3 |
| 311 | 9311314 | 239 | 957 | 29 | 8 | 2 |
| 312 | 9311315 | 204 | 1069 | 18 | 2 | 2 |
| 313 | 9311316 | 275 | 982 | 28 | 5 | 3 |
| 314 | 9311317 | 250 | 900 | 16 | 2 | 2 |
| 315 | 9311318 | 207 | 1001 | 19 | 5 | 1 |
| 316 | 9311319 | 239 | 1138 | 43 | 6 | 2 |
| 317 | 9311320 | 220 | 1043 | 19 | 6 | 1 |
| 318 | 9311321 | 179 | 846 | 15 | 2 | 0 |
| 319 | 9311322 | 202 | 808 | 21 | 16 | 2 |
| 320 | 9311323 | 253 | 1165 | 26 | 5 | 2 |
| 321 | 9311324 | 247 | 1133 | 24 | 5 | 1 |
| 322 | 9311325 | 254 | 1189 | 24 | 5 | 3 |
| 323 | 9311326 | 262 | 1450 | 16 | 8 | 1 |
| 324 | 9311327 | 224 | 890 | 24 | 3 | 2 |
| 325 | 9311328 | 247 | 1427 | 25 | 6 | 2 |
| 326 | 9311329 | 194 | 729 | 24 | 3 | 1 |
| 327 | 9311330 | 198 | 931 | 17 | 10 | 0 |
| 328 | 9311331 | 272 | 1385 | 31 | 2 | 2 |
| 329 | 9311332 | 207 | 580 | 18 | 3 | 2 |
| 330 | 9311333 | 254 | 1144 | 20 | 4 | 0 |
| 331 | 9311334 | 171 | 584 | 16 | 1 | 2 |
| 332 | 9311335 | 206 | 818 | 14 | 9 | 3 |
| 333 | 9311336 | 289 | 1523 | 38 | 8 | 2 |
| 334 | 9311337 | 191 | 866 | 17 | 10 | 1 |
| 335 | 9311338 | 239 | 1272 | 30 | 5 | 1 |
| 336 | 9311339 | 207 | 788 | 12 | 6 | 2 |
| 337 | 9311340 | 222 | 1032 | 25 | 9 | 2 |
| 338 | 9311341 | 221 | 968 | 22 | 7 | 3 |
| 339 | 9311342 | 244 | 1036 | 23 | 2 | 2 |
| 340 | 9311343 | 215 | 919 | 15 | 4 | 2 |
| 341 | 9311344 | 214 | 726 | 20 | 8 | 3 |
| 342 | 9311345 | 196 | 917 | 14 | 4 | 0 |
| 343 | 9311346 | 275 | 1565 | 30 | 13 | 5 |
| 344 | 9311347 | 192 | 825 | 16 | 3 | 3 |
| 345 | 9311348 | 285 | 1369 | 31 | 6 | 2 |
| 346 | 9311349 | 237 | 1065 | 13 | 4 | 1 |
| 347 | 9311350 | 248 | 1227 | 27 | 3 | 3 |
| 348 | 9311351 | 199 | 819 | 24 | 4 | 4 |
| 349 | 9311352 | 205 | 598 | 16 | 11 | 1 |
| 350 | 9311353 | 270 | 1488 | 36 | 2 | 4 |
| 351 | 9311354 | 174 | 633 | 19 | 6 | 0 |
| 352 | 9311355 | 219 | 1136 | 33 | 5 | 3 |
| 353 | 9311356 | 201 | 983 | 17 | 7 | 1 |
| 354 | 9311357 | 234 | 1105 | 27 | 8 | 1 |
| 355 | 9311358 | 318 | 1649 | 43 | 8 | 4 |
| 356 | 9311359 | 253 | 1212 | 21 | 1 | 4 |
| 357 | 9311360 | 198 | 856 | 19 | 8 | 2 |
| 358 | 9311361 | 232 | 1189 | 20 | 6 | 3 |
| 359 | 9311362 | 176 | 611 | 9 | 6 | 0 |
| 360 | 9311363 | 185 | 850 | 16 | 5 | 0 |
| 361 | 9311364 | 218 | 1040 | 25 | 6 | 1 |
| 362 | 9311365 | 245 | 1122 | 22 | 5 | 2 |
| 363 | 9311366 | 229 | 1061 | 27 | 9 | 3 |
| 364 | 9311367 | 216 | 850 | 17 | 4 | 3 |
| 365 | 9311368 | 217 | 1086 | 16 | 2 | 2 |
| 366 | 9311369 | 184 | 707 | 12 | 5 | 0 |
| 367 | 9311370 | 228 | 692 | 21 | 4 | 1 |
| 368 | 9311371 | 254 | 1409 | 18 | 5 | 1 |
| 369 | 9311372 | 244 | 1260 | 38 | 7 | 4 |
| 370 | 9311373 | 215 | 984 | 28 | 10 | 2 |
| 371 | 9311374 | 221 | 947 | 21 | 6 | 1 |
| 372 | 9311375 | 176 | 722 | 13 | 6 | 0 |
| 373 | 9311376 | 200 | 884 | 11 | 2 | 2 |
| 374 | 9311377 | 256 | 1311 | 35 | 7 | 1 |
| 375 | 9311378 | 177 | 668 | 5 | 3 | 1 |
| 376 | 9311379 | 227 | 819 | 20 | 3 | 1 |
| 377 | 9311380 | 333 | 1836 | 50 | 2 | 3 |
| 378 | 9311381 | 224 | 949 | 17 | 3 | 1 |
| 379 | 9311382 | 205 | 1010 | 16 | 4 | 1 |
| 380 | 9311383 | 248 | 1163 | 25 | 9 | 5 |
| 381 | 9311384 | 253 | 1244 | 21 | 10 | 2 |
| 382 | 9311385 | 217 | 957 | 16 | 2 | 1 |
| 383 | 9311386 | 188 | 830 | 22 | 2 | 2 |
| 384 | 9311387 | 240 | 1164 | 31 | 3 | 2 |
| 385 | 9311388 | 283 | 1402 | 26 | 4 | 2 |
| 386 | 9311389 | 183 | 728 | 16 | 7 | 0 |
| 387 | 9311390 | 244 | 1366 | 28 | 6 | 2 |
| 388 | 9311391 | 222 | 1101 | 20 | 2 | 3 |
| 389 | 9311392 | 169 | 704 | 18 | 2 | 2 |
| 390 | 9311393 | 190 | 826 | 18 | 2 | 2 |
| 391 | 9311394 | 274 | 1387 | 26 | 5 | 4 |
| 392 | 9311395 | 189 | 797 | 20 | 5 | 2 |
| 393 | 9311396 | 245 | 899 | 24 | 5 | 3 |
| 394 | 9311397 | 220 | 1132 | 31 | 5 | 3 |
| 395 | 9311398 | 192 | 958 | 22 | 2 | 2 |
| 396 | 9311399 | 192 | 920 | 22 | 1 | 2 |
| 397 | 9311400 | 211 | 661 | 7 | 3 | 2 |
| 398 | 9311401 | 312 | 1720 | 34 | 7 | 4 |
| 399 | 9311402 | 210 | 1087 | 26 | 6 | 0 |
| 400 | 9311403 | 189 | 784 | 12 | 4 | 1 |
| 401 | 9311404 | 222 | 970 | 21 | 4 | 2 |
| 402 | 9311405 | 215 | 1227 | 25 | 2 | 4 |
| 403 | 9311406 | 225 | 952 | 21 | 4 | 2 |
| 404 | 9311407 | 226 | 985 | 24 | 4 | 2 |
| 405 | 9311408 | 240 | 1313 | 22 | 2 | 3 |
| 406 | 9311409 | 218 | 1151 | 25 | 4 | 1 |
| 407 | 9311410 | 196 | 1030 | 19 | 9 | 1 |
| 408 | 9311411 | 207 | 610 | 20 | 2 | 2 |
| 409 | 9311412 | 245 | 1363 | 24 | 6 | 4 |
| 410 | 9311413 | 225 | 872 | 22 | 0 | 3 |
| 411 | 9311414 | 194 | 828 | 24 | 3 | 1 |
| 412 | 9311415 | 263 | 1410 | 33 | 5 | 2 |
| 413 | 9311416 | 205 | 1059 | 18 | 7 | 1 |
| 414 | 9311417 | 252 | 1254 | 32 | 3 | 1 |
| 415 | 9311418 | 196 | 900 | 28 | 5 | 1 |
| 416 | 9311419 | 227 | 815 | 20 | 4 | 3 |
| 417 | 9311420 | 230 | 959 | 28 | 6 | 4 |
| 418 | 9311421 | 295 | 1439 | 23 | 3 | 2 |
| 419 | 9311422 | 179 | 797 | 12 | 6 | 1 |
| 420 | 9311423 | 221 | 1090 | 27 | 1 | 3 |
| 421 | 9311424 | 222 | 1182 | 11 | 5 | 0 |
| 422 | 9311425 | 295 | 1259 | 28 | 7 | 3 |
| 423 | 9311426 | 200 | 847 | 12 | 2 | 1 |
| 424 | 9311427 | 226 | 997 | 25 | 3 | 4 |
| 425 | 9311428 | 208 | 738 | 16 | 6 | 3 |
| 426 | 9311429 | 257 | 861 | 30 | 10 | 5 |
| 427 | 9311430 | 194 | 858 | 22 | 4 | 2 |
| 428 | 9311431 | 203 | 858 | 4 | 3 | 1 |
| 429 | 9311432 | 211 | 951 | 20 | 2 | 2 |
| 430 | 9311433 | 252 | 1122 | 25 | 1 | 2 |
| 431 | 9311434 | 229 | 1128 | 24 | 6 | 3 |
| 432 | 9311435 | 284 | 1679 | 33 | 7 | 2 |
| 433 | 9311436 | 263 | 1220 | 25 | 3 | 4 |
| 434 | 9311437 | 238 | 1174 | 28 | 3 | 2 |
| 435 | 9311438 | 197 | 676 | 12 | 3 | 0 |
| 436 | 9311439 | 178 | 768 | 13 | 7 | 0 |
| 437 | 9311440 | 177 | 654 | 11 | 2 | 1 |
| 438 | 9311441 | 316 | 1688 | 45 | 11 | 4 |
| 439 | 9311442 | 273 | 1512 | 28 | 10 | 2 |
| 440 | 9311443 | 198 | 1054 | 17 | 5 | 5 |
| 441 | 9311444 | 228 | 1091 | 35 | 10 | 3 |
| 442 | 9311445 | 171 | 714 | 5 | 5 | 2 |
| 443 | 9311446 | 225 | 1105 | 25 | 4 | 1 |
| 444 | 9311447 | 193 | 873 | 16 | 4 | 1 |
| 445 | 9311448 | 231 | 1214 | 25 | 9 | 3 |
| 446 | 9311449 | 189 | 1051 | 17 | 2 | 1 |
| 447 | 9311450 | 246 | 1273 | 35 | 1 | 3 |
| 448 | 9311451 | 213 | 1019 | 25 | 1 | 2 |
| 449 | 9311452 | 198 | 932 | 25 | 5 | 2 |
| 450 | 9311453 | 224 | 932 | 12 | 5 | 1 |
| 451 | 9311454 | 257 | 995 | 22 | 5 | 1 |
| 452 | 9311455 | 214 | 1064 | 35 | 7 | 3 |
| 453 | 9311456 | 222 | 1190 | 25 | 1 | 3 |
| 454 | 9311457 | 239 | 1077 | 22 | 6 | 6 |
| 455 | 9311458 | 204 | 812 | 19 | 6 | 2 |
| 456 | 9311459 | 257 | 975 | 32 | 8 | 4 |
| 457 | 9311460 | 239 | 1364 | 17 | 5 | 2 |
| 458 | 9311461 | 230 | 1066 | 24 | 3 | 2 |
| 459 | 9311462 | 233 | 1191 | 15 | 5 | 2 |
| 460 | 9311463 | 271 | 1228 | 46 | 4 | 5 |
| 461 | 9311464 | 253 | 1201 | 26 | 7 | 3 |
| 462 | 9311465 | 224 | 1213 | 30 | 2 | 2 |
| 463 | 9311466 | 237 | 1175 | 32 | 4 | 2 |
| 464 | 9311467 | 226 | 1001 | 22 | 3 | 3 |
| 465 | 9311468 | 207 | 725 | 20 | 3 | 1 |
| 466 | 9311469 | 272 | 1213 | 16 | 8 | 0 |
| 467 | 9311470 | 271 | 1384 | 21 | 5 | 4 |
| 468 | 9311471 | 202 | 938 | 19 | 9 | 1 |
| 469 | 9311472 | 236 | 1068 | 23 | 5 | 1 |
| 470 | 9311473 | 228 | 943 | 28 | 7 | 4 |
| 471 | 9311474 | 252 | 1268 | 37 | 9 | 2 |
| 472 | 9311475 | 184 | 662 | 12 | 3 | 2 |
| 473 | 9311476 | 187 | 567 | 15 | 4 | 2 |
| 474 | 9311477 | 222 | 1235 | 23 | 3 | 1 |
| 475 | 9311478 | 225 | 1197 | 22 | 7 | 1 |
| 476 | 9311479 | 169 | 641 | 9 | 9 | 1 |
| 477 | 9311480 | 198 | 768 | 23 | 3 | 3 |
| 478 | 9311481 | 243 | 754 | 19 | 2 | 4 |
| 479 | 9311482 | 197 | 690 | 13 | 6 | 2 |
| 480 | 9311483 | 183 | 879 | 10 | 3 | 0 |
| 481 | 9311484 | 218 | 1081 | 13 | 15 | 4 |
| 482 | 9311485 | 178 | 783 | 20 | 3 | 0 |
| 483 | 9311486 | 274 | 1267 | 28 | 3 | 5 |
| 484 | 9311487 | 181 | 765 | 15 | 5 | 0 |
| 485 | 9311488 | 254 | 1103 | 16 | 11 | 3 |
| 486 | 9311489 | 308 | 1782 | 45 | 5 | 3 |
| 487 | 9311490 | 235 | 1279 | 38 | 3 | 1 |
| 488 | 9311491 | 198 | 1058 | 23 | 6 | 2 |
| 489 | 9311492 | 155 | 522 | 13 | 9 | 0 |
| 490 | 9311493 | 202 | 965 | 17 | 5 | 3 |
| 491 | 9311494 | 200 | 976 | 23 | 5 | 2 |
| 492 | 9311495 | 195 | 937 | 23 | 6 | 2 |
| 493 | 9311496 | 275 | 1532 | 27 | 3 | 1 |
| 494 | 9311497 | 165 | 592 | 12 | 1 | 2 |
| 495 | 9311498 | 184 | 957 | 19 | 1 | 1 |
| 496 | 9311499 | 204 | 852 | 15 | 6 | 0 |
| 497 | 9311500 | 237 | 1168 | 23 | 8 | 1 |
| 498 | 9311501 | 216 | 715 | 16 | 6 | 2 |
| 499 | 9311502 | 229 | 826 | 24 | 2 | 1 |
| 500 | 9311503 | 244 | 1150 | 12 | 10 | 2 |
| 501 | 9311504 | 215 | 967 | 24 | 6 | 1 |
| 502 | 9311505 | 237 | 1413 | 27 | 1 | 1 |
| 503 | 9311506 | 227 | 1126 | 32 | 6 | 1 |
| 504 | 9311507 | 211 | 1053 | 28 | 3 | 2 |
| 505 | 9311508 | 211 | 1075 | 17 | 5 | 2 |
| 506 | 9311509 | 240 | 989 | 21 | 8 | 2 |
| 507 | 9311510 | 241 | 1480 | 22 | 1 | 0 |
| 508 | 9311511 | 224 | 987 | 22 | 6 | 1 |
| 509 | 9311512 | 316 | 1595 | 33 | 8 | 3 |
| 510 | 9311513 | 181 | 696 | 10 | 7 | 1 |
| 511 | 9311514 | 197 | 880 | 28 | 3 | 1 |
| 512 | 9311515 | 178 | 730 | 19 | 5 | 2 |
| 513 | 9311516 | 282 | 1373 | 28 | 2 | 1 |
| 514 | 9311517 | 224 | 1144 | 15 | 3 | 0 |
| 515 | 9311518 | 333 | 2000 | 25 | 3 | 2 |
| 516 | 9311519 | 273 | 1561 | 34 | 9 | 1 |
| 517 | 9311520 | 221 | 793 | 14 | 6 | 3 |
| 518 | 9311521 | 255 | 1359 | 26 | 11 | 4 |
| 519 | 9311522 | 168 | 668 | 14 | 3 | 1 |
| 520 | 9311523 | 169 | 734 | 7 | 2 | 0 |
| 521 | 9311524 | 222 | 1024 | 23 | 3 | 4 |
| 522 | 9311525 | 234 | 1169 | 15 | 4 | 3 |
| 523 | 9311526 | 248 | 1235 | 24 | 11 | 3 |
| 524 | 9311527 | 225 | 1309 | 11 | 6 | 4 |
| 525 | 9311528 | 201 | 1074 | 22 | 3 | 1 |
| 526 | 9311529 | 241 | 1415 | 29 | 7 | 2 |
| 527 | 9311530 | 246 | 1169 | 22 | 7 | 2 |
| 528 | 9311531 | 231 | 969 | 26 | 5 | 3 |
| 529 | 9311532 | 241 | 1176 | 35 | 4 | 2 |
| 530 | 9311533 | 257 | 1391 | 24 | 1 | 1 |
| 531 | 9311534 | 221 | 995 | 21 | 2 | 1 |
| 532 | 9311535 | 261 | 1110 | 37 | 3 | 2 |
| 533 | 9311536 | 174 | 679 | 17 | 3 | 3 |
| 534 | 9311537 | 173 | 637 | 19 | 4 | 2 |
| 535 | 9311538 | 258 | 1358 | 27 | 4 | 3 |
| 536 | 9311539 | 235 | 1311 | 38 | 2 | 2 |
| 537 | 9311540 | 180 | 552 | 19 | 3 | 0 |
| 538 | 9311541 | 234 | 1147 | 23 | 3 | 2 |
| 539 | 9311542 | 181 | 768 | 18 | 8 | 1 |
| 540 | 9311543 | 275 | 1296 | 27 | 4 | 1 |
| 541 | 9311544 | 239 | 1163 | 27 | 4 | 2 |
| 542 | 9311545 | 220 | 1024 | 17 | 6 | 3 |
| 543 | 9311546 | 156 | 546 | 15 | 3 | 1 |
| 544 | 9311547 | 249 | 1167 | 33 | 3 | 2 |
| 545 | 9311548 | 179 | 792 | 8 | 5 | 1 |
| 546 | 9311549 | 227 | 1052 | 33 | 10 | 3 |
| 547 | 9311550 | 230 | 1106 | 17 | 3 | 2 |
| 548 | 9311551 | 267 | 1086 | 42 | 6 | 5 |
| 549 | 9311552 | 229 | 1246 | 26 | 9 | 3 |
| 550 | 9311553 | 173 | 888 | 21 | 4 | 0 |
| 551 | 9311554 | 252 | 1337 | 27 | 4 | 3 |
| 552 | 9311555 | 207 | 842 | 24 | 5 | 2 |
| 553 | 9311556 | 271 | 1493 | 30 | 9 | 2 |
| 554 | 9311557 | 252 | 1133 | 34 | 7 | 4 |
| 555 | 9311558 | 236 | 1089 | 20 | 5 | 2 |
| 556 | 9311559 | 244 | 1029 | 14 | 7 | 2 |
| 557 | 9311560 | 178 | 674 | 17 | 4 | 2 |
| 558 | 9311561 | 213 | 1080 | 28 | 3 | 0 |
| 559 | 9311562 | 298 | 1610 | 41 | 7 | 2 |
| 560 | 9311563 | 248 | 1032 | 27 | 5 | 3 |
| 561 | 9311564 | 261 | 1318 | 40 | 6 | 3 |
| 562 | 9311565 | 209 | 944 | 23 | 10 | 1 |
| 563 | 9311566 | 252 | 1130 | 31 | 4 | 3 |
| 564 | 9311567 | 202 | 929 | 21 | 2 | 2 |
| 565 | 9311568 | 232 | 982 | 21 | 5 | 1 |
| 566 | 9311569 | 225 | 1011 | 27 | 2 | 2 |
| 567 | 9311570 | 191 | 917 | 20 | 2 | 3 |
| 568 | 9311571 | 248 | 1346 | 24 | 4 | 5 |
| 569 | 9311572 | 239 | 1257 | 38 | 5 | 0 |
| 570 | 9311573 | 196 | 509 | 17 | 4 | 0 |
| 571 | 9311574 | 175 | 835 | 18 | 3 | 0 |
| 572 | 9311575 | 225 | 1125 | 27 | 1 | 2 |
| 573 | 9311576 | 278 | 1473 | 27 | 6 | 1 |
| 574 | 9311577 | 246 | 1124 | 34 | 8 | 3 |
| 575 | 9311578 | 256 | 1307 | 32 | 7 | 2 |
| 576 | 9311579 | 186 | 728 | 12 | 13 | 1 |
| 577 | 9311580 | 252 | 1324 | 37 | 10 | 4 |
| 578 | 9311581 | 212 | 1077 | 17 | 1 | 1 |
| 579 | 9311582 | 210 | 1017 | 26 | 4 | 2 |
| 580 | 9311583 | 261 | 1198 | 30 | 7 | 2 |
| 581 | 9311584 | 218 | 981 | 35 | 4 | 3 |
| 582 | 9311585 | 217 | 1133 | 34 | 6 | 2 |
| 583 | 9311586 | 193 | 626 | 16 | 7 | 3 |
| 584 | 9311587 | 246 | 1086 | 27 | 3 | 2 |
| 585 | 9311588 | 283 | 1505 | 35 | 8 | 3 |
| 586 | 9311589 | 186 | 788 | 9 | 1 | 1 |
| 587 | 9311590 | 247 | 930 | 28 | 7 | 3 |
| 588 | 9311591 | 161 | 645 | 15 | 7 | 0 |
| 589 | 9311592 | 170 | 591 | 22 | 9 | 2 |
| 590 | 9311593 | 167 | 615 | 11 | 2 | 3 |
| 591 | 9311594 | 217 | 1117 | 26 | 6 | 1 |
| 592 | 9311595 | 243 | 1133 | 20 | 4 | 2 |
| 593 | 9311596 | 165 | 624 | 5 | 4 | 1 |
| 594 | 9311597 | 200 | 825 | 27 | 9 | 3 |
| 595 | 9311598 | 253 | 1353 | 23 | 4 | 5 |
| 596 | 9311599 | 155 | 522 | 8 | 4 | 0 |
| 597 | 9311600 | 206 | 1007 | 18 | 5 | 1 |
| 598 | 9311601 | 184 | 788 | 15 | 3 | 3 |
| 599 | 9311602 | 209 | 1001 | 13 | 14 | 2 |
| 600 | 9311603 | 226 | 912 | 19 | 15 | 2 |
| 601 | 9311604 | 253 | 1289 | 16 | 4 | 2 |
| 602 | 9311605 | 254 | 1561 | 34 | 4 | 1 |
| 603 | 9311606 | 182 | 766 | 14 | 7 | 2 |
| 604 | 9311607 | 204 | 1098 | 22 | 8 | 4 |
| 605 | 9311608 | 185 | 705 | 16 | 8 | 1 |
| 606 | 9311609 | 241 | 1245 | 23 | 10 | 1 |
| 607 | 9311610 | 227 | 1086 | 22 | 7 | 3 |
| 608 | 9311611 | 231 | 1096 | 24 | 10 | 2 |
| 609 | 9311612 | 261 | 1462 | 14 | 5 | 2 |
| 610 | 9311613 | 223 | 1161 | 36 | 5 | 2 |
| 611 | 9311614 | 247 | 1297 | 31 | 2 | 3 |
| 612 | 9311615 | 198 | 982 | 23 | 11 | 2 |
| 613 | 9311616 | 251 | 1340 | 29 | 4 | 3 |
| 614 | 9311617 | 212 | 1056 | 23 | 3 | 1 |
| 615 | 9311618 | 195 | 859 | 17 | 6 | 3 |
| 616 | 9311619 | 234 | 1232 | 31 | 6 | 2 |
| 617 | 9311620 | 302 | 1476 | 38 | 3 | 3 |
| 618 | 9311621 | 273 | 1334 | 40 | 9 | 2 |
| 619 | 9311622 | 270 | 1432 | 32 | 7 | 2 |
| 620 | 9311623 | 226 | 1141 | 38 | 3 | 4 |
| 621 | 9311624 | 193 | 738 | 16 | 6 | 0 |
| 622 | 9311625 | 206 | 1043 | 29 | 3 | 0 |
| 623 | 9311626 | 220 | 1136 | 22 | 8 | 2 |
| 624 | 9311627 | 179 | 712 | 19 | 9 | 1 |
| 625 | 9311628 | 237 | 1210 | 22 | 3 | 4 |
| 626 | 9311629 | 221 | 1125 | 23 | 2 | 2 |
| 627 | 9311630 | 206 | 941 | 2 | 7 | 3 |
| 628 | 9311631 | 258 | 1336 | 29 | 12 | 1 |
| 629 | 9311632 | 269 | 1377 | 20 | 9 | 2 |
| 630 | 9311633 | 252 | 1384 | 22 | 1 | 2 |
| 631 | 9311634 | 216 | 829 | 15 | 7 | 1 |
| 632 | 9311635 | 191 | 962 | 14 | 3 | 1 |
| 633 | 9311636 | 228 | 1164 | 20 | 11 | 3 |
| 634 | 9311637 | 233 | 1180 | 23 | 3 | 4 |
| 635 | 9311638 | 297 | 1317 | 21 | 5 | 1 |
| 636 | 9311639 | 231 | 1191 | 34 | 6 | 3 |
| 637 | 9311640 | 294 | 1469 | 18 | 9 | 2 |
| 638 | 9311641 | 219 | 1041 | 20 | 1 | 0 |
| 639 | 9311642 | 233 | 1069 | 21 | 14 | 3 |
| 640 | 9311643 | 245 | 1222 | 18 | 3 | 3 |
| 641 | 9311644 | 215 | 1175 | 27 | 7 | 0 |
| 642 | 9311645 | 203 | 852 | 12 | 2 | 2 |
| 643 | 9311646 | 243 | 961 | 18 | 5 | 3 |
| 644 | 9311647 | 169 | 874 | 10 | 7 | 0 |
| 645 | 9311648 | 197 | 983 | 26 | 8 | 2 |
| 646 | 9311649 | 214 | 1170 | 24 | 3 | 1 |
| 647 | 9311650 | 228 | 821 | 22 | 4 | 3 |
| 648 | 9311651 | 203 | 992 | 23 | 7 | 1 |
| 649 | 9311652 | 244 | 1117 | 28 | 6 | 5 |
| 650 | 9311653 | 206 | 1057 | 24 | 6 | 3 |
| 651 | 9311654 | 245 | 932 | 25 | 4 | 1 |
| 652 | 9311655 | 277 | 1417 | 33 | 7 | 2 |
| 653 | 9311656 | 248 | 1013 | 27 | 9 | 1 |
| 654 | 9311657 | 235 | 1247 | 31 | 7 | 2 |
| 655 | 9311658 | 178 | 694 | 12 | 6 | 3 |
| 656 | 9311659 | 227 | 1118 | 27 | 1 | 2 |
| 657 | 9311660 | 172 | 720 | 12 | 5 | 2 |
| 658 | 9311661 | 188 | 774 | 17 | 5 | 2 |
| 659 | 9311662 | 210 | 1182 | 22 | 6 | 3 |
| 660 | 9311663 | 191 | 771 | 9 | 4 | 0 |
| 661 | 9311664 | 223 | 998 | 24 | 2 | 4 |
| 662 | 9311665 | 220 | 1200 | 19 | 8 | 0 |
| 663 | 9311666 | 206 | 932 | 16 | 4 | 2 |
| 664 | 9311667 | 256 | 1266 | 34 | 9 | 3 |
| 665 | 9311668 | 224 | 1230 | 25 | 4 | 2 |
| 666 | 9311669 | 243 | 1294 | 33 | 2 | 1 |
| 667 | 9311670 | 266 | 1539 | 38 | 5 | 3 |
| 668 | 9311671 | 216 | 945 | 29 | 5 | 3 |
| 669 | 9311672 | 267 | 1307 | 26 | 8 | 2 |
| 670 | 9311673 | 165 | 690 | 16 | 2 | 1 |
| 671 | 9311674 | 399 | 2597 | 48 | 6 | 3 |
| 672 | 9311675 | 182 | 748 | 19 | 4 | 0 |
| 673 | 9311676 | 293 | 1432 | 37 | 5 | 2 |
| 674 | 9311677 | 232 | 930 | 27 | 4 | 3 |
| 675 | 9311678 | 248 | 1307 | 30 | 1 | 2 |
| 676 | 9311679 | 233 | 1170 | 20 | 3 | 1 |
| 677 | 9311680 | 198 | 926 | 31 | 3 | 1 |
| 678 | 9311681 | 204 | 843 | 25 | 3 | 3 |
| 679 | 9311682 | 227 | 1251 | 27 | 4 | 2 |
| 680 | 9311683 | 235 | 972 | 17 | 5 | 2 |
| 681 | 9311684 | 174 | 795 | 15 | 2 | 2 |
| 682 | 9311685 | 190 | 871 | 16 | 7 | 1 |
| 683 | 9311686 | 251 | 1187 | 29 | 5 | 2 |
| 684 | 9311687 | 190 | 756 | 16 | 7 | 2 |
| 685 | 9311688 | 197 | 672 | 8 | 3 | 0 |
| 686 | 9311689 | 269 | 1108 | 32 | 4 | 2 |
| 687 | 9311690 | 280 | 1702 | 38 | 2 | 3 |
| 688 | 9311691 | 223 | 1200 | 31 | 6 | 2 |
| 689 | 9311692 | 276 | 1401 | 25 | 7 | 1 |
| 690 | 9311693 | 227 | 1176 | 26 | 4 | 1 |
| 691 | 9311694 | 218 | 927 | 18 | 0 | 3 |
| 692 | 9311695 | 228 | 1331 | 28 | 4 | 1 |
| 693 | 9311696 | 189 | 822 | 23 | 1 | 3 |
| 694 | 9311697 | 243 | 1093 | 26 | 8 | 1 |
| 695 | 9311698 | 217 | 1061 | 23 | 7 | 3 |
| 696 | 9311699 | 199 | 910 | 32 | 1 | 1 |
| 697 | 9311700 | 176 | 770 | 17 | 0 | 1 |
| 698 | 9311701 | 234 | 1085 | 23 | 9 | 4 |
| 699 | 9311702 | 199 | 1051 | 20 | 5 | 2 |
| 700 | 9311703 | 217 | 895 | 19 | 4 | 0 |
| 701 | 9311704 | 244 | 1089 | 30 | 6 | 3 |
| 702 | 9311705 | 232 | 1360 | 29 | 4 | 2 |
| 703 | 9311706 | 296 | 1357 | 37 | 4 | 3 |
| 704 | 9311707 | 204 | 916 | 28 | 8 | 1 |
| 705 | 9311708 | 209 | 908 | 17 | 5 | 1 |
| 706 | 9311709 | 188 | 875 | 23 | 3 | 1 |
| 707 | 9311710 | 252 | 1254 | 26 | 9 | 3 |
| 708 | 9311711 | 451 | 2911 | 43 | 5 | 3 |
| 709 | 9311712 | 202 | 854 | 21 | 6 | 3 |
| 710 | 9311713 | 245 | 1133 | 11 | 4 | 3 |
| 711 | 9311714 | 243 | 1310 | 17 | 2 | 3 |
| 712 | 9311715 | 230 | 1126 | 24 | 6 | 2 |
| 713 | 9311716 | 208 | 799 | 24 | 4 | 3 |
| 714 | 9311717 | 209 | 1009 | 16 | 6 | 2 |
| 715 | 9311718 | 222 | 1108 | 24 | 6 | 0 |
| 716 | 9311719 | 223 | 1078 | 19 | 8 | 2 |
| 717 | 9311720 | 227 | 877 | 24 | 6 | 3 |
| 718 | 9311721 | 267 | 1332 | 30 | 2 | 2 |
| 719 | 9311722 | 241 | 1363 | 28 | 8 | 3 |
| 720 | 9311723 | 293 | 1510 | 39 | 5 | 3 |
| 721 | 9311724 | 208 | 844 | 25 | 4 | 2 |
| 722 | 9311725 | 186 | 743 | 10 | 5 | 2 |
| 723 | 9311726 | 227 | 1159 | 14 | 3 | 4 |
| 724 | 9311727 | 236 | 1250 | 20 | 4 | 3 |
| 725 | 9311728 | 218 | 1024 | 33 | 9 | 3 |
| 726 | 9311729 | 264 | 1468 | 32 | 2 | 3 |
| 727 | 9311730 | 187 | 958 | 18 | 6 | 0 |
| 728 | 9311731 | 219 | 976 | 16 | 3 | 1 |
| 729 | 9311732 | 240 | 1092 | 23 | 3 | 1 |
| 730 | 9311733 | 207 | 1058 | 30 | 3 | 2 |
| 731 | 9311734 | 199 | 715 | 24 | 4 | 2 |
| 732 | 9311735 | 234 | 949 | 24 | 9 | 3 |
| 733 | 9311736 | 207 | 1003 | 22 | 4 | 2 |
| 734 | 9311737 | 227 | 998 | 17 | 8 | 4 |
| 735 | 9311738 | 270 | 1492 | 30 | 4 | 1 |
| 736 | 9311739 | 217 | 1000 | 31 | 1 | 2 |
| 737 | 9311740 | 139 | 434 | 9 | 3 | 0 |
| 738 | 9311741 | 234 | 1023 | 29 | 5 | 3 |
| 739 | 9311742 | 219 | 1013 | 28 | 4 | 3 |
| 740 | 9311743 | 236 | 1322 | 29 | 4 | 2 |
| 741 | 9311744 | 195 | 964 | 19 | 2 | 2 |
| 742 | 9311745 | 232 | 1180 | 25 | 1 | 1 |
| 743 | 9311746 | 255 | 1426 | 22 | 13 | 2 |
| 744 | 9311747 | 291 | 1845 | 27 | 6 | 2 |
| 745 | 9311748 | 227 | 1139 | 26 | 2 | 2 |
| 746 | 9311749 | 270 | 1330 | 31 | 2 | 3 |
| 747 | 9311750 | 222 | 1167 | 21 | 5 | 3 |
| 748 | 9311751 | 201 | 704 | 18 | 11 | 3 |
| 749 | 9311752 | 269 | 1573 | 36 | 4 | 2 |
| 750 | 9311753 | 245 | 1474 | 23 | 8 | 2 |
| 751 | 9311754 | 198 | 958 | 26 | 4 | 2 |
| 752 | 9311755 | 202 | 756 | 28 | 1 | 3 |
| 753 | 9311756 | 234 | 1000 | 19 | 5 | 2 |
| 754 | 9311757 | 226 | 1005 | 12 | 7 | 2 |
| 755 | 9311758 | 214 | 840 | 26 | 5 | 2 |
| 756 | 9311759 | 201 | 776 | 15 | 6 | 1 |
| 757 | 9311760 | 211 | 941 | 15 | 5 | 0 |
| 758 | 9311761 | 223 | 1185 | 28 | 11 | 5 |
| 759 | 9311762 | 220 | 1039 | 25 | 3 | 2 |
| 760 | 9311763 | 186 | 850 | 15 | 1 | 1 |
| 761 | 9311764 | 230 | 1000 | 29 | 6 | 3 |
| 762 | 9311765 | 193 | 646 | 11 | 6 | 3 |
| 763 | 9311766 | 206 | 738 | 26 | 8 | 2 |
| 764 | 9311767 | 235 | 1157 | 32 | 5 | 2 |
| 765 | 9311768 | 189 | 890 | 23 | 7 | 2 |
| 766 | 9311769 | 201 | 771 | 17 | 9 | 2 |
| 767 | 9311770 | 184 | 732 | 16 | 5 | 1 |
| 768 | 9311771 | 263 | 1234 | 13 | 9 | 5 |
| 769 | 9311772 | 225 | 1190 | 20 | 4 | 1 |
| 770 | 9311773 | 251 | 1432 | 11 | 6 | 2 |
| 771 | 9311774 | 248 | 1118 | 31 | 1 | 1 |
| 772 | 9311775 | 249 | 1173 | 28 | 11 | 2 |
| 773 | 9311776 | 203 | 789 | 14 | 9 | 3 |
| 774 | 9311777 | 222 | 1202 | 27 | 6 | 0 |
| 775 | 9311778 | 176 | 722 | 13 | 7 | 0 |
| 776 | 9311779 | 205 | 950 | 20 | 4 | 2 |
| 777 | 9311780 | 221 | 1066 | 25 | 3 | 3 |
| 778 | 9311781 | 231 | 1005 | 22 | 8 | 1 |
| 779 | 9311782 | 181 | 646 | 12 | 5 | 2 |
| 780 | 9311783 | 235 | 883 | 15 | 6 | 2 |
| 781 | 9311784 | 219 | 831 | 20 | 10 | 0 |
| 782 | 9311785 | 214 | 1137 | 13 | 5 | 2 |
| 783 | 9311786 | 238 | 1323 | 21 | 9 | 5 |
| 784 | 9311787 | 228 | 967 | 24 | 6 | 2 |
| 785 | 9311788 | 236 | 1022 | 33 | 2 | 3 |
| 786 | 9311789 | 198 | 910 | 11 | 4 | 1 |
| 787 | 9311790 | 192 | 782 | 16 | 5 | 0 |
| 788 | 9311791 | 252 | 1013 | 16 | 3 | 2 |
| 789 | 9311792 | 222 | 1043 | 9 | 3 | 3 |
| 790 | 9311793 | 238 | 1074 | 35 | 4 | 3 |
| 791 | 9311794 | 247 | 1210 | 27 | 3 | 2 |
| 792 | 9311795 | 315 | 1481 | 13 | 9 | 0 |
| 793 | 9311796 | 226 | 1200 | 25 | 4 | 4 |
| 794 | 9311797 | 169 | 434 | 16 | 4 | 2 |
| 795 | 9311798 | 245 | 1483 | 14 | 5 | 4 |
| 796 | 9311799 | 247 | 1324 | 16 | 1 | 1 |
| 797 | 9311800 | 258 | 1135 | 30 | 4 | 3 |
| 798 | 9311801 | 169 | 659 | 18 | 8 | 1 |
| 799 | 9311802 | 170 | 700 | 15 | 11 | 1 |
| 800 | 9311803 | 198 | 879 | 16 | 8 | 1 |
| 801 | 9311804 | 228 | 1272 | 24 | 6 | 2 |
| 802 | 9311805 | 181 | 648 | 9 | 3 | 0 |
| 803 | 9311806 | 282 | 1809 | 37 | 7 | 2 |
| 804 | 9311807 | 281 | 1586 | 30 | 8 | 1 |
| 805 | 9311808 | 268 | 1331 | 26 | 10 | 2 |
| 806 | 9311809 | 217 | 750 | 19 | 7 | 1 |
| 807 | 9311810 | 169 | 816 | 21 | 8 | 1 |
| 808 | 9311811 | 197 | 928 | 13 | 4 | 1 |
| 809 | 9311812 | 196 | 743 | 14 | 4 | 2 |
| 810 | 9311813 | 216 | 960 | 25 | 0 | 2 |
| 811 | 9311814 | 248 | 1138 | 24 | 7 | 1 |
| 812 | 9311815 | 247 | 1411 | 29 | 2 | 2 |
| 813 | 9311816 | 213 | 1108 | 18 | 2 | 2 |
| 814 | 9311817 | 211 | 1074 | 26 | 3 | 1 |
| 815 | 9311818 | 253 | 1257 | 22 | 6 | 2 |
| 816 | 9311819 | 225 | 1185 | 30 | 4 | 3 |
| 817 | 9311820 | 182 | 603 | 10 | 2 | 1 |
| 818 | 9311821 | 263 | 1032 | 25 | 3 | 5 |
| 819 | 9311822 | 334 | 1652 | 53 | 9 | 1 |
| 820 | 9311823 | 188 | 983 | 29 | 6 | 1 |
| 821 | 9311824 | 265 | 1029 | 35 | 6 | 2 |
| 822 | 9311825 | 314 | 1756 | 38 | 5 | 1 |
| 823 | 9311826 | 210 | 1078 | 26 | 3 | 1 |
| 824 | 9311827 | 211 | 1032 | 29 | 12 | 1 |
| 825 | 9311828 | 215 | 904 | 34 | 3 | 2 |
| 826 | 9311829 | 224 | 930 | 18 | 8 | 4 |
| 827 | 9311830 | 272 | 1582 | 35 | 2 | 1 |
| 828 | 9311831 | 249 | 1209 | 27 | 10 | 2 |
| 829 | 9311832 | 208 | 887 | 26 | 3 | 3 |
| 830 | 9311833 | 171 | 783 | 19 | 3 | 3 |
| 831 | 9311834 | 273 | 1339 | 38 | 2 | 3 |
| 832 | 9311835 | 232 | 851 | 21 | 4 | 2 |
| 833 | 9311836 | 189 | 797 | 20 | 3 | 1 |
| 834 | 9311837 | 220 | 762 | 21 | 3 | 2 |
| 835 | 9311838 | 167 | 651 | 15 | 9 | 0 |
| 836 | 9311839 | 236 | 1301 | 34 | 8 | 2 |
| 837 | 9311840 | 231 | 1110 | 29 | 3 | 0 |
| 838 | 9311841 | 257 | 1183 | 25 | 9 | 2 |
| 839 | 9311842 | 256 | 1280 | 38 | 5 | 3 |
| 840 | 9311843 | 230 | 1011 | 28 | 6 | 3 |
| 841 | 9311844 | 204 | 913 | 16 | 0 | 2 |
| 842 | 9311845 | 235 | 1153 | 12 | 5 | 5 |
| 843 | 9311846 | 258 | 1065 | 17 | 6 | 3 |
| 844 | 9311847 | 203 | 1018 | 17 | 4 | 2 |
| 845 | 9311848 | 304 | 1694 | 37 | 0 | 3 |
| 846 | 9311849 | 200 | 919 | 28 | 3 | 3 |
| 847 | 9311850 | 198 | 1006 | 11 | 2 | 1 |
| 848 | 9311851 | 173 | 690 | 14 | 3 | 4 |
| 849 | 9311852 | 273 | 1420 | 37 | 2 | 3 |
| 850 | 9311853 | 209 | 994 | 24 | 0 | 4 |
| 851 | 9311854 | 283 | 1105 | 31 | 4 | 2 |
| 852 | 9311855 | 268 | 1344 | 36 | 4 | 2 |
| 853 | 9311856 | 210 | 1038 | 22 | 3 | 2 |
| 854 | 9311857 | 176 | 738 | 6 | 8 | 3 |
| 855 | 9311858 | 273 | 1374 | 25 | 11 | 2 |
| 856 | 9311859 | 234 | 924 | 21 | 3 | 2 |
| 857 | 9311860 | 230 | 1127 | 25 | 9 | 1 |
| 858 | 9311861 | 208 | 953 | 21 | 5 | 2 |
| 859 | 9311862 | 207 | 912 | 17 | 4 | 1 |
| 860 | 9311863 | 180 | 756 | 16 | 9 | 1 |
| 861 | 9311864 | 215 | 1094 | 19 | 6 | 1 |
| 862 | 9311865 | 163 | 699 | 11 | 8 | 1 |
| 863 | 9311866 | 271 | 1331 | 28 | 5 | 4 |
| 864 | 9311867 | 240 | 980 | 18 | 2 | 2 |
| 865 | 9311868 | 210 | 1012 | 27 | 2 | 3 |
| 866 | 9311869 | 216 | 848 | 12 | 6 | 1 |
| 867 | 9311870 | 202 | 775 | 22 | 10 | 1 |
| 868 | 9311871 | 241 | 1186 | 26 | 7 | 4 |
| 869 | 9311872 | 263 | 1358 | 20 | 10 | 3 |
| 870 | 9311873 | 179 | 781 | 15 | 6 | 2 |
| 871 | 9311874 | 227 | 1125 | 27 | 2 | 1 |
| 872 | 9311875 | 219 | 1012 | 22 | 2 | 1 |
| 873 | 9311876 | 198 | 760 | 13 | 4 | 0 |
| 874 | 9311877 | 219 | 939 | 20 | 1 | 0 |
| 875 | 9311878 | 204 | 818 | 19 | 4 | 1 |
| 876 | 9311879 | 245 | 1414 | 34 | 3 | 2 |
| 877 | 9311880 | 247 | 1125 | 20 | 8 | 3 |
| 878 | 9311881 | 204 | 1117 | 23 | 5 | 2 |
| 879 | 9311882 | 228 | 1036 | 34 | 2 | 1 |
| 880 | 9311883 | 215 | 846 | 16 | 7 | 1 |
| 881 | 9311884 | 212 | 933 | 20 | 2 | 2 |
| 882 | 9311885 | 259 | 1241 | 35 | 4 | 3 |
| 883 | 9311886 | 292 | 1203 | 26 | 17 | 2 |
| 884 | 9311887 | 245 | 1389 | 26 | 2 | 3 |
| 885 | 9311888 | 198 | 755 | 19 | 4 | 1 |
| 886 | 9311889 | 241 | 1246 | 33 | 4 | 6 |
| 887 | 9311890 | 216 | 797 | 18 | 1 | 3 |
| 888 | 9311891 | 219 | 931 | 18 | 7 | 1 |
| 889 | 9311892 | 256 | 1086 | 17 | 4 | 3 |
| 890 | 9311893 | 171 | 950 | 16 | 3 | 0 |
| 891 | 9311894 | 235 | 1234 | 21 | 10 | 2 |
| 892 | 9311895 | 235 | 1212 | 26 | 5 | 2 |
| 893 | 9311896 | 318 | 1848 | 31 | 8 | 0 |
| 894 | 9311897 | 261 | 1490 | 28 | 4 | 1 |
| 895 | 9311898 | 198 | 889 | 18 | 3 | 2 |
| 896 | 9311899 | 226 | 1027 | 19 | 5 | 0 |
| 897 | 9311900 | 221 | 1157 | 33 | 7 | 2 |
| 898 | 9311901 | 225 | 919 | 24 | 0 | 1 |
| 899 | 9311902 | 265 | 1237 | 20 | 6 | 2 |
| 900 | 9311903 | 237 | 1203 | 22 | 4 | 5 |
| 901 | 9311904 | 290 | 1611 | 11 | 9 | 3 |
| 902 | 9311905 | 200 | 857 | 22 | 6 | 2 |
| 903 | 9311906 | 186 | 785 | 23 | 2 | 2 |
| 904 | 9311907 | 303 | 1683 | 29 | 13 | 2 |
| 905 | 9311908 | 201 | 887 | 22 | 3 | 3 |
| 906 | 9311909 | 303 | 1817 | 28 | 1 | 1 |
| 907 | 9311910 | 248 | 1134 | 30 | 3 | 3 |
| 908 | 9311911 | 217 | 858 | 12 | 10 | 1 |
| 909 | 9311912 | 207 | 897 | 17 | 6 | 1 |
| 910 | 9311913 | 238 | 1134 | 23 | 4 | 1 |
| 911 | 9311914 | 217 | 997 | 26 | 7 | 2 |
| 912 | 9311915 | 188 | 842 | 19 | 3 | 2 |
| 913 | 9311916 | 219 | 982 | 23 | 1 | 3 |
| 914 | 9311917 | 233 | 1177 | 26 | 0 | 4 |
| 915 | 9311918 | 208 | 845 | 20 | 4 | 2 |
| 916 | 9311919 | 257 | 1385 | 34 | 7 | 1 |
| 917 | 9311920 | 229 | 1039 | 34 | 4 | 3 |
| 918 | 9311921 | 267 | 1420 | 29 | 14 | 2 |
| 919 | 9311922 | 234 | 1035 | 27 | 7 | 2 |
| 920 | 9311923 | 246 | 1072 | 42 | 2 | 1 |
| 921 | 9311924 | 311 | 1345 | 27 | 5 | 2 |
| 922 | 9311925 | 277 | 1187 | 40 | 9 | 2 |
| 923 | 9311926 | 329 | 1707 | 24 | 2 | 2 |
| 924 | 9311927 | 319 | 1693 | 46 | 4 | 3 |
| 925 | 9311928 | 165 | 764 | 13 | 7 | 1 |
| 926 | 9311929 | 268 | 1342 | 30 | 6 | 3 |
| 927 | 9311930 | 232 | 1281 | 32 | 9 | 5 |
| 928 | 9311931 | 190 | 777 | 13 | 6 | 1 |
| 929 | 9311932 | 262 | 1150 | 26 | 7 | 3 |
| 930 | 9311933 | 236 | 1189 | 26 | 3 | 2 |
| 931 | 9311934 | 224 | 1137 | 19 | 3 | 1 |
| 932 | 9311935 | 229 | 1251 | 29 | 6 | 1 |
| 933 | 9311936 | 224 | 996 | 26 | 2 | 2 |
| 934 | 9311937 | 206 | 999 | 18 | 4 | 1 |
| 935 | 9311938 | 225 | 1104 | 17 | 2 | 1 |
| 936 | 9311939 | 202 | 765 | 27 | 5 | 2 |
| 937 | 9311940 | 248 | 1141 | 31 | 5 | 3 |
| 938 | 9311941 | 186 | 915 | 15 | 4 | 0 |
| 939 | 9311942 | 258 | 1260 | 15 | 7 | 3 |
| 940 | 9311943 | 207 | 953 | 21 | 5 | 0 |
| 941 | 9311944 | 264 | 1335 | 22 | 5 | 3 |
| 942 | 9311945 | 180 | 793 | 17 | 6 | 2 |
| 943 | 9311946 | 207 | 781 | 21 | 5 | 3 |
| 944 | 9311947 | 204 | 976 | 20 | 6 | 0 |
| 945 | 9311948 | 242 | 933 | 25 | 9 | 2 |
| 946 | 9311949 | 246 | 1450 | 17 | 5 | 2 |
| 947 | 9311950 | 314 | 1249 | 24 | 2 | 2 |
| 948 | 9311951 | 218 | 906 | 11 | 2 | 3 |
| 949 | 9311952 | 222 | 1307 | 22 | 3 | 0 |
| 950 | 9311953 | 219 | 963 | 15 | 5 | 2 |
| 951 | 9311954 | 240 | 1161 | 25 | 10 | 3 |
| 952 | 9311955 | 206 | 790 | 28 | 3 | 3 |
| 953 | 9311956 | 219 | 966 | 18 | 5 | 3 |
| 954 | 9311957 | 215 | 1034 | 27 | 0 | 3 |
| 955 | 9311958 | 271 | 1396 | 43 | 10 | 2 |
| 956 | 9311959 | 254 | 1159 | 30 | 10 | 6 |
| 957 | 9311960 | 205 | 1009 | 27 | 3 | 2 |
| 958 | 9311961 | 224 | 866 | 17 | 5 | 3 |
| 959 | 9311962 | 288 | 1386 | 33 | 12 | 3 |
| 960 | 9311963 | 261 | 1190 | 39 | 5 | 4 |
| 961 | 9311964 | 226 | 980 | 26 | 3 | 2 |
| 962 | 9311965 | 267 | 1313 | 22 | 0 | 1 |
| 963 | 9311966 | 276 | 1561 | 29 | 2 | 1 |
| 964 | 9311967 | 168 | 747 | 13 | 4 | 1 |
| 965 | 9311968 | 207 | 1018 | 30 | 10 | 3 |
| 966 | 9311969 | 137 | 463 | 8 | 3 | 0 |
| 967 | 9311970 | 269 | 1289 | 31 | 2 | 3 |
| 968 | 9311971 | 299 | 1650 | 30 | 3 | 2 |
| 969 | 9311972 | 281 | 1538 | 35 | 5 | 2 |
| 970 | 9311973 | 171 | 709 | 21 | 2 | 3 |
| 971 | 9311974 | 178 | 798 | 18 | 2 | 2 |
| 972 | 9311975 | 216 | 713 | 19 | 2 | 1 |
| 973 | 9311976 | 246 | 1289 | 24 | 3 | 2 |
| 974 | 9311977 | 185 | 781 | 13 | 9 | 1 |
| 975 | 9311978 | 252 | 1363 | 22 | 4 | 1 |
| 976 | 9311979 | 218 | 1312 | 23 | 11 | 3 |
| 977 | 9311980 | 207 | 785 | 15 | 5 | 0 |
| 978 | 9311981 | 203 | 748 | 20 | 5 | 3 |
| 979 | 9311982 | 219 | 908 | 23 | 5 | 3 |
| 980 | 9311983 | 235 | 1171 | 22 | 11 | 5 |
| 981 | 9311984 | 282 | 1425 | 20 | 4 | 3 |
| 982 | 9311985 | 210 | 902 | 26 | 3 | 2 |
| 983 | 9311986 | 214 | 978 | 23 | 7 | 2 |
| 984 | 9311987 | 201 | 820 | 20 | 3 | 4 |
| 985 | 9311988 | 196 | 841 | 29 | 2 | 2 |
| 986 | 9311989 | 233 | 1027 | 22 | 4 | 1 |
| 987 | 9311990 | 204 | 816 | 17 | 6 | 2 |
| 988 | 9311991 | 221 | 955 | 18 | 9 | 1 |
| 989 | 9311992 | 185 | 946 | 8 | 3 | 1 |
| 990 | 9311993 | 259 | 1601 | 28 | 5 | 0 |
| 991 | 9311994 | 213 | 1072 | 25 | 3 | 1 |
| 992 | 9311995 | 234 | 1206 | 25 | 7 | 2 |
| 993 | 9311996 | 207 | 885 | 24 | 7 | 2 |
| 994 | 9311997 | 243 | 1100 | 32 | 10 | 3 |
| 995 | 9311998 | 319 | 1442 | 36 | 9 | 2 |
| 996 | 9311999 | 350 | 1623 | 46 | 8 | 2 |
| 997 | 9312000 | 169 | 677 | 12 | 6 | 1 |
| 998 | 9312001 | 248 | 1437 | 32 | 8 | 1 |
| 999 | 9312002 | 243 | 1327 | 36 | 0 | 4 |

## Throughput

- Turns simulated: 226137 (first play of each game)
- Aggregate: 145.8 turns/s over 1550.5s wall
- Per game thread: 36.3 turns/s

## Determinism

Not checked (run with `--determinism`).

## Panics

None.
