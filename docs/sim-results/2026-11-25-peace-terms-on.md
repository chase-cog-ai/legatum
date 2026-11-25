# Simulation: deals-terms-on (2026-11-25)

1000 games, first seed 9311003, brain `scripted`, turn limit off, safety cap 1200, 8 threads, started 2026-11-25T09:27:43Z. Wall time 4116.4s.

Source checkout at run start: `5a46f3945d02fd23cfa1ff214968e8d92d1763ff`; dirty: no. Map 48×32, 8 initial seats. `first_setup` and exact binary argv are recorded in JSON. Ordinary games use seed + i (wrapping); the strategy tournament schedule is specified below. All games use a deterministic shuffle of the full faction pool, taking eight distinct factions, with any Proxy rotation described below.

Command argv: `["./sim","--players","8","--difficulty","hard","--games","1000","--seed","9311003","--out-dir","out","--threads","8","--deals","--label","deals-terms-on"]`

## Overview

| Metric | Value |
| --- | --- |
| Games (non-panic) | 1000 of 1000 |
| Game length, turns (mean / median / min-max) | 222.4 / 217 / 128-510 |
| Win reason mix | VP hold 99.4%, Capitals 0.6% |
| Median end turn | 217 |
| Ending shares of played games (capitals / VP hold / turn limit / other) | 0.6% / 99.4% / 0.0% / 0.0% |
| Games reaching the World War era (any empire) | 993 of 1000 (99.3%) |
| VP hold length | 10 turn ends (the reaching turn counts) |
| Holds started / broken / completed / unfinished | 1568 / 449 / 999 / 120 |
| Hold break rate | 28.6% of started; 31.0% of resolved (broken / (broken + completed)) |
| Hold wins | 994 |
| First reach to win, turns (mean / median / min-max) | 11.1 / 9 / 9-95 |
| Final hold start to win, turns | 9.0 / 9 / 9-9 |
| Comebacks (winner broke an earlier hold of its own) | 130 |
| Wins by a team another team beat to its target | 94 |

Game length by how the game ended (turns resolved; a stall is a safety-cap stop, never a win):

| Ended by | Games | Length mean / median / min-max |
| --- | --- | --- |
| VP hold | 994 | 222.0 / 217 / 128-510 |
| Capitals | 6 | 288.7 / 286 / 243-381 |

Winner share by faction (wins of that faction's seats / all winning seats):

| Faction | Wins | Share of wins |
| --- | --- | --- |
| Switzerland | 80 | 8.0% |
| India | 81 | 8.1% |
| Byzantium | 87 | 8.7% |
| Germany | 135 | 13.5% |
| Celts | 152 | 15.2% |
| Mongols | 83 | 8.3% |
| Sparta | 102 | 10.2% |
| Athens | 155 | 15.5% |
| Aztecs | 125 | 12.5% |

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
| Switzerland | 879 | 80 | 9.1% |
| India | 905 | 81 | 9.0% |
| Byzantium | 885 | 87 | 9.8% |
| Germany | 900 | 135 | 15.0% |
| Celts | 880 | 152 | 17.3% |
| Mongols | 878 | 83 | 9.5% |
| Sparta | 901 | 102 | 11.3% |
| Athens | 900 | 155 | 17.2% |
| Aztecs | 872 | 125 | 14.3% |

## Win rate by difficulty

All initial seats in non-panicked games, including eliminated empires and cap stops. Determinism replays are not additional samples.

| Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- |
| Hard | 8000 | 1000 | 12.5% |

## Win rate by faction and difficulty

| Faction | Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- | --- |
| Switzerland | Hard | 879 | 80 | 9.1% |
| India | Hard | 905 | 81 | 9.0% |
| Byzantium | Hard | 885 | 87 | 9.8% |
| Germany | Hard | 900 | 135 | 15.0% |
| Celts | Hard | 880 | 152 | 17.3% |
| Mongols | Hard | 878 | 83 | 9.5% |
| Sparta | Hard | 901 | 102 | 11.3% |
| Athens | Hard | 900 | 155 | 17.2% |
| Aztecs | Hard | 872 | 125 | 14.3% |

## Faction goal progress

Per seat: how often the faction's permanent goal was reached before the game ended, and the turn its VP was first awarded (over the seats that reached it). Seats are all initial empires of non-panicked games with telemetry, including eliminated empires and cap stops.

| Faction | Seats | Reached | Goal rate | Min | P10 | Median turn | P90 | Max |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Switzerland | 879 | 182 | 20.7% | 111 | 138 | 163 | 222 | 281 |
| India | 905 | 500 | 55.2% | 44 | 89 | 125 | 201 | 291 |
| Byzantium | 885 | 707 | 79.9% | 69 | 83 | 107 | 150 | 313 |
| Germany | 900 | 349 | 38.8% | 84 | 124 | 148 | 196 | 291 |
| Celts | 880 | 553 | 62.8% | 35 | 163 | 210 | 210 | 232 |
| Mongols | 878 | 131 | 14.9% | 105 | 142 | 193 | 234 | 341 |
| Sparta | 901 | 109 | 12.1% | 114 | 154 | 191 | 249 | 330 |
| Athens | 900 | 293 | 32.6% | 115 | 161 | 219 | 255 | 255 |
| Aztecs | 872 | 298 | 34.2% | 128 | 172 | 225 | 239 | 285 |

## Distributions

| Metric | Min | P10 | Median | P90 | Max |
| --- | --- | --- | --- | --- | --- |
| Observed game length (turns, including cap stops) | 128 | 180 | 217 | 270 | 510 |
| Winning team VP | 6 | 10 | 11 | 13 | 19 |
| First Soldier completed (turn, per empire) | 6 | 8 | 36 | 71 | 310 |
| First Settler completed (turn, per empire) | 6 | 12 | 14 | 25 | 157 |
| First city founded (turn, per empire) | 1 | 1 | 1 | 1 | 1 |
| Second city founded (turn, per empire) | 10 | 16 | 19 | 39 | 206 |

Safety-cap stops are censored observations, not completed game lengths.

Openings: 8000 observed empires in 1000 first plays; 0 games without opening telemetry excluded. Turn distributions include only empires that reached the milestone. Never means not reached by the end of observed play, including eliminated empires and safety-cap stops. Per-empire turns are in JSON (`games[].telemetry.openings`, keyed by player ID).

| Opening milestone | Reached empires | Never empires |
| --- | --- | --- |
| First Soldier completed | 7906 | 94 |
| First Settler completed | 7985 | 15 |
| First city founded | 8000 | 0 |
| Second city founded | 7919 | 81 |

Final VP per empire (all seats):

| VP | Empires |
| --- | --- |
| 0 | 1491 |
| 1 | 1315 |
| 2 | 1106 |
| 3 | 777 |
| 4 | 603 |
| 5 | 490 |
| 6 | 460 |
| 7 | 285 |
| 8 | 214 |
| 9 | 138 |
| 10 | 500 |
| 11 | 314 |
| 12 | 148 |
| 13 | 81 |
| 14 | 38 |
| 15 | 28 |
| 16 | 7 |
| 17 | 2 |
| 18 | 1 |
| 19 | 2 |

## End state

- Average game length: 222.4 turns
- Average cities per empire: 4.73
- Average techs per empire: 32.77
- Average eliminations per game: 1.64

## Strategy diversity: observed behavior (classifier v2)

1000 finished games; 8000 player seats; 1000 winning seats. Excluded 0 unfinished/panicked games and 0 games with missing labels. Winner entropy: **0.2975** (normalized over 6 categories, including zero-win categories).

Thresholds, biases and denominators: [strategy-diversity.md](../strategy-diversity.md).

| Archetype | Players | Wins | Win rate | All-player frequency | Winner frequency |
| --- | ---: | ---: | ---: | ---: | ---: |
| Rush | 1742 | 57 | 3.3% | 21.8% | 5.7% |
| Wide | 3764 | 855 | 22.7% | 47.0% | 85.5% |
| Science | 0 | 0 | n/a | 0.0% | 0.0% |
| Turtle/Wonder | 26 | 0 | 0.0% | 0.3% | 0.0% |
| Economic | 1229 | 82 | 6.7% | 15.4% | 8.2% |
| Generalist | 1239 | 6 | 0.5% | 15.5% | 0.6% |

### Matchup matrix

Cell = strict ahead % / tied % (co-occurrence games). Each game has equal weight; all cross-team seat pairs within it share that weight. Rank: winning team first, then surviving empires by VP, then eliminated empires by later elimination turn. Equal ranks tie; no ID tiebreak. This is a multiplayer association, not a duel win rate.

| Row vs column | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Rush | — | 15.8% / 4.6% (847) | n/a / n/a (0) | 42.4% / 19.2% (21) | 17.3% / 6.7% (614) | 59.4% / 7.0% (634) |
| Wide | 79.6% / 4.6% (847) | — | n/a / n/a (0) | 87.4% / 2.8% (24) | 59.7% / 6.8% (741) | 87.7% / 3.1% (762) |
| Science | n/a / n/a (0) | n/a / n/a (0) | — | n/a / n/a (0) | n/a / n/a (0) | n/a / n/a (0) |
| Turtle/Wonder | 38.4% / 19.2% (21) | 9.9% / 2.8% (24) | n/a / n/a (0) | — | 21.9% / 15.6% (16) | 61.9% / 9.5% (21) |
| Economic | 76.0% / 6.7% (614) | 33.5% / 6.8% (741) | n/a / n/a (0) | 62.5% / 15.6% (16) | — | 85.4% / 6.8% (546) |
| Generalist | 33.6% / 7.0% (634) | 9.2% / 3.1% (762) | n/a / n/a (0) | 28.6% / 9.5% (21) | 7.8% / 6.8% (546) | — |

### Faction wins by archetype

Cell = wins / seats; win rate in JSON.

| Faction | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Switzerland | 2 / 93 | 71 / 435 | 0 / 0 | 0 / 1 | 7 / 217 | 0 / 133 |
| India | 3 / 210 | 77 / 575 | 0 / 0 | 0 / 0 | 1 / 15 | 0 / 105 |
| Byzantium | 0 / 6 | 76 / 488 | 0 / 0 | 0 / 5 | 11 / 159 | 0 / 227 |
| Germany | 5 / 146 | 124 / 462 | 0 / 0 | 0 / 9 | 6 / 91 | 0 / 192 |
| Celts | 0 / 91 | 145 / 534 | 0 / 0 | 0 / 0 | 7 / 142 | 0 / 113 |
| Mongols | 23 / 533 | 57 / 212 | 0 / 0 | 0 / 0 | 3 / 47 | 0 / 86 |
| Sparta | 16 / 434 | 60 / 179 | 0 / 0 | 0 / 11 | 22 / 168 | 4 / 109 |
| Athens | 4 / 96 | 134 / 441 | 0 / 0 | 0 / 0 | 15 / 192 | 2 / 171 |
| Aztecs | 4 / 133 | 111 / 438 | 0 / 0 | 0 / 0 | 10 / 198 | 0 / 103 |

Guardrail: **Fail**. Entropy minimum 0.80; at least 20 games per strategy pair. Dominant by observed score: none. Without an observed counter (all pairs covered): none. Sampled but never won: Turtle/Wonder. Unobserved: Science. Under-sampled pairs: 6.

- insufficient evidence: 6 under-sampled matchups; 1000 winning seats

- winner entropy below 0.80

## Battle and capture telemetry

1000 observed first plays; 0 records without telemetry excluded.

| Event | Total / observed games | Mean per game | Games with event / observed games |
| --- | --- | --- | --- |
| War declarations | 14568/1000 | 14.568 | 1000/1000 (100.0%) |
| Peace offers | 93943/1000 | 93.943 | 1000/1000 (100.0%) |
| Peace agreements | 12316/1000 | 12.316 | 1000/1000 (100.0%) |
| Battles | 1008612/1000 | 1008.612 | 1000/1000 (100.0%) |
| City captures | 21211/1000 | 21.211 | 1000/1000 (100.0%) |
| Settler captures | 4964/1000 | 4.964 | 984/1000 (98.4%) |
| Eliminations | 1644/1000 | 1.644 | 863/1000 (86.3%) |

## Terminal VP sources

1000 finished games. Columns are separate populations; totals across columns must not be added. Every cell is VP / population observations.

| Source | All players | All teams | Winning teams |
| --- | --- | --- | --- |
| Held Great Works | 7758/8000 | 7758/8000 | 3054/1000 |
| Faction goals | 5114/8000 | 5114/8000 | 2056/1000 |
| Era objectives | 15506/8000 | 15506/8000 | 4548/1000 |
| Capitals taken | 2488/8000 | 2488/8000 | 1474/1000 |
| Total VP | 30866/8000 | 30866/8000 | 11132/1000 |

Reconciliation failures: 0 player observations, 0 team observations.

## Turn-50 team leaders

| Cohort | Eventual leader-set wins / eligible games | Draws (included) |
| --- | --- | --- |
| Unique leader | 64/397 (16.1%) | 0 |
| Tied leaders (winner in set) | 599/603 (99.3%) | 0 |
| Any leader set | 663/1000 (66.3%) | 0 |

Excluded: 0 games ended before turn 50, 0 unfinished games, 0 snapshots without teams, 0 records without telemetry.

## Era pacing

Resolved turn each empire entered each era. `Winner vs median` is the winning team's seats' entry turn minus the median entry of the empires that entered it in the same finished game (negative: winners got there first).

| Era | Entered / seats | Fastest | p10 | Median | p90 | Slowest | Winner vs median (mean, n) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Classical | 7902/8000 | 5 | 20 | 28 | 32 | 60 | -2.6 (1000) |
| Medieval | 7820/8000 | 35 | 56 | 60 | 63 | 129 | -1.6 (1000) |
| Renaissance | 7764/8000 | 63 | 81 | 86 | 93 | 188 | -2.6 (1000) |
| Industrial | 7577/8000 | 87 | 107 | 114 | 126 | 228 | -4.4 (1000) |
| Machine Age | 7036/8000 | 114 | 135 | 145 | 161 | 273 | -6.4 (1000) |
| World War | 5759/8000 | 134 | 160 | 173 | 192 | 277 | -9.1 (991) |
| Modern | 3568/8000 | 153 | 184 | 202 | 224 | 337 | -9.3 (840) |
| Future | 1593/8000 | 172 | 209 | 236 | 256 | 414 | -7.5 (487) |

Research idle turns (techs left, no research set): 4, of which stalled (two or more in a row): 0.

## Tech pace against the calendar

Median resolved turn each technology was learned, the display year of that turn (`calendar::calendar_label`) and the year the technology really arrived (n/a when no single year is established). Empires that never learned it are left out.

| Era | Technology | Learned / seats | Median turn | Median year | Real year |
| --- | --- | --- | --- | --- | --- |
| Ancient | Irrigation | 8000/8000 | 9 | 3173 BC | 6000 BC |
| Ancient | Bronze Working | 7984/8000 | 17 | 2345 BC | 3300 BC |
| Ancient | Masonry | 7998/8000 | 12 | 2863 BC | 3000 BC |
| Ancient | Currency | 7928/8000 | 27 | 1311 BC | 600 BC |
| Ancient | Writing | 7939/8000 | 25 | 1518 BC | 3200 BC |
| Classical | Mathematics | 7837/8000 | 43 | 351 BC | 300 BC |
| Classical | Engineering | 7828/8000 | 49 | 51 BC | n/a |
| Classical | Iron Working | 7852/8000 | 37 | 651 BC | 1200 BC |
| Classical | Code of Laws | 7822/8000 | 57 | 350 AD | 1754 BC |
| Classical | Philosophy | 7821/8000 | 58 | 400 AD | 500 BC |
| Medieval | Feudalism | 7794/8000 | 73 | 890 AD | n/a |
| Medieval | Machinery | 7810/8000 | 67 | 710 AD | n/a |
| Medieval | Chivalry | 6919/8000 | 77 | 1010 AD | n/a |
| Medieval | Education | 7776/8000 | 82 | 1160 AD | 1088 AD |
| Medieval | Crop Rotation | 7765/8000 | 85 | 1250 AD | n/a |
| Renaissance | Gunpowder | 7739/8000 | 92 | 1420 AD | n/a |
| Renaissance | Metallurgy | 7693/8000 | 102 | 1520 AD | n/a |
| Renaissance | Military Engineering | 7710/8000 | 100 | 1500 AD | n/a |
| Renaissance | Banking | 7625/8000 | 111 | 1610 AD | 1397 AD |
| Renaissance | Printing Press | 7586/8000 | 112 | 1620 AD | 1440 AD |
| Industrial | Rifling | 7250/8000 | 135 | 1787 AD | n/a |
| Industrial | Military Science | 7406/8000 | 127 | 1757 AD | n/a |
| Industrial | Ballistics | 7275/8000 | 137 | 1794 AD | n/a |
| Industrial | Industrialization | 7483/8000 | 122 | 1720 AD | 1769 AD |
| Industrial | Railways | 7042/8000 | 145 | 1824 AD | 1825 AD |
| Machine Age | Replaceable Parts | 6528/8000 | 159 | 1876 AD | 1800 AD |
| Machine Age | Combustion | 6783/8000 | 153 | 1854 AD | 1876 AD |
| Machine Age | Steel | 6476/8000 | 158 | 1872 AD | 1856 AD |
| Machine Age | Flight | 6077/8000 | 168 | 1889 AD | 1903 AD |
| Machine Age | Electricity | 5805/8000 | 173 | 1894 AD | 1879 AD |
| World War | Mass Mobilisation | 4670/8000 | 188 | 1911 AD | n/a |
| World War | Rocketry | 4322/8000 | 192 | 1916 AD | 1926 AD |
| World War | Armoured Warfare | 4055/8000 | 196 | 1922 AD | 1916 AD |
| World War | Air Power | 3843/8000 | 200 | 1928 AD | n/a |
| World War | Electronics | 5313/8000 | 180 | 1902 AD | n/a |
| Modern | Mechanised Warfare | 2404/8000 | 217 | 1954 AD | n/a |
| Modern | Composite Armour | 2109/8000 | 222 | 1966 AD | n/a |
| Modern | Guided Missiles | 1953/8000 | 228 | 1981 AD | n/a |
| Modern | Jet Engines | 1871/8000 | 231 | 1988 AD | 1939 AD |
| Modern | Green Revolution | 2976/8000 | 210 | 1942 AD | n/a |
| Future | Autonomous Systems | 920/8000 | 252 | 2028 AD | n/a |
| Future | Robotics | 695/8000 | 256 | 2032 AD | n/a |
| Future | Fusion Power | 767/8000 | 256 | 2032 AD | n/a |
| Future | Stealth Technology | 1145/8000 | 245 | 2018 AD | n/a |
| Future | Recursive AI | 673/8000 | 256 | 2032 AD | n/a |

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
| 0 | 0.00 | 74.51 |
| 1 | 58.00 | 16.51 |
| 2 | 18.36 | 4.86 |
| 3 | 4.38 | 1.24 |
| 4 | 0.82 | 0.24 |
| 5 | 0.16 | 0.06 |

## Per-game events

Each row is one observed first play; unavailable telemetry is shown as n/a. Full per-player/team score sources and turn-50 standings are in JSON.

| Game | Seed | Resolved turns | Battles | City captures | Settler captures | Eliminations |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 9311003 | 221 | 860 | 17 | 5 | 1 |
| 1 | 9311004 | 229 | 729 | 13 | 3 | 0 |
| 2 | 9311005 | 228 | 1110 | 26 | 8 | 3 |
| 3 | 9311006 | 259 | 1409 | 25 | 6 | 0 |
| 4 | 9311007 | 224 | 762 | 23 | 9 | 3 |
| 5 | 9311008 | 252 | 1328 | 41 | 4 | 1 |
| 6 | 9311009 | 211 | 991 | 21 | 5 | 2 |
| 7 | 9311010 | 193 | 847 | 20 | 9 | 1 |
| 8 | 9311011 | 257 | 900 | 10 | 10 | 3 |
| 9 | 9311012 | 225 | 1194 | 27 | 4 | 4 |
| 10 | 9311013 | 192 | 995 | 24 | 8 | 2 |
| 11 | 9311014 | 234 | 1380 | 26 | 4 | 2 |
| 12 | 9311015 | 238 | 957 | 24 | 10 | 2 |
| 13 | 9311016 | 257 | 1434 | 28 | 9 | 1 |
| 14 | 9311017 | 192 | 900 | 22 | 2 | 2 |
| 15 | 9311018 | 163 | 500 | 12 | 10 | 0 |
| 16 | 9311019 | 263 | 1107 | 29 | 9 | 4 |
| 17 | 9311020 | 244 | 1027 | 28 | 11 | 1 |
| 18 | 9311021 | 178 | 616 | 12 | 7 | 2 |
| 19 | 9311022 | 186 | 750 | 11 | 2 | 0 |
| 20 | 9311023 | 247 | 1290 | 21 | 7 | 0 |
| 21 | 9311024 | 362 | 1947 | 43 | 9 | 2 |
| 22 | 9311025 | 219 | 954 | 14 | 7 | 1 |
| 23 | 9311026 | 224 | 1044 | 24 | 2 | 3 |
| 24 | 9311027 | 272 | 1238 | 20 | 4 | 1 |
| 25 | 9311028 | 265 | 1294 | 30 | 7 | 2 |
| 26 | 9311029 | 214 | 751 | 23 | 7 | 2 |
| 27 | 9311030 | 177 | 487 | 9 | 9 | 0 |
| 28 | 9311031 | 304 | 1426 | 29 | 5 | 0 |
| 29 | 9311032 | 222 | 1021 | 21 | 7 | 1 |
| 30 | 9311033 | 254 | 1331 | 31 | 7 | 2 |
| 31 | 9311034 | 198 | 1031 | 21 | 6 | 0 |
| 32 | 9311035 | 202 | 968 | 25 | 4 | 3 |
| 33 | 9311036 | 202 | 875 | 21 | 7 | 1 |
| 34 | 9311037 | 198 | 784 | 13 | 3 | 1 |
| 35 | 9311038 | 278 | 1631 | 37 | 4 | 1 |
| 36 | 9311039 | 208 | 1163 | 25 | 5 | 2 |
| 37 | 9311040 | 203 | 1066 | 24 | 3 | 0 |
| 38 | 9311041 | 195 | 913 | 20 | 4 | 0 |
| 39 | 9311042 | 200 | 755 | 18 | 2 | 1 |
| 40 | 9311043 | 239 | 1050 | 21 | 2 | 2 |
| 41 | 9311044 | 294 | 1288 | 14 | 7 | 1 |
| 42 | 9311045 | 190 | 958 | 22 | 4 | 1 |
| 43 | 9311046 | 235 | 1009 | 16 | 5 | 1 |
| 44 | 9311047 | 251 | 1183 | 28 | 12 | 3 |
| 45 | 9311048 | 174 | 574 | 12 | 5 | 1 |
| 46 | 9311049 | 224 | 1174 | 17 | 3 | 1 |
| 47 | 9311050 | 212 | 950 | 22 | 4 | 2 |
| 48 | 9311051 | 208 | 1017 | 24 | 5 | 4 |
| 49 | 9311052 | 181 | 852 | 12 | 6 | 0 |
| 50 | 9311053 | 170 | 835 | 20 | 6 | 0 |
| 51 | 9311054 | 172 | 532 | 13 | 5 | 1 |
| 52 | 9311055 | 285 | 1471 | 21 | 4 | 1 |
| 53 | 9311056 | 233 | 1278 | 27 | 2 | 1 |
| 54 | 9311057 | 190 | 771 | 21 | 5 | 1 |
| 55 | 9311058 | 186 | 768 | 23 | 7 | 1 |
| 56 | 9311059 | 230 | 948 | 23 | 4 | 2 |
| 57 | 9311060 | 233 | 950 | 19 | 7 | 1 |
| 58 | 9311061 | 197 | 912 | 24 | 1 | 1 |
| 59 | 9311062 | 197 | 956 | 18 | 2 | 1 |
| 60 | 9311063 | 180 | 590 | 12 | 4 | 0 |
| 61 | 9311064 | 235 | 1066 | 19 | 2 | 3 |
| 62 | 9311065 | 211 | 940 | 18 | 4 | 2 |
| 63 | 9311066 | 230 | 1198 | 24 | 6 | 2 |
| 64 | 9311067 | 202 | 868 | 28 | 3 | 2 |
| 65 | 9311068 | 192 | 786 | 20 | 7 | 1 |
| 66 | 9311069 | 252 | 1308 | 39 | 4 | 2 |
| 67 | 9311070 | 224 | 1142 | 29 | 6 | 4 |
| 68 | 9311071 | 269 | 1475 | 26 | 2 | 1 |
| 69 | 9311072 | 182 | 775 | 17 | 7 | 1 |
| 70 | 9311073 | 209 | 920 | 19 | 2 | 2 |
| 71 | 9311074 | 225 | 1020 | 25 | 8 | 2 |
| 72 | 9311075 | 316 | 1793 | 35 | 18 | 2 |
| 73 | 9311076 | 234 | 1055 | 28 | 2 | 4 |
| 74 | 9311077 | 201 | 1099 | 15 | 10 | 0 |
| 75 | 9311078 | 206 | 768 | 14 | 6 | 2 |
| 76 | 9311079 | 224 | 1117 | 12 | 3 | 1 |
| 77 | 9311080 | 173 | 642 | 12 | 3 | 0 |
| 78 | 9311081 | 210 | 788 | 18 | 6 | 2 |
| 79 | 9311082 | 253 | 1141 | 24 | 6 | 2 |
| 80 | 9311083 | 240 | 1014 | 22 | 7 | 2 |
| 81 | 9311084 | 207 | 1221 | 20 | 6 | 3 |
| 82 | 9311085 | 188 | 860 | 26 | 3 | 2 |
| 83 | 9311086 | 224 | 997 | 16 | 9 | 2 |
| 84 | 9311087 | 178 | 533 | 13 | 8 | 1 |
| 85 | 9311088 | 244 | 1046 | 21 | 2 | 1 |
| 86 | 9311089 | 187 | 776 | 22 | 2 | 2 |
| 87 | 9311090 | 178 | 781 | 11 | 7 | 3 |
| 88 | 9311091 | 269 | 1478 | 34 | 6 | 2 |
| 89 | 9311092 | 216 | 974 | 18 | 10 | 0 |
| 90 | 9311093 | 226 | 1104 | 15 | 17 | 2 |
| 91 | 9311094 | 204 | 917 | 26 | 4 | 2 |
| 92 | 9311095 | 253 | 1235 | 25 | 6 | 2 |
| 93 | 9311096 | 221 | 1002 | 26 | 8 | 3 |
| 94 | 9311097 | 173 | 748 | 17 | 0 | 1 |
| 95 | 9311098 | 237 | 1083 | 22 | 10 | 1 |
| 96 | 9311099 | 204 | 880 | 19 | 2 | 0 |
| 97 | 9311100 | 240 | 1103 | 26 | 7 | 2 |
| 98 | 9311101 | 261 | 1117 | 26 | 2 | 1 |
| 99 | 9311102 | 224 | 1006 | 17 | 9 | 3 |
| 100 | 9311103 | 295 | 1341 | 48 | 6 | 2 |
| 101 | 9311104 | 288 | 1853 | 31 | 4 | 3 |
| 102 | 9311105 | 208 | 1189 | 26 | 7 | 2 |
| 103 | 9311106 | 179 | 642 | 13 | 3 | 2 |
| 104 | 9311107 | 185 | 958 | 25 | 1 | 1 |
| 105 | 9311108 | 199 | 697 | 13 | 5 | 0 |
| 106 | 9311109 | 232 | 1071 | 23 | 3 | 5 |
| 107 | 9311110 | 191 | 758 | 23 | 5 | 1 |
| 108 | 9311111 | 265 | 1268 | 33 | 6 | 2 |
| 109 | 9311112 | 244 | 1213 | 29 | 2 | 2 |
| 110 | 9311113 | 261 | 1458 | 33 | 5 | 1 |
| 111 | 9311114 | 250 | 1087 | 20 | 2 | 3 |
| 112 | 9311115 | 167 | 723 | 12 | 5 | 2 |
| 113 | 9311116 | 202 | 1053 | 26 | 4 | 3 |
| 114 | 9311117 | 177 | 741 | 14 | 2 | 1 |
| 115 | 9311118 | 199 | 845 | 9 | 10 | 2 |
| 116 | 9311119 | 238 | 819 | 17 | 4 | 1 |
| 117 | 9311120 | 201 | 824 | 18 | 6 | 2 |
| 118 | 9311121 | 179 | 872 | 14 | 9 | 2 |
| 119 | 9311122 | 198 | 844 | 17 | 6 | 2 |
| 120 | 9311123 | 240 | 1153 | 17 | 1 | 2 |
| 121 | 9311124 | 203 | 801 | 19 | 7 | 1 |
| 122 | 9311125 | 250 | 753 | 15 | 5 | 1 |
| 123 | 9311126 | 212 | 891 | 21 | 11 | 1 |
| 124 | 9311127 | 250 | 1107 | 28 | 3 | 1 |
| 125 | 9311128 | 187 | 828 | 8 | 5 | 1 |
| 126 | 9311129 | 217 | 899 | 22 | 3 | 2 |
| 127 | 9311130 | 252 | 1179 | 24 | 9 | 0 |
| 128 | 9311131 | 149 | 613 | 7 | 6 | 0 |
| 129 | 9311132 | 157 | 621 | 11 | 4 | 1 |
| 130 | 9311133 | 171 | 779 | 20 | 4 | 1 |
| 131 | 9311134 | 171 | 530 | 8 | 5 | 1 |
| 132 | 9311135 | 200 | 881 | 22 | 6 | 3 |
| 133 | 9311136 | 263 | 1316 | 30 | 2 | 2 |
| 134 | 9311137 | 242 | 976 | 22 | 1 | 1 |
| 135 | 9311138 | 237 | 1127 | 27 | 3 | 0 |
| 136 | 9311139 | 159 | 543 | 9 | 1 | 1 |
| 137 | 9311140 | 281 | 1362 | 30 | 8 | 3 |
| 138 | 9311141 | 225 | 1159 | 22 | 6 | 1 |
| 139 | 9311142 | 263 | 1329 | 21 | 6 | 1 |
| 140 | 9311143 | 266 | 1383 | 21 | 6 | 4 |
| 141 | 9311144 | 227 | 1324 | 21 | 8 | 2 |
| 142 | 9311145 | 209 | 706 | 20 | 4 | 3 |
| 143 | 9311146 | 225 | 870 | 29 | 6 | 2 |
| 144 | 9311147 | 278 | 1572 | 34 | 4 | 0 |
| 145 | 9311148 | 255 | 889 | 25 | 1 | 3 |
| 146 | 9311149 | 191 | 831 | 7 | 7 | 2 |
| 147 | 9311150 | 195 | 847 | 18 | 4 | 0 |
| 148 | 9311151 | 204 | 785 | 21 | 10 | 1 |
| 149 | 9311152 | 199 | 962 | 25 | 6 | 3 |
| 150 | 9311153 | 205 | 949 | 18 | 4 | 3 |
| 151 | 9311154 | 197 | 938 | 19 | 2 | 0 |
| 152 | 9311155 | 222 | 996 | 30 | 8 | 3 |
| 153 | 9311156 | 185 | 806 | 16 | 5 | 0 |
| 154 | 9311157 | 176 | 789 | 7 | 4 | 0 |
| 155 | 9311158 | 244 | 1050 | 13 | 7 | 4 |
| 156 | 9311159 | 254 | 1252 | 25 | 11 | 3 |
| 157 | 9311160 | 217 | 1071 | 31 | 6 | 1 |
| 158 | 9311161 | 254 | 1271 | 26 | 3 | 1 |
| 159 | 9311162 | 225 | 935 | 16 | 7 | 1 |
| 160 | 9311163 | 177 | 644 | 13 | 2 | 0 |
| 161 | 9311164 | 247 | 1384 | 32 | 0 | 2 |
| 162 | 9311165 | 261 | 1468 | 38 | 12 | 2 |
| 163 | 9311166 | 240 | 931 | 17 | 6 | 4 |
| 164 | 9311167 | 224 | 968 | 18 | 6 | 1 |
| 165 | 9311168 | 212 | 994 | 9 | 6 | 2 |
| 166 | 9311169 | 178 | 615 | 15 | 5 | 0 |
| 167 | 9311170 | 246 | 1135 | 24 | 4 | 3 |
| 168 | 9311171 | 206 | 1028 | 26 | 3 | 1 |
| 169 | 9311172 | 304 | 1268 | 21 | 5 | 3 |
| 170 | 9311173 | 213 | 1101 | 19 | 7 | 1 |
| 171 | 9311174 | 215 | 942 | 26 | 3 | 1 |
| 172 | 9311175 | 184 | 684 | 8 | 4 | 1 |
| 173 | 9311176 | 196 | 841 | 17 | 5 | 1 |
| 174 | 9311177 | 252 | 910 | 24 | 0 | 4 |
| 175 | 9311178 | 303 | 1597 | 23 | 1 | 0 |
| 176 | 9311179 | 233 | 1066 | 20 | 7 | 3 |
| 177 | 9311180 | 259 | 1089 | 34 | 2 | 3 |
| 178 | 9311181 | 257 | 1091 | 20 | 3 | 1 |
| 179 | 9311182 | 237 | 1296 | 34 | 3 | 1 |
| 180 | 9311183 | 214 | 984 | 26 | 10 | 1 |
| 181 | 9311184 | 199 | 870 | 23 | 6 | 2 |
| 182 | 9311185 | 222 | 860 | 22 | 6 | 2 |
| 183 | 9311186 | 219 | 850 | 21 | 5 | 1 |
| 184 | 9311187 | 233 | 1030 | 29 | 3 | 2 |
| 185 | 9311188 | 294 | 1341 | 19 | 8 | 3 |
| 186 | 9311189 | 250 | 1053 | 24 | 5 | 1 |
| 187 | 9311190 | 182 | 552 | 11 | 5 | 1 |
| 188 | 9311191 | 174 | 720 | 16 | 3 | 1 |
| 189 | 9311192 | 175 | 623 | 16 | 3 | 0 |
| 190 | 9311193 | 207 | 919 | 19 | 3 | 3 |
| 191 | 9311194 | 270 | 1415 | 17 | 8 | 3 |
| 192 | 9311195 | 198 | 1000 | 19 | 3 | 1 |
| 193 | 9311196 | 219 | 972 | 23 | 5 | 3 |
| 194 | 9311197 | 273 | 1568 | 37 | 6 | 3 |
| 195 | 9311198 | 212 | 1004 | 21 | 9 | 2 |
| 196 | 9311199 | 221 | 944 | 20 | 1 | 1 |
| 197 | 9311200 | 261 | 1132 | 28 | 3 | 2 |
| 198 | 9311201 | 217 | 1014 | 22 | 8 | 1 |
| 199 | 9311202 | 201 | 1010 | 15 | 4 | 1 |
| 200 | 9311203 | 240 | 1113 | 16 | 9 | 1 |
| 201 | 9311204 | 180 | 677 | 16 | 4 | 2 |
| 202 | 9311205 | 251 | 1354 | 28 | 2 | 1 |
| 203 | 9311206 | 250 | 989 | 20 | 5 | 1 |
| 204 | 9311207 | 203 | 1142 | 27 | 5 | 1 |
| 205 | 9311208 | 168 | 788 | 15 | 4 | 2 |
| 206 | 9311209 | 192 | 881 | 18 | 3 | 3 |
| 207 | 9311210 | 232 | 993 | 13 | 4 | 2 |
| 208 | 9311211 | 217 | 1166 | 26 | 2 | 1 |
| 209 | 9311212 | 153 | 517 | 9 | 6 | 1 |
| 210 | 9311213 | 219 | 881 | 17 | 5 | 2 |
| 211 | 9311214 | 188 | 698 | 12 | 5 | 1 |
| 212 | 9311215 | 321 | 1877 | 32 | 5 | 1 |
| 213 | 9311216 | 219 | 1029 | 21 | 1 | 0 |
| 214 | 9311217 | 208 | 1174 | 27 | 4 | 1 |
| 215 | 9311218 | 199 | 788 | 22 | 3 | 2 |
| 216 | 9311219 | 161 | 441 | 14 | 3 | 0 |
| 217 | 9311220 | 191 | 808 | 12 | 6 | 1 |
| 218 | 9311221 | 213 | 1046 | 17 | 6 | 2 |
| 219 | 9311222 | 208 | 940 | 23 | 4 | 0 |
| 220 | 9311223 | 261 | 1392 | 34 | 6 | 2 |
| 221 | 9311224 | 297 | 1454 | 40 | 7 | 2 |
| 222 | 9311225 | 232 | 1256 | 28 | 4 | 0 |
| 223 | 9311226 | 253 | 1256 | 17 | 5 | 1 |
| 224 | 9311227 | 255 | 1084 | 20 | 7 | 2 |
| 225 | 9311228 | 221 | 1073 | 23 | 4 | 1 |
| 226 | 9311229 | 191 | 889 | 15 | 4 | 2 |
| 227 | 9311230 | 203 | 839 | 12 | 3 | 1 |
| 228 | 9311231 | 196 | 764 | 14 | 4 | 0 |
| 229 | 9311232 | 243 | 1355 | 33 | 4 | 1 |
| 230 | 9311233 | 211 | 1114 | 20 | 4 | 2 |
| 231 | 9311234 | 209 | 805 | 19 | 4 | 1 |
| 232 | 9311235 | 206 | 1060 | 21 | 3 | 1 |
| 233 | 9311236 | 198 | 876 | 6 | 3 | 1 |
| 234 | 9311237 | 269 | 1498 | 18 | 3 | 2 |
| 235 | 9311238 | 241 | 663 | 13 | 4 | 2 |
| 236 | 9311239 | 282 | 1284 | 18 | 7 | 0 |
| 237 | 9311240 | 240 | 1121 | 20 | 7 | 1 |
| 238 | 9311241 | 208 | 1007 | 24 | 7 | 2 |
| 239 | 9311242 | 279 | 1087 | 9 | 9 | 2 |
| 240 | 9311243 | 177 | 716 | 19 | 2 | 2 |
| 241 | 9311244 | 267 | 1109 | 21 | 6 | 2 |
| 242 | 9311245 | 177 | 749 | 23 | 4 | 5 |
| 243 | 9311246 | 230 | 1039 | 24 | 5 | 3 |
| 244 | 9311247 | 168 | 635 | 10 | 7 | 1 |
| 245 | 9311248 | 193 | 589 | 14 | 6 | 1 |
| 246 | 9311249 | 240 | 871 | 24 | 5 | 1 |
| 247 | 9311250 | 204 | 768 | 20 | 12 | 2 |
| 248 | 9311251 | 178 | 774 | 10 | 8 | 1 |
| 249 | 9311252 | 282 | 1401 | 29 | 8 | 3 |
| 250 | 9311253 | 182 | 712 | 10 | 2 | 0 |
| 251 | 9311254 | 244 | 1214 | 29 | 4 | 3 |
| 252 | 9311255 | 273 | 1370 | 37 | 5 | 2 |
| 253 | 9311256 | 227 | 1200 | 23 | 2 | 2 |
| 254 | 9311257 | 197 | 681 | 16 | 8 | 2 |
| 255 | 9311258 | 285 | 1400 | 21 | 3 | 3 |
| 256 | 9311259 | 259 | 1141 | 16 | 1 | 0 |
| 257 | 9311260 | 221 | 739 | 18 | 4 | 4 |
| 258 | 9311261 | 232 | 1269 | 29 | 1 | 3 |
| 259 | 9311262 | 203 | 731 | 11 | 3 | 2 |
| 260 | 9311263 | 216 | 905 | 18 | 5 | 2 |
| 261 | 9311264 | 204 | 800 | 23 | 5 | 1 |
| 262 | 9311265 | 219 | 912 | 20 | 6 | 1 |
| 263 | 9311266 | 248 | 645 | 18 | 0 | 0 |
| 264 | 9311267 | 223 | 1092 | 22 | 8 | 3 |
| 265 | 9311268 | 184 | 748 | 4 | 3 | 1 |
| 266 | 9311269 | 201 | 868 | 19 | 2 | 3 |
| 267 | 9311270 | 234 | 1264 | 18 | 4 | 0 |
| 268 | 9311271 | 209 | 657 | 17 | 5 | 1 |
| 269 | 9311272 | 213 | 878 | 16 | 15 | 3 |
| 270 | 9311273 | 180 | 897 | 17 | 4 | 1 |
| 271 | 9311274 | 264 | 1223 | 21 | 5 | 1 |
| 272 | 9311275 | 176 | 506 | 18 | 2 | 1 |
| 273 | 9311276 | 209 | 812 | 17 | 6 | 1 |
| 274 | 9311277 | 223 | 1100 | 23 | 2 | 1 |
| 275 | 9311278 | 173 | 759 | 13 | 10 | 2 |
| 276 | 9311279 | 190 | 703 | 15 | 5 | 1 |
| 277 | 9311280 | 289 | 1422 | 35 | 5 | 2 |
| 278 | 9311281 | 226 | 912 | 18 | 1 | 1 |
| 279 | 9311282 | 184 | 659 | 18 | 3 | 1 |
| 280 | 9311283 | 179 | 684 | 7 | 2 | 1 |
| 281 | 9311284 | 156 | 453 | 12 | 3 | 2 |
| 282 | 9311285 | 290 | 1488 | 41 | 4 | 3 |
| 283 | 9311286 | 213 | 1075 | 29 | 5 | 3 |
| 284 | 9311287 | 214 | 1087 | 17 | 2 | 1 |
| 285 | 9311288 | 246 | 1218 | 22 | 4 | 2 |
| 286 | 9311289 | 223 | 1053 | 11 | 9 | 2 |
| 287 | 9311290 | 242 | 1148 | 37 | 7 | 2 |
| 288 | 9311291 | 188 | 658 | 10 | 4 | 0 |
| 289 | 9311292 | 225 | 1172 | 22 | 1 | 3 |
| 290 | 9311293 | 266 | 1566 | 27 | 6 | 2 |
| 291 | 9311294 | 213 | 783 | 10 | 9 | 2 |
| 292 | 9311295 | 217 | 967 | 15 | 6 | 2 |
| 293 | 9311296 | 252 | 970 | 18 | 6 | 3 |
| 294 | 9311297 | 201 | 833 | 26 | 1 | 2 |
| 295 | 9311298 | 187 | 693 | 12 | 3 | 2 |
| 296 | 9311299 | 264 | 1045 | 26 | 6 | 2 |
| 297 | 9311300 | 259 | 1222 | 22 | 8 | 0 |
| 298 | 9311301 | 191 | 739 | 13 | 5 | 0 |
| 299 | 9311302 | 231 | 1037 | 21 | 5 | 2 |
| 300 | 9311303 | 266 | 1514 | 15 | 9 | 1 |
| 301 | 9311304 | 232 | 719 | 12 | 4 | 2 |
| 302 | 9311305 | 234 | 991 | 25 | 5 | 1 |
| 303 | 9311306 | 216 | 888 | 20 | 5 | 1 |
| 304 | 9311307 | 207 | 858 | 25 | 3 | 1 |
| 305 | 9311308 | 284 | 1276 | 20 | 10 | 2 |
| 306 | 9311309 | 232 | 1073 | 39 | 7 | 1 |
| 307 | 9311310 | 185 | 920 | 13 | 0 | 1 |
| 308 | 9311311 | 199 | 1215 | 22 | 5 | 2 |
| 309 | 9311312 | 194 | 900 | 10 | 6 | 1 |
| 310 | 9311313 | 225 | 1025 | 19 | 9 | 2 |
| 311 | 9311314 | 217 | 951 | 19 | 9 | 1 |
| 312 | 9311315 | 202 | 1006 | 15 | 9 | 2 |
| 313 | 9311316 | 195 | 876 | 29 | 1 | 3 |
| 314 | 9311317 | 224 | 843 | 23 | 2 | 0 |
| 315 | 9311318 | 234 | 1346 | 32 | 7 | 1 |
| 316 | 9311319 | 234 | 1004 | 27 | 6 | 3 |
| 317 | 9311320 | 247 | 1204 | 23 | 8 | 0 |
| 318 | 9311321 | 176 | 694 | 11 | 1 | 1 |
| 319 | 9311322 | 180 | 738 | 9 | 10 | 3 |
| 320 | 9311323 | 227 | 1149 | 31 | 5 | 3 |
| 321 | 9311324 | 174 | 773 | 10 | 5 | 1 |
| 322 | 9311325 | 190 | 701 | 14 | 3 | 3 |
| 323 | 9311326 | 276 | 1625 | 21 | 4 | 2 |
| 324 | 9311327 | 198 | 814 | 22 | 3 | 1 |
| 325 | 9311328 | 287 | 1552 | 26 | 7 | 1 |
| 326 | 9311329 | 195 | 575 | 15 | 3 | 0 |
| 327 | 9311330 | 261 | 1057 | 21 | 8 | 1 |
| 328 | 9311331 | 197 | 898 | 16 | 4 | 1 |
| 329 | 9311332 | 303 | 1350 | 35 | 8 | 1 |
| 330 | 9311333 | 216 | 877 | 14 | 6 | 1 |
| 331 | 9311334 | 147 | 459 | 19 | 1 | 2 |
| 332 | 9311335 | 231 | 955 | 18 | 10 | 2 |
| 333 | 9311336 | 225 | 1189 | 19 | 11 | 3 |
| 334 | 9311337 | 208 | 894 | 19 | 4 | 1 |
| 335 | 9311338 | 242 | 1463 | 27 | 8 | 2 |
| 336 | 9311339 | 209 | 1015 | 17 | 7 | 4 |
| 337 | 9311340 | 346 | 1583 | 32 | 8 | 1 |
| 338 | 9311341 | 234 | 1116 | 17 | 8 | 2 |
| 339 | 9311342 | 261 | 1190 | 27 | 3 | 2 |
| 340 | 9311343 | 225 | 920 | 16 | 3 | 0 |
| 341 | 9311344 | 181 | 605 | 14 | 11 | 2 |
| 342 | 9311345 | 256 | 1478 | 34 | 3 | 0 |
| 343 | 9311346 | 207 | 1042 | 14 | 14 | 3 |
| 344 | 9311347 | 273 | 1445 | 25 | 4 | 2 |
| 345 | 9311348 | 193 | 807 | 26 | 2 | 1 |
| 346 | 9311349 | 208 | 829 | 20 | 6 | 1 |
| 347 | 9311350 | 235 | 1321 | 24 | 2 | 2 |
| 348 | 9311351 | 242 | 940 | 23 | 4 | 3 |
| 349 | 9311352 | 400 | 2130 | 45 | 13 | 3 |
| 350 | 9311353 | 208 | 982 | 17 | 2 | 0 |
| 351 | 9311354 | 194 | 785 | 26 | 5 | 2 |
| 352 | 9311355 | 198 | 924 | 20 | 3 | 1 |
| 353 | 9311356 | 197 | 880 | 18 | 7 | 0 |
| 354 | 9311357 | 204 | 982 | 27 | 6 | 2 |
| 355 | 9311358 | 194 | 855 | 18 | 5 | 2 |
| 356 | 9311359 | 205 | 692 | 14 | 2 | 4 |
| 357 | 9311360 | 218 | 1009 | 12 | 6 | 1 |
| 358 | 9311361 | 261 | 1341 | 21 | 3 | 1 |
| 359 | 9311362 | 215 | 1067 | 20 | 11 | 2 |
| 360 | 9311363 | 241 | 1352 | 20 | 6 | 2 |
| 361 | 9311364 | 211 | 811 | 16 | 7 | 3 |
| 362 | 9311365 | 201 | 881 | 21 | 9 | 2 |
| 363 | 9311366 | 302 | 1367 | 38 | 7 | 2 |
| 364 | 9311367 | 196 | 891 | 8 | 4 | 1 |
| 365 | 9311368 | 188 | 861 | 18 | 1 | 2 |
| 366 | 9311369 | 190 | 740 | 19 | 7 | 0 |
| 367 | 9311370 | 510 | 2819 | 59 | 4 | 2 |
| 368 | 9311371 | 197 | 1079 | 21 | 5 | 1 |
| 369 | 9311372 | 295 | 1398 | 38 | 5 | 2 |
| 370 | 9311373 | 190 | 679 | 12 | 2 | 1 |
| 371 | 9311374 | 201 | 884 | 25 | 5 | 3 |
| 372 | 9311375 | 223 | 911 | 20 | 7 | 2 |
| 373 | 9311376 | 198 | 901 | 20 | 2 | 3 |
| 374 | 9311377 | 177 | 637 | 11 | 3 | 0 |
| 375 | 9311378 | 287 | 1391 | 41 | 8 | 3 |
| 376 | 9311379 | 208 | 832 | 18 | 2 | 1 |
| 377 | 9311380 | 303 | 1550 | 33 | 1 | 2 |
| 378 | 9311381 | 234 | 1150 | 28 | 6 | 3 |
| 379 | 9311382 | 280 | 1434 | 26 | 7 | 3 |
| 380 | 9311383 | 216 | 777 | 17 | 5 | 3 |
| 381 | 9311384 | 199 | 899 | 29 | 7 | 0 |
| 382 | 9311385 | 257 | 1115 | 24 | 2 | 0 |
| 383 | 9311386 | 154 | 600 | 14 | 1 | 1 |
| 384 | 9311387 | 214 | 1080 | 24 | 6 | 2 |
| 385 | 9311388 | 235 | 1111 | 20 | 12 | 2 |
| 386 | 9311389 | 279 | 1335 | 24 | 8 | 3 |
| 387 | 9311390 | 204 | 855 | 15 | 6 | 0 |
| 388 | 9311391 | 183 | 790 | 14 | 4 | 1 |
| 389 | 9311392 | 170 | 597 | 14 | 10 | 2 |
| 390 | 9311393 | 185 | 653 | 17 | 3 | 0 |
| 391 | 9311394 | 191 | 622 | 10 | 6 | 1 |
| 392 | 9311395 | 217 | 873 | 28 | 4 | 2 |
| 393 | 9311396 | 221 | 947 | 24 | 4 | 3 |
| 394 | 9311397 | 214 | 988 | 15 | 10 | 1 |
| 395 | 9311398 | 192 | 792 | 20 | 2 | 3 |
| 396 | 9311399 | 227 | 1322 | 25 | 1 | 2 |
| 397 | 9311400 | 188 | 684 | 17 | 5 | 2 |
| 398 | 9311401 | 237 | 1230 | 29 | 5 | 3 |
| 399 | 9311402 | 232 | 1136 | 33 | 9 | 3 |
| 400 | 9311403 | 245 | 1334 | 27 | 5 | 0 |
| 401 | 9311404 | 244 | 1162 | 16 | 2 | 1 |
| 402 | 9311405 | 226 | 989 | 22 | 2 | 3 |
| 403 | 9311406 | 276 | 1359 | 26 | 4 | 3 |
| 404 | 9311407 | 169 | 558 | 11 | 6 | 0 |
| 405 | 9311408 | 222 | 1158 | 19 | 2 | 2 |
| 406 | 9311409 | 212 | 1104 | 15 | 6 | 1 |
| 407 | 9311410 | 185 | 812 | 21 | 6 | 1 |
| 408 | 9311411 | 174 | 571 | 12 | 3 | 1 |
| 409 | 9311412 | 210 | 941 | 21 | 4 | 2 |
| 410 | 9311413 | 203 | 761 | 22 | 1 | 2 |
| 411 | 9311414 | 181 | 857 | 17 | 3 | 1 |
| 412 | 9311415 | 215 | 845 | 23 | 6 | 1 |
| 413 | 9311416 | 195 | 877 | 25 | 7 | 1 |
| 414 | 9311417 | 274 | 1452 | 36 | 5 | 1 |
| 415 | 9311418 | 221 | 1224 | 37 | 6 | 1 |
| 416 | 9311419 | 277 | 1255 | 22 | 7 | 3 |
| 417 | 9311420 | 236 | 1080 | 37 | 5 | 3 |
| 418 | 9311421 | 238 | 1157 | 19 | 6 | 0 |
| 419 | 9311422 | 283 | 1550 | 25 | 6 | 1 |
| 420 | 9311423 | 265 | 1503 | 27 | 3 | 2 |
| 421 | 9311424 | 212 | 1047 | 19 | 3 | 1 |
| 422 | 9311425 | 175 | 559 | 7 | 2 | 1 |
| 423 | 9311426 | 198 | 869 | 19 | 2 | 0 |
| 424 | 9311427 | 224 | 914 | 19 | 4 | 3 |
| 425 | 9311428 | 221 | 801 | 17 | 5 | 1 |
| 426 | 9311429 | 191 | 598 | 19 | 9 | 1 |
| 427 | 9311430 | 294 | 1629 | 27 | 2 | 0 |
| 428 | 9311431 | 197 | 829 | 9 | 6 | 2 |
| 429 | 9311432 | 192 | 1043 | 24 | 3 | 2 |
| 430 | 9311433 | 233 | 1194 | 19 | 9 | 1 |
| 431 | 9311434 | 298 | 1666 | 49 | 5 | 2 |
| 432 | 9311435 | 217 | 840 | 17 | 4 | 1 |
| 433 | 9311436 | 189 | 866 | 16 | 8 | 1 |
| 434 | 9311437 | 201 | 822 | 13 | 5 | 0 |
| 435 | 9311438 | 218 | 790 | 13 | 3 | 0 |
| 436 | 9311439 | 259 | 1312 | 18 | 6 | 2 |
| 437 | 9311440 | 212 | 833 | 11 | 9 | 2 |
| 438 | 9311441 | 191 | 646 | 25 | 8 | 1 |
| 439 | 9311442 | 242 | 1292 | 26 | 4 | 3 |
| 440 | 9311443 | 235 | 1151 | 22 | 4 | 1 |
| 441 | 9311444 | 260 | 1349 | 27 | 8 | 2 |
| 442 | 9311445 | 162 | 778 | 11 | 6 | 3 |
| 443 | 9311446 | 215 | 1035 | 24 | 5 | 1 |
| 444 | 9311447 | 221 | 1101 | 15 | 3 | 2 |
| 445 | 9311448 | 381 | 1798 | 38 | 7 | 4 |
| 446 | 9311449 | 227 | 1247 | 19 | 2 | 2 |
| 447 | 9311450 | 188 | 726 | 17 | 6 | 2 |
| 448 | 9311451 | 163 | 750 | 14 | 3 | 1 |
| 449 | 9311452 | 202 | 1148 | 25 | 9 | 0 |
| 450 | 9311453 | 203 | 877 | 6 | 5 | 1 |
| 451 | 9311454 | 177 | 798 | 13 | 4 | 2 |
| 452 | 9311455 | 204 | 888 | 15 | 3 | 3 |
| 453 | 9311456 | 192 | 929 | 19 | 2 | 3 |
| 454 | 9311457 | 229 | 1261 | 25 | 6 | 3 |
| 455 | 9311458 | 173 | 609 | 13 | 7 | 3 |
| 456 | 9311459 | 224 | 911 | 24 | 8 | 2 |
| 457 | 9311460 | 212 | 1053 | 23 | 6 | 1 |
| 458 | 9311461 | 336 | 1582 | 24 | 4 | 0 |
| 459 | 9311462 | 189 | 898 | 26 | 4 | 0 |
| 460 | 9311463 | 315 | 1593 | 34 | 11 | 3 |
| 461 | 9311464 | 319 | 1626 | 36 | 7 | 1 |
| 462 | 9311465 | 218 | 1217 | 21 | 2 | 3 |
| 463 | 9311466 | 186 | 703 | 15 | 5 | 2 |
| 464 | 9311467 | 215 | 996 | 22 | 3 | 3 |
| 465 | 9311468 | 176 | 693 | 16 | 5 | 1 |
| 466 | 9311469 | 172 | 602 | 19 | 3 | 0 |
| 467 | 9311470 | 243 | 1235 | 29 | 5 | 3 |
| 468 | 9311471 | 184 | 841 | 20 | 4 | 1 |
| 469 | 9311472 | 248 | 1131 | 29 | 11 | 2 |
| 470 | 9311473 | 211 | 1156 | 23 | 4 | 3 |
| 471 | 9311474 | 230 | 1309 | 28 | 4 | 1 |
| 472 | 9311475 | 210 | 688 | 21 | 2 | 1 |
| 473 | 9311476 | 207 | 788 | 17 | 5 | 3 |
| 474 | 9311477 | 176 | 896 | 17 | 6 | 2 |
| 475 | 9311478 | 215 | 1093 | 22 | 3 | 2 |
| 476 | 9311479 | 161 | 535 | 12 | 4 | 1 |
| 477 | 9311480 | 225 | 999 | 26 | 0 | 2 |
| 478 | 9311481 | 252 | 814 | 17 | 4 | 2 |
| 479 | 9311482 | 286 | 1201 | 18 | 7 | 3 |
| 480 | 9311483 | 189 | 744 | 11 | 2 | 0 |
| 481 | 9311484 | 252 | 1238 | 15 | 3 | 1 |
| 482 | 9311485 | 198 | 772 | 27 | 4 | 2 |
| 483 | 9311486 | 226 | 787 | 9 | 4 | 1 |
| 484 | 9311487 | 338 | 2412 | 38 | 5 | 1 |
| 485 | 9311488 | 234 | 1103 | 13 | 4 | 1 |
| 486 | 9311489 | 226 | 820 | 15 | 9 | 2 |
| 487 | 9311490 | 220 | 1146 | 29 | 3 | 0 |
| 488 | 9311491 | 322 | 1772 | 37 | 5 | 3 |
| 489 | 9311492 | 260 | 1149 | 25 | 3 | 3 |
| 490 | 9311493 | 303 | 1583 | 32 | 3 | 0 |
| 491 | 9311494 | 197 | 904 | 16 | 5 | 0 |
| 492 | 9311495 | 196 | 942 | 19 | 2 | 1 |
| 493 | 9311496 | 275 | 1161 | 26 | 5 | 1 |
| 494 | 9311497 | 161 | 658 | 21 | 1 | 2 |
| 495 | 9311498 | 178 | 758 | 16 | 6 | 1 |
| 496 | 9311499 | 221 | 896 | 12 | 9 | 3 |
| 497 | 9311500 | 185 | 795 | 18 | 4 | 2 |
| 498 | 9311501 | 225 | 667 | 17 | 7 | 2 |
| 499 | 9311502 | 231 | 992 | 23 | 1 | 3 |
| 500 | 9311503 | 251 | 1060 | 6 | 22 | 3 |
| 501 | 9311504 | 187 | 656 | 10 | 3 | 1 |
| 502 | 9311505 | 235 | 1231 | 20 | 1 | 2 |
| 503 | 9311506 | 190 | 925 | 20 | 4 | 2 |
| 504 | 9311507 | 208 | 1039 | 25 | 4 | 4 |
| 505 | 9311508 | 201 | 909 | 13 | 5 | 2 |
| 506 | 9311509 | 289 | 1334 | 20 | 11 | 2 |
| 507 | 9311510 | 188 | 937 | 20 | 2 | 1 |
| 508 | 9311511 | 252 | 1054 | 26 | 12 | 2 |
| 509 | 9311512 | 333 | 1572 | 37 | 11 | 2 |
| 510 | 9311513 | 174 | 580 | 5 | 5 | 2 |
| 511 | 9311514 | 255 | 1190 | 24 | 9 | 1 |
| 512 | 9311515 | 165 | 549 | 8 | 6 | 2 |
| 513 | 9311516 | 267 | 1283 | 29 | 4 | 1 |
| 514 | 9311517 | 228 | 1080 | 19 | 8 | 3 |
| 515 | 9311518 | 254 | 1282 | 22 | 4 | 2 |
| 516 | 9311519 | 207 | 1066 | 19 | 8 | 1 |
| 517 | 9311520 | 202 | 791 | 18 | 4 | 1 |
| 518 | 9311521 | 172 | 407 | 8 | 6 | 2 |
| 519 | 9311522 | 195 | 886 | 11 | 3 | 2 |
| 520 | 9311523 | 199 | 878 | 8 | 4 | 0 |
| 521 | 9311524 | 235 | 994 | 37 | 3 | 1 |
| 522 | 9311525 | 187 | 798 | 9 | 3 | 2 |
| 523 | 9311526 | 153 | 584 | 10 | 5 | 3 |
| 524 | 9311527 | 183 | 846 | 14 | 5 | 2 |
| 525 | 9311528 | 200 | 995 | 10 | 3 | 2 |
| 526 | 9311529 | 207 | 889 | 23 | 10 | 2 |
| 527 | 9311530 | 242 | 1160 | 19 | 14 | 2 |
| 528 | 9311531 | 247 | 1336 | 17 | 3 | 2 |
| 529 | 9311532 | 191 | 942 | 18 | 5 | 1 |
| 530 | 9311533 | 229 | 922 | 16 | 9 | 2 |
| 531 | 9311534 | 214 | 1009 | 24 | 1 | 1 |
| 532 | 9311535 | 217 | 921 | 26 | 1 | 1 |
| 533 | 9311536 | 221 | 1023 | 24 | 3 | 3 |
| 534 | 9311537 | 350 | 1710 | 47 | 3 | 4 |
| 535 | 9311538 | 226 | 1057 | 27 | 3 | 1 |
| 536 | 9311539 | 183 | 840 | 17 | 2 | 1 |
| 537 | 9311540 | 188 | 683 | 21 | 4 | 2 |
| 538 | 9311541 | 195 | 880 | 13 | 3 | 1 |
| 539 | 9311542 | 233 | 950 | 19 | 0 | 1 |
| 540 | 9311543 | 226 | 959 | 20 | 8 | 2 |
| 541 | 9311544 | 248 | 1207 | 36 | 4 | 2 |
| 542 | 9311545 | 193 | 629 | 9 | 3 | 2 |
| 543 | 9311546 | 166 | 512 | 19 | 3 | 1 |
| 544 | 9311547 | 218 | 976 | 25 | 0 | 2 |
| 545 | 9311548 | 210 | 1056 | 17 | 3 | 1 |
| 546 | 9311549 | 246 | 1536 | 24 | 7 | 3 |
| 547 | 9311550 | 189 | 811 | 29 | 3 | 1 |
| 548 | 9311551 | 210 | 697 | 20 | 6 | 3 |
| 549 | 9311552 | 221 | 1084 | 21 | 4 | 2 |
| 550 | 9311553 | 201 | 903 | 19 | 3 | 0 |
| 551 | 9311554 | 197 | 898 | 23 | 6 | 3 |
| 552 | 9311555 | 226 | 1319 | 31 | 5 | 0 |
| 553 | 9311556 | 271 | 1516 | 27 | 14 | 2 |
| 554 | 9311557 | 187 | 590 | 11 | 4 | 3 |
| 555 | 9311558 | 304 | 1403 | 35 | 4 | 4 |
| 556 | 9311559 | 223 | 1204 | 18 | 7 | 1 |
| 557 | 9311560 | 200 | 828 | 23 | 3 | 2 |
| 558 | 9311561 | 213 | 1079 | 30 | 4 | 4 |
| 559 | 9311562 | 193 | 975 | 22 | 3 | 1 |
| 560 | 9311563 | 215 | 711 | 15 | 7 | 3 |
| 561 | 9311564 | 275 | 1446 | 23 | 3 | 2 |
| 562 | 9311565 | 160 | 610 | 12 | 9 | 1 |
| 563 | 9311566 | 217 | 876 | 28 | 5 | 4 |
| 564 | 9311567 | 190 | 820 | 13 | 2 | 1 |
| 565 | 9311568 | 233 | 995 | 16 | 5 | 3 |
| 566 | 9311569 | 236 | 1045 | 16 | 1 | 2 |
| 567 | 9311570 | 284 | 1404 | 24 | 4 | 3 |
| 568 | 9311571 | 251 | 1213 | 18 | 7 | 3 |
| 569 | 9311572 | 191 | 859 | 21 | 2 | 1 |
| 570 | 9311573 | 186 | 567 | 19 | 4 | 1 |
| 571 | 9311574 | 255 | 1254 | 37 | 5 | 1 |
| 572 | 9311575 | 216 | 662 | 10 | 2 | 1 |
| 573 | 9311576 | 249 | 1173 | 31 | 7 | 3 |
| 574 | 9311577 | 214 | 959 | 23 | 6 | 3 |
| 575 | 9311578 | 290 | 1510 | 32 | 1 | 1 |
| 576 | 9311579 | 208 | 943 | 22 | 7 | 1 |
| 577 | 9311580 | 234 | 1124 | 20 | 5 | 1 |
| 578 | 9311581 | 204 | 986 | 14 | 1 | 1 |
| 579 | 9311582 | 245 | 1221 | 22 | 3 | 1 |
| 580 | 9311583 | 231 | 1156 | 30 | 7 | 2 |
| 581 | 9311584 | 297 | 1467 | 30 | 4 | 2 |
| 582 | 9311585 | 222 | 1114 | 28 | 5 | 1 |
| 583 | 9311586 | 171 | 598 | 12 | 4 | 2 |
| 584 | 9311587 | 249 | 1159 | 25 | 7 | 1 |
| 585 | 9311588 | 260 | 1384 | 30 | 3 | 1 |
| 586 | 9311589 | 227 | 904 | 12 | 1 | 1 |
| 587 | 9311590 | 223 | 723 | 18 | 2 | 2 |
| 588 | 9311591 | 205 | 825 | 18 | 5 | 1 |
| 589 | 9311592 | 203 | 830 | 20 | 6 | 2 |
| 590 | 9311593 | 206 | 788 | 16 | 5 | 2 |
| 591 | 9311594 | 236 | 1186 | 31 | 7 | 1 |
| 592 | 9311595 | 217 | 1246 | 27 | 6 | 0 |
| 593 | 9311596 | 204 | 763 | 15 | 7 | 2 |
| 594 | 9311597 | 207 | 784 | 17 | 5 | 3 |
| 595 | 9311598 | 255 | 1124 | 21 | 2 | 2 |
| 596 | 9311599 | 223 | 1100 | 28 | 8 | 1 |
| 597 | 9311600 | 216 | 1055 | 16 | 4 | 0 |
| 598 | 9311601 | 149 | 539 | 6 | 5 | 2 |
| 599 | 9311602 | 186 | 911 | 10 | 6 | 2 |
| 600 | 9311603 | 197 | 664 | 20 | 8 | 2 |
| 601 | 9311604 | 231 | 1040 | 31 | 1 | 3 |
| 602 | 9311605 | 185 | 932 | 13 | 2 | 0 |
| 603 | 9311606 | 181 | 700 | 14 | 4 | 1 |
| 604 | 9311607 | 197 | 969 | 20 | 4 | 2 |
| 605 | 9311608 | 268 | 1219 | 39 | 17 | 2 |
| 606 | 9311609 | 244 | 1154 | 19 | 8 | 1 |
| 607 | 9311610 | 240 | 901 | 20 | 5 | 0 |
| 608 | 9311611 | 228 | 1059 | 18 | 7 | 2 |
| 609 | 9311612 | 212 | 1015 | 14 | 1 | 2 |
| 610 | 9311613 | 204 | 983 | 35 | 7 | 1 |
| 611 | 9311614 | 229 | 1281 | 37 | 5 | 3 |
| 612 | 9311615 | 251 | 1396 | 29 | 6 | 2 |
| 613 | 9311616 | 202 | 684 | 13 | 3 | 1 |
| 614 | 9311617 | 192 | 987 | 19 | 3 | 1 |
| 615 | 9311618 | 177 | 669 | 16 | 3 | 2 |
| 616 | 9311619 | 218 | 778 | 15 | 7 | 2 |
| 617 | 9311620 | 178 | 693 | 14 | 1 | 1 |
| 618 | 9311621 | 203 | 1085 | 19 | 2 | 0 |
| 619 | 9311622 | 262 | 1382 | 26 | 4 | 1 |
| 620 | 9311623 | 200 | 1011 | 17 | 3 | 4 |
| 621 | 9311624 | 185 | 726 | 16 | 2 | 1 |
| 622 | 9311625 | 175 | 754 | 23 | 8 | 0 |
| 623 | 9311626 | 226 | 1232 | 28 | 7 | 1 |
| 624 | 9311627 | 221 | 857 | 23 | 5 | 5 |
| 625 | 9311628 | 221 | 1163 | 26 | 3 | 3 |
| 626 | 9311629 | 233 | 1218 | 34 | 4 | 2 |
| 627 | 9311630 | 224 | 1026 | 12 | 4 | 3 |
| 628 | 9311631 | 257 | 1265 | 23 | 6 | 6 |
| 629 | 9311632 | 321 | 1749 | 35 | 9 | 1 |
| 630 | 9311633 | 182 | 823 | 14 | 3 | 1 |
| 631 | 9311634 | 179 | 598 | 11 | 8 | 2 |
| 632 | 9311635 | 239 | 1239 | 38 | 4 | 3 |
| 633 | 9311636 | 178 | 718 | 18 | 4 | 1 |
| 634 | 9311637 | 187 | 819 | 27 | 1 | 3 |
| 635 | 9311638 | 237 | 950 | 19 | 4 | 1 |
| 636 | 9311639 | 244 | 1192 | 27 | 10 | 1 |
| 637 | 9311640 | 221 | 932 | 23 | 6 | 5 |
| 638 | 9311641 | 229 | 1029 | 19 | 1 | 0 |
| 639 | 9311642 | 264 | 1389 | 16 | 7 | 3 |
| 640 | 9311643 | 197 | 956 | 10 | 5 | 1 |
| 641 | 9311644 | 218 | 960 | 19 | 10 | 0 |
| 642 | 9311645 | 190 | 549 | 17 | 3 | 2 |
| 643 | 9311646 | 222 | 997 | 14 | 5 | 2 |
| 644 | 9311647 | 221 | 1087 | 12 | 6 | 2 |
| 645 | 9311648 | 218 | 842 | 22 | 10 | 1 |
| 646 | 9311649 | 187 | 808 | 30 | 0 | 3 |
| 647 | 9311650 | 214 | 756 | 26 | 5 | 2 |
| 648 | 9311651 | 214 | 983 | 29 | 8 | 1 |
| 649 | 9311652 | 206 | 950 | 11 | 3 | 1 |
| 650 | 9311653 | 256 | 1269 | 31 | 5 | 4 |
| 651 | 9311654 | 378 | 1876 | 34 | 3 | 3 |
| 652 | 9311655 | 190 | 764 | 11 | 6 | 0 |
| 653 | 9311656 | 307 | 1572 | 29 | 2 | 1 |
| 654 | 9311657 | 278 | 1405 | 28 | 4 | 1 |
| 655 | 9311658 | 192 | 917 | 22 | 2 | 3 |
| 656 | 9311659 | 188 | 832 | 21 | 2 | 1 |
| 657 | 9311660 | 169 | 650 | 16 | 7 | 1 |
| 658 | 9311661 | 256 | 1161 | 23 | 5 | 3 |
| 659 | 9311662 | 181 | 883 | 13 | 4 | 0 |
| 660 | 9311663 | 203 | 723 | 23 | 12 | 4 |
| 661 | 9311664 | 198 | 862 | 18 | 3 | 2 |
| 662 | 9311665 | 237 | 1104 | 23 | 3 | 0 |
| 663 | 9311666 | 252 | 1504 | 23 | 7 | 1 |
| 664 | 9311667 | 282 | 1244 | 27 | 9 | 1 |
| 665 | 9311668 | 189 | 720 | 15 | 6 | 3 |
| 666 | 9311669 | 226 | 1090 | 31 | 2 | 2 |
| 667 | 9311670 | 229 | 1063 | 18 | 2 | 0 |
| 668 | 9311671 | 209 | 713 | 16 | 3 | 2 |
| 669 | 9311672 | 208 | 834 | 23 | 7 | 2 |
| 670 | 9311673 | 212 | 958 | 37 | 5 | 4 |
| 671 | 9311674 | 257 | 1508 | 28 | 6 | 1 |
| 672 | 9311675 | 163 | 521 | 12 | 4 | 0 |
| 673 | 9311676 | 225 | 1178 | 20 | 8 | 0 |
| 674 | 9311677 | 167 | 539 | 13 | 5 | 0 |
| 675 | 9311678 | 193 | 845 | 19 | 2 | 1 |
| 676 | 9311679 | 256 | 1121 | 41 | 3 | 2 |
| 677 | 9311680 | 177 | 776 | 18 | 2 | 3 |
| 678 | 9311681 | 180 | 776 | 10 | 5 | 1 |
| 679 | 9311682 | 180 | 488 | 16 | 5 | 2 |
| 680 | 9311683 | 199 | 851 | 13 | 4 | 1 |
| 681 | 9311684 | 227 | 1342 | 26 | 2 | 3 |
| 682 | 9311685 | 169 | 694 | 17 | 2 | 1 |
| 683 | 9311686 | 235 | 819 | 20 | 5 | 3 |
| 684 | 9311687 | 166 | 551 | 19 | 8 | 3 |
| 685 | 9311688 | 206 | 644 | 5 | 1 | 2 |
| 686 | 9311689 | 223 | 929 | 23 | 3 | 4 |
| 687 | 9311690 | 191 | 814 | 22 | 6 | 2 |
| 688 | 9311691 | 205 | 930 | 27 | 10 | 1 |
| 689 | 9311692 | 340 | 1698 | 31 | 8 | 3 |
| 690 | 9311693 | 231 | 1193 | 30 | 2 | 1 |
| 691 | 9311694 | 199 | 763 | 23 | 1 | 2 |
| 692 | 9311695 | 243 | 1333 | 35 | 3 | 2 |
| 693 | 9311696 | 184 | 687 | 14 | 1 | 1 |
| 694 | 9311697 | 228 | 839 | 25 | 6 | 2 |
| 695 | 9311698 | 205 | 914 | 17 | 2 | 1 |
| 696 | 9311699 | 212 | 1090 | 19 | 1 | 0 |
| 697 | 9311700 | 238 | 1000 | 26 | 0 | 2 |
| 698 | 9311701 | 180 | 432 | 9 | 9 | 1 |
| 699 | 9311702 | 180 | 766 | 9 | 6 | 0 |
| 700 | 9311703 | 220 | 823 | 23 | 3 | 2 |
| 701 | 9311704 | 219 | 825 | 22 | 4 | 0 |
| 702 | 9311705 | 252 | 1413 | 29 | 4 | 3 |
| 703 | 9311706 | 246 | 1024 | 23 | 4 | 2 |
| 704 | 9311707 | 212 | 889 | 22 | 6 | 0 |
| 705 | 9311708 | 228 | 1090 | 27 | 12 | 0 |
| 706 | 9311709 | 205 | 899 | 20 | 3 | 2 |
| 707 | 9311710 | 203 | 900 | 10 | 4 | 3 |
| 708 | 9311711 | 227 | 1071 | 23 | 2 | 2 |
| 709 | 9311712 | 222 | 940 | 19 | 10 | 3 |
| 710 | 9311713 | 242 | 1010 | 24 | 3 | 3 |
| 711 | 9311714 | 195 | 1080 | 18 | 5 | 2 |
| 712 | 9311715 | 228 | 1040 | 16 | 4 | 1 |
| 713 | 9311716 | 192 | 874 | 15 | 10 | 3 |
| 714 | 9311717 | 190 | 729 | 11 | 7 | 1 |
| 715 | 9311718 | 192 | 836 | 23 | 4 | 0 |
| 716 | 9311719 | 223 | 1001 | 28 | 7 | 2 |
| 717 | 9311720 | 206 | 793 | 12 | 3 | 2 |
| 718 | 9311721 | 196 | 660 | 23 | 3 | 2 |
| 719 | 9311722 | 250 | 1229 | 24 | 6 | 2 |
| 720 | 9311723 | 167 | 620 | 11 | 1 | 1 |
| 721 | 9311724 | 276 | 1309 | 29 | 8 | 2 |
| 722 | 9311725 | 203 | 1005 | 8 | 2 | 3 |
| 723 | 9311726 | 224 | 1194 | 18 | 8 | 1 |
| 724 | 9311727 | 265 | 1278 | 27 | 1 | 0 |
| 725 | 9311728 | 264 | 1185 | 29 | 3 | 1 |
| 726 | 9311729 | 208 | 971 | 28 | 7 | 1 |
| 727 | 9311730 | 227 | 1091 | 26 | 10 | 0 |
| 728 | 9311731 | 209 | 964 | 18 | 3 | 1 |
| 729 | 9311732 | 256 | 833 | 20 | 4 | 2 |
| 730 | 9311733 | 215 | 1251 | 32 | 6 | 2 |
| 731 | 9311734 | 204 | 944 | 28 | 2 | 3 |
| 732 | 9311735 | 226 | 834 | 15 | 12 | 1 |
| 733 | 9311736 | 219 | 864 | 16 | 6 | 1 |
| 734 | 9311737 | 383 | 2133 | 53 | 2 | 0 |
| 735 | 9311738 | 191 | 819 | 24 | 3 | 2 |
| 736 | 9311739 | 207 | 1094 | 33 | 2 | 1 |
| 737 | 9311740 | 219 | 1038 | 23 | 4 | 2 |
| 738 | 9311741 | 206 | 858 | 17 | 6 | 4 |
| 739 | 9311742 | 199 | 876 | 24 | 5 | 1 |
| 740 | 9311743 | 244 | 1343 | 19 | 5 | 0 |
| 741 | 9311744 | 260 | 1309 | 25 | 3 | 0 |
| 742 | 9311745 | 199 | 920 | 24 | 1 | 1 |
| 743 | 9311746 | 310 | 1671 | 27 | 8 | 3 |
| 744 | 9311747 | 201 | 913 | 15 | 6 | 1 |
| 745 | 9311748 | 193 | 818 | 8 | 1 | 1 |
| 746 | 9311749 | 243 | 1183 | 29 | 4 | 3 |
| 747 | 9311750 | 234 | 1143 | 33 | 3 | 1 |
| 748 | 9311751 | 190 | 600 | 14 | 8 | 1 |
| 749 | 9311752 | 264 | 1494 | 28 | 4 | 3 |
| 750 | 9311753 | 196 | 1085 | 17 | 8 | 1 |
| 751 | 9311754 | 257 | 1320 | 40 | 8 | 2 |
| 752 | 9311755 | 191 | 809 | 22 | 4 | 2 |
| 753 | 9311756 | 264 | 1137 | 26 | 5 | 3 |
| 754 | 9311757 | 203 | 819 | 9 | 2 | 0 |
| 755 | 9311758 | 286 | 1363 | 32 | 4 | 4 |
| 756 | 9311759 | 188 | 696 | 19 | 3 | 1 |
| 757 | 9311760 | 270 | 1260 | 25 | 8 | 1 |
| 758 | 9311761 | 227 | 1048 | 26 | 8 | 1 |
| 759 | 9311762 | 280 | 1509 | 29 | 5 | 2 |
| 760 | 9311763 | 206 | 961 | 15 | 1 | 2 |
| 761 | 9311764 | 213 | 913 | 24 | 5 | 1 |
| 762 | 9311765 | 255 | 1240 | 9 | 2 | 3 |
| 763 | 9311766 | 259 | 1160 | 28 | 13 | 2 |
| 764 | 9311767 | 258 | 1301 | 26 | 4 | 1 |
| 765 | 9311768 | 143 | 517 | 12 | 8 | 0 |
| 766 | 9311769 | 172 | 662 | 18 | 5 | 2 |
| 767 | 9311770 | 227 | 924 | 28 | 7 | 1 |
| 768 | 9311771 | 231 | 921 | 17 | 6 | 3 |
| 769 | 9311772 | 177 | 771 | 18 | 6 | 1 |
| 770 | 9311773 | 257 | 1390 | 12 | 3 | 0 |
| 771 | 9311774 | 231 | 869 | 21 | 4 | 2 |
| 772 | 9311775 | 228 | 1177 | 26 | 4 | 2 |
| 773 | 9311776 | 315 | 1490 | 24 | 12 | 4 |
| 774 | 9311777 | 246 | 1175 | 25 | 1 | 0 |
| 775 | 9311778 | 178 | 764 | 12 | 9 | 0 |
| 776 | 9311779 | 174 | 675 | 17 | 1 | 1 |
| 777 | 9311780 | 167 | 609 | 9 | 5 | 2 |
| 778 | 9311781 | 256 | 999 | 17 | 4 | 3 |
| 779 | 9311782 | 186 | 820 | 10 | 1 | 2 |
| 780 | 9311783 | 297 | 1546 | 21 | 4 | 1 |
| 781 | 9311784 | 198 | 958 | 15 | 5 | 1 |
| 782 | 9311785 | 198 | 893 | 18 | 6 | 2 |
| 783 | 9311786 | 190 | 895 | 17 | 2 | 1 |
| 784 | 9311787 | 267 | 1382 | 36 | 2 | 3 |
| 785 | 9311788 | 190 | 851 | 14 | 1 | 1 |
| 786 | 9311789 | 183 | 761 | 13 | 7 | 2 |
| 787 | 9311790 | 196 | 947 | 14 | 3 | 1 |
| 788 | 9311791 | 184 | 758 | 6 | 6 | 2 |
| 789 | 9311792 | 223 | 1060 | 16 | 9 | 2 |
| 790 | 9311793 | 174 | 741 | 21 | 3 | 1 |
| 791 | 9311794 | 219 | 951 | 21 | 7 | 0 |
| 792 | 9311795 | 283 | 1455 | 26 | 11 | 3 |
| 793 | 9311796 | 203 | 1079 | 23 | 10 | 1 |
| 794 | 9311797 | 190 | 623 | 20 | 4 | 2 |
| 795 | 9311798 | 226 | 1057 | 18 | 5 | 2 |
| 796 | 9311799 | 231 | 1013 | 18 | 2 | 1 |
| 797 | 9311800 | 159 | 550 | 18 | 2 | 1 |
| 798 | 9311801 | 225 | 945 | 14 | 5 | 0 |
| 799 | 9311802 | 168 | 766 | 12 | 5 | 1 |
| 800 | 9311803 | 274 | 1289 | 34 | 8 | 2 |
| 801 | 9311804 | 182 | 851 | 15 | 6 | 0 |
| 802 | 9311805 | 200 | 807 | 13 | 5 | 4 |
| 803 | 9311806 | 243 | 1480 | 29 | 4 | 2 |
| 804 | 9311807 | 300 | 1672 | 39 | 2 | 2 |
| 805 | 9311808 | 235 | 1156 | 13 | 7 | 0 |
| 806 | 9311809 | 237 | 914 | 24 | 10 | 0 |
| 807 | 9311810 | 180 | 798 | 18 | 8 | 1 |
| 808 | 9311811 | 205 | 994 | 20 | 5 | 1 |
| 809 | 9311812 | 209 | 743 | 19 | 5 | 3 |
| 810 | 9311813 | 267 | 1293 | 32 | 2 | 3 |
| 811 | 9311814 | 202 | 747 | 18 | 4 | 2 |
| 812 | 9311815 | 231 | 1221 | 32 | 2 | 1 |
| 813 | 9311816 | 214 | 955 | 22 | 2 | 1 |
| 814 | 9311817 | 200 | 871 | 36 | 1 | 2 |
| 815 | 9311818 | 231 | 1116 | 16 | 3 | 0 |
| 816 | 9311819 | 209 | 981 | 18 | 2 | 1 |
| 817 | 9311820 | 167 | 543 | 6 | 3 | 0 |
| 818 | 9311821 | 186 | 826 | 17 | 5 | 2 |
| 819 | 9311822 | 210 | 850 | 22 | 8 | 2 |
| 820 | 9311823 | 263 | 1485 | 33 | 8 | 0 |
| 821 | 9311824 | 274 | 1239 | 21 | 9 | 0 |
| 822 | 9311825 | 237 | 1144 | 31 | 5 | 1 |
| 823 | 9311826 | 211 | 878 | 20 | 4 | 1 |
| 824 | 9311827 | 224 | 1103 | 27 | 13 | 1 |
| 825 | 9311828 | 224 | 1009 | 20 | 4 | 2 |
| 826 | 9311829 | 196 | 654 | 12 | 6 | 1 |
| 827 | 9311830 | 251 | 1427 | 35 | 2 | 2 |
| 828 | 9311831 | 260 | 1117 | 22 | 9 | 1 |
| 829 | 9311832 | 267 | 1167 | 30 | 3 | 1 |
| 830 | 9311833 | 188 | 688 | 17 | 3 | 1 |
| 831 | 9311834 | 222 | 979 | 24 | 2 | 2 |
| 832 | 9311835 | 277 | 1366 | 35 | 5 | 4 |
| 833 | 9311836 | 224 | 987 | 26 | 2 | 2 |
| 834 | 9311837 | 294 | 1579 | 22 | 3 | 3 |
| 835 | 9311838 | 166 | 839 | 16 | 5 | 2 |
| 836 | 9311839 | 179 | 649 | 20 | 4 | 0 |
| 837 | 9311840 | 209 | 1001 | 18 | 5 | 0 |
| 838 | 9311841 | 247 | 939 | 17 | 13 | 4 |
| 839 | 9311842 | 269 | 1505 | 50 | 8 | 2 |
| 840 | 9311843 | 214 | 1058 | 26 | 2 | 3 |
| 841 | 9311844 | 200 | 1007 | 15 | 0 | 1 |
| 842 | 9311845 | 246 | 1203 | 13 | 4 | 1 |
| 843 | 9311846 | 200 | 678 | 11 | 4 | 1 |
| 844 | 9311847 | 223 | 1111 | 21 | 4 | 1 |
| 845 | 9311848 | 213 | 1156 | 20 | 1 | 0 |
| 846 | 9311849 | 214 | 905 | 30 | 4 | 3 |
| 847 | 9311850 | 279 | 1214 | 17 | 2 | 4 |
| 848 | 9311851 | 192 | 843 | 7 | 4 | 2 |
| 849 | 9311852 | 218 | 980 | 21 | 4 | 2 |
| 850 | 9311853 | 217 | 1095 | 25 | 0 | 1 |
| 851 | 9311854 | 269 | 1287 | 47 | 8 | 3 |
| 852 | 9311855 | 197 | 658 | 17 | 5 | 1 |
| 853 | 9311856 | 156 | 499 | 11 | 4 | 1 |
| 854 | 9311857 | 166 | 718 | 11 | 1 | 1 |
| 855 | 9311858 | 180 | 667 | 8 | 5 | 3 |
| 856 | 9311859 | 212 | 547 | 23 | 1 | 1 |
| 857 | 9311860 | 264 | 1660 | 33 | 7 | 2 |
| 858 | 9311861 | 243 | 1187 | 36 | 9 | 2 |
| 859 | 9311862 | 345 | 1550 | 37 | 9 | 2 |
| 860 | 9311863 | 161 | 571 | 16 | 6 | 0 |
| 861 | 9311864 | 251 | 1262 | 25 | 7 | 2 |
| 862 | 9311865 | 212 | 1087 | 20 | 1 | 2 |
| 863 | 9311866 | 288 | 1444 | 24 | 4 | 2 |
| 864 | 9311867 | 230 | 836 | 18 | 3 | 3 |
| 865 | 9311868 | 289 | 1575 | 19 | 4 | 2 |
| 866 | 9311869 | 183 | 748 | 17 | 5 | 1 |
| 867 | 9311870 | 258 | 1231 | 27 | 8 | 1 |
| 868 | 9311871 | 252 | 1385 | 20 | 3 | 3 |
| 869 | 9311872 | 220 | 1096 | 22 | 6 | 3 |
| 870 | 9311873 | 228 | 1277 | 21 | 4 | 2 |
| 871 | 9311874 | 230 | 1107 | 17 | 3 | 2 |
| 872 | 9311875 | 205 | 1001 | 22 | 1 | 1 |
| 873 | 9311876 | 193 | 715 | 10 | 6 | 1 |
| 874 | 9311877 | 252 | 1246 | 22 | 2 | 1 |
| 875 | 9311878 | 229 | 982 | 25 | 5 | 3 |
| 876 | 9311879 | 253 | 1267 | 29 | 3 | 3 |
| 877 | 9311880 | 230 | 889 | 18 | 9 | 3 |
| 878 | 9311881 | 205 | 970 | 19 | 6 | 2 |
| 879 | 9311882 | 213 | 835 | 22 | 1 | 1 |
| 880 | 9311883 | 245 | 607 | 19 | 4 | 1 |
| 881 | 9311884 | 218 | 1082 | 22 | 3 | 0 |
| 882 | 9311885 | 208 | 849 | 24 | 4 | 2 |
| 883 | 9311886 | 280 | 1256 | 27 | 9 | 0 |
| 884 | 9311887 | 274 | 1131 | 21 | 7 | 2 |
| 885 | 9311888 | 200 | 837 | 14 | 0 | 2 |
| 886 | 9311889 | 188 | 769 | 24 | 4 | 2 |
| 887 | 9311890 | 222 | 887 | 21 | 3 | 3 |
| 888 | 9311891 | 321 | 1725 | 30 | 9 | 2 |
| 889 | 9311892 | 222 | 881 | 20 | 1 | 2 |
| 890 | 9311893 | 238 | 1350 | 30 | 1 | 0 |
| 891 | 9311894 | 226 | 949 | 27 | 4 | 0 |
| 892 | 9311895 | 195 | 883 | 23 | 5 | 2 |
| 893 | 9311896 | 206 | 892 | 15 | 5 | 0 |
| 894 | 9311897 | 254 | 1219 | 31 | 4 | 2 |
| 895 | 9311898 | 178 | 677 | 18 | 2 | 1 |
| 896 | 9311899 | 300 | 1346 | 24 | 8 | 2 |
| 897 | 9311900 | 204 | 968 | 35 | 6 | 2 |
| 898 | 9311901 | 244 | 1247 | 32 | 6 | 1 |
| 899 | 9311902 | 255 | 1179 | 26 | 5 | 2 |
| 900 | 9311903 | 247 | 1344 | 28 | 3 | 3 |
| 901 | 9311904 | 207 | 1120 | 15 | 3 | 1 |
| 902 | 9311905 | 188 | 823 | 23 | 1 | 2 |
| 903 | 9311906 | 361 | 2134 | 34 | 9 | 3 |
| 904 | 9311907 | 175 | 863 | 18 | 3 | 0 |
| 905 | 9311908 | 188 | 886 | 20 | 5 | 2 |
| 906 | 9311909 | 201 | 1022 | 17 | 0 | 1 |
| 907 | 9311910 | 213 | 849 | 17 | 1 | 2 |
| 908 | 9311911 | 211 | 1017 | 14 | 6 | 0 |
| 909 | 9311912 | 185 | 874 | 13 | 6 | 2 |
| 910 | 9311913 | 208 | 936 | 27 | 8 | 2 |
| 911 | 9311914 | 297 | 1211 | 28 | 5 | 2 |
| 912 | 9311915 | 177 | 639 | 22 | 3 | 2 |
| 913 | 9311916 | 268 | 1405 | 24 | 3 | 3 |
| 914 | 9311917 | 206 | 858 | 12 | 3 | 2 |
| 915 | 9311918 | 249 | 846 | 29 | 1 | 1 |
| 916 | 9311919 | 279 | 1504 | 35 | 7 | 1 |
| 917 | 9311920 | 190 | 765 | 21 | 3 | 3 |
| 918 | 9311921 | 198 | 969 | 19 | 7 | 1 |
| 919 | 9311922 | 197 | 960 | 21 | 5 | 0 |
| 920 | 9311923 | 200 | 932 | 20 | 8 | 1 |
| 921 | 9311924 | 199 | 808 | 11 | 2 | 0 |
| 922 | 9311925 | 219 | 963 | 26 | 17 | 2 |
| 923 | 9311926 | 234 | 946 | 18 | 2 | 3 |
| 924 | 9311927 | 320 | 1741 | 40 | 4 | 0 |
| 925 | 9311928 | 236 | 1069 | 21 | 6 | 0 |
| 926 | 9311929 | 257 | 1267 | 21 | 3 | 1 |
| 927 | 9311930 | 218 | 919 | 19 | 6 | 1 |
| 928 | 9311931 | 225 | 857 | 15 | 6 | 3 |
| 929 | 9311932 | 237 | 985 | 19 | 6 | 2 |
| 930 | 9311933 | 242 | 1123 | 21 | 5 | 1 |
| 931 | 9311934 | 230 | 926 | 24 | 7 | 2 |
| 932 | 9311935 | 207 | 1008 | 29 | 6 | 2 |
| 933 | 9311936 | 245 | 1091 | 17 | 5 | 1 |
| 934 | 9311937 | 173 | 708 | 10 | 2 | 1 |
| 935 | 9311938 | 306 | 1668 | 30 | 5 | 2 |
| 936 | 9311939 | 200 | 820 | 17 | 3 | 1 |
| 937 | 9311940 | 190 | 767 | 25 | 2 | 2 |
| 938 | 9311941 | 210 | 1039 | 21 | 3 | 0 |
| 939 | 9311942 | 196 | 977 | 10 | 7 | 1 |
| 940 | 9311943 | 205 | 977 | 24 | 6 | 3 |
| 941 | 9311944 | 259 | 1260 | 16 | 6 | 2 |
| 942 | 9311945 | 210 | 926 | 23 | 5 | 3 |
| 943 | 9311946 | 161 | 613 | 15 | 5 | 1 |
| 944 | 9311947 | 245 | 1074 | 24 | 5 | 2 |
| 945 | 9311948 | 223 | 618 | 9 | 8 | 1 |
| 946 | 9311949 | 260 | 1163 | 34 | 3 | 1 |
| 947 | 9311950 | 232 | 695 | 18 | 3 | 2 |
| 948 | 9311951 | 222 | 914 | 15 | 4 | 1 |
| 949 | 9311952 | 205 | 1151 | 25 | 3 | 3 |
| 950 | 9311953 | 191 | 976 | 12 | 4 | 0 |
| 951 | 9311954 | 207 | 692 | 22 | 9 | 1 |
| 952 | 9311955 | 203 | 914 | 22 | 3 | 0 |
| 953 | 9311956 | 222 | 754 | 18 | 9 | 3 |
| 954 | 9311957 | 198 | 983 | 26 | 2 | 2 |
| 955 | 9311958 | 208 | 1008 | 29 | 7 | 1 |
| 956 | 9311959 | 249 | 810 | 19 | 11 | 2 |
| 957 | 9311960 | 181 | 727 | 24 | 6 | 3 |
| 958 | 9311961 | 213 | 899 | 16 | 3 | 2 |
| 959 | 9311962 | 218 | 885 | 27 | 4 | 2 |
| 960 | 9311963 | 259 | 1202 | 34 | 3 | 3 |
| 961 | 9311964 | 188 | 772 | 27 | 0 | 3 |
| 962 | 9311965 | 285 | 1202 | 17 | 2 | 5 |
| 963 | 9311966 | 232 | 1224 | 23 | 3 | 2 |
| 964 | 9311967 | 161 | 546 | 7 | 5 | 0 |
| 965 | 9311968 | 229 | 962 | 32 | 7 | 3 |
| 966 | 9311969 | 197 | 899 | 15 | 1 | 1 |
| 967 | 9311970 | 180 | 787 | 12 | 4 | 0 |
| 968 | 9311971 | 264 | 1522 | 15 | 5 | 1 |
| 969 | 9311972 | 231 | 1068 | 28 | 4 | 3 |
| 970 | 9311973 | 186 | 667 | 19 | 5 | 2 |
| 971 | 9311974 | 128 | 365 | 4 | 2 | 0 |
| 972 | 9311975 | 213 | 643 | 16 | 2 | 2 |
| 973 | 9311976 | 222 | 952 | 30 | 6 | 4 |
| 974 | 9311977 | 304 | 1401 | 28 | 13 | 2 |
| 975 | 9311978 | 238 | 1037 | 11 | 8 | 2 |
| 976 | 9311979 | 255 | 1285 | 33 | 8 | 2 |
| 977 | 9311980 | 209 | 1046 | 21 | 8 | 2 |
| 978 | 9311981 | 200 | 762 | 22 | 3 | 3 |
| 979 | 9311982 | 202 | 863 | 20 | 2 | 2 |
| 980 | 9311983 | 272 | 1239 | 20 | 6 | 3 |
| 981 | 9311984 | 219 | 1172 | 20 | 2 | 1 |
| 982 | 9311985 | 212 | 891 | 25 | 3 | 2 |
| 983 | 9311986 | 187 | 768 | 25 | 5 | 2 |
| 984 | 9311987 | 220 | 1099 | 23 | 3 | 4 |
| 985 | 9311988 | 177 | 879 | 15 | 3 | 0 |
| 986 | 9311989 | 242 | 1172 | 21 | 7 | 3 |
| 987 | 9311990 | 223 | 933 | 27 | 10 | 3 |
| 988 | 9311991 | 208 | 982 | 28 | 6 | 1 |
| 989 | 9311992 | 228 | 1173 | 18 | 4 | 3 |
| 990 | 9311993 | 253 | 1417 | 34 | 5 | 1 |
| 991 | 9311994 | 280 | 1362 | 41 | 4 | 3 |
| 992 | 9311995 | 220 | 953 | 18 | 2 | 1 |
| 993 | 9311996 | 192 | 681 | 19 | 7 | 3 |
| 994 | 9311997 | 296 | 1493 | 33 | 8 | 2 |
| 995 | 9311998 | 278 | 1087 | 32 | 7 | 3 |
| 996 | 9311999 | 261 | 1352 | 34 | 3 | 3 |
| 997 | 9312000 | 183 | 728 | 21 | 3 | 2 |
| 998 | 9312001 | 170 | 585 | 12 | 4 | 0 |
| 999 | 9312002 | 219 | 1030 | 16 | 0 | 1 |

## Throughput

- Turns simulated: 222365 (first play of each game)
- Aggregate: 54.0 turns/s over 4116.4s wall
- Per game thread: 6.8 turns/s

## Determinism

Not checked (run with `--determinism`).

## Panics

None.
