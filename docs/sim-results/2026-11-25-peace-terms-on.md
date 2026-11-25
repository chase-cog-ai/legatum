# Simulation: deals-terms-on (2026-11-25)

1000 games, first seed 9311003, brain `scripted`, turn limit off, safety cap 1200, 5 threads, started 2026-11-25T22:52:22Z. Wall time 1324.8s.

Source checkout at run start: `505937ecd0f1e1b47f8b1c7ed0dd0ac2a4cab5e9`; dirty: no. Map 48×32, 8 initial seats. `first_setup` and exact binary argv are recorded in JSON. Ordinary games use seed + i (wrapping); the strategy tournament schedule is specified below. All games use a deterministic shuffle of the full faction pool, taking eight distinct factions, with any Proxy rotation described below.

Command argv: `["./sim","--players","8","--difficulty","hard","--games","1000","--seed","9311003","--out-dir","out","--threads","5","--deals","--label","deals-terms-on"]`

## Overview

| Metric | Value |
| --- | --- |
| Games (non-panic) | 1000 of 1000 |
| Game length, turns (mean / median / min-max) | 225.6 / 222 / 139-432 |
| Win reason mix | VP hold 97.6%, Capitals 2.4% |
| Median end turn | 222 |
| Ending shares of played games (capitals / VP hold / turn limit / other) | 2.4% / 97.6% / 0.0% / 0.0% |
| Games reaching the World War era (any empire) | 994 of 1000 (99.4%) |
| VP hold length | 10 turn ends (the reaching turn counts) |
| Holds started / broken / completed / unfinished | 1423 / 337 / 980 / 106 |
| Hold break rate | 23.7% of started; 25.6% of resolved (broken / (broken + completed)) |
| Hold wins | 976 |
| First reach to win, turns (mean / median / min-max) | 11.1 / 9 / 9-136 |
| Final hold start to win, turns | 9.0 / 9 / 9-9 |
| Comebacks (winner broke an earlier hold of its own) | 120 |
| Wins by a team another team beat to its target | 90 |

Game length by how the game ended (turns resolved; a stall is a safety-cap stop, never a win):

| Ended by | Games | Length mean / median / min-max |
| --- | --- | --- |
| VP hold | 976 | 224.6 / 221 / 139-432 |
| Capitals | 24 | 268.0 / 266 / 225-333 |

Winner share by faction (wins of that faction's seats / all winning seats):

| Faction | Wins | Share of wins |
| --- | --- | --- |
| Switzerland | 84 | 8.4% |
| India | 83 | 8.3% |
| Byzantium | 77 | 7.7% |
| Germany | 149 | 14.9% |
| Celts | 129 | 12.9% |
| Mongols | 98 | 9.8% |
| Sparta | 108 | 10.8% |
| Athens | 150 | 15.0% |
| Aztecs | 122 | 12.2% |

## Outcomes

| Outcome | Games | Share |
| --- | --- | --- |
| VP hold | 976 | 97.6% |
| Conquest | 0 | 0.0% |
| Capitals | 24 | 2.4% |
| Turn limit | 0 | 0.0% |
| Draw | 0 | 0.0% |
| Stalled (safety cap) | 0 | 0.0% |
| Panic | 0 | 0.0% |

## Win rate by faction

Per seat: wins / seats of that faction. With 8 single-empire teams, an even game is 12.5% per seat.

| Faction | Seats | Wins | Win rate |
| --- | --- | --- | --- |
| Switzerland | 879 | 84 | 9.6% |
| India | 905 | 83 | 9.2% |
| Byzantium | 885 | 77 | 8.7% |
| Germany | 900 | 149 | 16.6% |
| Celts | 880 | 129 | 14.7% |
| Mongols | 878 | 98 | 11.2% |
| Sparta | 901 | 108 | 12.0% |
| Athens | 900 | 150 | 16.7% |
| Aztecs | 872 | 122 | 14.0% |

## Win rate by difficulty

All initial seats in non-panicked games, including eliminated empires and cap stops. Determinism replays are not additional samples.

| Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- |
| Hard | 8000 | 1000 | 12.5% |

## Win rate by faction and difficulty

| Faction | Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- | --- |
| Switzerland | Hard | 879 | 84 | 9.6% |
| India | Hard | 905 | 83 | 9.2% |
| Byzantium | Hard | 885 | 77 | 8.7% |
| Germany | Hard | 900 | 149 | 16.6% |
| Celts | Hard | 880 | 129 | 14.7% |
| Mongols | Hard | 878 | 98 | 11.2% |
| Sparta | Hard | 901 | 108 | 12.0% |
| Athens | Hard | 900 | 150 | 16.7% |
| Aztecs | Hard | 872 | 122 | 14.0% |

## Faction goal progress

Per seat: how often the faction's permanent goal was reached before the game ended, and the turn its VP was first awarded (over the seats that reached it). Seats are all initial empires of non-panicked games with telemetry, including eliminated empires and cap stops.

| Faction | Seats | Reached | Goal rate | Min | P10 | Median turn | P90 | Max |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Switzerland | 879 | 175 | 19.9% | 110 | 138 | 162 | 229 | 328 |
| India | 905 | 509 | 56.2% | 44 | 88 | 122 | 195 | 321 |
| Byzantium | 885 | 688 | 77.7% | 69 | 83 | 105 | 148 | 281 |
| Germany | 900 | 365 | 40.6% | 84 | 123 | 153 | 208 | 308 |
| Celts | 880 | 551 | 62.6% | 35 | 174 | 210 | 210 | 256 |
| Mongols | 878 | 158 | 18.0% | 105 | 149 | 192 | 232 | 364 |
| Sparta | 901 | 151 | 16.8% | 121 | 159 | 198 | 253 | 333 |
| Athens | 900 | 276 | 30.7% | 122 | 155 | 221 | 255 | 255 |
| Aztecs | 872 | 321 | 36.8% | 109 | 167 | 225 | 235 | 290 |

## Distributions

| Metric | Min | P10 | Median | P90 | Max |
| --- | --- | --- | --- | --- | --- |
| Observed game length (turns, including cap stops) | 139 | 182 | 222 | 271 | 432 |
| Winning team VP | 2 | 10 | 11 | 13 | 18 |
| First Soldier completed (turn, per empire) | 6 | 8 | 36 | 71 | 261 |
| First Settler completed (turn, per empire) | 6 | 12 | 14 | 25 | 206 |
| First city founded (turn, per empire) | 1 | 1 | 1 | 1 | 1 |
| Second city founded (turn, per empire) | 10 | 16 | 19 | 39 | 216 |

Safety-cap stops are censored observations, not completed game lengths.

Openings: 8000 observed empires in 1000 first plays; 0 games without opening telemetry excluded. Turn distributions include only empires that reached the milestone. Never means not reached by the end of observed play, including eliminated empires and safety-cap stops. Per-empire turns are in JSON (`games[].telemetry.openings`, keyed by player ID).

| Opening milestone | Reached empires | Never empires |
| --- | --- | --- |
| First Soldier completed | 7913 | 87 |
| First Settler completed | 7985 | 15 |
| First city founded | 8000 | 0 |
| Second city founded | 7910 | 90 |

Final VP per empire (all seats):

| VP | Empires |
| --- | --- |
| 0 | 1609 |
| 1 | 1445 |
| 2 | 1055 |
| 3 | 797 |
| 4 | 600 |
| 5 | 484 |
| 6 | 344 |
| 7 | 273 |
| 8 | 169 |
| 9 | 130 |
| 10 | 469 |
| 11 | 310 |
| 12 | 166 |
| 13 | 83 |
| 14 | 41 |
| 15 | 18 |
| 16 | 5 |
| 18 | 2 |

## End state

- Average game length: 225.6 turns
- Average cities per empire: 4.61
- Average techs per empire: 32.68
- Average eliminations per game: 2.07

## Strategy diversity: observed behavior (classifier v2)

1000 finished games; 8000 player seats; 1000 winning seats. Excluded 0 unfinished/panicked games and 0 games with missing labels. Winner entropy: **0.3310** (normalized over 6 categories, including zero-win categories).

Thresholds, biases and denominators: [strategy-diversity.md](../strategy-diversity.md).

| Archetype | Players | Wins | Win rate | All-player frequency | Winner frequency |
| --- | ---: | ---: | ---: | ---: | ---: |
| Rush | 1773 | 60 | 3.4% | 22.2% | 6.0% |
| Wide | 3752 | 843 | 22.5% | 46.9% | 84.3% |
| Science | 0 | 0 | n/a | 0.0% | 0.0% |
| Turtle/Wonder | 48 | 10 | 20.8% | 0.6% | 1.0% |
| Economic | 1056 | 81 | 7.7% | 13.2% | 8.1% |
| Generalist | 1371 | 6 | 0.4% | 17.1% | 0.6% |

### Matchup matrix

Cell = strict ahead % / tied % (co-occurrence games). Each game has equal weight; all cross-team seat pairs within it share that weight. Rank: winning team first, then surviving empires by VP, then eliminated empires by later elimination turn. Equal ranks tie; no ID tiebreak. This is a multiplayer association, not a duel win rate.

| Row vs column | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Rush | — | 18.0% / 4.6% (861) | n/a / n/a (0) | 14.6% / 5.5% (41) | 18.5% / 7.0% (550) | 59.4% / 6.2% (684) |
| Wide | 77.5% / 4.6% (861) | — | n/a / n/a (0) | 44.7% / 8.5% (48) | 56.4% / 5.2% (659) | 86.6% / 2.8% (812) |
| Science | n/a / n/a (0) | n/a / n/a (0) | — | n/a / n/a (0) | n/a / n/a (0) | n/a / n/a (0) |
| Turtle/Wonder | 79.9% / 5.5% (41) | 46.8% / 8.5% (48) | n/a / n/a (0) | — | 40.7% / 11.1% (27) | 90.4% / 2.6% (39) |
| Economic | 74.4% / 7.0% (550) | 38.3% / 5.2% (659) | n/a / n/a (0) | 48.1% / 11.1% (27) | — | 85.6% / 6.4% (517) |
| Generalist | 34.4% / 6.2% (684) | 10.6% / 2.8% (812) | n/a / n/a (0) | 7.1% / 2.6% (39) | 8.0% / 6.4% (517) | — |

### Faction wins by archetype

Cell = wins / seats; win rate in JSON.

| Faction | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Switzerland | 0 / 88 | 77 / 444 | 0 / 0 | 0 / 0 | 7 / 188 | 0 / 159 |
| India | 3 / 213 | 80 / 569 | 0 / 0 | 0 / 0 | 0 / 22 | 0 / 101 |
| Byzantium | 0 / 9 | 74 / 483 | 0 / 0 | 0 / 4 | 3 / 133 | 0 / 256 |
| Germany | 6 / 149 | 133 / 461 | 0 / 0 | 0 / 8 | 10 / 91 | 0 / 191 |
| Celts | 1 / 112 | 122 / 530 | 0 / 0 | 0 / 0 | 6 / 120 | 0 / 118 |
| Mongols | 26 / 543 | 68 / 207 | 0 / 0 | 0 / 0 | 4 / 51 | 0 / 77 |
| Sparta | 20 / 436 | 53 / 175 | 0 / 0 | 10 / 35 | 21 / 142 | 4 / 113 |
| Athens | 1 / 93 | 137 / 448 | 0 / 0 | 0 / 0 | 11 / 154 | 1 / 205 |
| Aztecs | 3 / 130 | 99 / 435 | 0 / 0 | 0 / 1 | 19 / 155 | 1 / 151 |

Guardrail: **Fail**. Entropy minimum 0.80; at least 20 games per strategy pair. Dominant by observed score: none. Without an observed counter (all pairs covered): none. Sampled but never won: none. Unobserved: Science. Under-sampled pairs: 5.

- insufficient evidence: 5 under-sampled matchups; 1000 winning seats

- winner entropy below 0.80

## Battle and capture telemetry

1000 observed first plays; 0 records without telemetry excluded.

| Event | Total / observed games | Mean per game | Games with event / observed games |
| --- | --- | --- | --- |
| War declarations | 15021/1000 | 15.021 | 1000/1000 (100.0%) |
| Peace offers | 94606/1000 | 94.606 | 1000/1000 (100.0%) |
| Peace agreements | 12396/1000 | 12.396 | 1000/1000 (100.0%) |
| Battles | 1061031/1000 | 1061.031 | 1000/1000 (100.0%) |
| City captures | 22892/1000 | 22.892 | 1000/1000 (100.0%) |
| Settler captures | 5265/1000 | 5.265 | 981/1000 (98.1%) |
| Eliminations | 2070/1000 | 2.070 | 900/1000 (90.0%) |

## Terminal VP sources

1000 finished games. Columns are separate populations; totals across columns must not be added. Every cell is VP / population observations.

| Source | All players | All teams | Winning teams |
| --- | --- | --- | --- |
| Held Great Works | 7338/8000 | 7338/8000 | 3212/1000 |
| Faction goals | 5016/8000 | 5016/8000 | 2154/1000 |
| Era objectives | 14307/8000 | 14307/8000 | 4149/1000 |
| Capitals taken | 2733/8000 | 2733/8000 | 1590/1000 |
| Total VP | 29394/8000 | 29394/8000 | 11105/1000 |

Reconciliation failures: 0 player observations, 0 team observations.

## Turn-50 team leaders

| Cohort | Eventual leader-set wins / eligible games | Draws (included) |
| --- | --- | --- |
| Unique leader | 64/416 (15.4%) | 0 |
| Tied leaders (winner in set) | 580/584 (99.3%) | 0 |
| Any leader set | 644/1000 (64.4%) | 0 |

Excluded: 0 games ended before turn 50, 0 unfinished games, 0 snapshots without teams, 0 records without telemetry.

## Era pacing

Resolved turn each empire entered each era. `Winner vs median` is the winning team's seats' entry turn minus the median entry of the empires that entered it in the same finished game (negative: winners got there first).

| Era | Entered / seats | Fastest | p10 | Median | p90 | Slowest | Winner vs median (mean, n) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Classical | 7896/8000 | 5 | 20 | 28 | 32 | 59 | -2.7 (1000) |
| Medieval | 7799/8000 | 35 | 56 | 60 | 63 | 125 | -1.6 (1000) |
| Renaissance | 7743/8000 | 63 | 81 | 86 | 93 | 173 | -2.6 (1000) |
| Industrial | 7569/8000 | 87 | 107 | 114 | 126 | 263 | -4.2 (1000) |
| Machine Age | 6978/8000 | 113 | 135 | 145 | 160 | 256 | -6.3 (1000) |
| World War | 5625/8000 | 133 | 160 | 173 | 192 | 281 | -8.6 (992) |
| Modern | 3582/8000 | 155 | 183 | 202 | 224 | 330 | -9.8 (859) |
| Future | 1633/8000 | 172 | 208 | 232 | 256 | 317 | -7.3 (555) |

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
| Classical | Mathematics | 7825/8000 | 43 | 351 BC | 300 BC |
| Classical | Engineering | 7810/8000 | 49 | 51 BC | n/a |
| Classical | Iron Working | 7844/8000 | 37 | 651 BC | 1200 BC |
| Classical | Code of Laws | 7804/8000 | 57 | 350 AD | 1754 BC |
| Classical | Philosophy | 7800/8000 | 58 | 400 AD | 500 BC |
| Medieval | Feudalism | 7779/8000 | 73 | 890 AD | n/a |
| Medieval | Machinery | 7790/8000 | 67 | 710 AD | n/a |
| Medieval | Chivalry | 6904/8000 | 78 | 1040 AD | n/a |
| Medieval | Education | 7753/8000 | 82 | 1160 AD | 1088 AD |
| Medieval | Crop Rotation | 7743/8000 | 85 | 1250 AD | n/a |
| Renaissance | Gunpowder | 7721/8000 | 92 | 1420 AD | n/a |
| Renaissance | Metallurgy | 7682/8000 | 102 | 1520 AD | n/a |
| Renaissance | Military Engineering | 7690/8000 | 100 | 1500 AD | n/a |
| Renaissance | Banking | 7618/8000 | 112 | 1620 AD | 1397 AD |
| Renaissance | Printing Press | 7576/8000 | 112 | 1620 AD | 1440 AD |
| Industrial | Rifling | 7210/8000 | 135 | 1787 AD | n/a |
| Industrial | Military Science | 7377/8000 | 127 | 1757 AD | n/a |
| Industrial | Ballistics | 7269/8000 | 137 | 1794 AD | n/a |
| Industrial | Industrialization | 7469/8000 | 122 | 1720 AD | 1769 AD |
| Industrial | Railways | 6983/8000 | 145 | 1824 AD | 1825 AD |
| Machine Age | Replaceable Parts | 6443/8000 | 159 | 1876 AD | 1800 AD |
| Machine Age | Combustion | 6706/8000 | 153 | 1854 AD | 1876 AD |
| Machine Age | Steel | 6415/8000 | 158 | 1872 AD | 1856 AD |
| Machine Age | Flight | 5953/8000 | 167 | 1887 AD | 1903 AD |
| Machine Age | Electricity | 5674/8000 | 173 | 1894 AD | 1879 AD |
| World War | Mass Mobilisation | 4631/8000 | 187 | 1910 AD | n/a |
| World War | Rocketry | 4315/8000 | 191 | 1915 AD | 1926 AD |
| World War | Armoured Warfare | 3997/8000 | 197 | 1924 AD | 1916 AD |
| World War | Air Power | 3825/8000 | 199 | 1926 AD | n/a |
| World War | Electronics | 5202/8000 | 180 | 1902 AD | n/a |
| Modern | Mechanised Warfare | 2478/8000 | 217 | 1954 AD | n/a |
| Modern | Composite Armour | 2192/8000 | 221 | 1964 AD | n/a |
| Modern | Guided Missiles | 2023/8000 | 226 | 1976 AD | n/a |
| Modern | Jet Engines | 1938/8000 | 229 | 1983 AD | 1939 AD |
| Modern | Green Revolution | 3031/8000 | 210 | 1942 AD | n/a |
| Future | Autonomous Systems | 953/8000 | 245 | 2018 AD | n/a |
| Future | Robotics | 729/8000 | 251 | 2027 AD | n/a |
| Future | Fusion Power | 728/8000 | 256 | 2032 AD | n/a |
| Future | Stealth Technology | 1182/8000 | 240 | 2010 AD | n/a |
| Future | Recursive AI | 638/8000 | 256 | 2032 AD | n/a |

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
| 0 | 0.00 | 68.98 |
| 1 | 63.56 | 15.69 |
| 2 | 22.36 | 5.70 |
| 3 | 5.88 | 1.72 |
| 4 | 1.16 | 0.40 |
| 5 | 0.21 | 0.10 |

## Per-game events

Each row is one observed first play; unavailable telemetry is shown as n/a. Full per-player/team score sources and turn-50 standings are in JSON.

| Game | Seed | Resolved turns | Battles | City captures | Settler captures | Eliminations |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 9311003 | 204 | 908 | 21 | 6 | 2 |
| 1 | 9311004 | 228 | 1197 | 14 | 7 | 3 |
| 2 | 9311005 | 183 | 711 | 19 | 7 | 2 |
| 3 | 9311006 | 242 | 1296 | 24 | 9 | 4 |
| 4 | 9311007 | 207 | 828 | 21 | 11 | 2 |
| 5 | 9311008 | 233 | 1110 | 25 | 2 | 1 |
| 6 | 9311009 | 223 | 944 | 25 | 7 | 1 |
| 7 | 9311010 | 233 | 1295 | 40 | 6 | 1 |
| 8 | 9311011 | 269 | 1073 | 17 | 5 | 5 |
| 9 | 9311012 | 244 | 1154 | 23 | 3 | 5 |
| 10 | 9311013 | 183 | 912 | 18 | 9 | 3 |
| 11 | 9311014 | 249 | 1681 | 27 | 2 | 3 |
| 12 | 9311015 | 185 | 722 | 16 | 3 | 2 |
| 13 | 9311016 | 302 | 1869 | 42 | 6 | 2 |
| 14 | 9311017 | 249 | 1264 | 29 | 4 | 1 |
| 15 | 9311018 | 177 | 739 | 15 | 9 | 1 |
| 16 | 9311019 | 267 | 1172 | 21 | 5 | 2 |
| 17 | 9311020 | 250 | 1123 | 30 | 10 | 1 |
| 18 | 9311021 | 205 | 758 | 7 | 8 | 2 |
| 19 | 9311022 | 184 | 781 | 16 | 3 | 0 |
| 20 | 9311023 | 286 | 1422 | 32 | 5 | 3 |
| 21 | 9311024 | 184 | 761 | 11 | 1 | 1 |
| 22 | 9311025 | 223 | 1178 | 25 | 12 | 2 |
| 23 | 9311026 | 194 | 1000 | 18 | 5 | 2 |
| 24 | 9311027 | 267 | 1036 | 18 | 4 | 2 |
| 25 | 9311028 | 168 | 635 | 17 | 6 | 0 |
| 26 | 9311029 | 190 | 559 | 16 | 5 | 2 |
| 27 | 9311030 | 196 | 764 | 25 | 5 | 2 |
| 28 | 9311031 | 190 | 809 | 18 | 4 | 2 |
| 29 | 9311032 | 268 | 1310 | 32 | 4 | 3 |
| 30 | 9311033 | 340 | 1870 | 47 | 7 | 2 |
| 31 | 9311034 | 182 | 699 | 19 | 3 | 0 |
| 32 | 9311035 | 212 | 1022 | 33 | 6 | 2 |
| 33 | 9311036 | 210 | 860 | 16 | 10 | 0 |
| 34 | 9311037 | 274 | 1366 | 32 | 1 | 2 |
| 35 | 9311038 | 285 | 1667 | 43 | 3 | 4 |
| 36 | 9311039 | 202 | 1053 | 27 | 4 | 3 |
| 37 | 9311040 | 253 | 1453 | 25 | 2 | 2 |
| 38 | 9311041 | 240 | 1341 | 19 | 5 | 0 |
| 39 | 9311042 | 229 | 967 | 23 | 0 | 2 |
| 40 | 9311043 | 250 | 1328 | 26 | 4 | 3 |
| 41 | 9311044 | 178 | 725 | 14 | 4 | 2 |
| 42 | 9311045 | 198 | 944 | 28 | 5 | 2 |
| 43 | 9311046 | 207 | 969 | 18 | 4 | 5 |
| 44 | 9311047 | 210 | 892 | 20 | 7 | 2 |
| 45 | 9311048 | 184 | 894 | 8 | 5 | 1 |
| 46 | 9311049 | 234 | 1479 | 28 | 6 | 2 |
| 47 | 9311050 | 181 | 550 | 10 | 5 | 2 |
| 48 | 9311051 | 259 | 1290 | 34 | 5 | 4 |
| 49 | 9311052 | 232 | 1203 | 35 | 6 | 2 |
| 50 | 9311053 | 214 | 962 | 31 | 12 | 0 |
| 51 | 9311054 | 208 | 664 | 20 | 14 | 1 |
| 52 | 9311055 | 253 | 1190 | 19 | 6 | 2 |
| 53 | 9311056 | 241 | 1243 | 17 | 1 | 1 |
| 54 | 9311057 | 200 | 917 | 26 | 5 | 3 |
| 55 | 9311058 | 213 | 1043 | 12 | 7 | 2 |
| 56 | 9311059 | 233 | 1070 | 25 | 2 | 1 |
| 57 | 9311060 | 255 | 1224 | 28 | 7 | 4 |
| 58 | 9311061 | 161 | 579 | 12 | 1 | 1 |
| 59 | 9311062 | 189 | 897 | 15 | 5 | 1 |
| 60 | 9311063 | 206 | 770 | 14 | 2 | 2 |
| 61 | 9311064 | 246 | 890 | 19 | 4 | 4 |
| 62 | 9311065 | 231 | 1041 | 32 | 4 | 4 |
| 63 | 9311066 | 237 | 1265 | 22 | 9 | 2 |
| 64 | 9311067 | 229 | 1124 | 21 | 5 | 2 |
| 65 | 9311068 | 182 | 723 | 17 | 8 | 0 |
| 66 | 9311069 | 252 | 1211 | 25 | 3 | 3 |
| 67 | 9311070 | 191 | 865 | 22 | 4 | 0 |
| 68 | 9311071 | 220 | 886 | 28 | 0 | 1 |
| 69 | 9311072 | 246 | 1015 | 22 | 8 | 4 |
| 70 | 9311073 | 224 | 1114 | 18 | 3 | 2 |
| 71 | 9311074 | 182 | 849 | 23 | 5 | 2 |
| 72 | 9311075 | 233 | 1093 | 25 | 15 | 2 |
| 73 | 9311076 | 247 | 1245 | 26 | 2 | 3 |
| 74 | 9311077 | 251 | 1086 | 25 | 8 | 2 |
| 75 | 9311078 | 188 | 669 | 17 | 9 | 3 |
| 76 | 9311079 | 220 | 1020 | 10 | 3 | 1 |
| 77 | 9311080 | 223 | 1043 | 28 | 6 | 4 |
| 78 | 9311081 | 235 | 1089 | 30 | 9 | 3 |
| 79 | 9311082 | 256 | 1311 | 30 | 4 | 3 |
| 80 | 9311083 | 275 | 1248 | 37 | 14 | 5 |
| 81 | 9311084 | 184 | 767 | 12 | 6 | 2 |
| 82 | 9311085 | 166 | 634 | 16 | 9 | 1 |
| 83 | 9311086 | 229 | 1154 | 21 | 7 | 0 |
| 84 | 9311087 | 247 | 1103 | 28 | 4 | 3 |
| 85 | 9311088 | 212 | 965 | 34 | 3 | 3 |
| 86 | 9311089 | 228 | 1343 | 38 | 1 | 2 |
| 87 | 9311090 | 210 | 899 | 26 | 10 | 4 |
| 88 | 9311091 | 373 | 2237 | 55 | 3 | 3 |
| 89 | 9311092 | 226 | 987 | 23 | 6 | 0 |
| 90 | 9311093 | 242 | 1138 | 28 | 14 | 1 |
| 91 | 9311094 | 168 | 690 | 16 | 4 | 1 |
| 92 | 9311095 | 220 | 1083 | 16 | 7 | 2 |
| 93 | 9311096 | 198 | 914 | 22 | 7 | 1 |
| 94 | 9311097 | 233 | 1056 | 31 | 2 | 1 |
| 95 | 9311098 | 286 | 1653 | 29 | 9 | 1 |
| 96 | 9311099 | 202 | 989 | 17 | 2 | 2 |
| 97 | 9311100 | 195 | 803 | 21 | 6 | 2 |
| 98 | 9311101 | 224 | 896 | 32 | 2 | 1 |
| 99 | 9311102 | 244 | 957 | 15 | 8 | 1 |
| 100 | 9311103 | 192 | 927 | 14 | 4 | 2 |
| 101 | 9311104 | 233 | 1111 | 19 | 6 | 4 |
| 102 | 9311105 | 177 | 876 | 22 | 8 | 3 |
| 103 | 9311106 | 188 | 723 | 17 | 2 | 0 |
| 104 | 9311107 | 233 | 1088 | 34 | 2 | 0 |
| 105 | 9311108 | 153 | 648 | 14 | 2 | 1 |
| 106 | 9311109 | 212 | 859 | 30 | 4 | 4 |
| 107 | 9311110 | 188 | 741 | 10 | 8 | 0 |
| 108 | 9311111 | 198 | 814 | 19 | 3 | 4 |
| 109 | 9311112 | 269 | 1230 | 28 | 5 | 2 |
| 110 | 9311113 | 223 | 1161 | 28 | 5 | 3 |
| 111 | 9311114 | 261 | 1410 | 35 | 4 | 1 |
| 112 | 9311115 | 240 | 1282 | 21 | 2 | 3 |
| 113 | 9311116 | 208 | 1216 | 30 | 2 | 2 |
| 114 | 9311117 | 235 | 1050 | 23 | 2 | 3 |
| 115 | 9311118 | 221 | 1100 | 20 | 3 | 3 |
| 116 | 9311119 | 232 | 1023 | 30 | 8 | 2 |
| 117 | 9311120 | 173 | 650 | 16 | 5 | 1 |
| 118 | 9311121 | 174 | 601 | 8 | 5 | 1 |
| 119 | 9311122 | 234 | 1022 | 28 | 9 | 3 |
| 120 | 9311123 | 194 | 937 | 13 | 0 | 1 |
| 121 | 9311124 | 270 | 1303 | 24 | 3 | 2 |
| 122 | 9311125 | 269 | 903 | 23 | 6 | 4 |
| 123 | 9311126 | 256 | 918 | 17 | 17 | 1 |
| 124 | 9311127 | 221 | 1090 | 20 | 4 | 1 |
| 125 | 9311128 | 160 | 444 | 14 | 4 | 1 |
| 126 | 9311129 | 267 | 1267 | 34 | 2 | 3 |
| 127 | 9311130 | 200 | 788 | 14 | 6 | 1 |
| 128 | 9311131 | 187 | 823 | 11 | 7 | 0 |
| 129 | 9311132 | 158 | 589 | 9 | 4 | 1 |
| 130 | 9311133 | 171 | 814 | 12 | 4 | 1 |
| 131 | 9311134 | 214 | 1014 | 12 | 4 | 3 |
| 132 | 9311135 | 193 | 821 | 21 | 5 | 2 |
| 133 | 9311136 | 268 | 1203 | 35 | 8 | 5 |
| 134 | 9311137 | 194 | 724 | 21 | 2 | 2 |
| 135 | 9311138 | 268 | 1485 | 33 | 9 | 1 |
| 136 | 9311139 | 211 | 930 | 20 | 3 | 1 |
| 137 | 9311140 | 230 | 1108 | 34 | 7 | 2 |
| 138 | 9311141 | 193 | 778 | 14 | 7 | 1 |
| 139 | 9311142 | 185 | 718 | 22 | 4 | 1 |
| 140 | 9311143 | 266 | 1334 | 32 | 2 | 2 |
| 141 | 9311144 | 212 | 1140 | 20 | 7 | 2 |
| 142 | 9311145 | 204 | 828 | 16 | 12 | 1 |
| 143 | 9311146 | 213 | 956 | 26 | 5 | 2 |
| 144 | 9311147 | 254 | 1255 | 36 | 5 | 1 |
| 145 | 9311148 | 240 | 914 | 35 | 2 | 3 |
| 146 | 9311149 | 195 | 889 | 7 | 4 | 1 |
| 147 | 9311150 | 208 | 1096 | 22 | 5 | 1 |
| 148 | 9311151 | 250 | 1044 | 25 | 16 | 2 |
| 149 | 9311152 | 218 | 1185 | 36 | 6 | 4 |
| 150 | 9311153 | 180 | 711 | 19 | 4 | 3 |
| 151 | 9311154 | 240 | 1429 | 25 | 5 | 1 |
| 152 | 9311155 | 227 | 1031 | 32 | 11 | 2 |
| 153 | 9311156 | 211 | 967 | 30 | 4 | 3 |
| 154 | 9311157 | 266 | 1466 | 25 | 8 | 2 |
| 155 | 9311158 | 204 | 1002 | 18 | 6 | 2 |
| 156 | 9311159 | 205 | 925 | 18 | 4 | 2 |
| 157 | 9311160 | 209 | 1059 | 24 | 3 | 1 |
| 158 | 9311161 | 258 | 1402 | 26 | 7 | 1 |
| 159 | 9311162 | 216 | 977 | 21 | 9 | 3 |
| 160 | 9311163 | 240 | 1253 | 32 | 8 | 2 |
| 161 | 9311164 | 190 | 987 | 27 | 1 | 2 |
| 162 | 9311165 | 223 | 1163 | 33 | 8 | 2 |
| 163 | 9311166 | 233 | 1177 | 19 | 4 | 2 |
| 164 | 9311167 | 173 | 661 | 12 | 7 | 0 |
| 165 | 9311168 | 220 | 1045 | 17 | 8 | 2 |
| 166 | 9311169 | 289 | 1606 | 27 | 4 | 1 |
| 167 | 9311170 | 238 | 1095 | 31 | 3 | 4 |
| 168 | 9311171 | 244 | 1409 | 17 | 7 | 0 |
| 169 | 9311172 | 268 | 1062 | 18 | 5 | 4 |
| 170 | 9311173 | 240 | 1334 | 25 | 5 | 2 |
| 171 | 9311174 | 215 | 960 | 20 | 6 | 3 |
| 172 | 9311175 | 199 | 873 | 12 | 4 | 2 |
| 173 | 9311176 | 200 | 1003 | 16 | 2 | 2 |
| 174 | 9311177 | 228 | 976 | 22 | 1 | 2 |
| 175 | 9311178 | 216 | 988 | 19 | 2 | 0 |
| 176 | 9311179 | 270 | 1036 | 28 | 10 | 3 |
| 177 | 9311180 | 274 | 1517 | 29 | 2 | 4 |
| 178 | 9311181 | 209 | 888 | 27 | 0 | 3 |
| 179 | 9311182 | 213 | 834 | 25 | 5 | 3 |
| 180 | 9311183 | 195 | 957 | 21 | 13 | 2 |
| 181 | 9311184 | 263 | 854 | 18 | 6 | 3 |
| 182 | 9311185 | 242 | 970 | 38 | 0 | 3 |
| 183 | 9311186 | 246 | 1089 | 31 | 9 | 2 |
| 184 | 9311187 | 287 | 1258 | 26 | 5 | 2 |
| 185 | 9311188 | 182 | 887 | 13 | 5 | 3 |
| 186 | 9311189 | 188 | 969 | 15 | 7 | 1 |
| 187 | 9311190 | 186 | 729 | 16 | 3 | 0 |
| 188 | 9311191 | 235 | 828 | 20 | 10 | 2 |
| 189 | 9311192 | 196 | 813 | 22 | 3 | 2 |
| 190 | 9311193 | 187 | 707 | 17 | 3 | 1 |
| 191 | 9311194 | 236 | 1232 | 16 | 6 | 3 |
| 192 | 9311195 | 170 | 700 | 14 | 4 | 1 |
| 193 | 9311196 | 217 | 993 | 28 | 4 | 1 |
| 194 | 9311197 | 277 | 1518 | 26 | 7 | 1 |
| 195 | 9311198 | 268 | 1247 | 31 | 18 | 2 |
| 196 | 9311199 | 234 | 900 | 19 | 7 | 1 |
| 197 | 9311200 | 193 | 824 | 17 | 5 | 0 |
| 198 | 9311201 | 205 | 1024 | 25 | 6 | 0 |
| 199 | 9311202 | 216 | 1165 | 21 | 8 | 3 |
| 200 | 9311203 | 238 | 1134 | 17 | 10 | 2 |
| 201 | 9311204 | 247 | 1264 | 12 | 5 | 1 |
| 202 | 9311205 | 218 | 1072 | 35 | 1 | 2 |
| 203 | 9311206 | 221 | 841 | 23 | 9 | 3 |
| 204 | 9311207 | 199 | 1038 | 18 | 7 | 3 |
| 205 | 9311208 | 196 | 922 | 13 | 8 | 3 |
| 206 | 9311209 | 197 | 970 | 21 | 2 | 2 |
| 207 | 9311210 | 241 | 1275 | 20 | 7 | 2 |
| 208 | 9311211 | 218 | 1070 | 24 | 3 | 2 |
| 209 | 9311212 | 164 | 546 | 11 | 6 | 1 |
| 210 | 9311213 | 329 | 1394 | 40 | 1 | 5 |
| 211 | 9311214 | 139 | 377 | 5 | 12 | 1 |
| 212 | 9311215 | 217 | 912 | 16 | 7 | 0 |
| 213 | 9311216 | 175 | 654 | 17 | 3 | 1 |
| 214 | 9311217 | 298 | 1408 | 27 | 9 | 0 |
| 215 | 9311218 | 222 | 818 | 14 | 5 | 2 |
| 216 | 9311219 | 176 | 732 | 16 | 2 | 1 |
| 217 | 9311220 | 204 | 828 | 18 | 6 | 1 |
| 218 | 9311221 | 234 | 1277 | 33 | 10 | 4 |
| 219 | 9311222 | 200 | 1082 | 22 | 3 | 2 |
| 220 | 9311223 | 263 | 1664 | 36 | 8 | 2 |
| 221 | 9311224 | 234 | 1043 | 18 | 7 | 2 |
| 222 | 9311225 | 201 | 1073 | 20 | 5 | 2 |
| 223 | 9311226 | 190 | 781 | 20 | 5 | 2 |
| 224 | 9311227 | 250 | 1171 | 22 | 13 | 3 |
| 225 | 9311228 | 167 | 791 | 16 | 4 | 1 |
| 226 | 9311229 | 232 | 1152 | 30 | 6 | 4 |
| 227 | 9311230 | 233 | 1054 | 12 | 5 | 1 |
| 228 | 9311231 | 188 | 777 | 13 | 4 | 0 |
| 229 | 9311232 | 228 | 1163 | 21 | 3 | 2 |
| 230 | 9311233 | 261 | 1328 | 24 | 3 | 1 |
| 231 | 9311234 | 223 | 973 | 28 | 3 | 2 |
| 232 | 9311235 | 178 | 760 | 21 | 2 | 3 |
| 233 | 9311236 | 313 | 1735 | 29 | 2 | 3 |
| 234 | 9311237 | 307 | 1583 | 39 | 6 | 4 |
| 235 | 9311238 | 206 | 734 | 23 | 3 | 3 |
| 236 | 9311239 | 256 | 1083 | 23 | 8 | 1 |
| 237 | 9311240 | 270 | 1312 | 29 | 6 | 2 |
| 238 | 9311241 | 189 | 827 | 22 | 7 | 2 |
| 239 | 9311242 | 246 | 1160 | 16 | 10 | 3 |
| 240 | 9311243 | 238 | 921 | 34 | 3 | 4 |
| 241 | 9311244 | 234 | 1132 | 20 | 4 | 5 |
| 242 | 9311245 | 204 | 845 | 20 | 4 | 3 |
| 243 | 9311246 | 175 | 801 | 17 | 2 | 1 |
| 244 | 9311247 | 197 | 1006 | 16 | 11 | 0 |
| 245 | 9311248 | 200 | 479 | 14 | 6 | 1 |
| 246 | 9311249 | 208 | 739 | 29 | 6 | 2 |
| 247 | 9311250 | 218 | 975 | 22 | 18 | 4 |
| 248 | 9311251 | 271 | 1476 | 25 | 3 | 2 |
| 249 | 9311252 | 224 | 1062 | 11 | 11 | 0 |
| 250 | 9311253 | 198 | 957 | 17 | 4 | 1 |
| 251 | 9311254 | 245 | 1185 | 23 | 3 | 4 |
| 252 | 9311255 | 262 | 1581 | 30 | 10 | 3 |
| 253 | 9311256 | 267 | 1486 | 34 | 3 | 2 |
| 254 | 9311257 | 242 | 991 | 29 | 17 | 3 |
| 255 | 9311258 | 277 | 1389 | 18 | 3 | 2 |
| 256 | 9311259 | 237 | 945 | 17 | 4 | 0 |
| 257 | 9311260 | 226 | 1020 | 10 | 8 | 0 |
| 258 | 9311261 | 213 | 1166 | 29 | 2 | 4 |
| 259 | 9311262 | 270 | 1178 | 18 | 5 | 4 |
| 260 | 9311263 | 333 | 1804 | 25 | 4 | 4 |
| 261 | 9311264 | 200 | 791 | 15 | 2 | 0 |
| 262 | 9311265 | 235 | 1078 | 28 | 5 | 4 |
| 263 | 9311266 | 165 | 479 | 6 | 2 | 1 |
| 264 | 9311267 | 278 | 1082 | 22 | 10 | 2 |
| 265 | 9311268 | 241 | 1199 | 31 | 7 | 0 |
| 266 | 9311269 | 200 | 816 | 30 | 6 | 4 |
| 267 | 9311270 | 208 | 1079 | 15 | 8 | 2 |
| 268 | 9311271 | 233 | 760 | 17 | 2 | 2 |
| 269 | 9311272 | 204 | 953 | 18 | 6 | 1 |
| 270 | 9311273 | 218 | 1053 | 29 | 4 | 2 |
| 271 | 9311274 | 287 | 1589 | 41 | 2 | 4 |
| 272 | 9311275 | 209 | 835 | 21 | 6 | 2 |
| 273 | 9311276 | 225 | 871 | 28 | 6 | 2 |
| 274 | 9311277 | 263 | 1343 | 27 | 2 | 3 |
| 275 | 9311278 | 247 | 1338 | 20 | 13 | 3 |
| 276 | 9311279 | 264 | 1417 | 37 | 3 | 2 |
| 277 | 9311280 | 281 | 1326 | 29 | 6 | 2 |
| 278 | 9311281 | 229 | 1184 | 28 | 6 | 3 |
| 279 | 9311282 | 196 | 771 | 22 | 4 | 2 |
| 280 | 9311283 | 246 | 1110 | 27 | 0 | 1 |
| 281 | 9311284 | 182 | 634 | 13 | 3 | 1 |
| 282 | 9311285 | 258 | 1319 | 36 | 4 | 5 |
| 283 | 9311286 | 198 | 918 | 19 | 3 | 1 |
| 284 | 9311287 | 203 | 926 | 16 | 4 | 5 |
| 285 | 9311288 | 251 | 1274 | 15 | 4 | 2 |
| 286 | 9311289 | 178 | 797 | 18 | 6 | 1 |
| 287 | 9311290 | 214 | 1087 | 31 | 2 | 2 |
| 288 | 9311291 | 190 | 733 | 17 | 4 | 1 |
| 289 | 9311292 | 217 | 1098 | 17 | 2 | 3 |
| 290 | 9311293 | 243 | 1237 | 38 | 8 | 0 |
| 291 | 9311294 | 247 | 1044 | 19 | 7 | 5 |
| 292 | 9311295 | 222 | 998 | 8 | 6 | 2 |
| 293 | 9311296 | 273 | 1216 | 28 | 7 | 3 |
| 294 | 9311297 | 193 | 678 | 19 | 3 | 1 |
| 295 | 9311298 | 186 | 538 | 13 | 5 | 3 |
| 296 | 9311299 | 268 | 1050 | 25 | 10 | 1 |
| 297 | 9311300 | 233 | 978 | 24 | 3 | 1 |
| 298 | 9311301 | 200 | 1023 | 20 | 7 | 3 |
| 299 | 9311302 | 266 | 1434 | 33 | 7 | 5 |
| 300 | 9311303 | 318 | 1725 | 34 | 7 | 1 |
| 301 | 9311304 | 206 | 736 | 14 | 2 | 3 |
| 302 | 9311305 | 192 | 557 | 12 | 3 | 0 |
| 303 | 9311306 | 157 | 567 | 11 | 3 | 0 |
| 304 | 9311307 | 232 | 1317 | 15 | 1 | 2 |
| 305 | 9311308 | 213 | 926 | 17 | 3 | 2 |
| 306 | 9311309 | 276 | 1482 | 24 | 7 | 0 |
| 307 | 9311310 | 226 | 1151 | 16 | 4 | 4 |
| 308 | 9311311 | 291 | 1748 | 33 | 4 | 3 |
| 309 | 9311312 | 214 | 1038 | 9 | 3 | 0 |
| 310 | 9311313 | 199 | 832 | 28 | 3 | 4 |
| 311 | 9311314 | 263 | 1098 | 30 | 14 | 5 |
| 312 | 9311315 | 275 | 1377 | 29 | 5 | 1 |
| 313 | 9311316 | 306 | 1575 | 40 | 9 | 2 |
| 314 | 9311317 | 197 | 798 | 18 | 2 | 2 |
| 315 | 9311318 | 272 | 1413 | 24 | 8 | 3 |
| 316 | 9311319 | 218 | 939 | 38 | 6 | 4 |
| 317 | 9311320 | 221 | 984 | 28 | 6 | 0 |
| 318 | 9311321 | 179 | 698 | 14 | 3 | 2 |
| 319 | 9311322 | 182 | 818 | 14 | 11 | 2 |
| 320 | 9311323 | 192 | 1013 | 18 | 4 | 2 |
| 321 | 9311324 | 247 | 1133 | 24 | 5 | 1 |
| 322 | 9311325 | 242 | 1203 | 20 | 6 | 1 |
| 323 | 9311326 | 237 | 1294 | 12 | 8 | 3 |
| 324 | 9311327 | 224 | 1015 | 22 | 3 | 1 |
| 325 | 9311328 | 247 | 1427 | 25 | 6 | 2 |
| 326 | 9311329 | 189 | 605 | 21 | 2 | 0 |
| 327 | 9311330 | 172 | 765 | 9 | 7 | 1 |
| 328 | 9311331 | 229 | 1153 | 24 | 2 | 1 |
| 329 | 9311332 | 173 | 582 | 17 | 6 | 1 |
| 330 | 9311333 | 259 | 1148 | 20 | 8 | 1 |
| 331 | 9311334 | 170 | 618 | 19 | 1 | 2 |
| 332 | 9311335 | 228 | 998 | 20 | 9 | 3 |
| 333 | 9311336 | 225 | 1234 | 18 | 13 | 5 |
| 334 | 9311337 | 188 | 800 | 30 | 12 | 2 |
| 335 | 9311338 | 256 | 1307 | 30 | 4 | 1 |
| 336 | 9311339 | 211 | 904 | 11 | 5 | 3 |
| 337 | 9311340 | 196 | 733 | 16 | 7 | 1 |
| 338 | 9311341 | 229 | 1023 | 28 | 11 | 2 |
| 339 | 9311342 | 225 | 842 | 22 | 1 | 2 |
| 340 | 9311343 | 174 | 643 | 8 | 4 | 0 |
| 341 | 9311344 | 177 | 491 | 9 | 7 | 1 |
| 342 | 9311345 | 177 | 857 | 26 | 3 | 2 |
| 343 | 9311346 | 250 | 1172 | 12 | 13 | 5 |
| 344 | 9311347 | 227 | 1151 | 26 | 3 | 2 |
| 345 | 9311348 | 247 | 1237 | 38 | 4 | 4 |
| 346 | 9311349 | 234 | 1173 | 17 | 4 | 1 |
| 347 | 9311350 | 214 | 1091 | 15 | 3 | 2 |
| 348 | 9311351 | 211 | 1019 | 16 | 4 | 2 |
| 349 | 9311352 | 234 | 1169 | 21 | 7 | 1 |
| 350 | 9311353 | 335 | 2011 | 43 | 2 | 4 |
| 351 | 9311354 | 182 | 712 | 20 | 6 | 2 |
| 352 | 9311355 | 270 | 1427 | 26 | 2 | 1 |
| 353 | 9311356 | 192 | 801 | 19 | 12 | 1 |
| 354 | 9311357 | 291 | 1443 | 35 | 11 | 1 |
| 355 | 9311358 | 271 | 1503 | 37 | 5 | 3 |
| 356 | 9311359 | 246 | 1133 | 19 | 0 | 3 |
| 357 | 9311360 | 182 | 803 | 15 | 4 | 1 |
| 358 | 9311361 | 231 | 1223 | 31 | 7 | 3 |
| 359 | 9311362 | 180 | 661 | 14 | 8 | 2 |
| 360 | 9311363 | 171 | 898 | 10 | 4 | 1 |
| 361 | 9311364 | 191 | 730 | 16 | 6 | 0 |
| 362 | 9311365 | 215 | 1008 | 15 | 9 | 1 |
| 363 | 9311366 | 217 | 1097 | 24 | 7 | 2 |
| 364 | 9311367 | 257 | 1376 | 21 | 4 | 2 |
| 365 | 9311368 | 288 | 1657 | 28 | 2 | 2 |
| 366 | 9311369 | 245 | 1183 | 28 | 7 | 1 |
| 367 | 9311370 | 241 | 972 | 28 | 4 | 2 |
| 368 | 9311371 | 277 | 1603 | 24 | 6 | 1 |
| 369 | 9311372 | 234 | 1279 | 37 | 3 | 4 |
| 370 | 9311373 | 188 | 720 | 14 | 5 | 1 |
| 371 | 9311374 | 217 | 1017 | 25 | 5 | 2 |
| 372 | 9311375 | 249 | 1069 | 29 | 8 | 1 |
| 373 | 9311376 | 243 | 1091 | 25 | 2 | 2 |
| 374 | 9311377 | 249 | 1349 | 28 | 11 | 2 |
| 375 | 9311378 | 177 | 668 | 5 | 3 | 1 |
| 376 | 9311379 | 225 | 735 | 14 | 3 | 1 |
| 377 | 9311380 | 216 | 1032 | 26 | 4 | 2 |
| 378 | 9311381 | 299 | 1438 | 31 | 7 | 3 |
| 379 | 9311382 | 180 | 780 | 14 | 5 | 1 |
| 380 | 9311383 | 196 | 814 | 22 | 3 | 0 |
| 381 | 9311384 | 221 | 1181 | 28 | 8 | 3 |
| 382 | 9311385 | 212 | 850 | 14 | 2 | 1 |
| 383 | 9311386 | 188 | 820 | 22 | 2 | 2 |
| 384 | 9311387 | 233 | 973 | 21 | 6 | 1 |
| 385 | 9311388 | 285 | 1526 | 26 | 9 | 3 |
| 386 | 9311389 | 211 | 802 | 18 | 13 | 0 |
| 387 | 9311390 | 294 | 1667 | 28 | 6 | 2 |
| 388 | 9311391 | 231 | 1031 | 15 | 6 | 2 |
| 389 | 9311392 | 161 | 585 | 13 | 9 | 1 |
| 390 | 9311393 | 200 | 873 | 18 | 4 | 2 |
| 391 | 9311394 | 249 | 1057 | 14 | 3 | 2 |
| 392 | 9311395 | 200 | 874 | 23 | 6 | 2 |
| 393 | 9311396 | 245 | 899 | 24 | 5 | 3 |
| 394 | 9311397 | 238 | 1317 | 34 | 4 | 4 |
| 395 | 9311398 | 257 | 1061 | 34 | 4 | 4 |
| 396 | 9311399 | 227 | 1163 | 25 | 4 | 4 |
| 397 | 9311400 | 250 | 1020 | 14 | 5 | 2 |
| 398 | 9311401 | 258 | 1435 | 29 | 8 | 2 |
| 399 | 9311402 | 257 | 1387 | 35 | 5 | 3 |
| 400 | 9311403 | 189 | 784 | 12 | 4 | 1 |
| 401 | 9311404 | 197 | 775 | 23 | 5 | 3 |
| 402 | 9311405 | 227 | 985 | 21 | 3 | 1 |
| 403 | 9311406 | 225 | 952 | 21 | 4 | 2 |
| 404 | 9311407 | 244 | 1115 | 28 | 5 | 4 |
| 405 | 9311408 | 240 | 1313 | 22 | 2 | 3 |
| 406 | 9311409 | 231 | 1237 | 27 | 6 | 1 |
| 407 | 9311410 | 203 | 934 | 29 | 6 | 2 |
| 408 | 9311411 | 207 | 599 | 19 | 2 | 1 |
| 409 | 9311412 | 239 | 1085 | 22 | 6 | 3 |
| 410 | 9311413 | 225 | 872 | 22 | 0 | 3 |
| 411 | 9311414 | 258 | 1228 | 37 | 3 | 2 |
| 412 | 9311415 | 197 | 925 | 22 | 5 | 2 |
| 413 | 9311416 | 214 | 1072 | 29 | 5 | 3 |
| 414 | 9311417 | 210 | 997 | 18 | 4 | 1 |
| 415 | 9311418 | 201 | 996 | 29 | 6 | 1 |
| 416 | 9311419 | 216 | 1072 | 19 | 4 | 2 |
| 417 | 9311420 | 220 | 1125 | 27 | 5 | 3 |
| 418 | 9311421 | 291 | 1434 | 23 | 3 | 0 |
| 419 | 9311422 | 197 | 859 | 15 | 4 | 0 |
| 420 | 9311423 | 278 | 1425 | 52 | 6 | 4 |
| 421 | 9311424 | 207 | 1142 | 16 | 3 | 2 |
| 422 | 9311425 | 220 | 886 | 16 | 6 | 1 |
| 423 | 9311426 | 169 | 685 | 12 | 1 | 0 |
| 424 | 9311427 | 253 | 1258 | 25 | 5 | 2 |
| 425 | 9311428 | 273 | 1136 | 26 | 5 | 3 |
| 426 | 9311429 | 221 | 827 | 17 | 9 | 1 |
| 427 | 9311430 | 220 | 959 | 28 | 4 | 3 |
| 428 | 9311431 | 208 | 1090 | 11 | 5 | 2 |
| 429 | 9311432 | 242 | 1245 | 24 | 1 | 2 |
| 430 | 9311433 | 218 | 1070 | 17 | 3 | 3 |
| 431 | 9311434 | 191 | 739 | 20 | 4 | 0 |
| 432 | 9311435 | 205 | 985 | 17 | 7 | 1 |
| 433 | 9311436 | 265 | 1378 | 29 | 9 | 6 |
| 434 | 9311437 | 242 | 1422 | 29 | 3 | 0 |
| 435 | 9311438 | 188 | 873 | 19 | 5 | 3 |
| 436 | 9311439 | 178 | 768 | 13 | 7 | 0 |
| 437 | 9311440 | 183 | 658 | 13 | 5 | 2 |
| 438 | 9311441 | 225 | 1062 | 22 | 10 | 3 |
| 439 | 9311442 | 289 | 1647 | 21 | 5 | 4 |
| 440 | 9311443 | 197 | 1081 | 13 | 3 | 1 |
| 441 | 9311444 | 228 | 1091 | 35 | 10 | 3 |
| 442 | 9311445 | 171 | 714 | 5 | 5 | 2 |
| 443 | 9311446 | 182 | 903 | 21 | 7 | 3 |
| 444 | 9311447 | 193 | 873 | 16 | 4 | 1 |
| 445 | 9311448 | 255 | 1260 | 22 | 6 | 2 |
| 446 | 9311449 | 189 | 1051 | 17 | 2 | 1 |
| 447 | 9311450 | 269 | 1318 | 36 | 3 | 4 |
| 448 | 9311451 | 199 | 833 | 24 | 0 | 1 |
| 449 | 9311452 | 199 | 924 | 25 | 5 | 2 |
| 450 | 9311453 | 234 | 923 | 14 | 6 | 4 |
| 451 | 9311454 | 223 | 1002 | 17 | 5 | 2 |
| 452 | 9311455 | 209 | 1137 | 26 | 6 | 2 |
| 453 | 9311456 | 191 | 910 | 15 | 1 | 3 |
| 454 | 9311457 | 248 | 1108 | 12 | 8 | 3 |
| 455 | 9311458 | 204 | 812 | 19 | 6 | 2 |
| 456 | 9311459 | 216 | 805 | 18 | 6 | 2 |
| 457 | 9311460 | 225 | 1162 | 18 | 6 | 0 |
| 458 | 9311461 | 230 | 1133 | 29 | 5 | 1 |
| 459 | 9311462 | 286 | 1910 | 28 | 7 | 1 |
| 460 | 9311463 | 271 | 1228 | 46 | 4 | 5 |
| 461 | 9311464 | 230 | 1050 | 21 | 7 | 2 |
| 462 | 9311465 | 237 | 1334 | 30 | 3 | 3 |
| 463 | 9311466 | 233 | 939 | 29 | 5 | 3 |
| 464 | 9311467 | 226 | 1013 | 30 | 5 | 5 |
| 465 | 9311468 | 205 | 710 | 14 | 5 | 2 |
| 466 | 9311469 | 201 | 970 | 16 | 3 | 0 |
| 467 | 9311470 | 210 | 1132 | 26 | 11 | 4 |
| 468 | 9311471 | 251 | 1403 | 24 | 5 | 1 |
| 469 | 9311472 | 238 | 998 | 13 | 5 | 0 |
| 470 | 9311473 | 238 | 1172 | 30 | 7 | 4 |
| 471 | 9311474 | 279 | 1594 | 39 | 8 | 2 |
| 472 | 9311475 | 183 | 659 | 12 | 3 | 2 |
| 473 | 9311476 | 187 | 567 | 15 | 4 | 2 |
| 474 | 9311477 | 222 | 1235 | 23 | 3 | 1 |
| 475 | 9311478 | 250 | 1292 | 22 | 6 | 1 |
| 476 | 9311479 | 184 | 827 | 6 | 9 | 1 |
| 477 | 9311480 | 188 | 683 | 22 | 3 | 3 |
| 478 | 9311481 | 265 | 888 | 22 | 2 | 3 |
| 479 | 9311482 | 188 | 728 | 17 | 2 | 2 |
| 480 | 9311483 | 173 | 760 | 11 | 3 | 0 |
| 481 | 9311484 | 214 | 1046 | 11 | 9 | 3 |
| 482 | 9311485 | 187 | 894 | 19 | 4 | 1 |
| 483 | 9311486 | 274 | 1267 | 28 | 3 | 5 |
| 484 | 9311487 | 195 | 858 | 16 | 7 | 1 |
| 485 | 9311488 | 247 | 892 | 13 | 11 | 2 |
| 486 | 9311489 | 270 | 1221 | 33 | 15 | 5 |
| 487 | 9311490 | 246 | 1172 | 35 | 5 | 4 |
| 488 | 9311491 | 299 | 1628 | 44 | 5 | 2 |
| 489 | 9311492 | 187 | 731 | 14 | 7 | 2 |
| 490 | 9311493 | 201 | 950 | 16 | 5 | 2 |
| 491 | 9311494 | 173 | 732 | 20 | 5 | 3 |
| 492 | 9311495 | 210 | 975 | 21 | 3 | 4 |
| 493 | 9311496 | 277 | 1475 | 25 | 3 | 1 |
| 494 | 9311497 | 200 | 644 | 19 | 0 | 2 |
| 495 | 9311498 | 191 | 964 | 18 | 1 | 1 |
| 496 | 9311499 | 239 | 1104 | 25 | 6 | 4 |
| 497 | 9311500 | 248 | 1222 | 26 | 14 | 3 |
| 498 | 9311501 | 211 | 762 | 18 | 6 | 3 |
| 499 | 9311502 | 209 | 867 | 20 | 1 | 1 |
| 500 | 9311503 | 253 | 1192 | 8 | 11 | 2 |
| 501 | 9311504 | 215 | 967 | 24 | 6 | 1 |
| 502 | 9311505 | 237 | 1413 | 27 | 1 | 1 |
| 503 | 9311506 | 242 | 1269 | 23 | 16 | 2 |
| 504 | 9311507 | 205 | 831 | 22 | 2 | 3 |
| 505 | 9311508 | 263 | 1446 | 17 | 6 | 4 |
| 506 | 9311509 | 249 | 1046 | 17 | 12 | 2 |
| 507 | 9311510 | 213 | 1085 | 29 | 4 | 5 |
| 508 | 9311511 | 250 | 1078 | 25 | 6 | 2 |
| 509 | 9311512 | 275 | 1359 | 29 | 3 | 2 |
| 510 | 9311513 | 181 | 696 | 10 | 7 | 1 |
| 511 | 9311514 | 278 | 1390 | 40 | 3 | 3 |
| 512 | 9311515 | 178 | 730 | 19 | 5 | 2 |
| 513 | 9311516 | 278 | 1471 | 32 | 5 | 2 |
| 514 | 9311517 | 184 | 953 | 22 | 2 | 0 |
| 515 | 9311518 | 267 | 1489 | 20 | 4 | 1 |
| 516 | 9311519 | 222 | 1210 | 23 | 8 | 1 |
| 517 | 9311520 | 384 | 1833 | 37 | 2 | 1 |
| 518 | 9311521 | 242 | 1139 | 18 | 5 | 1 |
| 519 | 9311522 | 178 | 719 | 20 | 4 | 3 |
| 520 | 9311523 | 169 | 711 | 8 | 2 | 0 |
| 521 | 9311524 | 219 | 881 | 15 | 2 | 2 |
| 522 | 9311525 | 240 | 1167 | 17 | 6 | 3 |
| 523 | 9311526 | 187 | 734 | 17 | 13 | 3 |
| 524 | 9311527 | 245 | 1316 | 17 | 6 | 3 |
| 525 | 9311528 | 229 | 1418 | 14 | 2 | 4 |
| 526 | 9311529 | 315 | 2063 | 36 | 12 | 2 |
| 527 | 9311530 | 203 | 1033 | 17 | 8 | 1 |
| 528 | 9311531 | 231 | 969 | 26 | 5 | 3 |
| 529 | 9311532 | 238 | 1047 | 27 | 4 | 1 |
| 530 | 9311533 | 257 | 1397 | 23 | 1 | 1 |
| 531 | 9311534 | 289 | 1382 | 37 | 1 | 4 |
| 532 | 9311535 | 246 | 1022 | 32 | 1 | 3 |
| 533 | 9311536 | 174 | 679 | 17 | 3 | 3 |
| 534 | 9311537 | 161 | 702 | 18 | 3 | 2 |
| 535 | 9311538 | 258 | 1362 | 27 | 4 | 3 |
| 536 | 9311539 | 185 | 836 | 19 | 2 | 2 |
| 537 | 9311540 | 180 | 564 | 21 | 3 | 0 |
| 538 | 9311541 | 254 | 1251 | 28 | 4 | 2 |
| 539 | 9311542 | 213 | 1067 | 25 | 7 | 2 |
| 540 | 9311543 | 306 | 1776 | 19 | 5 | 1 |
| 541 | 9311544 | 216 | 1117 | 24 | 4 | 1 |
| 542 | 9311545 | 204 | 915 | 11 | 4 | 1 |
| 543 | 9311546 | 156 | 546 | 15 | 3 | 1 |
| 544 | 9311547 | 283 | 1370 | 41 | 1 | 2 |
| 545 | 9311548 | 288 | 1479 | 26 | 7 | 3 |
| 546 | 9311549 | 223 | 1124 | 23 | 7 | 1 |
| 547 | 9311550 | 225 | 1120 | 16 | 3 | 4 |
| 548 | 9311551 | 246 | 976 | 31 | 3 | 4 |
| 549 | 9311552 | 223 | 1090 | 16 | 4 | 2 |
| 550 | 9311553 | 173 | 888 | 21 | 4 | 0 |
| 551 | 9311554 | 184 | 872 | 18 | 9 | 2 |
| 552 | 9311555 | 231 | 1191 | 26 | 8 | 2 |
| 553 | 9311556 | 189 | 778 | 16 | 6 | 1 |
| 554 | 9311557 | 201 | 893 | 19 | 4 | 0 |
| 555 | 9311558 | 236 | 1089 | 20 | 5 | 2 |
| 556 | 9311559 | 244 | 1029 | 14 | 7 | 2 |
| 557 | 9311560 | 178 | 674 | 17 | 4 | 2 |
| 558 | 9311561 | 213 | 1079 | 28 | 3 | 0 |
| 559 | 9311562 | 235 | 1259 | 24 | 4 | 0 |
| 560 | 9311563 | 212 | 816 | 19 | 6 | 2 |
| 561 | 9311564 | 278 | 1426 | 44 | 6 | 4 |
| 562 | 9311565 | 199 | 652 | 17 | 9 | 1 |
| 563 | 9311566 | 244 | 1174 | 26 | 5 | 2 |
| 564 | 9311567 | 212 | 991 | 15 | 1 | 1 |
| 565 | 9311568 | 263 | 1170 | 24 | 6 | 1 |
| 566 | 9311569 | 217 | 1141 | 16 | 1 | 3 |
| 567 | 9311570 | 194 | 932 | 24 | 2 | 2 |
| 568 | 9311571 | 249 | 1318 | 23 | 4 | 3 |
| 569 | 9311572 | 212 | 954 | 37 | 3 | 2 |
| 570 | 9311573 | 227 | 698 | 19 | 4 | 1 |
| 571 | 9311574 | 215 | 1140 | 25 | 5 | 1 |
| 572 | 9311575 | 239 | 1171 | 27 | 2 | 3 |
| 573 | 9311576 | 216 | 1103 | 29 | 6 | 3 |
| 574 | 9311577 | 223 | 947 | 24 | 7 | 2 |
| 575 | 9311578 | 432 | 2437 | 86 | 11 | 4 |
| 576 | 9311579 | 186 | 728 | 12 | 13 | 1 |
| 577 | 9311580 | 157 | 537 | 17 | 6 | 0 |
| 578 | 9311581 | 186 | 862 | 12 | 1 | 1 |
| 579 | 9311582 | 219 | 1120 | 33 | 5 | 3 |
| 580 | 9311583 | 298 | 1454 | 38 | 7 | 3 |
| 581 | 9311584 | 244 | 1089 | 31 | 6 | 4 |
| 582 | 9311585 | 219 | 1124 | 36 | 7 | 2 |
| 583 | 9311586 | 210 | 824 | 23 | 8 | 3 |
| 584 | 9311587 | 240 | 1155 | 26 | 7 | 4 |
| 585 | 9311588 | 243 | 1328 | 24 | 5 | 5 |
| 586 | 9311589 | 186 | 788 | 9 | 1 | 1 |
| 587 | 9311590 | 248 | 942 | 29 | 4 | 2 |
| 588 | 9311591 | 140 | 444 | 9 | 6 | 1 |
| 589 | 9311592 | 169 | 721 | 15 | 6 | 1 |
| 590 | 9311593 | 188 | 869 | 15 | 4 | 2 |
| 591 | 9311594 | 201 | 995 | 33 | 8 | 3 |
| 592 | 9311595 | 227 | 1010 | 20 | 4 | 5 |
| 593 | 9311596 | 176 | 677 | 10 | 9 | 2 |
| 594 | 9311597 | 200 | 825 | 27 | 9 | 3 |
| 595 | 9311598 | 225 | 1175 | 21 | 4 | 4 |
| 596 | 9311599 | 210 | 1005 | 25 | 7 | 2 |
| 597 | 9311600 | 263 | 1467 | 27 | 4 | 2 |
| 598 | 9311601 | 165 | 654 | 11 | 4 | 3 |
| 599 | 9311602 | 210 | 1101 | 16 | 6 | 2 |
| 600 | 9311603 | 195 | 764 | 17 | 7 | 2 |
| 601 | 9311604 | 207 | 953 | 19 | 4 | 2 |
| 602 | 9311605 | 220 | 1164 | 19 | 2 | 0 |
| 603 | 9311606 | 182 | 766 | 14 | 7 | 2 |
| 604 | 9311607 | 184 | 828 | 14 | 4 | 2 |
| 605 | 9311608 | 236 | 853 | 25 | 6 | 2 |
| 606 | 9311609 | 239 | 1089 | 26 | 10 | 3 |
| 607 | 9311610 | 242 | 1138 | 24 | 12 | 3 |
| 608 | 9311611 | 250 | 1155 | 21 | 11 | 1 |
| 609 | 9311612 | 261 | 1462 | 14 | 5 | 2 |
| 610 | 9311613 | 223 | 1167 | 38 | 5 | 2 |
| 611 | 9311614 | 337 | 1789 | 51 | 2 | 4 |
| 612 | 9311615 | 203 | 889 | 19 | 6 | 2 |
| 613 | 9311616 | 264 | 1448 | 27 | 7 | 4 |
| 614 | 9311617 | 205 | 1018 | 29 | 4 | 1 |
| 615 | 9311618 | 195 | 859 | 17 | 6 | 3 |
| 616 | 9311619 | 234 | 1232 | 31 | 6 | 2 |
| 617 | 9311620 | 231 | 1171 | 27 | 4 | 3 |
| 618 | 9311621 | 280 | 1592 | 30 | 3 | 2 |
| 619 | 9311622 | 270 | 1432 | 32 | 7 | 2 |
| 620 | 9311623 | 268 | 1535 | 23 | 5 | 5 |
| 621 | 9311624 | 218 | 1136 | 23 | 6 | 2 |
| 622 | 9311625 | 188 | 840 | 26 | 4 | 3 |
| 623 | 9311626 | 246 | 1304 | 25 | 8 | 1 |
| 624 | 9311627 | 217 | 954 | 32 | 11 | 3 |
| 625 | 9311628 | 241 | 1228 | 28 | 3 | 4 |
| 626 | 9311629 | 292 | 1614 | 25 | 1 | 2 |
| 627 | 9311630 | 226 | 1139 | 6 | 4 | 5 |
| 628 | 9311631 | 266 | 1384 | 28 | 12 | 3 |
| 629 | 9311632 | 269 | 1377 | 20 | 9 | 2 |
| 630 | 9311633 | 229 | 1106 | 25 | 1 | 2 |
| 631 | 9311634 | 196 | 796 | 18 | 13 | 2 |
| 632 | 9311635 | 244 | 1285 | 21 | 5 | 5 |
| 633 | 9311636 | 231 | 1183 | 23 | 8 | 4 |
| 634 | 9311637 | 210 | 1100 | 29 | 0 | 3 |
| 635 | 9311638 | 245 | 1036 | 20 | 1 | 2 |
| 636 | 9311639 | 208 | 924 | 21 | 2 | 1 |
| 637 | 9311640 | 224 | 1163 | 25 | 9 | 4 |
| 638 | 9311641 | 219 | 1041 | 20 | 1 | 0 |
| 639 | 9311642 | 233 | 1089 | 21 | 13 | 3 |
| 640 | 9311643 | 225 | 1088 | 17 | 5 | 3 |
| 641 | 9311644 | 237 | 1128 | 20 | 8 | 2 |
| 642 | 9311645 | 228 | 863 | 13 | 3 | 2 |
| 643 | 9311646 | 272 | 1107 | 13 | 6 | 3 |
| 644 | 9311647 | 282 | 1628 | 19 | 6 | 2 |
| 645 | 9311648 | 283 | 1648 | 29 | 10 | 2 |
| 646 | 9311649 | 208 | 940 | 22 | 4 | 0 |
| 647 | 9311650 | 215 | 673 | 17 | 2 | 2 |
| 648 | 9311651 | 296 | 1462 | 41 | 9 | 3 |
| 649 | 9311652 | 244 | 1117 | 28 | 6 | 5 |
| 650 | 9311653 | 195 | 957 | 20 | 7 | 2 |
| 651 | 9311654 | 196 | 777 | 18 | 5 | 0 |
| 652 | 9311655 | 177 | 752 | 20 | 4 | 0 |
| 653 | 9311656 | 252 | 1228 | 31 | 4 | 1 |
| 654 | 9311657 | 228 | 1125 | 25 | 3 | 2 |
| 655 | 9311658 | 285 | 1204 | 25 | 2 | 2 |
| 656 | 9311659 | 167 | 683 | 18 | 1 | 0 |
| 657 | 9311660 | 168 | 734 | 13 | 4 | 1 |
| 658 | 9311661 | 188 | 774 | 17 | 5 | 2 |
| 659 | 9311662 | 207 | 1134 | 25 | 5 | 2 |
| 660 | 9311663 | 198 | 843 | 14 | 5 | 1 |
| 661 | 9311664 | 223 | 998 | 24 | 2 | 4 |
| 662 | 9311665 | 184 | 856 | 14 | 3 | 0 |
| 663 | 9311666 | 221 | 1270 | 24 | 4 | 2 |
| 664 | 9311667 | 246 | 1326 | 32 | 7 | 3 |
| 665 | 9311668 | 228 | 1352 | 25 | 2 | 3 |
| 666 | 9311669 | 240 | 1267 | 21 | 1 | 0 |
| 667 | 9311670 | 270 | 1511 | 44 | 4 | 3 |
| 668 | 9311671 | 216 | 945 | 29 | 5 | 3 |
| 669 | 9311672 | 245 | 1164 | 18 | 5 | 0 |
| 670 | 9311673 | 219 | 834 | 22 | 3 | 2 |
| 671 | 9311674 | 304 | 1858 | 39 | 6 | 1 |
| 672 | 9311675 | 185 | 838 | 16 | 4 | 1 |
| 673 | 9311676 | 221 | 1276 | 37 | 9 | 2 |
| 674 | 9311677 | 195 | 741 | 22 | 4 | 3 |
| 675 | 9311678 | 221 | 1115 | 32 | 4 | 3 |
| 676 | 9311679 | 208 | 886 | 26 | 2 | 1 |
| 677 | 9311680 | 266 | 1340 | 44 | 3 | 5 |
| 678 | 9311681 | 204 | 843 | 25 | 3 | 3 |
| 679 | 9311682 | 227 | 1251 | 27 | 4 | 2 |
| 680 | 9311683 | 164 | 578 | 12 | 5 | 1 |
| 681 | 9311684 | 181 | 851 | 19 | 2 | 2 |
| 682 | 9311685 | 260 | 1382 | 30 | 9 | 4 |
| 683 | 9311686 | 227 | 1029 | 28 | 5 | 3 |
| 684 | 9311687 | 190 | 756 | 16 | 7 | 2 |
| 685 | 9311688 | 203 | 842 | 9 | 3 | 1 |
| 686 | 9311689 | 264 | 1100 | 31 | 3 | 2 |
| 687 | 9311690 | 233 | 1267 | 29 | 0 | 3 |
| 688 | 9311691 | 210 | 1213 | 25 | 7 | 1 |
| 689 | 9311692 | 216 | 1111 | 19 | 10 | 1 |
| 690 | 9311693 | 241 | 1293 | 23 | 4 | 0 |
| 691 | 9311694 | 264 | 1312 | 34 | 3 | 4 |
| 692 | 9311695 | 239 | 1468 | 38 | 2 | 0 |
| 693 | 9311696 | 210 | 1037 | 28 | 2 | 3 |
| 694 | 9311697 | 224 | 1056 | 27 | 10 | 3 |
| 695 | 9311698 | 273 | 1255 | 22 | 2 | 0 |
| 696 | 9311699 | 225 | 1209 | 24 | 3 | 3 |
| 697 | 9311700 | 294 | 1359 | 26 | 3 | 3 |
| 698 | 9311701 | 183 | 679 | 16 | 12 | 3 |
| 699 | 9311702 | 220 | 1286 | 23 | 4 | 2 |
| 700 | 9311703 | 217 | 895 | 19 | 4 | 0 |
| 701 | 9311704 | 244 | 1089 | 30 | 6 | 3 |
| 702 | 9311705 | 251 | 1308 | 36 | 4 | 5 |
| 703 | 9311706 | 264 | 1280 | 41 | 5 | 2 |
| 704 | 9311707 | 200 | 802 | 24 | 9 | 2 |
| 705 | 9311708 | 232 | 992 | 21 | 5 | 1 |
| 706 | 9311709 | 191 | 755 | 17 | 3 | 0 |
| 707 | 9311710 | 252 | 1254 | 26 | 9 | 3 |
| 708 | 9311711 | 314 | 1890 | 27 | 8 | 1 |
| 709 | 9311712 | 240 | 900 | 29 | 4 | 2 |
| 710 | 9311713 | 210 | 828 | 16 | 1 | 4 |
| 711 | 9311714 | 208 | 1004 | 15 | 2 | 1 |
| 712 | 9311715 | 230 | 1128 | 24 | 4 | 1 |
| 713 | 9311716 | 185 | 917 | 16 | 5 | 1 |
| 714 | 9311717 | 219 | 928 | 17 | 7 | 4 |
| 715 | 9311718 | 217 | 992 | 38 | 6 | 4 |
| 716 | 9311719 | 234 | 1021 | 28 | 8 | 3 |
| 717 | 9311720 | 252 | 1218 | 33 | 9 | 5 |
| 718 | 9311721 | 232 | 1025 | 17 | 1 | 0 |
| 719 | 9311722 | 252 | 1302 | 36 | 8 | 5 |
| 720 | 9311723 | 269 | 1410 | 25 | 0 | 0 |
| 721 | 9311724 | 276 | 1513 | 30 | 5 | 2 |
| 722 | 9311725 | 193 | 808 | 14 | 5 | 2 |
| 723 | 9311726 | 199 | 936 | 7 | 6 | 2 |
| 724 | 9311727 | 271 | 1372 | 31 | 3 | 2 |
| 725 | 9311728 | 237 | 972 | 30 | 5 | 2 |
| 726 | 9311729 | 226 | 1272 | 23 | 3 | 0 |
| 727 | 9311730 | 228 | 1144 | 34 | 6 | 2 |
| 728 | 9311731 | 203 | 931 | 25 | 3 | 2 |
| 729 | 9311732 | 204 | 788 | 21 | 5 | 2 |
| 730 | 9311733 | 207 | 1058 | 30 | 3 | 2 |
| 731 | 9311734 | 199 | 715 | 24 | 4 | 2 |
| 732 | 9311735 | 237 | 1019 | 26 | 13 | 3 |
| 733 | 9311736 | 227 | 1084 | 19 | 5 | 3 |
| 734 | 9311737 | 227 | 998 | 17 | 8 | 4 |
| 735 | 9311738 | 191 | 837 | 21 | 5 | 2 |
| 736 | 9311739 | 210 | 936 | 26 | 3 | 3 |
| 737 | 9311740 | 268 | 1404 | 38 | 7 | 4 |
| 738 | 9311741 | 224 | 1093 | 23 | 6 | 4 |
| 739 | 9311742 | 168 | 537 | 15 | 4 | 2 |
| 740 | 9311743 | 332 | 2197 | 31 | 4 | 3 |
| 741 | 9311744 | 177 | 875 | 23 | 2 | 3 |
| 742 | 9311745 | 254 | 1081 | 21 | 1 | 1 |
| 743 | 9311746 | 255 | 1413 | 22 | 13 | 2 |
| 744 | 9311747 | 216 | 1163 | 26 | 8 | 4 |
| 745 | 9311748 | 240 | 1469 | 27 | 3 | 3 |
| 746 | 9311749 | 255 | 1292 | 36 | 3 | 3 |
| 747 | 9311750 | 222 | 1167 | 21 | 5 | 3 |
| 748 | 9311751 | 205 | 757 | 23 | 11 | 4 |
| 749 | 9311752 | 342 | 1828 | 33 | 1 | 2 |
| 750 | 9311753 | 203 | 1053 | 24 | 6 | 1 |
| 751 | 9311754 | 189 | 1066 | 26 | 3 | 1 |
| 752 | 9311755 | 179 | 643 | 20 | 1 | 2 |
| 753 | 9311756 | 234 | 1000 | 19 | 5 | 2 |
| 754 | 9311757 | 211 | 1059 | 13 | 2 | 3 |
| 755 | 9311758 | 300 | 1539 | 31 | 6 | 4 |
| 756 | 9311759 | 201 | 776 | 15 | 6 | 1 |
| 757 | 9311760 | 207 | 903 | 22 | 6 | 2 |
| 758 | 9311761 | 180 | 919 | 28 | 9 | 1 |
| 759 | 9311762 | 253 | 1400 | 26 | 7 | 2 |
| 760 | 9311763 | 190 | 788 | 15 | 1 | 0 |
| 761 | 9311764 | 230 | 1000 | 29 | 6 | 3 |
| 762 | 9311765 | 205 | 814 | 12 | 6 | 4 |
| 763 | 9311766 | 184 | 731 | 21 | 7 | 1 |
| 764 | 9311767 | 235 | 1224 | 32 | 6 | 2 |
| 765 | 9311768 | 143 | 434 | 10 | 8 | 0 |
| 766 | 9311769 | 166 | 477 | 8 | 9 | 0 |
| 767 | 9311770 | 217 | 1204 | 32 | 6 | 3 |
| 768 | 9311771 | 268 | 1334 | 13 | 6 | 4 |
| 769 | 9311772 | 222 | 1059 | 28 | 8 | 2 |
| 770 | 9311773 | 182 | 959 | 13 | 1 | 0 |
| 771 | 9311774 | 316 | 1448 | 38 | 4 | 3 |
| 772 | 9311775 | 253 | 1167 | 28 | 11 | 2 |
| 773 | 9311776 | 203 | 887 | 14 | 8 | 3 |
| 774 | 9311777 | 222 | 1202 | 27 | 6 | 0 |
| 775 | 9311778 | 202 | 840 | 21 | 10 | 2 |
| 776 | 9311779 | 191 | 900 | 20 | 2 | 2 |
| 777 | 9311780 | 221 | 1066 | 25 | 3 | 3 |
| 778 | 9311781 | 293 | 1469 | 35 | 15 | 4 |
| 779 | 9311782 | 154 | 550 | 11 | 3 | 2 |
| 780 | 9311783 | 276 | 1338 | 30 | 5 | 3 |
| 781 | 9311784 | 156 | 560 | 11 | 3 | 1 |
| 782 | 9311785 | 229 | 1154 | 17 | 5 | 2 |
| 783 | 9311786 | 226 | 1208 | 21 | 7 | 1 |
| 784 | 9311787 | 198 | 849 | 20 | 6 | 3 |
| 785 | 9311788 | 238 | 1180 | 34 | 0 | 2 |
| 786 | 9311789 | 168 | 699 | 10 | 5 | 1 |
| 787 | 9311790 | 178 | 734 | 15 | 8 | 2 |
| 788 | 9311791 | 255 | 1061 | 19 | 5 | 2 |
| 789 | 9311792 | 222 | 1043 | 9 | 3 | 3 |
| 790 | 9311793 | 212 | 949 | 23 | 5 | 2 |
| 791 | 9311794 | 211 | 1016 | 24 | 5 | 2 |
| 792 | 9311795 | 229 | 1107 | 11 | 9 | 0 |
| 793 | 9311796 | 270 | 1535 | 30 | 5 | 2 |
| 794 | 9311797 | 169 | 434 | 16 | 4 | 2 |
| 795 | 9311798 | 242 | 1288 | 19 | 6 | 4 |
| 796 | 9311799 | 196 | 1017 | 17 | 3 | 1 |
| 797 | 9311800 | 274 | 1365 | 34 | 3 | 5 |
| 798 | 9311801 | 188 | 972 | 32 | 6 | 2 |
| 799 | 9311802 | 249 | 1368 | 23 | 5 | 1 |
| 800 | 9311803 | 178 | 738 | 16 | 7 | 1 |
| 801 | 9311804 | 227 | 1267 | 24 | 6 | 2 |
| 802 | 9311805 | 201 | 868 | 14 | 7 | 2 |
| 803 | 9311806 | 240 | 1316 | 27 | 6 | 3 |
| 804 | 9311807 | 240 | 1299 | 20 | 2 | 2 |
| 805 | 9311808 | 261 | 1135 | 16 | 5 | 3 |
| 806 | 9311809 | 212 | 763 | 26 | 8 | 1 |
| 807 | 9311810 | 169 | 816 | 21 | 8 | 1 |
| 808 | 9311811 | 197 | 928 | 13 | 4 | 1 |
| 809 | 9311812 | 197 | 716 | 16 | 5 | 4 |
| 810 | 9311813 | 216 | 960 | 25 | 0 | 2 |
| 811 | 9311814 | 248 | 1138 | 24 | 7 | 1 |
| 812 | 9311815 | 247 | 1411 | 29 | 2 | 2 |
| 813 | 9311816 | 240 | 1116 | 26 | 5 | 3 |
| 814 | 9311817 | 250 | 1301 | 33 | 1 | 2 |
| 815 | 9311818 | 248 | 1089 | 23 | 3 | 1 |
| 816 | 9311819 | 207 | 1028 | 31 | 2 | 2 |
| 817 | 9311820 | 182 | 603 | 10 | 2 | 1 |
| 818 | 9311821 | 224 | 1094 | 15 | 4 | 3 |
| 819 | 9311822 | 256 | 1158 | 27 | 5 | 1 |
| 820 | 9311823 | 201 | 1226 | 32 | 4 | 1 |
| 821 | 9311824 | 254 | 1262 | 45 | 12 | 4 |
| 822 | 9311825 | 232 | 1148 | 28 | 4 | 4 |
| 823 | 9311826 | 249 | 1182 | 25 | 2 | 2 |
| 824 | 9311827 | 220 | 1116 | 26 | 13 | 0 |
| 825 | 9311828 | 218 | 925 | 28 | 6 | 2 |
| 826 | 9311829 | 231 | 970 | 14 | 7 | 3 |
| 827 | 9311830 | 272 | 1670 | 33 | 7 | 4 |
| 828 | 9311831 | 235 | 1233 | 25 | 11 | 1 |
| 829 | 9311832 | 195 | 988 | 26 | 1 | 0 |
| 830 | 9311833 | 171 | 783 | 19 | 3 | 3 |
| 831 | 9311834 | 216 | 1001 | 20 | 4 | 1 |
| 832 | 9311835 | 212 | 816 | 19 | 3 | 2 |
| 833 | 9311836 | 217 | 1136 | 23 | 3 | 4 |
| 834 | 9311837 | 220 | 762 | 21 | 3 | 2 |
| 835 | 9311838 | 164 | 729 | 14 | 6 | 0 |
| 836 | 9311839 | 236 | 1301 | 34 | 8 | 2 |
| 837 | 9311840 | 271 | 1446 | 45 | 8 | 2 |
| 838 | 9311841 | 257 | 1183 | 25 | 9 | 2 |
| 839 | 9311842 | 252 | 1244 | 36 | 4 | 2 |
| 840 | 9311843 | 197 | 821 | 29 | 8 | 4 |
| 841 | 9311844 | 204 | 913 | 16 | 0 | 2 |
| 842 | 9311845 | 221 | 1207 | 21 | 7 | 4 |
| 843 | 9311846 | 334 | 1381 | 18 | 3 | 3 |
| 844 | 9311847 | 201 | 908 | 19 | 5 | 2 |
| 845 | 9311848 | 243 | 1377 | 25 | 1 | 1 |
| 846 | 9311849 | 195 | 877 | 24 | 3 | 2 |
| 847 | 9311850 | 188 | 905 | 13 | 2 | 2 |
| 848 | 9311851 | 180 | 844 | 13 | 2 | 3 |
| 849 | 9311852 | 280 | 1691 | 31 | 4 | 2 |
| 850 | 9311853 | 271 | 1245 | 31 | 1 | 3 |
| 851 | 9311854 | 264 | 1193 | 36 | 6 | 2 |
| 852 | 9311855 | 229 | 1053 | 21 | 6 | 2 |
| 853 | 9311856 | 167 | 670 | 12 | 3 | 0 |
| 854 | 9311857 | 208 | 931 | 13 | 3 | 4 |
| 855 | 9311858 | 216 | 957 | 15 | 10 | 2 |
| 856 | 9311859 | 216 | 880 | 18 | 3 | 2 |
| 857 | 9311860 | 209 | 1039 | 34 | 8 | 2 |
| 858 | 9311861 | 208 | 953 | 21 | 5 | 2 |
| 859 | 9311862 | 207 | 912 | 17 | 4 | 1 |
| 860 | 9311863 | 180 | 731 | 17 | 9 | 1 |
| 861 | 9311864 | 246 | 1336 | 15 | 4 | 1 |
| 862 | 9311865 | 211 | 978 | 23 | 13 | 1 |
| 863 | 9311866 | 199 | 935 | 24 | 6 | 4 |
| 864 | 9311867 | 240 | 980 | 18 | 2 | 2 |
| 865 | 9311868 | 243 | 1230 | 20 | 6 | 2 |
| 866 | 9311869 | 245 | 1100 | 29 | 1 | 2 |
| 867 | 9311870 | 202 | 786 | 22 | 10 | 1 |
| 868 | 9311871 | 205 | 1128 | 15 | 4 | 1 |
| 869 | 9311872 | 269 | 1421 | 23 | 18 | 2 |
| 870 | 9311873 | 179 | 781 | 15 | 6 | 2 |
| 871 | 9311874 | 305 | 1615 | 37 | 3 | 3 |
| 872 | 9311875 | 206 | 955 | 25 | 1 | 1 |
| 873 | 9311876 | 246 | 1151 | 15 | 7 | 0 |
| 874 | 9311877 | 218 | 878 | 16 | 3 | 1 |
| 875 | 9311878 | 187 | 766 | 23 | 5 | 1 |
| 876 | 9311879 | 251 | 1449 | 34 | 2 | 1 |
| 877 | 9311880 | 239 | 1013 | 21 | 8 | 2 |
| 878 | 9311881 | 277 | 1469 | 30 | 7 | 3 |
| 879 | 9311882 | 217 | 886 | 22 | 3 | 1 |
| 880 | 9311883 | 215 | 846 | 16 | 7 | 1 |
| 881 | 9311884 | 212 | 943 | 23 | 2 | 4 |
| 882 | 9311885 | 233 | 966 | 35 | 6 | 3 |
| 883 | 9311886 | 297 | 1489 | 50 | 13 | 3 |
| 884 | 9311887 | 227 | 1178 | 22 | 5 | 5 |
| 885 | 9311888 | 198 | 755 | 19 | 4 | 1 |
| 886 | 9311889 | 187 | 938 | 18 | 4 | 2 |
| 887 | 9311890 | 222 | 851 | 28 | 1 | 4 |
| 888 | 9311891 | 175 | 620 | 11 | 9 | 3 |
| 889 | 9311892 | 247 | 1161 | 18 | 1 | 3 |
| 890 | 9311893 | 171 | 929 | 18 | 3 | 0 |
| 891 | 9311894 | 237 | 926 | 21 | 10 | 2 |
| 892 | 9311895 | 310 | 1610 | 32 | 10 | 3 |
| 893 | 9311896 | 285 | 1526 | 21 | 8 | 1 |
| 894 | 9311897 | 289 | 1596 | 38 | 7 | 3 |
| 895 | 9311898 | 164 | 729 | 14 | 2 | 2 |
| 896 | 9311899 | 243 | 1142 | 26 | 14 | 3 |
| 897 | 9311900 | 219 | 1073 | 32 | 7 | 2 |
| 898 | 9311901 | 204 | 865 | 16 | 1 | 0 |
| 899 | 9311902 | 243 | 1328 | 25 | 5 | 2 |
| 900 | 9311903 | 199 | 990 | 28 | 2 | 3 |
| 901 | 9311904 | 225 | 1084 | 22 | 5 | 0 |
| 902 | 9311905 | 200 | 860 | 22 | 6 | 3 |
| 903 | 9311906 | 306 | 2097 | 30 | 3 | 2 |
| 904 | 9311907 | 194 | 946 | 26 | 10 | 0 |
| 905 | 9311908 | 230 | 1265 | 21 | 7 | 3 |
| 906 | 9311909 | 203 | 1085 | 32 | 2 | 2 |
| 907 | 9311910 | 248 | 1134 | 30 | 3 | 3 |
| 908 | 9311911 | 251 | 1350 | 18 | 6 | 2 |
| 909 | 9311912 | 166 | 626 | 15 | 4 | 0 |
| 910 | 9311913 | 209 | 1068 | 23 | 5 | 1 |
| 911 | 9311914 | 217 | 962 | 28 | 8 | 3 |
| 912 | 9311915 | 188 | 830 | 17 | 3 | 2 |
| 913 | 9311916 | 272 | 1396 | 27 | 0 | 2 |
| 914 | 9311917 | 192 | 779 | 17 | 2 | 3 |
| 915 | 9311918 | 208 | 845 | 20 | 4 | 2 |
| 916 | 9311919 | 298 | 1505 | 38 | 5 | 2 |
| 917 | 9311920 | 228 | 1016 | 32 | 3 | 2 |
| 918 | 9311921 | 207 | 1062 | 22 | 5 | 1 |
| 919 | 9311922 | 261 | 1353 | 31 | 9 | 3 |
| 920 | 9311923 | 409 | 1923 | 53 | 8 | 4 |
| 921 | 9311924 | 311 | 1345 | 27 | 5 | 2 |
| 922 | 9311925 | 176 | 655 | 17 | 9 | 1 |
| 923 | 9311926 | 329 | 1707 | 24 | 2 | 2 |
| 924 | 9311927 | 319 | 1694 | 47 | 4 | 4 |
| 925 | 9311928 | 243 | 1186 | 29 | 6 | 1 |
| 926 | 9311929 | 224 | 1100 | 27 | 5 | 3 |
| 927 | 9311930 | 218 | 1189 | 23 | 6 | 2 |
| 928 | 9311931 | 235 | 1016 | 22 | 8 | 2 |
| 929 | 9311932 | 215 | 832 | 18 | 1 | 0 |
| 930 | 9311933 | 236 | 1189 | 26 | 3 | 2 |
| 931 | 9311934 | 193 | 811 | 17 | 5 | 1 |
| 932 | 9311935 | 252 | 1270 | 26 | 8 | 1 |
| 933 | 9311936 | 227 | 1183 | 20 | 1 | 3 |
| 934 | 9311937 | 194 | 906 | 17 | 4 | 2 |
| 935 | 9311938 | 219 | 1046 | 15 | 3 | 1 |
| 936 | 9311939 | 221 | 970 | 24 | 3 | 0 |
| 937 | 9311940 | 265 | 1230 | 28 | 3 | 3 |
| 938 | 9311941 | 251 | 1488 | 35 | 5 | 2 |
| 939 | 9311942 | 213 | 1036 | 8 | 6 | 3 |
| 940 | 9311943 | 172 | 588 | 15 | 9 | 2 |
| 941 | 9311944 | 180 | 832 | 11 | 4 | 0 |
| 942 | 9311945 | 180 | 800 | 17 | 6 | 2 |
| 943 | 9311946 | 241 | 932 | 33 | 4 | 3 |
| 944 | 9311947 | 236 | 1369 | 25 | 9 | 2 |
| 945 | 9311948 | 227 | 778 | 16 | 8 | 2 |
| 946 | 9311949 | 281 | 1757 | 16 | 3 | 3 |
| 947 | 9311950 | 314 | 1249 | 24 | 2 | 2 |
| 948 | 9311951 | 216 | 818 | 10 | 3 | 2 |
| 949 | 9311952 | 193 | 963 | 19 | 7 | 1 |
| 950 | 9311953 | 225 | 1146 | 18 | 7 | 2 |
| 951 | 9311954 | 246 | 1282 | 29 | 8 | 3 |
| 952 | 9311955 | 184 | 679 | 15 | 4 | 2 |
| 953 | 9311956 | 214 | 738 | 17 | 5 | 3 |
| 954 | 9311957 | 198 | 929 | 21 | 0 | 3 |
| 955 | 9311958 | 287 | 1776 | 43 | 9 | 1 |
| 956 | 9311959 | 226 | 1069 | 24 | 7 | 4 |
| 957 | 9311960 | 250 | 1030 | 37 | 9 | 3 |
| 958 | 9311961 | 224 | 866 | 17 | 5 | 3 |
| 959 | 9311962 | 264 | 1103 | 43 | 4 | 3 |
| 960 | 9311963 | 218 | 1053 | 26 | 3 | 4 |
| 961 | 9311964 | 218 | 848 | 20 | 6 | 2 |
| 962 | 9311965 | 267 | 1414 | 13 | 3 | 2 |
| 963 | 9311966 | 281 | 1503 | 26 | 2 | 0 |
| 964 | 9311967 | 168 | 747 | 13 | 4 | 1 |
| 965 | 9311968 | 232 | 1262 | 25 | 10 | 3 |
| 966 | 9311969 | 177 | 791 | 18 | 4 | 0 |
| 967 | 9311970 | 230 | 1150 | 34 | 1 | 1 |
| 968 | 9311971 | 256 | 1253 | 28 | 1 | 2 |
| 969 | 9311972 | 255 | 1469 | 24 | 4 | 3 |
| 970 | 9311973 | 171 | 709 | 21 | 2 | 3 |
| 971 | 9311974 | 203 | 1018 | 18 | 2 | 1 |
| 972 | 9311975 | 204 | 625 | 16 | 1 | 0 |
| 973 | 9311976 | 286 | 1407 | 34 | 4 | 5 |
| 974 | 9311977 | 197 | 698 | 12 | 9 | 0 |
| 975 | 9311978 | 236 | 1338 | 37 | 4 | 2 |
| 976 | 9311979 | 218 | 1312 | 23 | 11 | 3 |
| 977 | 9311980 | 207 | 784 | 15 | 5 | 0 |
| 978 | 9311981 | 214 | 801 | 21 | 6 | 4 |
| 979 | 9311982 | 219 | 908 | 23 | 5 | 3 |
| 980 | 9311983 | 221 | 1202 | 22 | 8 | 3 |
| 981 | 9311984 | 282 | 1425 | 20 | 4 | 3 |
| 982 | 9311985 | 227 | 978 | 25 | 2 | 3 |
| 983 | 9311986 | 195 | 800 | 21 | 8 | 2 |
| 984 | 9311987 | 220 | 916 | 25 | 4 | 3 |
| 985 | 9311988 | 185 | 739 | 21 | 2 | 1 |
| 986 | 9311989 | 227 | 949 | 19 | 2 | 1 |
| 987 | 9311990 | 191 | 792 | 6 | 4 | 2 |
| 988 | 9311991 | 221 | 955 | 18 | 9 | 1 |
| 989 | 9311992 | 178 | 866 | 18 | 2 | 2 |
| 990 | 9311993 | 213 | 1036 | 23 | 5 | 1 |
| 991 | 9311994 | 381 | 1846 | 51 | 3 | 2 |
| 992 | 9311995 | 210 | 819 | 20 | 5 | 1 |
| 993 | 9311996 | 221 | 948 | 26 | 5 | 3 |
| 994 | 9311997 | 235 | 907 | 24 | 6 | 3 |
| 995 | 9311998 | 205 | 864 | 20 | 5 | 2 |
| 996 | 9311999 | 264 | 1471 | 29 | 4 | 1 |
| 997 | 9312000 | 227 | 1061 | 30 | 8 | 4 |
| 998 | 9312001 | 182 | 723 | 13 | 5 | 0 |
| 999 | 9312002 | 202 | 1095 | 23 | 0 | 2 |

## Throughput

- Turns simulated: 225597 (first play of each game)
- Aggregate: 170.3 turns/s over 1324.8s wall
- Per game thread: 34.1 turns/s

## Determinism

Not checked (run with `--determinism`).

## Panics

None.
