# Simulation: terms-on (2026-11-25)

1000 games, first seed 9311003, brain `scripted`, turn limit off, safety cap 1200, 6 threads, started 2026-11-25T06:56:00Z. Wall time 1243.4s.

Source checkout at run start: `aba0f5b708f7e02b9844d9ac2cdeb3130c361c26`; dirty: no. Map 48×32, 8 initial seats. `first_setup` and exact binary argv are recorded in JSON. Ordinary games use seed + i (wrapping); the strategy tournament schedule is specified below. All games use a deterministic shuffle of the full faction pool, taking eight distinct factions, with any Proxy rotation described below.

Command argv: `["./sim","--players","8","--difficulty","hard","--games","1000","--seed","9311003","--label","terms-on","--out-dir","out","--threads","6"]`

## Overview

| Metric | Value |
| --- | --- |
| Games (non-panic) | 1000 of 1000 |
| Game length, turns (mean / median / min-max) | 220.0 / 216 / 135-442 |
| Win reason mix | VP hold 99.4%, Capitals 0.6% |
| Median end turn | 216 |
| Ending shares of played games (capitals / VP hold / turn limit / other) | 0.6% / 99.4% / 0.0% / 0.0% |
| Games reaching the World War era (any empire) | 992 of 1000 (99.2%) |
| VP hold length | 10 turn ends (the reaching turn counts) |
| Holds started / broken / completed / unfinished | 1510 / 415 / 997 / 98 |
| Hold break rate | 27.5% of started; 29.4% of resolved (broken / (broken + completed)) |
| Hold wins | 994 |
| First reach to win, turns (mean / median / min-max) | 11.1 / 9 / 9-106 |
| Final hold start to win, turns | 9.0 / 9 / 9-9 |
| Comebacks (winner broke an earlier hold of its own) | 122 |
| Wins by a team another team beat to its target | 78 |

Game length by how the game ended (turns resolved; a stall is a safety-cap stop, never a win):

| Ended by | Games | Length mean / median / min-max |
| --- | --- | --- |
| VP hold | 994 | 219.7 / 216 / 135-442 |
| Capitals | 6 | 279.5 / 295 / 226-334 |

Winner share by faction (wins of that faction's seats / all winning seats):

| Faction | Wins | Share of wins |
| --- | --- | --- |
| Switzerland | 84 | 8.4% |
| India | 85 | 8.5% |
| Byzantium | 80 | 8.0% |
| Germany | 136 | 13.6% |
| Celts | 141 | 14.1% |
| Mongols | 78 | 7.8% |
| Sparta | 105 | 10.5% |
| Athens | 156 | 15.6% |
| Aztecs | 135 | 13.5% |

## Outcomes

| Outcome | Games | Share |
| --- | --- | --- |
| VP hold | 994 | 99.4% |
| Conquest | 0 | 0.0% |
| Capitals | 6 | 0.6% |
| Turn limit | 0 | 0.0% |
| Draw | 0 | 0.0% |
| Stalled (safety cap) | 0 | 0.0% |
| Panic | 0 | 0.0% |

## Win rate by faction

Per seat: wins / seats of that faction. With 8 single-empire teams, an even game is 12.5% per seat.

| Faction | Seats | Wins | Win rate |
| --- | --- | --- | --- |
| Switzerland | 879 | 84 | 9.6% |
| India | 905 | 85 | 9.4% |
| Byzantium | 885 | 80 | 9.0% |
| Germany | 900 | 136 | 15.1% |
| Celts | 880 | 141 | 16.0% |
| Mongols | 878 | 78 | 8.9% |
| Sparta | 901 | 105 | 11.7% |
| Athens | 900 | 156 | 17.3% |
| Aztecs | 872 | 135 | 15.5% |

## Win rate by difficulty

All initial seats in non-panicked games, including eliminated empires and cap stops. Determinism replays are not additional samples.

| Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- |
| Hard | 8000 | 1000 | 12.5% |

## Win rate by faction and difficulty

| Faction | Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- | --- |
| Switzerland | Hard | 879 | 84 | 9.6% |
| India | Hard | 905 | 85 | 9.4% |
| Byzantium | Hard | 885 | 80 | 9.0% |
| Germany | Hard | 900 | 136 | 15.1% |
| Celts | Hard | 880 | 141 | 16.0% |
| Mongols | Hard | 878 | 78 | 8.9% |
| Sparta | Hard | 901 | 105 | 11.7% |
| Athens | Hard | 900 | 156 | 17.3% |
| Aztecs | Hard | 872 | 135 | 15.5% |

## Faction goal progress

Per seat: how often the faction's permanent goal was reached before the game ended, and the turn its VP was first awarded (over the seats that reached it). Seats are all initial empires of non-panicked games with telemetry, including eliminated empires and cap stops.

| Faction | Seats | Reached | Goal rate | Min | P10 | Median turn | P90 | Max |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Switzerland | 879 | 200 | 22.8% | 119 | 139 | 163 | 224 | 288 |
| India | 905 | 513 | 56.7% | 45 | 89 | 123 | 189 | 303 |
| Byzantium | 885 | 709 | 80.1% | 69 | 83 | 106 | 146 | 262 |
| Germany | 900 | 352 | 39.1% | 85 | 122 | 149 | 193 | 290 |
| Celts | 880 | 536 | 60.9% | 35 | 168 | 210 | 210 | 320 |
| Mongols | 878 | 141 | 16.1% | 111 | 144 | 188 | 223 | 300 |
| Sparta | 901 | 114 | 12.7% | 117 | 153 | 201 | 250 | 390 |
| Athens | 900 | 281 | 31.2% | 119 | 155 | 211 | 255 | 255 |
| Aztecs | 872 | 305 | 35.0% | 127 | 163 | 225 | 234 | 285 |

## Distributions

| Metric | Min | P10 | Median | P90 | Max |
| --- | --- | --- | --- | --- | --- |
| Observed game length (turns, including cap stops) | 135 | 179 | 216 | 265 | 442 |
| Winning team VP | 4 | 10 | 11 | 13 | 16 |
| First Soldier completed (turn, per empire) | 6 | 8 | 36 | 72 | 278 |
| First Settler completed (turn, per empire) | 6 | 12 | 14 | 25 | 239 |
| First city founded (turn, per empire) | 1 | 1 | 1 | 1 | 1 |
| Second city founded (turn, per empire) | 10 | 16 | 19 | 39 | 241 |

Safety-cap stops are censored observations, not completed game lengths.

Openings: 8000 observed empires in 1000 first plays; 0 games without opening telemetry excluded. Turn distributions include only empires that reached the milestone. Never means not reached by the end of observed play, including eliminated empires and safety-cap stops. Per-empire turns are in JSON (`games[].telemetry.openings`, keyed by player ID).

| Opening milestone | Reached empires | Never empires |
| --- | --- | --- |
| First Soldier completed | 7909 | 91 |
| First Settler completed | 7987 | 13 |
| First city founded | 8000 | 0 |
| Second city founded | 7916 | 84 |

Final VP per empire (all seats):

| VP | Empires |
| --- | --- |
| 0 | 1526 |
| 1 | 1312 |
| 2 | 1092 |
| 3 | 780 |
| 4 | 609 |
| 5 | 575 |
| 6 | 399 |
| 7 | 261 |
| 8 | 203 |
| 9 | 146 |
| 10 | 454 |
| 11 | 334 |
| 12 | 169 |
| 13 | 80 |
| 14 | 38 |
| 15 | 12 |
| 16 | 10 |

## End state

- Average game length: 220.0 turns
- Average cities per empire: 4.68
- Average techs per empire: 32.54
- Average eliminations per game: 1.66

## Strategy diversity: observed behavior (classifier v2)

1000 finished games; 8000 player seats; 1000 winning seats. Excluded 0 unfinished/panicked games and 0 games with missing labels. Winner entropy: **0.3184** (normalized over 6 categories, including zero-win categories).

Thresholds, biases and denominators: [strategy-diversity.md](../strategy-diversity.md).

| Archetype | Players | Wins | Win rate | All-player frequency | Winner frequency |
| --- | ---: | ---: | ---: | ---: | ---: |
| Rush | 1762 | 68 | 3.9% | 22.0% | 6.8% |
| Wide | 3759 | 837 | 22.3% | 47.0% | 83.7% |
| Science | 0 | 0 | n/a | 0.0% | 0.0% |
| Turtle/Wonder | 19 | 1 | 5.3% | 0.2% | 0.1% |
| Economic | 1232 | 92 | 7.5% | 15.4% | 9.2% |
| Generalist | 1228 | 2 | 0.2% | 15.3% | 0.2% |

### Matchup matrix

Cell = strict ahead % / tied % (co-occurrence games). Each game has equal weight; all cross-team seat pairs within it share that weight. Rank: winning team first, then surviving empires by VP, then eliminated empires by later elimination turn. Equal ranks tie; no ID tiebreak. This is a multiplayer association, not a duel win rate.

| Row vs column | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Rush | — | 17.1% / 4.4% (875) | n/a / n/a (0) | 25.6% / 13.3% (15) | 17.0% / 8.1% (631) | 55.8% / 7.1% (649) |
| Wide | 78.5% / 4.4% (875) | — | n/a / n/a (0) | 72.5% / 13.1% (18) | 57.0% / 6.5% (741) | 87.1% / 3.1% (760) |
| Science | n/a / n/a (0) | n/a / n/a (0) | — | n/a / n/a (0) | n/a / n/a (0) | n/a / n/a (0) |
| Turtle/Wonder | 61.1% / 13.3% (15) | 14.4% / 13.1% (18) | n/a / n/a (0) | — | 18.8% / 11.5% (16) | 60.7% / 14.3% (14) |
| Economic | 74.9% / 8.1% (631) | 36.4% / 6.5% (741) | n/a / n/a (0) | 69.8% / 11.5% (16) | — | 84.2% / 6.8% (544) |
| Generalist | 37.1% / 7.1% (649) | 9.9% / 3.1% (760) | n/a / n/a (0) | 25.0% / 14.3% (14) | 9.0% / 6.8% (544) | — |

### Faction wins by archetype

Cell = wins / seats; win rate in JSON.

| Faction | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Switzerland | 1 / 85 | 74 / 440 | 0 / 0 | 0 / 1 | 9 / 220 | 0 / 133 |
| India | 3 / 197 | 81 / 591 | 0 / 0 | 0 / 0 | 1 / 17 | 0 / 100 |
| Byzantium | 0 / 9 | 75 / 487 | 0 / 0 | 0 / 3 | 5 / 156 | 0 / 230 |
| Germany | 3 / 135 | 128 / 468 | 0 / 0 | 0 / 3 | 5 / 101 | 0 / 193 |
| Celts | 0 / 107 | 133 / 522 | 0 / 0 | 0 / 0 | 8 / 149 | 0 / 102 |
| Mongols | 31 / 566 | 43 / 192 | 0 / 0 | 0 / 0 | 4 / 43 | 0 / 77 |
| Sparta | 20 / 441 | 52 / 176 | 0 / 0 | 1 / 10 | 30 / 180 | 2 / 94 |
| Athens | 5 / 108 | 136 / 429 | 0 / 0 | 0 / 1 | 15 / 191 | 0 / 171 |
| Aztecs | 5 / 114 | 115 / 454 | 0 / 0 | 0 / 1 | 15 / 175 | 0 / 128 |

Guardrail: **Fail**. Entropy minimum 0.80; at least 20 games per strategy pair. Dominant by observed score: none. Without an observed counter (all pairs covered): none. Sampled but never won: none. Unobserved: Science. Under-sampled pairs: 9.

- insufficient evidence: 9 under-sampled matchups; 1000 winning seats

- winner entropy below 0.80

## Battle and capture telemetry

1000 observed first plays; 0 records without telemetry excluded.

| Event | Total / observed games | Mean per game | Games with event / observed games |
| --- | --- | --- | --- |
| War declarations | 14468/1000 | 14.468 | 1000/1000 (100.0%) |
| Peace offers | 94385/1000 | 94.385 | 1000/1000 (100.0%) |
| Peace agreements | 12184/1000 | 12.184 | 999/1000 (99.9%) |
| Battles | 988718/1000 | 988.718 | 1000/1000 (100.0%) |
| City captures | 20741/1000 | 20.741 | 1000/1000 (100.0%) |
| Settler captures | 4974/1000 | 4.974 | 982/1000 (98.2%) |
| Eliminations | 1657/1000 | 1.657 | 869/1000 (86.9%) |

## Terminal VP sources

1000 finished games. Columns are separate populations; totals across columns must not be added. Every cell is VP / population observations.

| Source | All players | All teams | Winning teams |
| --- | --- | --- | --- |
| Held Great Works | 7652/8000 | 7652/8000 | 3056/1000 |
| Faction goals | 5064/8000 | 5064/8000 | 2064/1000 |
| Era objectives | 15180/8000 | 15180/8000 | 4454/1000 |
| Capitals taken | 2564/8000 | 2564/8000 | 1540/1000 |
| Total VP | 30460/8000 | 30460/8000 | 11114/1000 |

Reconciliation failures: 0 player observations, 0 team observations.

## Turn-50 team leaders

| Cohort | Eventual leader-set wins / eligible games | Draws (included) |
| --- | --- | --- |
| Unique leader | 61/392 (15.6%) | 0 |
| Tied leaders (winner in set) | 603/608 (99.2%) | 0 |
| Any leader set | 664/1000 (66.4%) | 0 |

Excluded: 0 games ended before turn 50, 0 unfinished games, 0 snapshots without teams, 0 records without telemetry.

## Era pacing

Resolved turn each empire entered each era. `Winner vs median` is the winning team's seats' entry turn minus the median entry of the empires that entered it in the same finished game (negative: winners got there first).

| Era | Entered / seats | Fastest | p10 | Median | p90 | Slowest | Winner vs median (mean, n) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Classical | 7903/8000 | 5 | 20 | 28 | 32 | 60 | -2.3 (1000) |
| Medieval | 7817/8000 | 35 | 56 | 60 | 63 | 124 | -1.6 (1000) |
| Renaissance | 7767/8000 | 62 | 81 | 86 | 93 | 180 | -2.7 (1000) |
| Industrial | 7571/8000 | 85 | 107 | 114 | 126 | 256 | -4.3 (1000) |
| Machine Age | 7018/8000 | 112 | 136 | 145 | 160 | 289 | -6.5 (1000) |
| World War | 5701/8000 | 132 | 160 | 173 | 192 | 282 | -9.2 (987) |
| Modern | 3451/8000 | 153 | 183 | 202 | 225 | 307 | -8.8 (839) |
| Future | 1447/8000 | 174 | 208 | 234 | 256 | 332 | -6.5 (473) |

Research idle turns (techs left, no research set): 4, of which stalled (two or more in a row): 0.

## Tech pace against the calendar

Median resolved turn each technology was learned, the display year of that turn (`calendar::calendar_label`) and the year the technology really arrived (n/a when no single year is established). Empires that never learned it are left out.

| Era | Technology | Learned / seats | Median turn | Median year | Real year |
| --- | --- | --- | --- | --- | --- |
| Ancient | Irrigation | 8000/8000 | 9 | 3173 BC | 6000 BC |
| Ancient | Bronze Working | 7984/8000 | 17 | 2345 BC | 3300 BC |
| Ancient | Masonry | 7998/8000 | 12 | 2863 BC | 3000 BC |
| Ancient | Currency | 7929/8000 | 27 | 1311 BC | 600 BC |
| Ancient | Writing | 7940/8000 | 25 | 1518 BC | 3200 BC |
| Classical | Mathematics | 7839/8000 | 43 | 351 BC | 300 BC |
| Classical | Engineering | 7830/8000 | 49 | 51 BC | n/a |
| Classical | Iron Working | 7856/8000 | 37 | 651 BC | 1200 BC |
| Classical | Code of Laws | 7824/8000 | 57 | 350 AD | 1754 BC |
| Classical | Philosophy | 7818/8000 | 58 | 400 AD | 500 BC |
| Medieval | Feudalism | 7799/8000 | 73 | 890 AD | n/a |
| Medieval | Machinery | 7810/8000 | 67 | 710 AD | n/a |
| Medieval | Chivalry | 6921/8000 | 77 | 1010 AD | n/a |
| Medieval | Education | 7776/8000 | 82 | 1160 AD | 1088 AD |
| Medieval | Crop Rotation | 7767/8000 | 85 | 1250 AD | n/a |
| Renaissance | Gunpowder | 7747/8000 | 92 | 1420 AD | n/a |
| Renaissance | Metallurgy | 7692/8000 | 102 | 1520 AD | n/a |
| Renaissance | Military Engineering | 7708/8000 | 100 | 1500 AD | n/a |
| Renaissance | Banking | 7626/8000 | 111 | 1610 AD | 1397 AD |
| Renaissance | Printing Press | 7580/8000 | 112 | 1620 AD | 1440 AD |
| Industrial | Rifling | 7239/8000 | 135 | 1787 AD | n/a |
| Industrial | Military Science | 7389/8000 | 128 | 1761 AD | n/a |
| Industrial | Ballistics | 7280/8000 | 137 | 1794 AD | n/a |
| Industrial | Industrialization | 7483/8000 | 122 | 1720 AD | 1769 AD |
| Industrial | Railways | 7024/8000 | 145 | 1824 AD | 1825 AD |
| Machine Age | Replaceable Parts | 6480/8000 | 159 | 1876 AD | 1800 AD |
| Machine Age | Combustion | 6779/8000 | 153 | 1854 AD | 1876 AD |
| Machine Age | Steel | 6480/8000 | 158 | 1872 AD | 1856 AD |
| Machine Age | Flight | 6023/8000 | 167 | 1887 AD | 1903 AD |
| Machine Age | Electricity | 5761/8000 | 173 | 1894 AD | 1879 AD |
| World War | Mass Mobilisation | 4645/8000 | 188 | 1911 AD | n/a |
| World War | Rocketry | 4273/8000 | 192 | 1916 AD | 1926 AD |
| World War | Armoured Warfare | 3957/8000 | 196 | 1922 AD | 1916 AD |
| World War | Air Power | 3722/8000 | 200 | 1928 AD | n/a |
| World War | Electronics | 5257/8000 | 180 | 1902 AD | n/a |
| Modern | Mechanised Warfare | 2263/8000 | 217 | 1954 AD | n/a |
| Modern | Composite Armour | 1982/8000 | 222 | 1966 AD | n/a |
| Modern | Guided Missiles | 1834/8000 | 227 | 1978 AD | n/a |
| Modern | Jet Engines | 1715/8000 | 228 | 1981 AD | 1939 AD |
| Modern | Green Revolution | 2844/8000 | 210 | 1942 AD | n/a |
| Future | Autonomous Systems | 790/8000 | 250 | 2026 AD | n/a |
| Future | Robotics | 605/8000 | 256 | 2032 AD | n/a |
| Future | Fusion Power | 616/8000 | 256 | 2032 AD | n/a |
| Future | Stealth Technology | 1007/8000 | 242 | 2013 AD | n/a |
| Future | Recursive AI | 574/8000 | 256 | 2032 AD | n/a |

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
| 0 | 0.00 | 74.47 |
| 1 | 57.00 | 15.94 |
| 2 | 18.02 | 4.86 |
| 3 | 4.24 | 1.24 |
| 4 | 0.78 | 0.26 |
| 5 | 0.15 | 0.07 |

## Per-game events

Each row is one observed first play; unavailable telemetry is shown as n/a. Full per-player/team score sources and turn-50 standings are in JSON.

| Game | Seed | Resolved turns | Battles | City captures | Settler captures | Eliminations |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 9311003 | 185 | 663 | 15 | 5 | 1 |
| 1 | 9311004 | 195 | 647 | 14 | 6 | 0 |
| 2 | 9311005 | 248 | 1090 | 28 | 5 | 2 |
| 3 | 9311006 | 231 | 1212 | 32 | 9 | 0 |
| 4 | 9311007 | 175 | 576 | 9 | 6 | 0 |
| 5 | 9311008 | 216 | 1183 | 19 | 2 | 0 |
| 6 | 9311009 | 188 | 776 | 17 | 10 | 1 |
| 7 | 9311010 | 189 | 811 | 22 | 6 | 1 |
| 8 | 9311011 | 194 | 644 | 13 | 5 | 1 |
| 9 | 9311012 | 181 | 641 | 11 | 4 | 1 |
| 10 | 9311013 | 161 | 660 | 12 | 8 | 1 |
| 11 | 9311014 | 208 | 1123 | 26 | 4 | 3 |
| 12 | 9311015 | 201 | 802 | 10 | 8 | 2 |
| 13 | 9311016 | 294 | 1575 | 38 | 4 | 2 |
| 14 | 9311017 | 260 | 1405 | 28 | 14 | 2 |
| 15 | 9311018 | 169 | 491 | 9 | 8 | 0 |
| 16 | 9311019 | 230 | 667 | 18 | 4 | 1 |
| 17 | 9311020 | 202 | 881 | 18 | 10 | 1 |
| 18 | 9311021 | 185 | 663 | 13 | 7 | 4 |
| 19 | 9311022 | 161 | 573 | 8 | 2 | 0 |
| 20 | 9311023 | 255 | 1272 | 26 | 13 | 2 |
| 21 | 9311024 | 235 | 1038 | 20 | 1 | 3 |
| 22 | 9311025 | 207 | 896 | 22 | 11 | 2 |
| 23 | 9311026 | 252 | 1259 | 24 | 1 | 1 |
| 24 | 9311027 | 263 | 1051 | 17 | 3 | 1 |
| 25 | 9311028 | 182 | 829 | 19 | 4 | 0 |
| 26 | 9311029 | 244 | 951 | 15 | 8 | 3 |
| 27 | 9311030 | 166 | 482 | 15 | 7 | 1 |
| 28 | 9311031 | 201 | 933 | 13 | 8 | 2 |
| 29 | 9311032 | 227 | 1059 | 18 | 6 | 2 |
| 30 | 9311033 | 216 | 1054 | 24 | 7 | 3 |
| 31 | 9311034 | 209 | 933 | 25 | 4 | 1 |
| 32 | 9311035 | 201 | 994 | 23 | 11 | 4 |
| 33 | 9311036 | 202 | 951 | 26 | 8 | 2 |
| 34 | 9311037 | 205 | 1018 | 24 | 2 | 1 |
| 35 | 9311038 | 209 | 985 | 15 | 3 | 1 |
| 36 | 9311039 | 163 | 698 | 19 | 4 | 0 |
| 37 | 9311040 | 237 | 1533 | 37 | 2 | 2 |
| 38 | 9311041 | 181 | 553 | 10 | 5 | 0 |
| 39 | 9311042 | 259 | 1060 | 25 | 1 | 2 |
| 40 | 9311043 | 239 | 1092 | 37 | 2 | 4 |
| 41 | 9311044 | 174 | 721 | 14 | 5 | 2 |
| 42 | 9311045 | 193 | 940 | 14 | 5 | 0 |
| 43 | 9311046 | 228 | 1042 | 23 | 5 | 3 |
| 44 | 9311047 | 206 | 1091 | 19 | 5 | 2 |
| 45 | 9311048 | 204 | 731 | 17 | 5 | 1 |
| 46 | 9311049 | 245 | 1431 | 40 | 4 | 4 |
| 47 | 9311050 | 209 | 937 | 22 | 5 | 1 |
| 48 | 9311051 | 273 | 1565 | 36 | 6 | 2 |
| 49 | 9311052 | 234 | 1069 | 21 | 8 | 1 |
| 50 | 9311053 | 200 | 806 | 28 | 8 | 3 |
| 51 | 9311054 | 181 | 599 | 13 | 6 | 3 |
| 52 | 9311055 | 185 | 818 | 10 | 3 | 2 |
| 53 | 9311056 | 280 | 1605 | 17 | 2 | 1 |
| 54 | 9311057 | 171 | 665 | 15 | 6 | 0 |
| 55 | 9311058 | 219 | 1072 | 20 | 5 | 1 |
| 56 | 9311059 | 217 | 904 | 24 | 2 | 2 |
| 57 | 9311060 | 237 | 1071 | 21 | 6 | 2 |
| 58 | 9311061 | 182 | 756 | 12 | 1 | 1 |
| 59 | 9311062 | 188 | 732 | 18 | 2 | 0 |
| 60 | 9311063 | 153 | 568 | 12 | 6 | 0 |
| 61 | 9311064 | 217 | 976 | 16 | 2 | 3 |
| 62 | 9311065 | 218 | 929 | 28 | 2 | 1 |
| 63 | 9311066 | 318 | 1919 | 24 | 5 | 1 |
| 64 | 9311067 | 160 | 684 | 15 | 3 | 1 |
| 65 | 9311068 | 202 | 884 | 22 | 10 | 1 |
| 66 | 9311069 | 242 | 1107 | 26 | 3 | 2 |
| 67 | 9311070 | 234 | 1113 | 26 | 6 | 3 |
| 68 | 9311071 | 144 | 472 | 11 | 1 | 1 |
| 69 | 9311072 | 243 | 1285 | 30 | 12 | 3 |
| 70 | 9311073 | 251 | 1165 | 8 | 3 | 2 |
| 71 | 9311074 | 251 | 1138 | 30 | 10 | 3 |
| 72 | 9311075 | 172 | 637 | 9 | 12 | 1 |
| 73 | 9311076 | 229 | 1024 | 18 | 0 | 1 |
| 74 | 9311077 | 239 | 1257 | 30 | 8 | 3 |
| 75 | 9311078 | 206 | 768 | 14 | 6 | 2 |
| 76 | 9311079 | 224 | 1095 | 26 | 5 | 2 |
| 77 | 9311080 | 210 | 824 | 12 | 5 | 1 |
| 78 | 9311081 | 215 | 860 | 29 | 4 | 3 |
| 79 | 9311082 | 268 | 1339 | 20 | 6 | 2 |
| 80 | 9311083 | 256 | 1144 | 26 | 11 | 2 |
| 81 | 9311084 | 342 | 1443 | 32 | 11 | 2 |
| 82 | 9311085 | 216 | 1205 | 26 | 6 | 1 |
| 83 | 9311086 | 239 | 1152 | 23 | 9 | 1 |
| 84 | 9311087 | 191 | 782 | 20 | 8 | 3 |
| 85 | 9311088 | 242 | 1270 | 27 | 2 | 2 |
| 86 | 9311089 | 206 | 807 | 24 | 2 | 3 |
| 87 | 9311090 | 223 | 1095 | 22 | 8 | 3 |
| 88 | 9311091 | 197 | 902 | 17 | 2 | 1 |
| 89 | 9311092 | 182 | 797 | 14 | 6 | 0 |
| 90 | 9311093 | 255 | 1058 | 17 | 11 | 2 |
| 91 | 9311094 | 189 | 712 | 25 | 7 | 0 |
| 92 | 9311095 | 244 | 1162 | 30 | 5 | 1 |
| 93 | 9311096 | 183 | 684 | 19 | 10 | 1 |
| 94 | 9311097 | 197 | 886 | 24 | 0 | 1 |
| 95 | 9311098 | 178 | 699 | 18 | 7 | 2 |
| 96 | 9311099 | 201 | 958 | 35 | 2 | 0 |
| 97 | 9311100 | 182 | 715 | 22 | 9 | 2 |
| 98 | 9311101 | 242 | 1105 | 30 | 5 | 2 |
| 99 | 9311102 | 245 | 1052 | 19 | 7 | 0 |
| 100 | 9311103 | 214 | 833 | 16 | 9 | 2 |
| 101 | 9311104 | 185 | 960 | 24 | 1 | 1 |
| 102 | 9311105 | 251 | 1407 | 36 | 13 | 2 |
| 103 | 9311106 | 259 | 1104 | 14 | 5 | 1 |
| 104 | 9311107 | 168 | 846 | 23 | 1 | 1 |
| 105 | 9311108 | 258 | 734 | 16 | 6 | 2 |
| 106 | 9311109 | 239 | 1148 | 21 | 3 | 2 |
| 107 | 9311110 | 225 | 930 | 25 | 11 | 1 |
| 108 | 9311111 | 203 | 962 | 25 | 4 | 2 |
| 109 | 9311112 | 263 | 1083 | 26 | 5 | 1 |
| 110 | 9311113 | 309 | 1919 | 43 | 4 | 1 |
| 111 | 9311114 | 228 | 1007 | 28 | 5 | 1 |
| 112 | 9311115 | 196 | 946 | 19 | 3 | 3 |
| 113 | 9311116 | 234 | 1198 | 31 | 3 | 3 |
| 114 | 9311117 | 173 | 572 | 13 | 5 | 1 |
| 115 | 9311118 | 215 | 955 | 17 | 5 | 2 |
| 116 | 9311119 | 219 | 824 | 20 | 4 | 1 |
| 117 | 9311120 | 188 | 776 | 17 | 9 | 0 |
| 118 | 9311121 | 167 | 663 | 9 | 4 | 2 |
| 119 | 9311122 | 220 | 1055 | 21 | 6 | 1 |
| 120 | 9311123 | 210 | 961 | 19 | 3 | 1 |
| 121 | 9311124 | 220 | 886 | 20 | 6 | 1 |
| 122 | 9311125 | 200 | 477 | 8 | 3 | 0 |
| 123 | 9311126 | 201 | 776 | 18 | 9 | 1 |
| 124 | 9311127 | 246 | 1146 | 33 | 6 | 2 |
| 125 | 9311128 | 210 | 916 | 18 | 11 | 1 |
| 126 | 9311129 | 215 | 984 | 22 | 5 | 1 |
| 127 | 9311130 | 241 | 1074 | 16 | 9 | 0 |
| 128 | 9311131 | 215 | 1181 | 26 | 7 | 3 |
| 129 | 9311132 | 185 | 704 | 13 | 3 | 3 |
| 130 | 9311133 | 157 | 688 | 12 | 3 | 1 |
| 131 | 9311134 | 258 | 1098 | 16 | 10 | 3 |
| 132 | 9311135 | 189 | 859 | 19 | 3 | 1 |
| 133 | 9311136 | 193 | 848 | 21 | 2 | 2 |
| 134 | 9311137 | 228 | 902 | 20 | 9 | 2 |
| 135 | 9311138 | 181 | 761 | 17 | 7 | 1 |
| 136 | 9311139 | 194 | 726 | 21 | 7 | 1 |
| 137 | 9311140 | 219 | 976 | 22 | 4 | 1 |
| 138 | 9311141 | 241 | 1136 | 9 | 3 | 1 |
| 139 | 9311142 | 249 | 1331 | 23 | 7 | 0 |
| 140 | 9311143 | 246 | 1061 | 14 | 1 | 1 |
| 141 | 9311144 | 220 | 958 | 19 | 5 | 0 |
| 142 | 9311145 | 199 | 777 | 13 | 6 | 1 |
| 143 | 9311146 | 180 | 791 | 22 | 6 | 1 |
| 144 | 9311147 | 177 | 544 | 16 | 5 | 2 |
| 145 | 9311148 | 442 | 2229 | 73 | 4 | 6 |
| 146 | 9311149 | 214 | 888 | 14 | 6 | 2 |
| 147 | 9311150 | 202 | 915 | 19 | 5 | 1 |
| 148 | 9311151 | 229 | 1102 | 26 | 8 | 2 |
| 149 | 9311152 | 208 | 981 | 26 | 10 | 3 |
| 150 | 9311153 | 165 | 685 | 10 | 4 | 1 |
| 151 | 9311154 | 259 | 1426 | 32 | 1 | 1 |
| 152 | 9311155 | 206 | 721 | 18 | 8 | 2 |
| 153 | 9311156 | 225 | 953 | 33 | 5 | 0 |
| 154 | 9311157 | 208 | 1093 | 22 | 4 | 0 |
| 155 | 9311158 | 197 | 1038 | 18 | 5 | 5 |
| 156 | 9311159 | 208 | 789 | 19 | 5 | 1 |
| 157 | 9311160 | 229 | 734 | 22 | 9 | 2 |
| 158 | 9311161 | 218 | 774 | 19 | 3 | 1 |
| 159 | 9311162 | 236 | 1159 | 18 | 8 | 1 |
| 160 | 9311163 | 240 | 1031 | 26 | 2 | 0 |
| 161 | 9311164 | 203 | 1109 | 22 | 0 | 2 |
| 162 | 9311165 | 201 | 884 | 31 | 8 | 2 |
| 163 | 9311166 | 252 | 1039 | 17 | 4 | 4 |
| 164 | 9311167 | 230 | 935 | 16 | 10 | 4 |
| 165 | 9311168 | 194 | 759 | 7 | 2 | 2 |
| 166 | 9311169 | 216 | 918 | 16 | 4 | 1 |
| 167 | 9311170 | 204 | 883 | 21 | 3 | 2 |
| 168 | 9311171 | 204 | 869 | 18 | 3 | 0 |
| 169 | 9311172 | 257 | 992 | 18 | 4 | 3 |
| 170 | 9311173 | 229 | 1119 | 13 | 7 | 2 |
| 171 | 9311174 | 233 | 1070 | 22 | 4 | 3 |
| 172 | 9311175 | 215 | 703 | 12 | 1 | 0 |
| 173 | 9311176 | 213 | 1008 | 18 | 6 | 0 |
| 174 | 9311177 | 158 | 578 | 15 | 0 | 3 |
| 175 | 9311178 | 268 | 1441 | 27 | 6 | 3 |
| 176 | 9311179 | 251 | 1065 | 20 | 6 | 2 |
| 177 | 9311180 | 237 | 1083 | 22 | 1 | 2 |
| 178 | 9311181 | 288 | 1104 | 32 | 2 | 3 |
| 179 | 9311182 | 184 | 848 | 23 | 2 | 1 |
| 180 | 9311183 | 173 | 659 | 11 | 8 | 2 |
| 181 | 9311184 | 187 | 784 | 19 | 4 | 2 |
| 182 | 9311185 | 226 | 936 | 35 | 7 | 3 |
| 183 | 9311186 | 196 | 910 | 31 | 5 | 2 |
| 184 | 9311187 | 290 | 1419 | 28 | 2 | 1 |
| 185 | 9311188 | 273 | 1354 | 15 | 7 | 2 |
| 186 | 9311189 | 206 | 839 | 14 | 5 | 0 |
| 187 | 9311190 | 209 | 879 | 31 | 7 | 3 |
| 188 | 9311191 | 297 | 1556 | 45 | 4 | 3 |
| 189 | 9311192 | 179 | 632 | 16 | 2 | 2 |
| 190 | 9311193 | 180 | 687 | 14 | 1 | 2 |
| 191 | 9311194 | 195 | 1000 | 15 | 4 | 1 |
| 192 | 9311195 | 186 | 865 | 26 | 4 | 1 |
| 193 | 9311196 | 249 | 1170 | 21 | 4 | 2 |
| 194 | 9311197 | 215 | 1187 | 22 | 9 | 1 |
| 195 | 9311198 | 224 | 969 | 10 | 12 | 3 |
| 196 | 9311199 | 247 | 1148 | 24 | 5 | 1 |
| 197 | 9311200 | 208 | 762 | 16 | 4 | 1 |
| 198 | 9311201 | 191 | 730 | 17 | 6 | 1 |
| 199 | 9311202 | 315 | 1651 | 23 | 7 | 2 |
| 200 | 9311203 | 242 | 1414 | 16 | 6 | 1 |
| 201 | 9311204 | 231 | 1125 | 21 | 8 | 2 |
| 202 | 9311205 | 235 | 1169 | 20 | 1 | 0 |
| 203 | 9311206 | 202 | 808 | 19 | 7 | 2 |
| 204 | 9311207 | 242 | 1372 | 40 | 6 | 2 |
| 205 | 9311208 | 195 | 944 | 21 | 6 | 3 |
| 206 | 9311209 | 268 | 1279 | 38 | 3 | 2 |
| 207 | 9311210 | 224 | 1187 | 15 | 3 | 2 |
| 208 | 9311211 | 190 | 910 | 13 | 2 | 0 |
| 209 | 9311212 | 177 | 577 | 16 | 3 | 1 |
| 210 | 9311213 | 288 | 1693 | 31 | 2 | 2 |
| 211 | 9311214 | 180 | 754 | 14 | 8 | 3 |
| 212 | 9311215 | 214 | 1148 | 13 | 3 | 0 |
| 213 | 9311216 | 182 | 705 | 14 | 2 | 1 |
| 214 | 9311217 | 305 | 1547 | 36 | 7 | 2 |
| 215 | 9311218 | 198 | 676 | 19 | 5 | 1 |
| 216 | 9311219 | 219 | 1037 | 21 | 1 | 0 |
| 217 | 9311220 | 201 | 987 | 29 | 9 | 3 |
| 218 | 9311221 | 195 | 887 | 15 | 8 | 1 |
| 219 | 9311222 | 198 | 832 | 21 | 3 | 3 |
| 220 | 9311223 | 221 | 961 | 30 | 6 | 4 |
| 221 | 9311224 | 232 | 1069 | 21 | 5 | 2 |
| 222 | 9311225 | 230 | 1205 | 32 | 5 | 4 |
| 223 | 9311226 | 226 | 1129 | 35 | 5 | 3 |
| 224 | 9311227 | 201 | 668 | 15 | 7 | 3 |
| 225 | 9311228 | 274 | 1376 | 28 | 5 | 2 |
| 226 | 9311229 | 295 | 1487 | 21 | 4 | 4 |
| 227 | 9311230 | 164 | 625 | 8 | 5 | 1 |
| 228 | 9311231 | 338 | 1802 | 19 | 3 | 0 |
| 229 | 9311232 | 194 | 941 | 19 | 4 | 2 |
| 230 | 9311233 | 273 | 1403 | 18 | 2 | 1 |
| 231 | 9311234 | 260 | 1168 | 35 | 5 | 5 |
| 232 | 9311235 | 223 | 1058 | 34 | 4 | 2 |
| 233 | 9311236 | 235 | 1094 | 14 | 3 | 1 |
| 234 | 9311237 | 248 | 1228 | 26 | 1 | 2 |
| 235 | 9311238 | 201 | 732 | 20 | 3 | 2 |
| 236 | 9311239 | 199 | 686 | 16 | 7 | 0 |
| 237 | 9311240 | 260 | 1191 | 40 | 12 | 3 |
| 238 | 9311241 | 176 | 677 | 8 | 5 | 2 |
| 239 | 9311242 | 243 | 969 | 13 | 8 | 1 |
| 240 | 9311243 | 200 | 694 | 19 | 5 | 3 |
| 241 | 9311244 | 247 | 1166 | 13 | 6 | 2 |
| 242 | 9311245 | 211 | 854 | 21 | 7 | 4 |
| 243 | 9311246 | 221 | 1020 | 33 | 9 | 3 |
| 244 | 9311247 | 190 | 745 | 11 | 6 | 0 |
| 245 | 9311248 | 233 | 1036 | 25 | 9 | 2 |
| 246 | 9311249 | 175 | 714 | 25 | 3 | 1 |
| 247 | 9311250 | 184 | 602 | 15 | 6 | 0 |
| 248 | 9311251 | 154 | 513 | 10 | 6 | 0 |
| 249 | 9311252 | 210 | 723 | 24 | 9 | 4 |
| 250 | 9311253 | 191 | 765 | 11 | 8 | 0 |
| 251 | 9311254 | 248 | 1293 | 24 | 5 | 4 |
| 252 | 9311255 | 245 | 1295 | 28 | 4 | 2 |
| 253 | 9311256 | 231 | 1149 | 25 | 3 | 0 |
| 254 | 9311257 | 176 | 714 | 21 | 6 | 2 |
| 255 | 9311258 | 239 | 1102 | 25 | 2 | 2 |
| 256 | 9311259 | 276 | 1284 | 21 | 1 | 1 |
| 257 | 9311260 | 201 | 732 | 11 | 5 | 1 |
| 258 | 9311261 | 193 | 946 | 26 | 0 | 2 |
| 259 | 9311262 | 210 | 811 | 13 | 2 | 1 |
| 260 | 9311263 | 229 | 1109 | 14 | 3 | 2 |
| 261 | 9311264 | 241 | 1053 | 23 | 4 | 3 |
| 262 | 9311265 | 258 | 1063 | 24 | 13 | 2 |
| 263 | 9311266 | 223 | 778 | 15 | 0 | 0 |
| 264 | 9311267 | 268 | 1038 | 17 | 5 | 1 |
| 265 | 9311268 | 183 | 722 | 8 | 3 | 0 |
| 266 | 9311269 | 187 | 767 | 25 | 3 | 4 |
| 267 | 9311270 | 262 | 1333 | 21 | 16 | 2 |
| 268 | 9311271 | 221 | 672 | 9 | 2 | 2 |
| 269 | 9311272 | 185 | 859 | 18 | 11 | 3 |
| 270 | 9311273 | 275 | 1549 | 38 | 6 | 1 |
| 271 | 9311274 | 234 | 971 | 28 | 7 | 2 |
| 272 | 9311275 | 310 | 989 | 44 | 9 | 3 |
| 273 | 9311276 | 179 | 655 | 11 | 5 | 0 |
| 274 | 9311277 | 230 | 999 | 27 | 1 | 2 |
| 275 | 9311278 | 272 | 1490 | 33 | 5 | 3 |
| 276 | 9311279 | 242 | 1092 | 18 | 4 | 2 |
| 277 | 9311280 | 252 | 1163 | 30 | 5 | 1 |
| 278 | 9311281 | 250 | 865 | 15 | 1 | 2 |
| 279 | 9311282 | 228 | 1000 | 18 | 7 | 1 |
| 280 | 9311283 | 169 | 674 | 9 | 3 | 2 |
| 281 | 9311284 | 233 | 1012 | 30 | 4 | 3 |
| 282 | 9311285 | 203 | 783 | 16 | 3 | 0 |
| 283 | 9311286 | 298 | 1441 | 44 | 4 | 4 |
| 284 | 9311287 | 231 | 1074 | 21 | 2 | 3 |
| 285 | 9311288 | 217 | 899 | 16 | 7 | 0 |
| 286 | 9311289 | 307 | 1764 | 33 | 8 | 1 |
| 287 | 9311290 | 220 | 1038 | 19 | 1 | 1 |
| 288 | 9311291 | 206 | 790 | 16 | 4 | 2 |
| 289 | 9311292 | 215 | 1046 | 18 | 1 | 3 |
| 290 | 9311293 | 201 | 1054 | 22 | 6 | 1 |
| 291 | 9311294 | 218 | 886 | 15 | 5 | 3 |
| 292 | 9311295 | 224 | 1046 | 15 | 6 | 4 |
| 293 | 9311296 | 210 | 847 | 18 | 8 | 3 |
| 294 | 9311297 | 191 | 765 | 14 | 3 | 1 |
| 295 | 9311298 | 220 | 926 | 14 | 8 | 1 |
| 296 | 9311299 | 252 | 836 | 14 | 6 | 1 |
| 297 | 9311300 | 279 | 1094 | 37 | 10 | 3 |
| 298 | 9311301 | 189 | 813 | 13 | 6 | 3 |
| 299 | 9311302 | 210 | 888 | 23 | 4 | 2 |
| 300 | 9311303 | 235 | 1095 | 10 | 5 | 2 |
| 301 | 9311304 | 168 | 489 | 9 | 2 | 1 |
| 302 | 9311305 | 209 | 653 | 14 | 4 | 1 |
| 303 | 9311306 | 231 | 827 | 22 | 2 | 1 |
| 304 | 9311307 | 297 | 1462 | 26 | 1 | 1 |
| 305 | 9311308 | 229 | 1032 | 22 | 8 | 3 |
| 306 | 9311309 | 227 | 1168 | 31 | 6 | 0 |
| 307 | 9311310 | 201 | 1014 | 18 | 0 | 1 |
| 308 | 9311311 | 229 | 1105 | 11 | 7 | 3 |
| 309 | 9311312 | 168 | 740 | 10 | 7 | 0 |
| 310 | 9311313 | 255 | 1062 | 21 | 11 | 3 |
| 311 | 9311314 | 244 | 1197 | 28 | 9 | 4 |
| 312 | 9311315 | 193 | 756 | 22 | 6 | 1 |
| 313 | 9311316 | 223 | 891 | 28 | 1 | 3 |
| 314 | 9311317 | 190 | 836 | 17 | 5 | 3 |
| 315 | 9311318 | 299 | 1620 | 19 | 8 | 0 |
| 316 | 9311319 | 210 | 996 | 25 | 7 | 3 |
| 317 | 9311320 | 212 | 1048 | 18 | 10 | 1 |
| 318 | 9311321 | 177 | 712 | 15 | 1 | 1 |
| 319 | 9311322 | 229 | 1017 | 17 | 14 | 1 |
| 320 | 9311323 | 197 | 890 | 24 | 4 | 1 |
| 321 | 9311324 | 226 | 1017 | 17 | 5 | 3 |
| 322 | 9311325 | 240 | 1246 | 15 | 4 | 0 |
| 323 | 9311326 | 235 | 1088 | 19 | 6 | 2 |
| 324 | 9311327 | 212 | 891 | 24 | 3 | 2 |
| 325 | 9311328 | 212 | 884 | 16 | 4 | 1 |
| 326 | 9311329 | 194 | 511 | 12 | 2 | 0 |
| 327 | 9311330 | 180 | 621 | 15 | 8 | 1 |
| 328 | 9311331 | 257 | 1324 | 31 | 4 | 1 |
| 329 | 9311332 | 252 | 1020 | 22 | 7 | 2 |
| 330 | 9311333 | 198 | 796 | 21 | 10 | 4 |
| 331 | 9311334 | 165 | 477 | 18 | 2 | 2 |
| 332 | 9311335 | 193 | 753 | 16 | 8 | 2 |
| 333 | 9311336 | 239 | 1353 | 17 | 3 | 2 |
| 334 | 9311337 | 204 | 848 | 20 | 6 | 1 |
| 335 | 9311338 | 198 | 918 | 8 | 4 | 0 |
| 336 | 9311339 | 224 | 1037 | 12 | 5 | 2 |
| 337 | 9311340 | 210 | 715 | 12 | 6 | 1 |
| 338 | 9311341 | 194 | 919 | 28 | 3 | 1 |
| 339 | 9311342 | 205 | 633 | 16 | 0 | 1 |
| 340 | 9311343 | 248 | 1327 | 24 | 4 | 0 |
| 341 | 9311344 | 176 | 597 | 15 | 11 | 2 |
| 342 | 9311345 | 232 | 1196 | 29 | 3 | 2 |
| 343 | 9311346 | 248 | 1289 | 35 | 16 | 4 |
| 344 | 9311347 | 228 | 904 | 21 | 4 | 2 |
| 345 | 9311348 | 201 | 987 | 24 | 4 | 1 |
| 346 | 9311349 | 226 | 836 | 18 | 10 | 1 |
| 347 | 9311350 | 246 | 1336 | 25 | 3 | 2 |
| 348 | 9311351 | 216 | 735 | 15 | 1 | 2 |
| 349 | 9311352 | 268 | 1278 | 29 | 7 | 1 |
| 350 | 9311353 | 228 | 1218 | 27 | 2 | 1 |
| 351 | 9311354 | 214 | 1042 | 35 | 6 | 3 |
| 352 | 9311355 | 204 | 970 | 20 | 1 | 2 |
| 353 | 9311356 | 234 | 1221 | 24 | 11 | 3 |
| 354 | 9311357 | 199 | 830 | 18 | 5 | 0 |
| 355 | 9311358 | 240 | 1255 | 27 | 10 | 2 |
| 356 | 9311359 | 223 | 1037 | 9 | 5 | 1 |
| 357 | 9311360 | 238 | 968 | 10 | 11 | 2 |
| 358 | 9311361 | 251 | 1415 | 32 | 3 | 3 |
| 359 | 9311362 | 195 | 833 | 17 | 7 | 1 |
| 360 | 9311363 | 202 | 957 | 20 | 8 | 2 |
| 361 | 9311364 | 168 | 463 | 11 | 5 | 0 |
| 362 | 9311365 | 232 | 999 | 21 | 14 | 1 |
| 363 | 9311366 | 381 | 1961 | 40 | 6 | 1 |
| 364 | 9311367 | 190 | 805 | 7 | 4 | 2 |
| 365 | 9311368 | 201 | 953 | 23 | 1 | 1 |
| 366 | 9311369 | 200 | 822 | 17 | 8 | 0 |
| 367 | 9311370 | 230 | 881 | 26 | 4 | 1 |
| 368 | 9311371 | 227 | 955 | 23 | 6 | 2 |
| 369 | 9311372 | 219 | 936 | 17 | 8 | 1 |
| 370 | 9311373 | 225 | 719 | 17 | 3 | 2 |
| 371 | 9311374 | 264 | 1320 | 36 | 5 | 1 |
| 372 | 9311375 | 197 | 629 | 10 | 6 | 1 |
| 373 | 9311376 | 243 | 1003 | 26 | 4 | 3 |
| 374 | 9311377 | 209 | 992 | 23 | 10 | 2 |
| 375 | 9311378 | 243 | 1073 | 23 | 3 | 2 |
| 376 | 9311379 | 292 | 1077 | 9 | 2 | 1 |
| 377 | 9311380 | 238 | 1004 | 28 | 2 | 1 |
| 378 | 9311381 | 249 | 1213 | 37 | 3 | 2 |
| 379 | 9311382 | 258 | 1277 | 30 | 6 | 2 |
| 380 | 9311383 | 230 | 1039 | 20 | 1 | 1 |
| 381 | 9311384 | 193 | 856 | 20 | 6 | 2 |
| 382 | 9311385 | 222 | 1020 | 20 | 3 | 1 |
| 383 | 9311386 | 185 | 752 | 18 | 4 | 1 |
| 384 | 9311387 | 199 | 853 | 22 | 9 | 0 |
| 385 | 9311388 | 171 | 652 | 7 | 7 | 2 |
| 386 | 9311389 | 197 | 781 | 15 | 7 | 0 |
| 387 | 9311390 | 172 | 539 | 7 | 6 | 0 |
| 388 | 9311391 | 211 | 839 | 20 | 5 | 2 |
| 389 | 9311392 | 162 | 589 | 15 | 6 | 1 |
| 390 | 9311393 | 159 | 586 | 13 | 3 | 0 |
| 391 | 9311394 | 212 | 910 | 9 | 9 | 1 |
| 392 | 9311395 | 169 | 667 | 16 | 4 | 2 |
| 393 | 9311396 | 289 | 1359 | 36 | 5 | 1 |
| 394 | 9311397 | 205 | 965 | 18 | 14 | 1 |
| 395 | 9311398 | 226 | 923 | 23 | 2 | 2 |
| 396 | 9311399 | 254 | 1344 | 31 | 4 | 4 |
| 397 | 9311400 | 224 | 910 | 33 | 3 | 1 |
| 398 | 9311401 | 222 | 1139 | 30 | 6 | 3 |
| 399 | 9311402 | 214 | 1076 | 28 | 8 | 1 |
| 400 | 9311403 | 211 | 902 | 17 | 2 | 1 |
| 401 | 9311404 | 278 | 1402 | 21 | 5 | 1 |
| 402 | 9311405 | 273 | 1375 | 16 | 2 | 2 |
| 403 | 9311406 | 179 | 618 | 10 | 5 | 1 |
| 404 | 9311407 | 239 | 966 | 18 | 5 | 1 |
| 405 | 9311408 | 201 | 1017 | 17 | 2 | 1 |
| 406 | 9311409 | 205 | 1081 | 16 | 4 | 1 |
| 407 | 9311410 | 215 | 842 | 26 | 8 | 2 |
| 408 | 9311411 | 204 | 568 | 16 | 3 | 1 |
| 409 | 9311412 | 174 | 815 | 15 | 5 | 3 |
| 410 | 9311413 | 238 | 899 | 20 | 6 | 3 |
| 411 | 9311414 | 184 | 916 | 19 | 4 | 1 |
| 412 | 9311415 | 204 | 895 | 17 | 8 | 2 |
| 413 | 9311416 | 220 | 1223 | 25 | 6 | 2 |
| 414 | 9311417 | 246 | 1272 | 35 | 6 | 1 |
| 415 | 9311418 | 180 | 769 | 20 | 5 | 1 |
| 416 | 9311419 | 236 | 1036 | 28 | 5 | 1 |
| 417 | 9311420 | 241 | 1214 | 32 | 9 | 2 |
| 418 | 9311421 | 243 | 962 | 20 | 1 | 2 |
| 419 | 9311422 | 264 | 1327 | 22 | 8 | 2 |
| 420 | 9311423 | 277 | 1820 | 34 | 3 | 0 |
| 421 | 9311424 | 274 | 1598 | 27 | 7 | 2 |
| 422 | 9311425 | 185 | 520 | 6 | 4 | 2 |
| 423 | 9311426 | 195 | 725 | 17 | 3 | 1 |
| 424 | 9311427 | 202 | 756 | 7 | 3 | 0 |
| 425 | 9311428 | 151 | 612 | 12 | 4 | 2 |
| 426 | 9311429 | 197 | 673 | 26 | 7 | 1 |
| 427 | 9311430 | 197 | 727 | 17 | 1 | 0 |
| 428 | 9311431 | 231 | 841 | 16 | 3 | 1 |
| 429 | 9311432 | 203 | 960 | 26 | 3 | 3 |
| 430 | 9311433 | 238 | 1196 | 29 | 2 | 2 |
| 431 | 9311434 | 228 | 909 | 18 | 8 | 2 |
| 432 | 9311435 | 240 | 1081 | 21 | 5 | 1 |
| 433 | 9311436 | 186 | 837 | 12 | 4 | 2 |
| 434 | 9311437 | 283 | 1431 | 22 | 2 | 2 |
| 435 | 9311438 | 155 | 554 | 13 | 3 | 1 |
| 436 | 9311439 | 214 | 881 | 13 | 6 | 1 |
| 437 | 9311440 | 192 | 698 | 13 | 7 | 1 |
| 438 | 9311441 | 161 | 510 | 9 | 6 | 0 |
| 439 | 9311442 | 330 | 1580 | 32 | 6 | 2 |
| 440 | 9311443 | 211 | 1073 | 13 | 5 | 1 |
| 441 | 9311444 | 238 | 1304 | 35 | 5 | 3 |
| 442 | 9311445 | 238 | 1237 | 14 | 6 | 1 |
| 443 | 9311446 | 214 | 968 | 22 | 2 | 2 |
| 444 | 9311447 | 243 | 858 | 17 | 3 | 4 |
| 445 | 9311448 | 206 | 990 | 27 | 5 | 1 |
| 446 | 9311449 | 227 | 1148 | 24 | 2 | 3 |
| 447 | 9311450 | 215 | 970 | 24 | 6 | 2 |
| 448 | 9311451 | 218 | 1083 | 26 | 1 | 3 |
| 449 | 9311452 | 220 | 1048 | 34 | 7 | 0 |
| 450 | 9311453 | 175 | 643 | 7 | 4 | 2 |
| 451 | 9311454 | 270 | 1268 | 21 | 4 | 2 |
| 452 | 9311455 | 245 | 1122 | 20 | 3 | 2 |
| 453 | 9311456 | 209 | 898 | 23 | 0 | 3 |
| 454 | 9311457 | 186 | 837 | 19 | 3 | 1 |
| 455 | 9311458 | 251 | 1161 | 15 | 4 | 2 |
| 456 | 9311459 | 242 | 971 | 23 | 10 | 1 |
| 457 | 9311460 | 215 | 1015 | 28 | 3 | 0 |
| 458 | 9311461 | 189 | 728 | 9 | 4 | 1 |
| 459 | 9311462 | 201 | 971 | 18 | 8 | 2 |
| 460 | 9311463 | 216 | 943 | 19 | 6 | 2 |
| 461 | 9311464 | 175 | 764 | 25 | 5 | 0 |
| 462 | 9311465 | 225 | 1194 | 31 | 1 | 1 |
| 463 | 9311466 | 206 | 976 | 16 | 5 | 1 |
| 464 | 9311467 | 207 | 907 | 21 | 2 | 2 |
| 465 | 9311468 | 167 | 721 | 23 | 2 | 1 |
| 466 | 9311469 | 231 | 938 | 28 | 2 | 2 |
| 467 | 9311470 | 211 | 1063 | 13 | 8 | 1 |
| 468 | 9311471 | 223 | 1240 | 21 | 3 | 0 |
| 469 | 9311472 | 222 | 1099 | 23 | 15 | 3 |
| 470 | 9311473 | 220 | 1057 | 24 | 6 | 3 |
| 471 | 9311474 | 206 | 967 | 11 | 1 | 1 |
| 472 | 9311475 | 183 | 585 | 12 | 5 | 3 |
| 473 | 9311476 | 209 | 984 | 23 | 4 | 2 |
| 474 | 9311477 | 257 | 1131 | 17 | 6 | 2 |
| 475 | 9311478 | 247 | 1236 | 18 | 6 | 1 |
| 476 | 9311479 | 208 | 774 | 14 | 7 | 1 |
| 477 | 9311480 | 248 | 915 | 22 | 1 | 3 |
| 478 | 9311481 | 240 | 684 | 13 | 2 | 1 |
| 479 | 9311482 | 260 | 1009 | 17 | 3 | 3 |
| 480 | 9311483 | 193 | 991 | 13 | 2 | 2 |
| 481 | 9311484 | 174 | 732 | 7 | 2 | 1 |
| 482 | 9311485 | 161 | 573 | 13 | 4 | 0 |
| 483 | 9311486 | 267 | 1074 | 9 | 6 | 1 |
| 484 | 9311487 | 166 | 510 | 11 | 2 | 1 |
| 485 | 9311488 | 227 | 1116 | 16 | 9 | 1 |
| 486 | 9311489 | 210 | 787 | 14 | 11 | 1 |
| 487 | 9311490 | 183 | 730 | 14 | 4 | 0 |
| 488 | 9311491 | 208 | 893 | 13 | 4 | 1 |
| 489 | 9311492 | 237 | 1235 | 30 | 4 | 2 |
| 490 | 9311493 | 195 | 1032 | 17 | 3 | 1 |
| 491 | 9311494 | 207 | 784 | 17 | 6 | 1 |
| 492 | 9311495 | 224 | 989 | 19 | 3 | 4 |
| 493 | 9311496 | 272 | 1222 | 28 | 6 | 2 |
| 494 | 9311497 | 222 | 886 | 19 | 4 | 1 |
| 495 | 9311498 | 218 | 1129 | 14 | 5 | 4 |
| 496 | 9311499 | 188 | 840 | 22 | 8 | 2 |
| 497 | 9311500 | 210 | 897 | 19 | 2 | 0 |
| 498 | 9311501 | 228 | 779 | 21 | 9 | 2 |
| 499 | 9311502 | 235 | 819 | 18 | 1 | 2 |
| 500 | 9311503 | 312 | 1674 | 20 | 8 | 2 |
| 501 | 9311504 | 208 | 889 | 15 | 4 | 1 |
| 502 | 9311505 | 175 | 896 | 7 | 1 | 1 |
| 503 | 9311506 | 194 | 874 | 22 | 8 | 2 |
| 504 | 9311507 | 211 | 981 | 26 | 3 | 3 |
| 505 | 9311508 | 192 | 861 | 12 | 0 | 2 |
| 506 | 9311509 | 272 | 1417 | 15 | 7 | 2 |
| 507 | 9311510 | 255 | 1413 | 31 | 3 | 3 |
| 508 | 9311511 | 233 | 744 | 22 | 8 | 3 |
| 509 | 9311512 | 190 | 777 | 18 | 3 | 1 |
| 510 | 9311513 | 215 | 1113 | 14 | 8 | 1 |
| 511 | 9311514 | 206 | 834 | 19 | 5 | 1 |
| 512 | 9311515 | 222 | 925 | 12 | 6 | 2 |
| 513 | 9311516 | 226 | 1000 | 23 | 3 | 3 |
| 514 | 9311517 | 213 | 1112 | 14 | 8 | 1 |
| 515 | 9311518 | 215 | 1051 | 14 | 5 | 1 |
| 516 | 9311519 | 204 | 855 | 18 | 11 | 2 |
| 517 | 9311520 | 336 | 1549 | 29 | 6 | 2 |
| 518 | 9311521 | 180 | 471 | 12 | 7 | 1 |
| 519 | 9311522 | 155 | 522 | 9 | 3 | 0 |
| 520 | 9311523 | 189 | 884 | 12 | 4 | 1 |
| 521 | 9311524 | 226 | 879 | 29 | 5 | 2 |
| 522 | 9311525 | 167 | 711 | 5 | 4 | 1 |
| 523 | 9311526 | 202 | 853 | 22 | 3 | 2 |
| 524 | 9311527 | 191 | 948 | 13 | 7 | 2 |
| 525 | 9311528 | 208 | 976 | 20 | 3 | 2 |
| 526 | 9311529 | 175 | 797 | 14 | 7 | 1 |
| 527 | 9311530 | 227 | 1115 | 22 | 10 | 0 |
| 528 | 9311531 | 193 | 838 | 11 | 5 | 1 |
| 529 | 9311532 | 228 | 1091 | 31 | 4 | 3 |
| 530 | 9311533 | 362 | 1861 | 21 | 6 | 3 |
| 531 | 9311534 | 198 | 924 | 15 | 2 | 2 |
| 532 | 9311535 | 219 | 1054 | 30 | 5 | 2 |
| 533 | 9311536 | 194 | 768 | 18 | 4 | 4 |
| 534 | 9311537 | 190 | 851 | 22 | 2 | 2 |
| 535 | 9311538 | 211 | 975 | 22 | 2 | 2 |
| 536 | 9311539 | 218 | 1076 | 26 | 2 | 4 |
| 537 | 9311540 | 213 | 906 | 30 | 4 | 1 |
| 538 | 9311541 | 218 | 1009 | 14 | 6 | 2 |
| 539 | 9311542 | 242 | 1026 | 23 | 7 | 1 |
| 540 | 9311543 | 244 | 844 | 21 | 9 | 1 |
| 541 | 9311544 | 272 | 1300 | 24 | 2 | 1 |
| 542 | 9311545 | 204 | 861 | 11 | 4 | 3 |
| 543 | 9311546 | 207 | 783 | 23 | 4 | 3 |
| 544 | 9311547 | 186 | 697 | 18 | 0 | 0 |
| 545 | 9311548 | 222 | 740 | 13 | 7 | 2 |
| 546 | 9311549 | 210 | 892 | 18 | 7 | 1 |
| 547 | 9311550 | 175 | 692 | 18 | 4 | 2 |
| 548 | 9311551 | 215 | 774 | 18 | 6 | 2 |
| 549 | 9311552 | 237 | 1204 | 25 | 3 | 2 |
| 550 | 9311553 | 192 | 988 | 22 | 2 | 1 |
| 551 | 9311554 | 186 | 773 | 19 | 6 | 4 |
| 552 | 9311555 | 178 | 669 | 11 | 7 | 1 |
| 553 | 9311556 | 200 | 770 | 8 | 8 | 1 |
| 554 | 9311557 | 210 | 824 | 21 | 7 | 3 |
| 555 | 9311558 | 224 | 807 | 14 | 4 | 2 |
| 556 | 9311559 | 212 | 1066 | 23 | 5 | 2 |
| 557 | 9311560 | 188 | 749 | 20 | 5 | 2 |
| 558 | 9311561 | 189 | 932 | 28 | 1 | 3 |
| 559 | 9311562 | 223 | 1201 | 24 | 3 | 1 |
| 560 | 9311563 | 230 | 996 | 24 | 5 | 2 |
| 561 | 9311564 | 258 | 1311 | 32 | 2 | 3 |
| 562 | 9311565 | 157 | 442 | 6 | 7 | 0 |
| 563 | 9311566 | 242 | 1164 | 31 | 4 | 3 |
| 564 | 9311567 | 219 | 1052 | 12 | 1 | 1 |
| 565 | 9311568 | 281 | 1277 | 15 | 8 | 3 |
| 566 | 9311569 | 284 | 1425 | 35 | 1 | 3 |
| 567 | 9311570 | 192 | 796 | 23 | 2 | 1 |
| 568 | 9311571 | 238 | 1118 | 17 | 6 | 1 |
| 569 | 9311572 | 211 | 1033 | 25 | 4 | 0 |
| 570 | 9311573 | 194 | 542 | 15 | 2 | 0 |
| 571 | 9311574 | 208 | 965 | 26 | 7 | 1 |
| 572 | 9311575 | 194 | 780 | 25 | 0 | 2 |
| 573 | 9311576 | 206 | 969 | 19 | 5 | 0 |
| 574 | 9311577 | 288 | 1267 | 27 | 9 | 1 |
| 575 | 9311578 | 201 | 858 | 23 | 5 | 2 |
| 576 | 9311579 | 180 | 753 | 22 | 7 | 1 |
| 577 | 9311580 | 273 | 1319 | 30 | 6 | 2 |
| 578 | 9311581 | 226 | 1041 | 20 | 3 | 1 |
| 579 | 9311582 | 222 | 1088 | 36 | 5 | 3 |
| 580 | 9311583 | 247 | 1233 | 41 | 12 | 2 |
| 581 | 9311584 | 240 | 1107 | 32 | 5 | 1 |
| 582 | 9311585 | 243 | 1405 | 34 | 3 | 1 |
| 583 | 9311586 | 197 | 669 | 11 | 4 | 2 |
| 584 | 9311587 | 190 | 688 | 6 | 7 | 1 |
| 585 | 9311588 | 208 | 1101 | 25 | 3 | 2 |
| 586 | 9311589 | 308 | 1782 | 34 | 1 | 1 |
| 587 | 9311590 | 194 | 737 | 15 | 2 | 2 |
| 588 | 9311591 | 204 | 782 | 18 | 5 | 1 |
| 589 | 9311592 | 227 | 991 | 24 | 7 | 1 |
| 590 | 9311593 | 230 | 967 | 28 | 2 | 4 |
| 591 | 9311594 | 216 | 1040 | 19 | 5 | 2 |
| 592 | 9311595 | 248 | 1553 | 33 | 8 | 3 |
| 593 | 9311596 | 191 | 754 | 12 | 7 | 2 |
| 594 | 9311597 | 178 | 753 | 23 | 6 | 3 |
| 595 | 9311598 | 305 | 1460 | 34 | 5 | 3 |
| 596 | 9311599 | 238 | 1249 | 24 | 4 | 2 |
| 597 | 9311600 | 191 | 789 | 12 | 6 | 1 |
| 598 | 9311601 | 195 | 836 | 12 | 3 | 1 |
| 599 | 9311602 | 213 | 867 | 11 | 5 | 3 |
| 600 | 9311603 | 211 | 772 | 15 | 7 | 3 |
| 601 | 9311604 | 194 | 766 | 13 | 0 | 2 |
| 602 | 9311605 | 220 | 1211 | 24 | 6 | 1 |
| 603 | 9311606 | 212 | 1146 | 19 | 11 | 2 |
| 604 | 9311607 | 214 | 994 | 13 | 8 | 5 |
| 605 | 9311608 | 356 | 1655 | 33 | 18 | 1 |
| 606 | 9311609 | 242 | 1111 | 25 | 6 | 0 |
| 607 | 9311610 | 293 | 1423 | 40 | 4 | 2 |
| 608 | 9311611 | 249 | 1217 | 21 | 13 | 3 |
| 609 | 9311612 | 265 | 1522 | 22 | 3 | 3 |
| 610 | 9311613 | 227 | 1016 | 29 | 3 | 1 |
| 611 | 9311614 | 258 | 1461 | 41 | 3 | 2 |
| 612 | 9311615 | 200 | 895 | 19 | 10 | 0 |
| 613 | 9311616 | 227 | 949 | 20 | 3 | 2 |
| 614 | 9311617 | 253 | 1268 | 26 | 5 | 2 |
| 615 | 9311618 | 192 | 863 | 13 | 4 | 2 |
| 616 | 9311619 | 275 | 1360 | 23 | 7 | 4 |
| 617 | 9311620 | 180 | 692 | 15 | 1 | 1 |
| 618 | 9311621 | 288 | 1515 | 42 | 1 | 3 |
| 619 | 9311622 | 306 | 1960 | 39 | 7 | 2 |
| 620 | 9311623 | 309 | 1688 | 39 | 8 | 3 |
| 621 | 9311624 | 210 | 1019 | 20 | 2 | 0 |
| 622 | 9311625 | 217 | 1012 | 21 | 3 | 1 |
| 623 | 9311626 | 271 | 1295 | 34 | 6 | 1 |
| 624 | 9311627 | 244 | 1117 | 25 | 7 | 1 |
| 625 | 9311628 | 195 | 947 | 22 | 2 | 3 |
| 626 | 9311629 | 274 | 1515 | 29 | 3 | 3 |
| 627 | 9311630 | 293 | 1442 | 17 | 6 | 2 |
| 628 | 9311631 | 242 | 1111 | 22 | 8 | 3 |
| 629 | 9311632 | 250 | 1217 | 27 | 9 | 1 |
| 630 | 9311633 | 189 | 977 | 14 | 6 | 2 |
| 631 | 9311634 | 187 | 745 | 11 | 9 | 2 |
| 632 | 9311635 | 236 | 1187 | 27 | 9 | 1 |
| 633 | 9311636 | 187 | 781 | 15 | 8 | 0 |
| 634 | 9311637 | 208 | 978 | 22 | 2 | 3 |
| 635 | 9311638 | 238 | 1040 | 21 | 3 | 1 |
| 636 | 9311639 | 218 | 1144 | 21 | 4 | 2 |
| 637 | 9311640 | 199 | 848 | 21 | 6 | 3 |
| 638 | 9311641 | 297 | 1558 | 41 | 2 | 1 |
| 639 | 9311642 | 243 | 1290 | 31 | 4 | 2 |
| 640 | 9311643 | 188 | 901 | 8 | 2 | 2 |
| 641 | 9311644 | 276 | 1601 | 33 | 7 | 0 |
| 642 | 9311645 | 175 | 687 | 18 | 3 | 1 |
| 643 | 9311646 | 235 | 864 | 15 | 4 | 2 |
| 644 | 9311647 | 170 | 843 | 14 | 5 | 0 |
| 645 | 9311648 | 183 | 886 | 20 | 7 | 2 |
| 646 | 9311649 | 279 | 1384 | 23 | 3 | 0 |
| 647 | 9311650 | 194 | 762 | 16 | 1 | 2 |
| 648 | 9311651 | 212 | 1315 | 28 | 8 | 0 |
| 649 | 9311652 | 179 | 694 | 11 | 2 | 1 |
| 650 | 9311653 | 235 | 1070 | 22 | 5 | 1 |
| 651 | 9311654 | 223 | 835 | 18 | 6 | 1 |
| 652 | 9311655 | 210 | 835 | 21 | 3 | 0 |
| 653 | 9311656 | 240 | 1007 | 22 | 8 | 1 |
| 654 | 9311657 | 205 | 908 | 16 | 4 | 0 |
| 655 | 9311658 | 211 | 961 | 20 | 2 | 0 |
| 656 | 9311659 | 230 | 1177 | 29 | 2 | 3 |
| 657 | 9311660 | 167 | 657 | 16 | 5 | 0 |
| 658 | 9311661 | 165 | 547 | 15 | 5 | 2 |
| 659 | 9311662 | 179 | 742 | 18 | 10 | 2 |
| 660 | 9311663 | 180 | 783 | 17 | 7 | 0 |
| 661 | 9311664 | 216 | 935 | 25 | 3 | 2 |
| 662 | 9311665 | 239 | 1296 | 26 | 9 | 2 |
| 663 | 9311666 | 205 | 1005 | 29 | 7 | 1 |
| 664 | 9311667 | 256 | 1003 | 27 | 6 | 2 |
| 665 | 9311668 | 188 | 805 | 19 | 4 | 2 |
| 666 | 9311669 | 172 | 729 | 25 | 1 | 1 |
| 667 | 9311670 | 248 | 1405 | 20 | 2 | 1 |
| 668 | 9311671 | 260 | 1171 | 37 | 6 | 2 |
| 669 | 9311672 | 264 | 1234 | 38 | 6 | 1 |
| 670 | 9311673 | 187 | 734 | 20 | 3 | 1 |
| 671 | 9311674 | 178 | 788 | 19 | 2 | 3 |
| 672 | 9311675 | 189 | 643 | 16 | 4 | 0 |
| 673 | 9311676 | 284 | 1716 | 40 | 3 | 1 |
| 674 | 9311677 | 170 | 562 | 13 | 3 | 0 |
| 675 | 9311678 | 197 | 1029 | 23 | 1 | 1 |
| 676 | 9311679 | 176 | 611 | 20 | 2 | 1 |
| 677 | 9311680 | 206 | 869 | 18 | 8 | 0 |
| 678 | 9311681 | 180 | 748 | 10 | 4 | 0 |
| 679 | 9311682 | 180 | 488 | 16 | 5 | 2 |
| 680 | 9311683 | 170 | 795 | 13 | 4 | 0 |
| 681 | 9311684 | 250 | 1357 | 34 | 2 | 1 |
| 682 | 9311685 | 248 | 1093 | 27 | 7 | 2 |
| 683 | 9311686 | 304 | 1382 | 36 | 4 | 4 |
| 684 | 9311687 | 232 | 883 | 25 | 7 | 4 |
| 685 | 9311688 | 253 | 912 | 16 | 6 | 1 |
| 686 | 9311689 | 249 | 1077 | 20 | 8 | 1 |
| 687 | 9311690 | 193 | 886 | 18 | 3 | 1 |
| 688 | 9311691 | 182 | 782 | 26 | 8 | 1 |
| 689 | 9311692 | 315 | 1736 | 21 | 8 | 4 |
| 690 | 9311693 | 182 | 969 | 17 | 4 | 1 |
| 691 | 9311694 | 210 | 812 | 23 | 3 | 3 |
| 692 | 9311695 | 182 | 909 | 15 | 2 | 0 |
| 693 | 9311696 | 191 | 765 | 18 | 1 | 2 |
| 694 | 9311697 | 246 | 1173 | 31 | 9 | 2 |
| 695 | 9311698 | 233 | 1089 | 16 | 5 | 0 |
| 696 | 9311699 | 367 | 2110 | 51 | 3 | 2 |
| 697 | 9311700 | 161 | 687 | 15 | 1 | 0 |
| 698 | 9311701 | 176 | 447 | 8 | 10 | 0 |
| 699 | 9311702 | 224 | 942 | 24 | 3 | 1 |
| 700 | 9311703 | 269 | 1079 | 26 | 3 | 2 |
| 701 | 9311704 | 188 | 701 | 14 | 10 | 0 |
| 702 | 9311705 | 176 | 898 | 20 | 4 | 1 |
| 703 | 9311706 | 212 | 1097 | 19 | 7 | 2 |
| 704 | 9311707 | 211 | 1002 | 25 | 5 | 2 |
| 705 | 9311708 | 196 | 650 | 13 | 14 | 1 |
| 706 | 9311709 | 203 | 897 | 14 | 4 | 1 |
| 707 | 9311710 | 219 | 966 | 7 | 3 | 4 |
| 708 | 9311711 | 274 | 1435 | 33 | 2 | 4 |
| 709 | 9311712 | 218 | 901 | 22 | 9 | 1 |
| 710 | 9311713 | 270 | 1172 | 32 | 4 | 4 |
| 711 | 9311714 | 193 | 886 | 16 | 2 | 1 |
| 712 | 9311715 | 190 | 808 | 16 | 3 | 2 |
| 713 | 9311716 | 219 | 1032 | 20 | 5 | 1 |
| 714 | 9311717 | 248 | 1244 | 14 | 7 | 5 |
| 715 | 9311718 | 187 | 826 | 25 | 6 | 2 |
| 716 | 9311719 | 253 | 1124 | 35 | 6 | 3 |
| 717 | 9311720 | 206 | 725 | 13 | 2 | 1 |
| 718 | 9311721 | 255 | 1203 | 14 | 3 | 1 |
| 719 | 9311722 | 216 | 923 | 32 | 6 | 1 |
| 720 | 9311723 | 194 | 948 | 25 | 4 | 2 |
| 721 | 9311724 | 223 | 979 | 27 | 7 | 2 |
| 722 | 9311725 | 173 | 734 | 9 | 2 | 1 |
| 723 | 9311726 | 201 | 870 | 12 | 7 | 2 |
| 724 | 9311727 | 216 | 1008 | 17 | 2 | 3 |
| 725 | 9311728 | 216 | 1036 | 25 | 4 | 5 |
| 726 | 9311729 | 306 | 1600 | 36 | 4 | 2 |
| 727 | 9311730 | 240 | 1078 | 19 | 5 | 1 |
| 728 | 9311731 | 194 | 799 | 17 | 1 | 1 |
| 729 | 9311732 | 251 | 1028 | 14 | 4 | 2 |
| 730 | 9311733 | 239 | 1339 | 26 | 3 | 1 |
| 731 | 9311734 | 173 | 696 | 25 | 1 | 1 |
| 732 | 9311735 | 198 | 803 | 12 | 6 | 1 |
| 733 | 9311736 | 233 | 1079 | 30 | 7 | 2 |
| 734 | 9311737 | 260 | 1241 | 17 | 3 | 3 |
| 735 | 9311738 | 237 | 1164 | 21 | 4 | 1 |
| 736 | 9311739 | 220 | 1101 | 29 | 2 | 1 |
| 737 | 9311740 | 173 | 856 | 21 | 3 | 2 |
| 738 | 9311741 | 232 | 984 | 24 | 8 | 5 |
| 739 | 9311742 | 179 | 790 | 20 | 5 | 2 |
| 740 | 9311743 | 251 | 1626 | 25 | 7 | 2 |
| 741 | 9311744 | 265 | 1283 | 26 | 3 | 2 |
| 742 | 9311745 | 195 | 856 | 17 | 1 | 0 |
| 743 | 9311746 | 391 | 2676 | 28 | 7 | 1 |
| 744 | 9311747 | 203 | 1164 | 18 | 4 | 0 |
| 745 | 9311748 | 223 | 1071 | 21 | 1 | 3 |
| 746 | 9311749 | 227 | 1284 | 30 | 8 | 1 |
| 747 | 9311750 | 172 | 743 | 16 | 2 | 1 |
| 748 | 9311751 | 264 | 1093 | 26 | 11 | 1 |
| 749 | 9311752 | 310 | 1700 | 28 | 4 | 0 |
| 750 | 9311753 | 170 | 843 | 14 | 5 | 0 |
| 751 | 9311754 | 263 | 1332 | 25 | 3 | 1 |
| 752 | 9311755 | 175 | 619 | 18 | 3 | 1 |
| 753 | 9311756 | 201 | 786 | 20 | 5 | 2 |
| 754 | 9311757 | 231 | 978 | 15 | 2 | 2 |
| 755 | 9311758 | 211 | 938 | 23 | 3 | 2 |
| 756 | 9311759 | 228 | 842 | 22 | 4 | 2 |
| 757 | 9311760 | 206 | 858 | 8 | 4 | 1 |
| 758 | 9311761 | 208 | 968 | 25 | 7 | 1 |
| 759 | 9311762 | 236 | 1093 | 27 | 9 | 3 |
| 760 | 9311763 | 236 | 1048 | 21 | 3 | 2 |
| 761 | 9311764 | 189 | 755 | 20 | 6 | 2 |
| 762 | 9311765 | 209 | 1108 | 15 | 3 | 3 |
| 763 | 9311766 | 162 | 509 | 14 | 4 | 1 |
| 764 | 9311767 | 209 | 1176 | 35 | 5 | 1 |
| 765 | 9311768 | 135 | 480 | 12 | 8 | 0 |
| 766 | 9311769 | 187 | 789 | 16 | 5 | 1 |
| 767 | 9311770 | 176 | 854 | 19 | 8 | 2 |
| 768 | 9311771 | 241 | 1006 | 11 | 4 | 2 |
| 769 | 9311772 | 214 | 863 | 14 | 7 | 1 |
| 770 | 9311773 | 192 | 961 | 12 | 6 | 2 |
| 771 | 9311774 | 222 | 725 | 14 | 3 | 1 |
| 772 | 9311775 | 236 | 1176 | 29 | 15 | 1 |
| 773 | 9311776 | 278 | 1310 | 17 | 13 | 2 |
| 774 | 9311777 | 263 | 1406 | 34 | 3 | 0 |
| 775 | 9311778 | 206 | 706 | 13 | 10 | 1 |
| 776 | 9311779 | 204 | 984 | 26 | 2 | 2 |
| 777 | 9311780 | 180 | 696 | 9 | 6 | 2 |
| 778 | 9311781 | 216 | 871 | 15 | 10 | 1 |
| 779 | 9311782 | 159 | 557 | 10 | 2 | 2 |
| 780 | 9311783 | 220 | 775 | 13 | 4 | 1 |
| 781 | 9311784 | 152 | 571 | 10 | 3 | 0 |
| 782 | 9311785 | 213 | 843 | 12 | 3 | 2 |
| 783 | 9311786 | 243 | 1296 | 23 | 3 | 2 |
| 784 | 9311787 | 240 | 999 | 20 | 11 | 2 |
| 785 | 9311788 | 295 | 1592 | 41 | 1 | 2 |
| 786 | 9311789 | 170 | 727 | 11 | 6 | 1 |
| 787 | 9311790 | 261 | 1327 | 18 | 6 | 1 |
| 788 | 9311791 | 224 | 1022 | 7 | 5 | 3 |
| 789 | 9311792 | 246 | 1132 | 23 | 6 | 3 |
| 790 | 9311793 | 163 | 682 | 15 | 3 | 1 |
| 791 | 9311794 | 235 | 1064 | 31 | 2 | 3 |
| 792 | 9311795 | 230 | 896 | 14 | 11 | 2 |
| 793 | 9311796 | 195 | 940 | 10 | 5 | 1 |
| 794 | 9311797 | 179 | 532 | 14 | 4 | 2 |
| 795 | 9311798 | 274 | 1366 | 22 | 6 | 4 |
| 796 | 9311799 | 244 | 1056 | 20 | 6 | 2 |
| 797 | 9311800 | 220 | 974 | 25 | 2 | 2 |
| 798 | 9311801 | 202 | 783 | 16 | 6 | 2 |
| 799 | 9311802 | 212 | 1029 | 16 | 7 | 2 |
| 800 | 9311803 | 268 | 1376 | 32 | 6 | 3 |
| 801 | 9311804 | 246 | 1260 | 22 | 4 | 3 |
| 802 | 9311805 | 209 | 963 | 18 | 8 | 2 |
| 803 | 9311806 | 200 | 1071 | 18 | 6 | 3 |
| 804 | 9311807 | 225 | 1111 | 31 | 4 | 1 |
| 805 | 9311808 | 191 | 840 | 12 | 5 | 0 |
| 806 | 9311809 | 266 | 1179 | 21 | 10 | 1 |
| 807 | 9311810 | 162 | 601 | 15 | 9 | 1 |
| 808 | 9311811 | 191 | 803 | 18 | 2 | 1 |
| 809 | 9311812 | 193 | 714 | 13 | 7 | 1 |
| 810 | 9311813 | 208 | 841 | 12 | 3 | 2 |
| 811 | 9311814 | 268 | 1171 | 16 | 2 | 3 |
| 812 | 9311815 | 232 | 1250 | 31 | 2 | 2 |
| 813 | 9311816 | 205 | 1138 | 24 | 3 | 1 |
| 814 | 9311817 | 250 | 1149 | 19 | 3 | 2 |
| 815 | 9311818 | 239 | 1129 | 20 | 6 | 1 |
| 816 | 9311819 | 184 | 947 | 28 | 3 | 3 |
| 817 | 9311820 | 163 | 604 | 5 | 2 | 1 |
| 818 | 9311821 | 254 | 1044 | 21 | 3 | 3 |
| 819 | 9311822 | 224 | 1052 | 15 | 6 | 1 |
| 820 | 9311823 | 162 | 606 | 13 | 6 | 0 |
| 821 | 9311824 | 243 | 845 | 17 | 6 | 1 |
| 822 | 9311825 | 249 | 1364 | 22 | 5 | 2 |
| 823 | 9311826 | 175 | 845 | 23 | 1 | 0 |
| 824 | 9311827 | 207 | 1076 | 25 | 9 | 0 |
| 825 | 9311828 | 168 | 588 | 15 | 3 | 3 |
| 826 | 9311829 | 190 | 666 | 7 | 6 | 2 |
| 827 | 9311830 | 258 | 1322 | 26 | 1 | 0 |
| 828 | 9311831 | 196 | 867 | 15 | 6 | 1 |
| 829 | 9311832 | 223 | 865 | 25 | 5 | 1 |
| 830 | 9311833 | 240 | 1141 | 28 | 2 | 2 |
| 831 | 9311834 | 222 | 885 | 23 | 6 | 0 |
| 832 | 9311835 | 220 | 931 | 12 | 3 | 0 |
| 833 | 9311836 | 222 | 1047 | 25 | 1 | 4 |
| 834 | 9311837 | 214 | 708 | 15 | 9 | 2 |
| 835 | 9311838 | 155 | 607 | 9 | 5 | 0 |
| 836 | 9311839 | 222 | 964 | 22 | 5 | 2 |
| 837 | 9311840 | 172 | 771 | 9 | 6 | 1 |
| 838 | 9311841 | 232 | 944 | 10 | 11 | 4 |
| 839 | 9311842 | 275 | 1424 | 33 | 1 | 2 |
| 840 | 9311843 | 218 | 943 | 19 | 4 | 3 |
| 841 | 9311844 | 218 | 1050 | 13 | 2 | 1 |
| 842 | 9311845 | 249 | 1129 | 17 | 6 | 3 |
| 843 | 9311846 | 241 | 961 | 15 | 6 | 1 |
| 844 | 9311847 | 184 | 914 | 15 | 3 | 1 |
| 845 | 9311848 | 334 | 2051 | 49 | 2 | 2 |
| 846 | 9311849 | 211 | 898 | 27 | 2 | 4 |
| 847 | 9311850 | 265 | 1465 | 15 | 2 | 3 |
| 848 | 9311851 | 175 | 955 | 14 | 3 | 2 |
| 849 | 9311852 | 228 | 1017 | 26 | 3 | 2 |
| 850 | 9311853 | 218 | 881 | 21 | 1 | 2 |
| 851 | 9311854 | 227 | 1049 | 29 | 5 | 2 |
| 852 | 9311855 | 221 | 875 | 13 | 4 | 1 |
| 853 | 9311856 | 201 | 862 | 14 | 4 | 1 |
| 854 | 9311857 | 243 | 1026 | 23 | 1 | 3 |
| 855 | 9311858 | 172 | 719 | 10 | 5 | 2 |
| 856 | 9311859 | 235 | 781 | 25 | 2 | 3 |
| 857 | 9311860 | 237 | 1319 | 36 | 10 | 2 |
| 858 | 9311861 | 244 | 1100 | 23 | 7 | 1 |
| 859 | 9311862 | 287 | 1363 | 31 | 7 | 1 |
| 860 | 9311863 | 233 | 849 | 18 | 7 | 3 |
| 861 | 9311864 | 287 | 1447 | 25 | 7 | 3 |
| 862 | 9311865 | 169 | 658 | 11 | 2 | 1 |
| 863 | 9311866 | 345 | 1815 | 37 | 3 | 2 |
| 864 | 9311867 | 178 | 726 | 11 | 0 | 2 |
| 865 | 9311868 | 216 | 991 | 24 | 4 | 3 |
| 866 | 9311869 | 240 | 1102 | 23 | 1 | 2 |
| 867 | 9311870 | 197 | 617 | 17 | 9 | 2 |
| 868 | 9311871 | 202 | 775 | 14 | 3 | 2 |
| 869 | 9311872 | 206 | 1263 | 17 | 5 | 1 |
| 870 | 9311873 | 228 | 1116 | 23 | 6 | 2 |
| 871 | 9311874 | 183 | 705 | 13 | 3 | 2 |
| 872 | 9311875 | 244 | 1079 | 29 | 4 | 2 |
| 873 | 9311876 | 202 | 783 | 18 | 4 | 3 |
| 874 | 9311877 | 277 | 1403 | 26 | 0 | 2 |
| 875 | 9311878 | 216 | 940 | 21 | 6 | 3 |
| 876 | 9311879 | 245 | 1207 | 34 | 3 | 1 |
| 877 | 9311880 | 229 | 832 | 12 | 1 | 2 |
| 878 | 9311881 | 211 | 905 | 18 | 4 | 2 |
| 879 | 9311882 | 216 | 928 | 31 | 1 | 3 |
| 880 | 9311883 | 254 | 964 | 16 | 5 | 0 |
| 881 | 9311884 | 209 | 1109 | 26 | 2 | 0 |
| 882 | 9311885 | 215 | 1001 | 34 | 6 | 2 |
| 883 | 9311886 | 224 | 1102 | 20 | 9 | 1 |
| 884 | 9311887 | 232 | 1077 | 21 | 4 | 1 |
| 885 | 9311888 | 201 | 750 | 18 | 1 | 2 |
| 886 | 9311889 | 214 | 948 | 16 | 5 | 2 |
| 887 | 9311890 | 195 | 838 | 24 | 2 | 4 |
| 888 | 9311891 | 269 | 942 | 10 | 8 | 2 |
| 889 | 9311892 | 260 | 1101 | 21 | 1 | 1 |
| 890 | 9311893 | 184 | 977 | 19 | 3 | 0 |
| 891 | 9311894 | 242 | 992 | 21 | 6 | 1 |
| 892 | 9311895 | 185 | 894 | 16 | 11 | 1 |
| 893 | 9311896 | 204 | 1016 | 15 | 4 | 0 |
| 894 | 9311897 | 259 | 1242 | 17 | 3 | 3 |
| 895 | 9311898 | 179 | 627 | 17 | 4 | 3 |
| 896 | 9311899 | 255 | 1334 | 19 | 7 | 1 |
| 897 | 9311900 | 186 | 724 | 16 | 7 | 0 |
| 898 | 9311901 | 230 | 1075 | 23 | 4 | 1 |
| 899 | 9311902 | 207 | 1023 | 21 | 5 | 2 |
| 900 | 9311903 | 238 | 1338 | 29 | 4 | 2 |
| 901 | 9311904 | 183 | 715 | 17 | 1 | 2 |
| 902 | 9311905 | 242 | 990 | 27 | 1 | 2 |
| 903 | 9311906 | 271 | 1407 | 37 | 3 | 2 |
| 904 | 9311907 | 199 | 1064 | 27 | 5 | 0 |
| 905 | 9311908 | 181 | 926 | 20 | 4 | 3 |
| 906 | 9311909 | 202 | 1167 | 20 | 0 | 0 |
| 907 | 9311910 | 293 | 1204 | 31 | 2 | 2 |
| 908 | 9311911 | 204 | 963 | 19 | 8 | 2 |
| 909 | 9311912 | 158 | 727 | 13 | 4 | 1 |
| 910 | 9311913 | 203 | 742 | 19 | 7 | 2 |
| 911 | 9311914 | 221 | 1023 | 18 | 7 | 2 |
| 912 | 9311915 | 175 | 842 | 20 | 4 | 2 |
| 913 | 9311916 | 270 | 1534 | 20 | 3 | 2 |
| 914 | 9311917 | 237 | 1030 | 19 | 1 | 2 |
| 915 | 9311918 | 229 | 946 | 30 | 1 | 1 |
| 916 | 9311919 | 228 | 1048 | 33 | 6 | 2 |
| 917 | 9311920 | 185 | 795 | 27 | 1 | 3 |
| 918 | 9311921 | 229 | 1194 | 30 | 8 | 2 |
| 919 | 9311922 | 223 | 1046 | 22 | 8 | 1 |
| 920 | 9311923 | 218 | 1010 | 26 | 6 | 3 |
| 921 | 9311924 | 194 | 738 | 13 | 4 | 1 |
| 922 | 9311925 | 226 | 743 | 19 | 11 | 1 |
| 923 | 9311926 | 250 | 1001 | 17 | 7 | 3 |
| 924 | 9311927 | 245 | 1419 | 37 | 3 | 2 |
| 925 | 9311928 | 213 | 947 | 21 | 6 | 2 |
| 926 | 9311929 | 235 | 1266 | 22 | 8 | 2 |
| 927 | 9311930 | 229 | 1207 | 18 | 7 | 3 |
| 928 | 9311931 | 169 | 498 | 8 | 8 | 1 |
| 929 | 9311932 | 296 | 1443 | 36 | 7 | 2 |
| 930 | 9311933 | 247 | 937 | 15 | 0 | 1 |
| 931 | 9311934 | 243 | 1110 | 17 | 6 | 4 |
| 932 | 9311935 | 229 | 1129 | 39 | 5 | 4 |
| 933 | 9311936 | 261 | 1525 | 25 | 2 | 1 |
| 934 | 9311937 | 181 | 769 | 11 | 3 | 3 |
| 935 | 9311938 | 209 | 942 | 12 | 3 | 0 |
| 936 | 9311939 | 194 | 843 | 14 | 2 | 0 |
| 937 | 9311940 | 200 | 901 | 25 | 2 | 2 |
| 938 | 9311941 | 185 | 978 | 20 | 6 | 0 |
| 939 | 9311942 | 188 | 750 | 15 | 6 | 2 |
| 940 | 9311943 | 195 | 1075 | 28 | 5 | 4 |
| 941 | 9311944 | 230 | 983 | 17 | 4 | 3 |
| 942 | 9311945 | 213 | 875 | 22 | 3 | 3 |
| 943 | 9311946 | 166 | 569 | 17 | 6 | 1 |
| 944 | 9311947 | 224 | 923 | 21 | 5 | 1 |
| 945 | 9311948 | 225 | 648 | 13 | 6 | 1 |
| 946 | 9311949 | 279 | 1370 | 27 | 4 | 1 |
| 947 | 9311950 | 230 | 740 | 17 | 3 | 4 |
| 948 | 9311951 | 234 | 1194 | 14 | 3 | 3 |
| 949 | 9311952 | 218 | 1002 | 21 | 3 | 1 |
| 950 | 9311953 | 237 | 1177 | 22 | 5 | 2 |
| 951 | 9311954 | 243 | 1208 | 24 | 9 | 2 |
| 952 | 9311955 | 237 | 1183 | 26 | 5 | 1 |
| 953 | 9311956 | 193 | 734 | 15 | 4 | 4 |
| 954 | 9311957 | 199 | 1007 | 29 | 3 | 2 |
| 955 | 9311958 | 180 | 813 | 18 | 4 | 0 |
| 956 | 9311959 | 308 | 1253 | 22 | 6 | 2 |
| 957 | 9311960 | 219 | 985 | 38 | 7 | 1 |
| 958 | 9311961 | 180 | 742 | 31 | 2 | 1 |
| 959 | 9311962 | 200 | 846 | 17 | 4 | 1 |
| 960 | 9311963 | 245 | 1178 | 29 | 8 | 1 |
| 961 | 9311964 | 210 | 976 | 27 | 0 | 2 |
| 962 | 9311965 | 256 | 1172 | 18 | 5 | 4 |
| 963 | 9311966 | 255 | 1202 | 31 | 2 | 2 |
| 964 | 9311967 | 186 | 830 | 17 | 13 | 1 |
| 965 | 9311968 | 192 | 871 | 15 | 8 | 2 |
| 966 | 9311969 | 195 | 702 | 19 | 5 | 0 |
| 967 | 9311970 | 185 | 605 | 13 | 2 | 2 |
| 968 | 9311971 | 214 | 927 | 12 | 3 | 0 |
| 969 | 9311972 | 228 | 1033 | 17 | 5 | 1 |
| 970 | 9311973 | 196 | 636 | 12 | 6 | 2 |
| 971 | 9311974 | 148 | 552 | 8 | 1 | 0 |
| 972 | 9311975 | 199 | 676 | 16 | 4 | 1 |
| 973 | 9311976 | 210 | 817 | 19 | 2 | 0 |
| 974 | 9311977 | 187 | 710 | 17 | 9 | 1 |
| 975 | 9311978 | 240 | 1095 | 19 | 3 | 1 |
| 976 | 9311979 | 245 | 1364 | 40 | 2 | 1 |
| 977 | 9311980 | 215 | 913 | 25 | 8 | 1 |
| 978 | 9311981 | 203 | 879 | 20 | 1 | 2 |
| 979 | 9311982 | 204 | 894 | 27 | 4 | 3 |
| 980 | 9311983 | 247 | 1469 | 20 | 6 | 1 |
| 981 | 9311984 | 179 | 723 | 15 | 6 | 2 |
| 982 | 9311985 | 189 | 718 | 19 | 2 | 2 |
| 983 | 9311986 | 229 | 820 | 20 | 8 | 1 |
| 984 | 9311987 | 287 | 1300 | 29 | 3 | 4 |
| 985 | 9311988 | 201 | 870 | 10 | 2 | 0 |
| 986 | 9311989 | 221 | 1029 | 16 | 4 | 1 |
| 987 | 9311990 | 194 | 978 | 14 | 3 | 3 |
| 988 | 9311991 | 253 | 1272 | 30 | 6 | 3 |
| 989 | 9311992 | 163 | 722 | 9 | 2 | 1 |
| 990 | 9311993 | 252 | 1288 | 35 | 7 | 1 |
| 991 | 9311994 | 289 | 1540 | 44 | 2 | 2 |
| 992 | 9311995 | 195 | 815 | 16 | 4 | 1 |
| 993 | 9311996 | 182 | 631 | 14 | 8 | 3 |
| 994 | 9311997 | 297 | 1565 | 25 | 5 | 5 |
| 995 | 9311998 | 217 | 833 | 24 | 11 | 1 |
| 996 | 9311999 | 211 | 857 | 16 | 3 | 2 |
| 997 | 9312000 | 196 | 804 | 28 | 7 | 1 |
| 998 | 9312001 | 226 | 1077 | 30 | 6 | 2 |
| 999 | 9312002 | 228 | 1142 | 23 | 1 | 3 |

## Throughput

- Turns simulated: 220022 (first play of each game)
- Aggregate: 177.0 turns/s over 1243.4s wall
- Per game thread: 29.6 turns/s

## Determinism

Not checked (run with `--determinism`).

## Panics

None.
