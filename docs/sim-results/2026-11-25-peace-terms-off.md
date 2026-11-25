# Simulation: deals-terms-off (2026-11-25)

1000 games, first seed 9311003, brain `scripted`, turn limit off, safety cap 1200, 6 threads, started 2026-11-25T12:50:23Z. Wall time 1238.5s.

Source checkout at run start: `460d50aaa05846a188340be67e929a2cd5b76b67`; dirty: no. Map 48×32, 8 initial seats. `first_setup` and exact binary argv are recorded in JSON. Ordinary games use seed + i (wrapping); the strategy tournament schedule is specified below. All games use a deterministic shuffle of the full faction pool, taking eight distinct factions, with any Proxy rotation described below.

Command argv: `["./sim","--players","8","--difficulty","hard","--games","1000","--seed","9311003","--out-dir","out","--threads","6","--deals","--ai-params","off.json","--label","deals-terms-off"]`

## Overview

| Metric | Value |
| --- | --- |
| Games (non-panic) | 1000 of 1000 |
| Game length, turns (mean / median / min-max) | 220.9 / 216 / 143-525 |
| Win reason mix | VP hold 98.5%, Capitals 1.5% |
| Median end turn | 216 |
| Ending shares of played games (capitals / VP hold / turn limit / other) | 1.5% / 98.5% / 0.0% / 0.0% |
| Games reaching the World War era (any empire) | 992 of 1000 (99.2%) |
| VP hold length | 10 turn ends (the reaching turn counts) |
| Holds started / broken / completed / unfinished | 1506 / 398 / 992 / 116 |
| Hold break rate | 26.4% of started; 28.6% of resolved (broken / (broken + completed)) |
| Hold wins | 985 |
| First reach to win, turns (mean / median / min-max) | 11.7 / 9 / 9-170 |
| Final hold start to win, turns | 9.0 / 9 / 9-9 |
| Comebacks (winner broke an earlier hold of its own) | 129 |
| Wins by a team another team beat to its target | 71 |

Game length by how the game ended (turns resolved; a stall is a safety-cap stop, never a win):

| Ended by | Games | Length mean / median / min-max |
| --- | --- | --- |
| VP hold | 985 | 220.4 / 215 / 143-525 |
| Capitals | 15 | 252.7 / 249 / 207-303 |

Winner share by faction (wins of that faction's seats / all winning seats):

| Faction | Wins | Share of wins |
| --- | --- | --- |
| Switzerland | 82 | 8.2% |
| India | 78 | 7.8% |
| Byzantium | 85 | 8.5% |
| Germany | 134 | 13.4% |
| Celts | 151 | 15.1% |
| Mongols | 80 | 8.0% |
| Sparta | 111 | 11.1% |
| Athens | 155 | 15.5% |
| Aztecs | 124 | 12.4% |

## Outcomes

| Outcome | Games | Share |
| --- | --- | --- |
| VP hold | 985 | 98.5% |
| Conquest | 0 | 0.0% |
| Capitals | 15 | 1.5% |
| Turn limit | 0 | 0.0% |
| Draw | 0 | 0.0% |
| Stalled (safety cap) | 0 | 0.0% |
| Panic | 0 | 0.0% |

## Win rate by faction

Per seat: wins / seats of that faction. With 8 single-empire teams, an even game is 12.5% per seat.

| Faction | Seats | Wins | Win rate |
| --- | --- | --- | --- |
| Switzerland | 879 | 82 | 9.3% |
| India | 905 | 78 | 8.6% |
| Byzantium | 885 | 85 | 9.6% |
| Germany | 900 | 134 | 14.9% |
| Celts | 880 | 151 | 17.2% |
| Mongols | 878 | 80 | 9.1% |
| Sparta | 901 | 111 | 12.3% |
| Athens | 900 | 155 | 17.2% |
| Aztecs | 872 | 124 | 14.2% |

## Win rate by difficulty

All initial seats in non-panicked games, including eliminated empires and cap stops. Determinism replays are not additional samples.

| Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- |
| Hard | 8000 | 1000 | 12.5% |

## Win rate by faction and difficulty

| Faction | Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- | --- |
| Switzerland | Hard | 879 | 82 | 9.3% |
| India | Hard | 905 | 78 | 8.6% |
| Byzantium | Hard | 885 | 85 | 9.6% |
| Germany | Hard | 900 | 134 | 14.9% |
| Celts | Hard | 880 | 151 | 17.2% |
| Mongols | Hard | 878 | 80 | 9.1% |
| Sparta | Hard | 901 | 111 | 12.3% |
| Athens | Hard | 900 | 155 | 17.2% |
| Aztecs | Hard | 872 | 124 | 14.2% |

## Faction goal progress

Per seat: how often the faction's permanent goal was reached before the game ended, and the turn its VP was first awarded (over the seats that reached it). Seats are all initial empires of non-panicked games with telemetry, including eliminated empires and cap stops.

| Faction | Seats | Reached | Goal rate | Min | P10 | Median turn | P90 | Max |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Switzerland | 879 | 177 | 20.1% | 119 | 138 | 159 | 222 | 272 |
| India | 905 | 508 | 56.1% | 44 | 87 | 126 | 194 | 322 |
| Byzantium | 885 | 714 | 80.7% | 69 | 83 | 107 | 148 | 270 |
| Germany | 900 | 347 | 38.6% | 84 | 124 | 148 | 202 | 285 |
| Celts | 880 | 527 | 59.9% | 35 | 161 | 210 | 210 | 246 |
| Mongols | 878 | 153 | 17.4% | 105 | 148 | 187 | 240 | 319 |
| Sparta | 901 | 117 | 13.0% | 138 | 157 | 191 | 256 | 327 |
| Athens | 900 | 264 | 29.3% | 124 | 162 | 217 | 255 | 255 |
| Aztecs | 872 | 306 | 35.1% | 128 | 165 | 225 | 235 | 285 |

## Distributions

| Metric | Min | P10 | Median | P90 | Max |
| --- | --- | --- | --- | --- | --- |
| Observed game length (turns, including cap stops) | 143 | 181 | 216 | 268 | 525 |
| Winning team VP | 4 | 10 | 11 | 13 | 19 |
| First Soldier completed (turn, per empire) | 6 | 8 | 36 | 71 | 288 |
| First Settler completed (turn, per empire) | 6 | 12 | 14 | 25 | 225 |
| First city founded (turn, per empire) | 1 | 1 | 1 | 1 | 1 |
| Second city founded (turn, per empire) | 10 | 16 | 19 | 39 | 235 |

Safety-cap stops are censored observations, not completed game lengths.

Openings: 8000 observed empires in 1000 first plays; 0 games without opening telemetry excluded. Turn distributions include only empires that reached the milestone. Never means not reached by the end of observed play, including eliminated empires and safety-cap stops. Per-empire turns are in JSON (`games[].telemetry.openings`, keyed by player ID).

| Opening milestone | Reached empires | Never empires |
| --- | --- | --- |
| First Soldier completed | 7907 | 93 |
| First Settler completed | 7984 | 16 |
| First city founded | 8000 | 0 |
| Second city founded | 7915 | 85 |

Final VP per empire (all seats):

| VP | Empires |
| --- | --- |
| 0 | 1571 |
| 1 | 1280 |
| 2 | 1094 |
| 3 | 779 |
| 4 | 615 |
| 5 | 505 |
| 6 | 403 |
| 7 | 274 |
| 8 | 214 |
| 9 | 153 |
| 10 | 505 |
| 11 | 320 |
| 12 | 153 |
| 13 | 70 |
| 14 | 38 |
| 15 | 18 |
| 16 | 6 |
| 17 | 1 |
| 19 | 1 |

## End state

- Average game length: 220.9 turns
- Average cities per empire: 4.70
- Average techs per empire: 32.57
- Average eliminations per game: 1.64

## Strategy diversity: observed behavior (classifier v2)

1000 finished games; 8000 player seats; 1000 winning seats. Excluded 0 unfinished/panicked games and 0 games with missing labels. Winner entropy: **0.3019** (normalized over 6 categories, including zero-win categories).

Thresholds, biases and denominators: [strategy-diversity.md](../strategy-diversity.md).

| Archetype | Players | Wins | Win rate | All-player frequency | Winner frequency |
| --- | ---: | ---: | ---: | ---: | ---: |
| Rush | 1755 | 63 | 3.6% | 21.9% | 6.3% |
| Wide | 3770 | 850 | 22.5% | 47.1% | 85.0% |
| Science | 0 | 0 | n/a | 0.0% | 0.0% |
| Turtle/Wonder | 22 | 0 | 0.0% | 0.3% | 0.0% |
| Economic | 1179 | 83 | 7.0% | 14.7% | 8.3% |
| Generalist | 1274 | 4 | 0.3% | 15.9% | 0.4% |

### Matchup matrix

Cell = strict ahead % / tied % (co-occurrence games). Each game has equal weight; all cross-team seat pairs within it share that weight. Rank: winning team first, then surviving empires by VP, then eliminated empires by later elimination turn. Equal ranks tie; no ID tiebreak. This is a multiplayer association, not a duel win rate.

| Row vs column | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Rush | — | 16.4% / 4.7% (833) | n/a / n/a (0) | 29.8% / 7.0% (19) | 18.6% / 9.5% (577) | 59.2% / 8.3% (637) |
| Wide | 78.9% / 4.7% (833) | — | n/a / n/a (0) | 72.0% / 4.9% (22) | 59.3% / 6.5% (718) | 87.1% / 3.4% (780) |
| Science | n/a / n/a (0) | n/a / n/a (0) | — | n/a / n/a (0) | n/a / n/a (0) | n/a / n/a (0) |
| Turtle/Wonder | 63.2% / 7.0% (19) | 23.1% / 4.9% (22) | n/a / n/a (0) | — | 41.7% / 8.3% (12) | 74.6% / 6.3% (21) |
| Economic | 71.8% / 9.5% (577) | 34.2% / 6.5% (718) | n/a / n/a (0) | 50.0% / 8.3% (12) | — | 85.4% / 5.8% (545) |
| Generalist | 32.5% / 8.3% (637) | 9.4% / 3.4% (780) | n/a / n/a (0) | 19.0% / 6.3% (21) | 8.8% / 5.8% (545) | — |

### Faction wins by archetype

Cell = wins / seats; win rate in JSON.

| Faction | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Switzerland | 0 / 88 | 76 / 429 | 0 / 0 | 0 / 1 | 6 / 242 | 0 / 119 |
| India | 8 / 217 | 70 / 569 | 0 / 0 | 0 / 0 | 0 / 14 | 0 / 105 |
| Byzantium | 0 / 7 | 77 / 492 | 0 / 0 | 0 / 3 | 8 / 136 | 0 / 247 |
| Germany | 4 / 146 | 127 / 468 | 0 / 0 | 0 / 5 | 3 / 85 | 0 / 196 |
| Celts | 0 / 91 | 147 / 531 | 0 / 0 | 0 / 0 | 4 / 144 | 0 / 114 |
| Mongols | 23 / 540 | 54 / 207 | 0 / 0 | 0 / 0 | 3 / 49 | 0 / 82 |
| Sparta | 21 / 437 | 62 / 179 | 0 / 0 | 0 / 13 | 25 / 158 | 3 / 114 |
| Athens | 2 / 96 | 134 / 454 | 0 / 0 | 0 / 0 | 18 / 167 | 1 / 183 |
| Aztecs | 5 / 133 | 103 / 441 | 0 / 0 | 0 / 0 | 16 / 184 | 0 / 114 |

Guardrail: **Fail**. Entropy minimum 0.80; at least 20 games per strategy pair. Dominant by observed score: none. Without an observed counter (all pairs covered): none. Sampled but never won: Turtle/Wonder. Unobserved: Science. Under-sampled pairs: 7.

- insufficient evidence: 7 under-sampled matchups; 1000 winning seats

- winner entropy below 0.80

## Battle and capture telemetry

1000 observed first plays; 0 records without telemetry excluded.

| Event | Total / observed games | Mean per game | Games with event / observed games |
| --- | --- | --- | --- |
| War declarations | 14640/1000 | 14.640 | 1000/1000 (100.0%) |
| Peace offers | 86814/1000 | 86.814 | 1000/1000 (100.0%) |
| Peace agreements | 12530/1000 | 12.530 | 1000/1000 (100.0%) |
| Battles | 995546/1000 | 995.546 | 1000/1000 (100.0%) |
| City captures | 21045/1000 | 21.045 | 1000/1000 (100.0%) |
| Settler captures | 5006/1000 | 5.006 | 981/1000 (98.1%) |
| Eliminations | 1638/1000 | 1.638 | 849/1000 (84.9%) |

## Terminal VP sources

1000 finished games. Columns are separate populations; totals across columns must not be added. Every cell is VP / population observations.

| Source | All players | All teams | Winning teams |
| --- | --- | --- | --- |
| Held Great Works | 7630/8000 | 7630/8000 | 3006/1000 |
| Faction goals | 5016/8000 | 5016/8000 | 2050/1000 |
| Era objectives | 15334/8000 | 15334/8000 | 4508/1000 |
| Capitals taken | 2485/8000 | 2485/8000 | 1455/1000 |
| Total VP | 30465/8000 | 30465/8000 | 11019/1000 |

Reconciliation failures: 0 player observations, 0 team observations.

## Turn-50 team leaders

| Cohort | Eventual leader-set wins / eligible games | Draws (included) |
| --- | --- | --- |
| Unique leader | 68/396 (17.2%) | 0 |
| Tied leaders (winner in set) | 600/604 (99.3%) | 0 |
| Any leader set | 668/1000 (66.8%) | 0 |

Excluded: 0 games ended before turn 50, 0 unfinished games, 0 snapshots without teams, 0 records without telemetry.

## Era pacing

Resolved turn each empire entered each era. `Winner vs median` is the winning team's seats' entry turn minus the median entry of the empires that entered it in the same finished game (negative: winners got there first).

| Era | Entered / seats | Fastest | p10 | Median | p90 | Slowest | Winner vs median (mean, n) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Classical | 7902/8000 | 5 | 20 | 28 | 32 | 59 | -2.6 (1000) |
| Medieval | 7821/8000 | 35 | 56 | 60 | 63 | 129 | -1.7 (1000) |
| Renaissance | 7770/8000 | 63 | 81 | 86 | 93 | 186 | -2.7 (1000) |
| Industrial | 7586/8000 | 87 | 107 | 114 | 126 | 212 | -4.5 (1000) |
| Machine Age | 7006/8000 | 113 | 135 | 145 | 160 | 292 | -6.7 (1000) |
| World War | 5681/8000 | 130 | 160 | 173 | 192 | 322 | -9.3 (987) |
| Modern | 3539/8000 | 150 | 183 | 202 | 225 | 295 | -9.3 (846) |
| Future | 1428/8000 | 172 | 208 | 232 | 256 | 337 | -8.3 (463) |

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
| Classical | Mathematics | 7838/8000 | 43 | 351 BC | 300 BC |
| Classical | Engineering | 7829/8000 | 49 | 51 BC | n/a |
| Classical | Iron Working | 7853/8000 | 37 | 651 BC | 1200 BC |
| Classical | Code of Laws | 7823/8000 | 57 | 350 AD | 1754 BC |
| Classical | Philosophy | 7822/8000 | 58 | 400 AD | 500 BC |
| Medieval | Feudalism | 7799/8000 | 73 | 890 AD | n/a |
| Medieval | Machinery | 7812/8000 | 67 | 710 AD | n/a |
| Medieval | Chivalry | 6923/8000 | 77 | 1010 AD | n/a |
| Medieval | Education | 7777/8000 | 82 | 1160 AD | 1088 AD |
| Medieval | Crop Rotation | 7771/8000 | 85 | 1250 AD | n/a |
| Renaissance | Gunpowder | 7738/8000 | 92 | 1420 AD | n/a |
| Renaissance | Metallurgy | 7700/8000 | 102 | 1520 AD | n/a |
| Renaissance | Military Engineering | 7707/8000 | 100 | 1500 AD | n/a |
| Renaissance | Banking | 7624/8000 | 111 | 1610 AD | 1397 AD |
| Renaissance | Printing Press | 7594/8000 | 112 | 1620 AD | 1440 AD |
| Industrial | Rifling | 7234/8000 | 135 | 1787 AD | n/a |
| Industrial | Military Science | 7397/8000 | 127 | 1757 AD | n/a |
| Industrial | Ballistics | 7281/8000 | 137 | 1794 AD | n/a |
| Industrial | Industrialization | 7480/8000 | 122 | 1720 AD | 1769 AD |
| Industrial | Railways | 7012/8000 | 145 | 1824 AD | 1825 AD |
| Machine Age | Replaceable Parts | 6471/8000 | 159 | 1876 AD | 1800 AD |
| Machine Age | Combustion | 6748/8000 | 153 | 1854 AD | 1876 AD |
| Machine Age | Steel | 6465/8000 | 158 | 1872 AD | 1856 AD |
| Machine Age | Flight | 6036/8000 | 167 | 1887 AD | 1903 AD |
| Machine Age | Electricity | 5726/8000 | 173 | 1894 AD | 1879 AD |
| World War | Mass Mobilisation | 4604/8000 | 187 | 1910 AD | n/a |
| World War | Rocketry | 4308/8000 | 191 | 1915 AD | 1926 AD |
| World War | Armoured Warfare | 3997/8000 | 196 | 1922 AD | 1916 AD |
| World War | Air Power | 3786/8000 | 199 | 1926 AD | n/a |
| World War | Electronics | 5257/8000 | 179 | 1901 AD | n/a |
| Modern | Mechanised Warfare | 2292/8000 | 216 | 1952 AD | n/a |
| Modern | Composite Armour | 1971/8000 | 221 | 1964 AD | n/a |
| Modern | Guided Missiles | 1825/8000 | 226 | 1976 AD | n/a |
| Modern | Jet Engines | 1748/8000 | 227 | 1978 AD | 1939 AD |
| Modern | Green Revolution | 2899/8000 | 210 | 1942 AD | n/a |
| Future | Autonomous Systems | 822/8000 | 245 | 2018 AD | n/a |
| Future | Robotics | 638/8000 | 252 | 2028 AD | n/a |
| Future | Fusion Power | 636/8000 | 255 | 2031 AD | n/a |
| Future | Stealth Technology | 1027/8000 | 240 | 2010 AD | n/a |
| Future | Recursive AI | 577/8000 | 256 | 2032 AD | n/a |

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
| 0 | 0.00 | 75.63 |
| 1 | 57.35 | 16.00 |
| 2 | 18.21 | 4.93 |
| 3 | 4.38 | 1.33 |
| 4 | 0.78 | 0.23 |
| 5 | 0.15 | 0.07 |

## Per-game events

Each row is one observed first play; unavailable telemetry is shown as n/a. Full per-player/team score sources and turn-50 standings are in JSON.

| Game | Seed | Resolved turns | Battles | City captures | Settler captures | Eliminations |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 9311003 | 208 | 921 | 23 | 5 | 1 |
| 1 | 9311004 | 229 | 729 | 13 | 3 | 0 |
| 2 | 9311005 | 228 | 1110 | 26 | 8 | 3 |
| 3 | 9311006 | 227 | 1162 | 16 | 7 | 0 |
| 4 | 9311007 | 215 | 1064 | 24 | 5 | 0 |
| 5 | 9311008 | 245 | 1141 | 26 | 5 | 3 |
| 6 | 9311009 | 197 | 1084 | 24 | 5 | 0 |
| 7 | 9311010 | 197 | 869 | 31 | 7 | 2 |
| 8 | 9311011 | 257 | 900 | 10 | 10 | 3 |
| 9 | 9311012 | 201 | 1012 | 22 | 4 | 2 |
| 10 | 9311013 | 238 | 1275 | 35 | 15 | 3 |
| 11 | 9311014 | 316 | 1822 | 38 | 6 | 2 |
| 12 | 9311015 | 207 | 693 | 12 | 7 | 2 |
| 13 | 9311016 | 241 | 1284 | 20 | 10 | 0 |
| 14 | 9311017 | 208 | 971 | 27 | 5 | 2 |
| 15 | 9311018 | 163 | 500 | 12 | 10 | 0 |
| 16 | 9311019 | 284 | 1221 | 24 | 9 | 1 |
| 17 | 9311020 | 228 | 776 | 14 | 11 | 0 |
| 18 | 9311021 | 206 | 893 | 17 | 11 | 3 |
| 19 | 9311022 | 186 | 730 | 9 | 2 | 0 |
| 20 | 9311023 | 222 | 1061 | 17 | 5 | 0 |
| 21 | 9311024 | 244 | 1084 | 29 | 4 | 5 |
| 22 | 9311025 | 251 | 1249 | 20 | 12 | 1 |
| 23 | 9311026 | 228 | 1221 | 34 | 2 | 3 |
| 24 | 9311027 | 262 | 1124 | 17 | 3 | 2 |
| 25 | 9311028 | 272 | 1230 | 28 | 6 | 2 |
| 26 | 9311029 | 164 | 550 | 11 | 10 | 0 |
| 27 | 9311030 | 194 | 676 | 15 | 6 | 1 |
| 28 | 9311031 | 201 | 807 | 12 | 11 | 0 |
| 29 | 9311032 | 227 | 1063 | 27 | 5 | 1 |
| 30 | 9311033 | 192 | 743 | 20 | 3 | 1 |
| 31 | 9311034 | 245 | 1328 | 33 | 5 | 0 |
| 32 | 9311035 | 172 | 766 | 17 | 3 | 0 |
| 33 | 9311036 | 271 | 1235 | 18 | 8 | 1 |
| 34 | 9311037 | 193 | 820 | 11 | 3 | 1 |
| 35 | 9311038 | 252 | 1138 | 18 | 1 | 0 |
| 36 | 9311039 | 195 | 1015 | 27 | 4 | 2 |
| 37 | 9311040 | 203 | 1066 | 24 | 3 | 0 |
| 38 | 9311041 | 190 | 908 | 17 | 8 | 1 |
| 39 | 9311042 | 226 | 861 | 29 | 8 | 1 |
| 40 | 9311043 | 237 | 1059 | 20 | 2 | 2 |
| 41 | 9311044 | 186 | 777 | 21 | 4 | 3 |
| 42 | 9311045 | 184 | 859 | 11 | 3 | 2 |
| 43 | 9311046 | 235 | 1009 | 16 | 5 | 1 |
| 44 | 9311047 | 241 | 1247 | 26 | 8 | 2 |
| 45 | 9311048 | 183 | 755 | 11 | 13 | 1 |
| 46 | 9311049 | 401 | 2685 | 32 | 3 | 1 |
| 47 | 9311050 | 244 | 1077 | 25 | 4 | 2 |
| 48 | 9311051 | 213 | 1080 | 24 | 5 | 2 |
| 49 | 9311052 | 181 | 852 | 12 | 6 | 0 |
| 50 | 9311053 | 170 | 835 | 20 | 6 | 0 |
| 51 | 9311054 | 208 | 717 | 12 | 5 | 2 |
| 52 | 9311055 | 183 | 799 | 9 | 7 | 2 |
| 53 | 9311056 | 258 | 1421 | 31 | 3 | 0 |
| 54 | 9311057 | 235 | 1343 | 25 | 7 | 2 |
| 55 | 9311058 | 174 | 678 | 13 | 7 | 2 |
| 56 | 9311059 | 190 | 707 | 10 | 2 | 1 |
| 57 | 9311060 | 293 | 1406 | 27 | 6 | 0 |
| 58 | 9311061 | 181 | 776 | 14 | 1 | 1 |
| 59 | 9311062 | 225 | 1134 | 22 | 3 | 2 |
| 60 | 9311063 | 197 | 680 | 17 | 5 | 2 |
| 61 | 9311064 | 331 | 1670 | 30 | 5 | 4 |
| 62 | 9311065 | 209 | 1082 | 20 | 4 | 1 |
| 63 | 9311066 | 230 | 1198 | 24 | 6 | 2 |
| 64 | 9311067 | 190 | 817 | 16 | 3 | 0 |
| 65 | 9311068 | 193 | 870 | 21 | 8 | 0 |
| 66 | 9311069 | 193 | 703 | 20 | 3 | 0 |
| 67 | 9311070 | 216 | 966 | 21 | 3 | 3 |
| 68 | 9311071 | 228 | 1168 | 32 | 0 | 2 |
| 69 | 9311072 | 206 | 892 | 22 | 9 | 3 |
| 70 | 9311073 | 229 | 959 | 19 | 9 | 2 |
| 71 | 9311074 | 217 | 857 | 26 | 8 | 2 |
| 72 | 9311075 | 221 | 1145 | 24 | 12 | 2 |
| 73 | 9311076 | 221 | 977 | 21 | 2 | 2 |
| 74 | 9311077 | 195 | 901 | 19 | 7 | 0 |
| 75 | 9311078 | 249 | 964 | 14 | 3 | 2 |
| 76 | 9311079 | 228 | 1078 | 15 | 6 | 2 |
| 77 | 9311080 | 310 | 1397 | 10 | 4 | 1 |
| 78 | 9311081 | 185 | 726 | 19 | 6 | 1 |
| 79 | 9311082 | 213 | 924 | 22 | 9 | 2 |
| 80 | 9311083 | 203 | 768 | 21 | 7 | 2 |
| 81 | 9311084 | 179 | 792 | 14 | 6 | 2 |
| 82 | 9311085 | 266 | 1537 | 26 | 2 | 1 |
| 83 | 9311086 | 215 | 975 | 23 | 4 | 2 |
| 84 | 9311087 | 200 | 827 | 17 | 8 | 1 |
| 85 | 9311088 | 191 | 780 | 19 | 3 | 1 |
| 86 | 9311089 | 187 | 767 | 21 | 3 | 1 |
| 87 | 9311090 | 201 | 913 | 18 | 6 | 2 |
| 88 | 9311091 | 228 | 1078 | 28 | 4 | 2 |
| 89 | 9311092 | 272 | 1661 | 31 | 12 | 0 |
| 90 | 9311093 | 215 | 992 | 16 | 8 | 2 |
| 91 | 9311094 | 212 | 967 | 26 | 4 | 0 |
| 92 | 9311095 | 277 | 1475 | 28 | 4 | 1 |
| 93 | 9311096 | 178 | 804 | 17 | 3 | 0 |
| 94 | 9311097 | 173 | 748 | 17 | 0 | 1 |
| 95 | 9311098 | 188 | 727 | 18 | 8 | 1 |
| 96 | 9311099 | 209 | 1043 | 20 | 1 | 0 |
| 97 | 9311100 | 238 | 1050 | 25 | 8 | 3 |
| 98 | 9311101 | 226 | 927 | 18 | 2 | 0 |
| 99 | 9311102 | 223 | 972 | 23 | 8 | 4 |
| 100 | 9311103 | 232 | 852 | 15 | 7 | 2 |
| 101 | 9311104 | 258 | 1312 | 17 | 4 | 2 |
| 102 | 9311105 | 194 | 902 | 26 | 3 | 0 |
| 103 | 9311106 | 237 | 1016 | 22 | 2 | 2 |
| 104 | 9311107 | 213 | 1179 | 26 | 3 | 1 |
| 105 | 9311108 | 160 | 569 | 12 | 4 | 1 |
| 106 | 9311109 | 201 | 787 | 18 | 3 | 2 |
| 107 | 9311110 | 191 | 758 | 23 | 5 | 1 |
| 108 | 9311111 | 237 | 1063 | 29 | 5 | 3 |
| 109 | 9311112 | 220 | 938 | 22 | 4 | 1 |
| 110 | 9311113 | 250 | 1168 | 21 | 4 | 1 |
| 111 | 9311114 | 299 | 1307 | 27 | 2 | 3 |
| 112 | 9311115 | 206 | 877 | 14 | 5 | 2 |
| 113 | 9311116 | 274 | 1450 | 41 | 2 | 2 |
| 114 | 9311117 | 200 | 953 | 11 | 2 | 0 |
| 115 | 9311118 | 235 | 1154 | 13 | 5 | 3 |
| 116 | 9311119 | 285 | 1185 | 26 | 4 | 1 |
| 117 | 9311120 | 224 | 1012 | 18 | 9 | 0 |
| 118 | 9311121 | 211 | 971 | 17 | 1 | 4 |
| 119 | 9311122 | 255 | 1225 | 25 | 5 | 2 |
| 120 | 9311123 | 281 | 1341 | 28 | 1 | 2 |
| 121 | 9311124 | 240 | 1107 | 20 | 4 | 1 |
| 122 | 9311125 | 232 | 686 | 13 | 5 | 1 |
| 123 | 9311126 | 204 | 764 | 14 | 10 | 1 |
| 124 | 9311127 | 260 | 1273 | 29 | 3 | 2 |
| 125 | 9311128 | 184 | 723 | 12 | 6 | 1 |
| 126 | 9311129 | 217 | 899 | 22 | 3 | 2 |
| 127 | 9311130 | 209 | 901 | 10 | 5 | 0 |
| 128 | 9311131 | 203 | 858 | 14 | 9 | 0 |
| 129 | 9311132 | 166 | 616 | 10 | 4 | 1 |
| 130 | 9311133 | 172 | 739 | 19 | 3 | 0 |
| 131 | 9311134 | 207 | 990 | 14 | 5 | 2 |
| 132 | 9311135 | 154 | 631 | 10 | 3 | 1 |
| 133 | 9311136 | 312 | 1661 | 49 | 7 | 5 |
| 134 | 9311137 | 230 | 835 | 18 | 2 | 1 |
| 135 | 9311138 | 249 | 1214 | 26 | 3 | 0 |
| 136 | 9311139 | 212 | 787 | 16 | 7 | 1 |
| 137 | 9311140 | 281 | 1362 | 30 | 8 | 3 |
| 138 | 9311141 | 223 | 822 | 20 | 3 | 1 |
| 139 | 9311142 | 199 | 868 | 22 | 5 | 2 |
| 140 | 9311143 | 246 | 1190 | 23 | 2 | 3 |
| 141 | 9311144 | 190 | 950 | 13 | 6 | 0 |
| 142 | 9311145 | 209 | 706 | 20 | 4 | 3 |
| 143 | 9311146 | 225 | 870 | 29 | 6 | 2 |
| 144 | 9311147 | 218 | 1116 | 25 | 5 | 2 |
| 145 | 9311148 | 255 | 889 | 25 | 1 | 3 |
| 146 | 9311149 | 190 | 851 | 7 | 6 | 2 |
| 147 | 9311150 | 188 | 781 | 22 | 5 | 2 |
| 148 | 9311151 | 236 | 1193 | 31 | 10 | 3 |
| 149 | 9311152 | 191 | 856 | 22 | 4 | 3 |
| 150 | 9311153 | 191 | 959 | 17 | 4 | 3 |
| 151 | 9311154 | 193 | 928 | 20 | 6 | 2 |
| 152 | 9311155 | 221 | 1162 | 23 | 5 | 2 |
| 153 | 9311156 | 208 | 1025 | 24 | 4 | 0 |
| 154 | 9311157 | 203 | 806 | 9 | 5 | 1 |
| 155 | 9311158 | 213 | 1067 | 18 | 4 | 3 |
| 156 | 9311159 | 254 | 1252 | 25 | 11 | 3 |
| 157 | 9311160 | 206 | 1000 | 30 | 6 | 1 |
| 158 | 9311161 | 209 | 1085 | 28 | 1 | 1 |
| 159 | 9311162 | 188 | 755 | 14 | 7 | 0 |
| 160 | 9311163 | 204 | 946 | 26 | 3 | 1 |
| 161 | 9311164 | 224 | 1189 | 32 | 0 | 3 |
| 162 | 9311165 | 206 | 1049 | 25 | 10 | 1 |
| 163 | 9311166 | 242 | 930 | 21 | 7 | 4 |
| 164 | 9311167 | 220 | 1014 | 29 | 5 | 3 |
| 165 | 9311168 | 245 | 1335 | 31 | 5 | 3 |
| 166 | 9311169 | 237 | 1046 | 32 | 9 | 3 |
| 167 | 9311170 | 208 | 916 | 16 | 4 | 2 |
| 168 | 9311171 | 254 | 1352 | 27 | 7 | 2 |
| 169 | 9311172 | 287 | 800 | 10 | 4 | 4 |
| 170 | 9311173 | 241 | 1156 | 20 | 5 | 2 |
| 171 | 9311174 | 248 | 1173 | 26 | 3 | 3 |
| 172 | 9311175 | 257 | 911 | 12 | 2 | 1 |
| 173 | 9311176 | 205 | 869 | 17 | 5 | 2 |
| 174 | 9311177 | 171 | 675 | 16 | 0 | 3 |
| 175 | 9311178 | 185 | 773 | 18 | 2 | 1 |
| 176 | 9311179 | 233 | 1062 | 20 | 7 | 3 |
| 177 | 9311180 | 259 | 1089 | 34 | 2 | 3 |
| 178 | 9311181 | 251 | 1121 | 23 | 3 | 1 |
| 179 | 9311182 | 184 | 914 | 29 | 2 | 2 |
| 180 | 9311183 | 215 | 992 | 22 | 9 | 1 |
| 181 | 9311184 | 197 | 911 | 26 | 5 | 4 |
| 182 | 9311185 | 222 | 855 | 22 | 6 | 2 |
| 183 | 9311186 | 193 | 749 | 21 | 5 | 2 |
| 184 | 9311187 | 221 | 973 | 31 | 3 | 3 |
| 185 | 9311188 | 281 | 1219 | 15 | 8 | 3 |
| 186 | 9311189 | 163 | 673 | 6 | 6 | 1 |
| 187 | 9311190 | 181 | 653 | 11 | 8 | 0 |
| 188 | 9311191 | 245 | 1168 | 27 | 4 | 1 |
| 189 | 9311192 | 201 | 803 | 22 | 3 | 0 |
| 190 | 9311193 | 183 | 859 | 12 | 3 | 1 |
| 191 | 9311194 | 211 | 1035 | 13 | 6 | 2 |
| 192 | 9311195 | 216 | 996 | 23 | 5 | 2 |
| 193 | 9311196 | 219 | 913 | 19 | 2 | 2 |
| 194 | 9311197 | 224 | 1293 | 17 | 11 | 2 |
| 195 | 9311198 | 207 | 847 | 22 | 16 | 2 |
| 196 | 9311199 | 221 | 944 | 20 | 1 | 1 |
| 197 | 9311200 | 223 | 901 | 26 | 3 | 2 |
| 198 | 9311201 | 242 | 1198 | 31 | 8 | 1 |
| 199 | 9311202 | 187 | 957 | 14 | 3 | 1 |
| 200 | 9311203 | 235 | 1071 | 16 | 11 | 3 |
| 201 | 9311204 | 182 | 747 | 21 | 6 | 2 |
| 202 | 9311205 | 279 | 1438 | 41 | 1 | 3 |
| 203 | 9311206 | 242 | 947 | 25 | 6 | 3 |
| 204 | 9311207 | 228 | 1194 | 17 | 6 | 2 |
| 205 | 9311208 | 202 | 841 | 14 | 5 | 1 |
| 206 | 9311209 | 192 | 881 | 18 | 3 | 3 |
| 207 | 9311210 | 216 | 1042 | 18 | 1 | 1 |
| 208 | 9311211 | 208 | 1108 | 27 | 4 | 2 |
| 209 | 9311212 | 183 | 555 | 15 | 2 | 2 |
| 210 | 9311213 | 357 | 1825 | 45 | 4 | 2 |
| 211 | 9311214 | 179 | 665 | 16 | 5 | 2 |
| 212 | 9311215 | 362 | 2155 | 40 | 7 | 4 |
| 213 | 9311216 | 206 | 969 | 21 | 1 | 1 |
| 214 | 9311217 | 227 | 1152 | 22 | 12 | 1 |
| 215 | 9311218 | 155 | 464 | 12 | 3 | 1 |
| 216 | 9311219 | 213 | 1013 | 25 | 2 | 2 |
| 217 | 9311220 | 191 | 808 | 12 | 6 | 1 |
| 218 | 9311221 | 185 | 952 | 22 | 7 | 2 |
| 219 | 9311222 | 182 | 840 | 15 | 3 | 1 |
| 220 | 9311223 | 219 | 1153 | 23 | 4 | 0 |
| 221 | 9311224 | 274 | 1444 | 25 | 4 | 4 |
| 222 | 9311225 | 237 | 1165 | 33 | 5 | 0 |
| 223 | 9311226 | 248 | 1030 | 24 | 6 | 2 |
| 224 | 9311227 | 237 | 1062 | 23 | 5 | 1 |
| 225 | 9311228 | 221 | 1073 | 23 | 4 | 1 |
| 226 | 9311229 | 197 | 950 | 19 | 3 | 1 |
| 227 | 9311230 | 177 | 681 | 16 | 5 | 3 |
| 228 | 9311231 | 179 | 816 | 19 | 4 | 2 |
| 229 | 9311232 | 243 | 1355 | 33 | 4 | 1 |
| 230 | 9311233 | 267 | 1330 | 38 | 4 | 4 |
| 231 | 9311234 | 228 | 923 | 24 | 5 | 1 |
| 232 | 9311235 | 206 | 1063 | 23 | 3 | 1 |
| 233 | 9311236 | 244 | 1253 | 18 | 5 | 2 |
| 234 | 9311237 | 269 | 1498 | 18 | 3 | 2 |
| 235 | 9311238 | 251 | 863 | 25 | 3 | 3 |
| 236 | 9311239 | 218 | 855 | 16 | 3 | 1 |
| 237 | 9311240 | 210 | 1050 | 25 | 2 | 1 |
| 238 | 9311241 | 215 | 935 | 25 | 6 | 1 |
| 239 | 9311242 | 396 | 2297 | 35 | 8 | 3 |
| 240 | 9311243 | 180 | 670 | 23 | 2 | 2 |
| 241 | 9311244 | 243 | 1063 | 19 | 14 | 4 |
| 242 | 9311245 | 159 | 614 | 16 | 6 | 3 |
| 243 | 9311246 | 211 | 1053 | 25 | 6 | 1 |
| 244 | 9311247 | 220 | 1179 | 21 | 10 | 1 |
| 245 | 9311248 | 193 | 589 | 14 | 6 | 1 |
| 246 | 9311249 | 179 | 780 | 26 | 5 | 2 |
| 247 | 9311250 | 203 | 784 | 23 | 10 | 2 |
| 248 | 9311251 | 178 | 774 | 10 | 8 | 1 |
| 249 | 9311252 | 230 | 869 | 14 | 10 | 2 |
| 250 | 9311253 | 182 | 712 | 10 | 2 | 0 |
| 251 | 9311254 | 220 | 1025 | 22 | 6 | 2 |
| 252 | 9311255 | 272 | 1350 | 34 | 4 | 2 |
| 253 | 9311256 | 243 | 1160 | 24 | 3 | 2 |
| 254 | 9311257 | 153 | 578 | 14 | 5 | 2 |
| 255 | 9311258 | 212 | 858 | 18 | 6 | 2 |
| 256 | 9311259 | 211 | 916 | 17 | 3 | 0 |
| 257 | 9311260 | 275 | 883 | 9 | 7 | 2 |
| 258 | 9311261 | 267 | 1672 | 26 | 1 | 2 |
| 259 | 9311262 | 198 | 760 | 14 | 3 | 2 |
| 260 | 9311263 | 253 | 1214 | 20 | 7 | 1 |
| 261 | 9311264 | 211 | 835 | 24 | 5 | 2 |
| 262 | 9311265 | 205 | 978 | 18 | 7 | 2 |
| 263 | 9311266 | 209 | 815 | 13 | 3 | 0 |
| 264 | 9311267 | 199 | 943 | 21 | 6 | 2 |
| 265 | 9311268 | 199 | 852 | 7 | 5 | 0 |
| 266 | 9311269 | 197 | 799 | 21 | 4 | 1 |
| 267 | 9311270 | 234 | 1264 | 18 | 4 | 0 |
| 268 | 9311271 | 209 | 658 | 17 | 5 | 1 |
| 269 | 9311272 | 283 | 1436 | 19 | 11 | 1 |
| 270 | 9311273 | 190 | 865 | 18 | 6 | 1 |
| 271 | 9311274 | 248 | 1064 | 26 | 3 | 1 |
| 272 | 9311275 | 280 | 905 | 37 | 13 | 3 |
| 273 | 9311276 | 251 | 1146 | 23 | 4 | 2 |
| 274 | 9311277 | 274 | 1430 | 22 | 4 | 2 |
| 275 | 9311278 | 241 | 1403 | 22 | 8 | 2 |
| 276 | 9311279 | 193 | 703 | 15 | 5 | 1 |
| 277 | 9311280 | 248 | 1247 | 34 | 4 | 1 |
| 278 | 9311281 | 215 | 813 | 19 | 1 | 2 |
| 279 | 9311282 | 183 | 671 | 18 | 4 | 1 |
| 280 | 9311283 | 219 | 765 | 9 | 3 | 0 |
| 281 | 9311284 | 156 | 453 | 12 | 3 | 2 |
| 282 | 9311285 | 184 | 949 | 10 | 5 | 0 |
| 283 | 9311286 | 186 | 898 | 22 | 3 | 1 |
| 284 | 9311287 | 214 | 1087 | 17 | 2 | 1 |
| 285 | 9311288 | 233 | 1172 | 27 | 6 | 2 |
| 286 | 9311289 | 294 | 1703 | 32 | 9 | 1 |
| 287 | 9311290 | 220 | 1108 | 23 | 2 | 1 |
| 288 | 9311291 | 250 | 1045 | 36 | 9 | 5 |
| 289 | 9311292 | 225 | 1172 | 22 | 1 | 3 |
| 290 | 9311293 | 231 | 1105 | 40 | 11 | 4 |
| 291 | 9311294 | 230 | 947 | 14 | 6 | 1 |
| 292 | 9311295 | 215 | 917 | 10 | 9 | 3 |
| 293 | 9311296 | 272 | 884 | 24 | 7 | 1 |
| 294 | 9311297 | 194 | 808 | 19 | 1 | 1 |
| 295 | 9311298 | 199 | 622 | 15 | 3 | 0 |
| 296 | 9311299 | 239 | 925 | 22 | 10 | 2 |
| 297 | 9311300 | 249 | 831 | 30 | 6 | 3 |
| 298 | 9311301 | 195 | 809 | 17 | 6 | 1 |
| 299 | 9311302 | 229 | 1033 | 21 | 4 | 1 |
| 300 | 9311303 | 242 | 1107 | 16 | 7 | 1 |
| 301 | 9311304 | 184 | 589 | 12 | 4 | 1 |
| 302 | 9311305 | 220 | 1046 | 19 | 5 | 2 |
| 303 | 9311306 | 269 | 1035 | 20 | 1 | 1 |
| 304 | 9311307 | 209 | 835 | 23 | 5 | 2 |
| 305 | 9311308 | 284 | 1276 | 20 | 10 | 2 |
| 306 | 9311309 | 167 | 716 | 12 | 6 | 0 |
| 307 | 9311310 | 185 | 941 | 13 | 0 | 1 |
| 308 | 9311311 | 192 | 825 | 18 | 2 | 1 |
| 309 | 9311312 | 190 | 916 | 16 | 5 | 1 |
| 310 | 9311313 | 200 | 1027 | 18 | 10 | 3 |
| 311 | 9311314 | 250 | 1025 | 26 | 7 | 0 |
| 312 | 9311315 | 232 | 854 | 18 | 1 | 0 |
| 313 | 9311316 | 195 | 860 | 24 | 0 | 3 |
| 314 | 9311317 | 180 | 608 | 10 | 2 | 0 |
| 315 | 9311318 | 234 | 1341 | 31 | 7 | 1 |
| 316 | 9311319 | 234 | 1004 | 27 | 6 | 3 |
| 317 | 9311320 | 238 | 1249 | 28 | 8 | 3 |
| 318 | 9311321 | 198 | 822 | 13 | 4 | 2 |
| 319 | 9311322 | 185 | 713 | 10 | 12 | 1 |
| 320 | 9311323 | 181 | 828 | 11 | 5 | 1 |
| 321 | 9311324 | 174 | 773 | 10 | 5 | 1 |
| 322 | 9311325 | 211 | 944 | 14 | 3 | 1 |
| 323 | 9311326 | 200 | 946 | 10 | 6 | 0 |
| 324 | 9311327 | 192 | 827 | 25 | 7 | 1 |
| 325 | 9311328 | 231 | 1144 | 24 | 7 | 1 |
| 326 | 9311329 | 203 | 562 | 13 | 2 | 0 |
| 327 | 9311330 | 184 | 819 | 19 | 8 | 2 |
| 328 | 9311331 | 258 | 1334 | 23 | 7 | 1 |
| 329 | 9311332 | 196 | 601 | 13 | 3 | 1 |
| 330 | 9311333 | 219 | 891 | 14 | 8 | 1 |
| 331 | 9311334 | 148 | 394 | 14 | 1 | 2 |
| 332 | 9311335 | 215 | 929 | 21 | 7 | 2 |
| 333 | 9311336 | 202 | 1005 | 7 | 6 | 2 |
| 334 | 9311337 | 181 | 716 | 16 | 5 | 1 |
| 335 | 9311338 | 259 | 1508 | 39 | 2 | 2 |
| 336 | 9311339 | 244 | 1234 | 23 | 6 | 1 |
| 337 | 9311340 | 228 | 1215 | 30 | 9 | 2 |
| 338 | 9311341 | 207 | 820 | 25 | 6 | 3 |
| 339 | 9311342 | 244 | 1069 | 20 | 3 | 0 |
| 340 | 9311343 | 252 | 1012 | 20 | 3 | 1 |
| 341 | 9311344 | 201 | 632 | 12 | 10 | 2 |
| 342 | 9311345 | 208 | 1135 | 22 | 3 | 1 |
| 343 | 9311346 | 241 | 1177 | 14 | 6 | 2 |
| 344 | 9311347 | 225 | 1006 | 23 | 5 | 2 |
| 345 | 9311348 | 205 | 958 | 20 | 4 | 2 |
| 346 | 9311349 | 208 | 829 | 20 | 6 | 1 |
| 347 | 9311350 | 190 | 852 | 10 | 2 | 1 |
| 348 | 9311351 | 219 | 860 | 24 | 2 | 3 |
| 349 | 9311352 | 237 | 845 | 27 | 15 | 3 |
| 350 | 9311353 | 230 | 1204 | 26 | 5 | 0 |
| 351 | 9311354 | 233 | 840 | 20 | 3 | 0 |
| 352 | 9311355 | 198 | 973 | 16 | 3 | 2 |
| 353 | 9311356 | 223 | 1250 | 19 | 10 | 0 |
| 354 | 9311357 | 285 | 1519 | 40 | 6 | 2 |
| 355 | 9311358 | 184 | 700 | 19 | 5 | 1 |
| 356 | 9311359 | 211 | 777 | 14 | 0 | 4 |
| 357 | 9311360 | 178 | 658 | 10 | 5 | 0 |
| 358 | 9311361 | 261 | 1360 | 20 | 3 | 1 |
| 359 | 9311362 | 282 | 1462 | 26 | 11 | 1 |
| 360 | 9311363 | 269 | 1619 | 20 | 10 | 1 |
| 361 | 9311364 | 205 | 867 | 18 | 5 | 0 |
| 362 | 9311365 | 182 | 710 | 15 | 9 | 1 |
| 363 | 9311366 | 310 | 1244 | 39 | 10 | 1 |
| 364 | 9311367 | 204 | 879 | 13 | 8 | 3 |
| 365 | 9311368 | 235 | 1134 | 22 | 1 | 2 |
| 366 | 9311369 | 197 | 888 | 14 | 8 | 0 |
| 367 | 9311370 | 347 | 1396 | 33 | 5 | 3 |
| 368 | 9311371 | 187 | 895 | 16 | 4 | 1 |
| 369 | 9311372 | 212 | 879 | 19 | 2 | 2 |
| 370 | 9311373 | 190 | 678 | 12 | 2 | 1 |
| 371 | 9311374 | 201 | 884 | 25 | 5 | 3 |
| 372 | 9311375 | 203 | 804 | 16 | 7 | 1 |
| 373 | 9311376 | 198 | 901 | 20 | 2 | 3 |
| 374 | 9311377 | 230 | 1018 | 30 | 4 | 2 |
| 375 | 9311378 | 232 | 1231 | 19 | 5 | 2 |
| 376 | 9311379 | 208 | 832 | 18 | 2 | 1 |
| 377 | 9311380 | 262 | 1182 | 24 | 0 | 2 |
| 378 | 9311381 | 236 | 1130 | 18 | 7 | 3 |
| 379 | 9311382 | 291 | 1495 | 27 | 7 | 3 |
| 380 | 9311383 | 197 | 609 | 10 | 1 | 1 |
| 381 | 9311384 | 173 | 817 | 15 | 6 | 0 |
| 382 | 9311385 | 196 | 751 | 14 | 2 | 0 |
| 383 | 9311386 | 196 | 735 | 15 | 2 | 1 |
| 384 | 9311387 | 233 | 1015 | 37 | 7 | 2 |
| 385 | 9311388 | 243 | 1038 | 17 | 8 | 1 |
| 386 | 9311389 | 233 | 1016 | 18 | 4 | 1 |
| 387 | 9311390 | 212 | 914 | 22 | 5 | 1 |
| 388 | 9311391 | 183 | 790 | 14 | 4 | 1 |
| 389 | 9311392 | 234 | 784 | 20 | 5 | 2 |
| 390 | 9311393 | 185 | 653 | 17 | 3 | 0 |
| 391 | 9311394 | 194 | 712 | 7 | 8 | 2 |
| 392 | 9311395 | 194 | 727 | 17 | 5 | 1 |
| 393 | 9311396 | 221 | 947 | 24 | 4 | 3 |
| 394 | 9311397 | 202 | 1011 | 31 | 5 | 3 |
| 395 | 9311398 | 176 | 714 | 18 | 1 | 0 |
| 396 | 9311399 | 256 | 1442 | 31 | 3 | 2 |
| 397 | 9311400 | 174 | 735 | 10 | 4 | 3 |
| 398 | 9311401 | 199 | 952 | 16 | 5 | 4 |
| 399 | 9311402 | 231 | 1022 | 27 | 7 | 1 |
| 400 | 9311403 | 257 | 1278 | 24 | 4 | 0 |
| 401 | 9311404 | 207 | 955 | 12 | 1 | 0 |
| 402 | 9311405 | 250 | 1024 | 25 | 2 | 2 |
| 403 | 9311406 | 240 | 1055 | 19 | 5 | 2 |
| 404 | 9311407 | 169 | 558 | 11 | 6 | 0 |
| 405 | 9311408 | 196 | 947 | 20 | 3 | 3 |
| 406 | 9311409 | 259 | 1515 | 25 | 5 | 1 |
| 407 | 9311410 | 173 | 632 | 15 | 13 | 0 |
| 408 | 9311411 | 174 | 571 | 12 | 3 | 1 |
| 409 | 9311412 | 201 | 938 | 13 | 4 | 2 |
| 410 | 9311413 | 203 | 761 | 22 | 1 | 2 |
| 411 | 9311414 | 168 | 572 | 12 | 6 | 1 |
| 412 | 9311415 | 202 | 744 | 17 | 5 | 2 |
| 413 | 9311416 | 189 | 825 | 19 | 7 | 2 |
| 414 | 9311417 | 261 | 1399 | 32 | 2 | 0 |
| 415 | 9311418 | 221 | 1219 | 36 | 7 | 2 |
| 416 | 9311419 | 272 | 1451 | 12 | 5 | 2 |
| 417 | 9311420 | 244 | 1286 | 28 | 12 | 3 |
| 418 | 9311421 | 233 | 1067 | 24 | 7 | 0 |
| 419 | 9311422 | 192 | 723 | 16 | 6 | 3 |
| 420 | 9311423 | 276 | 1322 | 30 | 2 | 2 |
| 421 | 9311424 | 325 | 1850 | 15 | 5 | 1 |
| 422 | 9311425 | 211 | 590 | 11 | 3 | 0 |
| 423 | 9311426 | 189 | 755 | 20 | 2 | 1 |
| 424 | 9311427 | 175 | 747 | 10 | 2 | 1 |
| 425 | 9311428 | 178 | 609 | 14 | 4 | 1 |
| 426 | 9311429 | 206 | 694 | 26 | 9 | 3 |
| 427 | 9311430 | 240 | 947 | 21 | 1 | 0 |
| 428 | 9311431 | 192 | 739 | 2 | 7 | 1 |
| 429 | 9311432 | 192 | 1043 | 24 | 3 | 2 |
| 430 | 9311433 | 224 | 1015 | 19 | 5 | 3 |
| 431 | 9311434 | 298 | 1666 | 49 | 5 | 2 |
| 432 | 9311435 | 227 | 987 | 28 | 8 | 3 |
| 433 | 9311436 | 181 | 872 | 17 | 7 | 0 |
| 434 | 9311437 | 253 | 1134 | 25 | 5 | 3 |
| 435 | 9311438 | 218 | 815 | 15 | 3 | 1 |
| 436 | 9311439 | 290 | 1616 | 27 | 7 | 1 |
| 437 | 9311440 | 189 | 697 | 11 | 4 | 1 |
| 438 | 9311441 | 191 | 646 | 25 | 8 | 1 |
| 439 | 9311442 | 207 | 929 | 26 | 6 | 3 |
| 440 | 9311443 | 245 | 1134 | 23 | 3 | 3 |
| 441 | 9311444 | 230 | 1167 | 25 | 8 | 1 |
| 442 | 9311445 | 160 | 748 | 11 | 9 | 4 |
| 443 | 9311446 | 202 | 930 | 23 | 5 | 0 |
| 444 | 9311447 | 221 | 1101 | 15 | 3 | 2 |
| 445 | 9311448 | 280 | 1374 | 23 | 6 | 3 |
| 446 | 9311449 | 226 | 1231 | 19 | 2 | 2 |
| 447 | 9311450 | 186 | 705 | 15 | 5 | 1 |
| 448 | 9311451 | 169 | 762 | 19 | 4 | 2 |
| 449 | 9311452 | 185 | 908 | 21 | 5 | 1 |
| 450 | 9311453 | 196 | 768 | 8 | 5 | 1 |
| 451 | 9311454 | 177 | 798 | 13 | 4 | 2 |
| 452 | 9311455 | 204 | 888 | 15 | 3 | 3 |
| 453 | 9311456 | 180 | 753 | 17 | 3 | 0 |
| 454 | 9311457 | 264 | 1397 | 28 | 4 | 3 |
| 455 | 9311458 | 185 | 735 | 21 | 7 | 3 |
| 456 | 9311459 | 224 | 911 | 24 | 8 | 2 |
| 457 | 9311460 | 242 | 1251 | 24 | 6 | 1 |
| 458 | 9311461 | 224 | 955 | 20 | 1 | 0 |
| 459 | 9311462 | 232 | 1272 | 22 | 4 | 0 |
| 460 | 9311463 | 281 | 1365 | 34 | 11 | 1 |
| 461 | 9311464 | 205 | 946 | 26 | 4 | 0 |
| 462 | 9311465 | 216 | 1120 | 24 | 1 | 2 |
| 463 | 9311466 | 197 | 924 | 23 | 6 | 1 |
| 464 | 9311467 | 263 | 1265 | 24 | 2 | 2 |
| 465 | 9311468 | 169 | 647 | 15 | 3 | 2 |
| 466 | 9311469 | 218 | 890 | 19 | 3 | 1 |
| 467 | 9311470 | 243 | 1248 | 30 | 5 | 2 |
| 468 | 9311471 | 210 | 1128 | 17 | 4 | 0 |
| 469 | 9311472 | 261 | 1363 | 32 | 8 | 3 |
| 470 | 9311473 | 237 | 989 | 27 | 5 | 3 |
| 471 | 9311474 | 247 | 1095 | 30 | 2 | 2 |
| 472 | 9311475 | 224 | 863 | 21 | 3 | 2 |
| 473 | 9311476 | 207 | 811 | 19 | 5 | 2 |
| 474 | 9311477 | 176 | 896 | 17 | 6 | 2 |
| 475 | 9311478 | 236 | 1201 | 28 | 6 | 1 |
| 476 | 9311479 | 161 | 535 | 12 | 4 | 1 |
| 477 | 9311480 | 197 | 767 | 18 | 0 | 2 |
| 478 | 9311481 | 252 | 814 | 17 | 4 | 2 |
| 479 | 9311482 | 229 | 974 | 18 | 4 | 4 |
| 480 | 9311483 | 168 | 790 | 11 | 2 | 1 |
| 481 | 9311484 | 252 | 1238 | 15 | 3 | 1 |
| 482 | 9311485 | 162 | 619 | 24 | 3 | 0 |
| 483 | 9311486 | 216 | 823 | 12 | 4 | 3 |
| 484 | 9311487 | 209 | 972 | 19 | 3 | 2 |
| 485 | 9311488 | 196 | 890 | 8 | 5 | 0 |
| 486 | 9311489 | 228 | 859 | 14 | 12 | 1 |
| 487 | 9311490 | 220 | 1134 | 29 | 3 | 1 |
| 488 | 9311491 | 263 | 1306 | 25 | 8 | 1 |
| 489 | 9311492 | 201 | 879 | 30 | 2 | 1 |
| 490 | 9311493 | 247 | 1358 | 27 | 3 | 1 |
| 491 | 9311494 | 203 | 933 | 21 | 9 | 1 |
| 492 | 9311495 | 240 | 971 | 21 | 7 | 1 |
| 493 | 9311496 | 274 | 1099 | 32 | 5 | 2 |
| 494 | 9311497 | 164 | 654 | 23 | 1 | 2 |
| 495 | 9311498 | 210 | 1094 | 21 | 3 | 2 |
| 496 | 9311499 | 221 | 891 | 12 | 8 | 3 |
| 497 | 9311500 | 185 | 795 | 18 | 4 | 2 |
| 498 | 9311501 | 211 | 590 | 13 | 6 | 2 |
| 499 | 9311502 | 204 | 916 | 20 | 0 | 3 |
| 500 | 9311503 | 230 | 931 | 5 | 10 | 3 |
| 501 | 9311504 | 180 | 698 | 13 | 3 | 1 |
| 502 | 9311505 | 223 | 1180 | 19 | 1 | 1 |
| 503 | 9311506 | 189 | 924 | 22 | 4 | 2 |
| 504 | 9311507 | 206 | 1084 | 27 | 5 | 4 |
| 505 | 9311508 | 190 | 821 | 10 | 3 | 2 |
| 506 | 9311509 | 185 | 649 | 8 | 4 | 0 |
| 507 | 9311510 | 237 | 1277 | 27 | 5 | 1 |
| 508 | 9311511 | 252 | 1054 | 26 | 12 | 2 |
| 509 | 9311512 | 391 | 2020 | 32 | 7 | 3 |
| 510 | 9311513 | 172 | 585 | 5 | 3 | 1 |
| 511 | 9311514 | 184 | 662 | 20 | 5 | 1 |
| 512 | 9311515 | 165 | 549 | 8 | 6 | 2 |
| 513 | 9311516 | 216 | 911 | 26 | 3 | 0 |
| 514 | 9311517 | 239 | 1034 | 14 | 10 | 2 |
| 515 | 9311518 | 219 | 964 | 13 | 5 | 2 |
| 516 | 9311519 | 249 | 1122 | 25 | 15 | 3 |
| 517 | 9311520 | 211 | 784 | 23 | 4 | 3 |
| 518 | 9311521 | 172 | 407 | 8 | 6 | 2 |
| 519 | 9311522 | 164 | 717 | 20 | 3 | 1 |
| 520 | 9311523 | 176 | 837 | 12 | 3 | 0 |
| 521 | 9311524 | 263 | 1106 | 45 | 3 | 1 |
| 522 | 9311525 | 207 | 921 | 14 | 4 | 2 |
| 523 | 9311526 | 195 | 644 | 6 | 6 | 3 |
| 524 | 9311527 | 183 | 845 | 14 | 5 | 2 |
| 525 | 9311528 | 239 | 1302 | 16 | 3 | 2 |
| 526 | 9311529 | 231 | 1089 | 22 | 7 | 1 |
| 527 | 9311530 | 209 | 1063 | 14 | 10 | 1 |
| 528 | 9311531 | 247 | 1336 | 17 | 3 | 2 |
| 529 | 9311532 | 181 | 749 | 18 | 8 | 0 |
| 530 | 9311533 | 230 | 917 | 16 | 9 | 2 |
| 531 | 9311534 | 193 | 714 | 14 | 1 | 2 |
| 532 | 9311535 | 242 | 1076 | 33 | 3 | 2 |
| 533 | 9311536 | 297 | 1377 | 22 | 4 | 4 |
| 534 | 9311537 | 178 | 720 | 18 | 4 | 0 |
| 535 | 9311538 | 208 | 891 | 26 | 4 | 1 |
| 536 | 9311539 | 171 | 800 | 19 | 3 | 2 |
| 537 | 9311540 | 190 | 650 | 18 | 4 | 1 |
| 538 | 9311541 | 192 | 832 | 10 | 3 | 1 |
| 539 | 9311542 | 233 | 950 | 19 | 0 | 1 |
| 540 | 9311543 | 226 | 959 | 20 | 8 | 2 |
| 541 | 9311544 | 243 | 1160 | 29 | 3 | 0 |
| 542 | 9311545 | 193 | 629 | 9 | 3 | 2 |
| 543 | 9311546 | 191 | 651 | 19 | 3 | 1 |
| 544 | 9311547 | 303 | 1597 | 59 | 6 | 3 |
| 545 | 9311548 | 188 | 722 | 8 | 5 | 1 |
| 546 | 9311549 | 217 | 1079 | 29 | 8 | 4 |
| 547 | 9311550 | 188 | 940 | 10 | 7 | 2 |
| 548 | 9311551 | 236 | 781 | 29 | 5 | 4 |
| 549 | 9311552 | 208 | 1005 | 34 | 5 | 2 |
| 550 | 9311553 | 187 | 884 | 23 | 2 | 0 |
| 551 | 9311554 | 203 | 828 | 22 | 5 | 4 |
| 552 | 9311555 | 217 | 1218 | 32 | 7 | 3 |
| 553 | 9311556 | 235 | 1015 | 17 | 10 | 4 |
| 554 | 9311557 | 233 | 783 | 18 | 6 | 3 |
| 555 | 9311558 | 237 | 948 | 26 | 5 | 3 |
| 556 | 9311559 | 241 | 1147 | 29 | 8 | 1 |
| 557 | 9311560 | 193 | 797 | 18 | 3 | 2 |
| 558 | 9311561 | 274 | 1513 | 40 | 5 | 2 |
| 559 | 9311562 | 194 | 983 | 21 | 3 | 0 |
| 560 | 9311563 | 245 | 1115 | 27 | 4 | 3 |
| 561 | 9311564 | 268 | 1411 | 35 | 8 | 2 |
| 562 | 9311565 | 164 | 640 | 9 | 10 | 1 |
| 563 | 9311566 | 216 | 889 | 21 | 4 | 2 |
| 564 | 9311567 | 192 | 847 | 27 | 1 | 2 |
| 565 | 9311568 | 233 | 995 | 16 | 5 | 3 |
| 566 | 9311569 | 240 | 1101 | 21 | 2 | 2 |
| 567 | 9311570 | 181 | 723 | 22 | 3 | 2 |
| 568 | 9311571 | 251 | 1197 | 17 | 7 | 3 |
| 569 | 9311572 | 243 | 1037 | 27 | 7 | 0 |
| 570 | 9311573 | 213 | 850 | 19 | 5 | 2 |
| 571 | 9311574 | 255 | 1254 | 37 | 5 | 1 |
| 572 | 9311575 | 200 | 901 | 23 | 2 | 3 |
| 573 | 9311576 | 272 | 1344 | 34 | 6 | 3 |
| 574 | 9311577 | 244 | 1115 | 34 | 5 | 2 |
| 575 | 9311578 | 229 | 1143 | 30 | 1 | 1 |
| 576 | 9311579 | 180 | 786 | 19 | 6 | 0 |
| 577 | 9311580 | 255 | 1297 | 18 | 8 | 1 |
| 578 | 9311581 | 198 | 899 | 22 | 0 | 0 |
| 579 | 9311582 | 209 | 985 | 15 | 3 | 1 |
| 580 | 9311583 | 186 | 933 | 18 | 4 | 1 |
| 581 | 9311584 | 253 | 1164 | 26 | 8 | 1 |
| 582 | 9311585 | 216 | 1228 | 20 | 3 | 1 |
| 583 | 9311586 | 202 | 611 | 15 | 6 | 2 |
| 584 | 9311587 | 203 | 907 | 20 | 7 | 1 |
| 585 | 9311588 | 252 | 1331 | 28 | 5 | 4 |
| 586 | 9311589 | 227 | 904 | 12 | 1 | 1 |
| 587 | 9311590 | 223 | 723 | 18 | 2 | 2 |
| 588 | 9311591 | 220 | 1036 | 38 | 7 | 5 |
| 589 | 9311592 | 194 | 764 | 12 | 9 | 2 |
| 590 | 9311593 | 199 | 716 | 15 | 5 | 4 |
| 591 | 9311594 | 244 | 1074 | 19 | 6 | 2 |
| 592 | 9311595 | 228 | 1201 | 27 | 8 | 2 |
| 593 | 9311596 | 209 | 734 | 10 | 5 | 2 |
| 594 | 9311597 | 207 | 784 | 17 | 5 | 3 |
| 595 | 9311598 | 210 | 798 | 16 | 4 | 1 |
| 596 | 9311599 | 199 | 781 | 23 | 7 | 1 |
| 597 | 9311600 | 250 | 1119 | 23 | 5 | 2 |
| 598 | 9311601 | 149 | 539 | 6 | 5 | 2 |
| 599 | 9311602 | 175 | 642 | 10 | 3 | 2 |
| 600 | 9311603 | 183 | 692 | 9 | 9 | 1 |
| 601 | 9311604 | 231 | 1039 | 29 | 5 | 3 |
| 602 | 9311605 | 258 | 1293 | 22 | 4 | 1 |
| 603 | 9311606 | 189 | 960 | 13 | 5 | 1 |
| 604 | 9311607 | 195 | 893 | 13 | 9 | 3 |
| 605 | 9311608 | 242 | 662 | 15 | 20 | 1 |
| 606 | 9311609 | 270 | 1347 | 18 | 7 | 1 |
| 607 | 9311610 | 262 | 1245 | 24 | 12 | 1 |
| 608 | 9311611 | 198 | 743 | 22 | 10 | 3 |
| 609 | 9311612 | 269 | 1475 | 27 | 3 | 3 |
| 610 | 9311613 | 240 | 1171 | 40 | 3 | 0 |
| 611 | 9311614 | 218 | 1243 | 33 | 3 | 2 |
| 612 | 9311615 | 207 | 826 | 15 | 4 | 2 |
| 613 | 9311616 | 176 | 578 | 11 | 3 | 1 |
| 614 | 9311617 | 251 | 1353 | 16 | 3 | 1 |
| 615 | 9311618 | 177 | 669 | 16 | 3 | 2 |
| 616 | 9311619 | 200 | 616 | 16 | 6 | 0 |
| 617 | 9311620 | 192 | 765 | 13 | 4 | 1 |
| 618 | 9311621 | 247 | 1293 | 32 | 4 | 1 |
| 619 | 9311622 | 233 | 1283 | 21 | 4 | 0 |
| 620 | 9311623 | 204 | 1007 | 35 | 8 | 2 |
| 621 | 9311624 | 190 | 647 | 12 | 2 | 1 |
| 622 | 9311625 | 175 | 700 | 18 | 3 | 0 |
| 623 | 9311626 | 223 | 934 | 17 | 8 | 1 |
| 624 | 9311627 | 193 | 763 | 18 | 5 | 3 |
| 625 | 9311628 | 241 | 1077 | 18 | 3 | 3 |
| 626 | 9311629 | 217 | 1150 | 36 | 4 | 1 |
| 627 | 9311630 | 182 | 788 | 6 | 4 | 1 |
| 628 | 9311631 | 257 | 1265 | 23 | 6 | 6 |
| 629 | 9311632 | 223 | 1075 | 19 | 11 | 1 |
| 630 | 9311633 | 276 | 1522 | 36 | 4 | 1 |
| 631 | 9311634 | 215 | 901 | 14 | 16 | 2 |
| 632 | 9311635 | 209 | 1016 | 25 | 6 | 2 |
| 633 | 9311636 | 277 | 1520 | 27 | 8 | 3 |
| 634 | 9311637 | 188 | 802 | 19 | 1 | 2 |
| 635 | 9311638 | 246 | 1119 | 18 | 1 | 1 |
| 636 | 9311639 | 178 | 822 | 19 | 5 | 0 |
| 637 | 9311640 | 216 | 954 | 24 | 6 | 4 |
| 638 | 9311641 | 229 | 1029 | 19 | 1 | 0 |
| 639 | 9311642 | 268 | 1454 | 27 | 6 | 2 |
| 640 | 9311643 | 197 | 956 | 10 | 5 | 1 |
| 641 | 9311644 | 255 | 1342 | 20 | 9 | 2 |
| 642 | 9311645 | 168 | 558 | 9 | 2 | 0 |
| 643 | 9311646 | 222 | 997 | 14 | 5 | 2 |
| 644 | 9311647 | 230 | 1334 | 21 | 4 | 1 |
| 645 | 9311648 | 295 | 1376 | 33 | 8 | 0 |
| 646 | 9311649 | 234 | 1087 | 27 | 0 | 0 |
| 647 | 9311650 | 193 | 660 | 22 | 3 | 2 |
| 648 | 9311651 | 296 | 1620 | 33 | 6 | 1 |
| 649 | 9311652 | 196 | 882 | 13 | 3 | 3 |
| 650 | 9311653 | 208 | 1044 | 25 | 8 | 3 |
| 651 | 9311654 | 262 | 909 | 20 | 6 | 2 |
| 652 | 9311655 | 162 | 609 | 14 | 4 | 3 |
| 653 | 9311656 | 204 | 890 | 28 | 5 | 1 |
| 654 | 9311657 | 200 | 891 | 24 | 3 | 2 |
| 655 | 9311658 | 231 | 1199 | 26 | 5 | 0 |
| 656 | 9311659 | 188 | 832 | 21 | 2 | 1 |
| 657 | 9311660 | 160 | 657 | 11 | 5 | 1 |
| 658 | 9311661 | 208 | 782 | 21 | 6 | 3 |
| 659 | 9311662 | 181 | 883 | 13 | 4 | 0 |
| 660 | 9311663 | 183 | 700 | 21 | 12 | 2 |
| 661 | 9311664 | 225 | 1042 | 23 | 3 | 3 |
| 662 | 9311665 | 275 | 1449 | 37 | 4 | 1 |
| 663 | 9311666 | 269 | 1282 | 28 | 4 | 0 |
| 664 | 9311667 | 244 | 1067 | 40 | 7 | 4 |
| 665 | 9311668 | 216 | 1147 | 26 | 4 | 2 |
| 666 | 9311669 | 196 | 928 | 17 | 7 | 2 |
| 667 | 9311670 | 229 | 1063 | 18 | 2 | 0 |
| 668 | 9311671 | 212 | 754 | 19 | 4 | 2 |
| 669 | 9311672 | 208 | 834 | 23 | 7 | 2 |
| 670 | 9311673 | 256 | 1203 | 32 | 7 | 3 |
| 671 | 9311674 | 182 | 909 | 16 | 5 | 1 |
| 672 | 9311675 | 163 | 494 | 11 | 4 | 0 |
| 673 | 9311676 | 320 | 2181 | 37 | 7 | 4 |
| 674 | 9311677 | 215 | 947 | 28 | 2 | 1 |
| 675 | 9311678 | 193 | 845 | 19 | 2 | 1 |
| 676 | 9311679 | 204 | 921 | 18 | 2 | 1 |
| 677 | 9311680 | 177 | 825 | 28 | 2 | 2 |
| 678 | 9311681 | 188 | 797 | 9 | 5 | 1 |
| 679 | 9311682 | 180 | 488 | 16 | 5 | 2 |
| 680 | 9311683 | 199 | 830 | 23 | 3 | 1 |
| 681 | 9311684 | 211 | 1023 | 20 | 2 | 2 |
| 682 | 9311685 | 199 | 910 | 22 | 6 | 1 |
| 683 | 9311686 | 236 | 881 | 21 | 5 | 3 |
| 684 | 9311687 | 167 | 556 | 17 | 8 | 3 |
| 685 | 9311688 | 200 | 646 | 5 | 2 | 2 |
| 686 | 9311689 | 222 | 925 | 22 | 3 | 4 |
| 687 | 9311690 | 191 | 814 | 22 | 6 | 2 |
| 688 | 9311691 | 196 | 887 | 23 | 6 | 1 |
| 689 | 9311692 | 310 | 1522 | 22 | 10 | 2 |
| 690 | 9311693 | 281 | 1683 | 17 | 2 | 1 |
| 691 | 9311694 | 199 | 763 | 23 | 1 | 2 |
| 692 | 9311695 | 175 | 903 | 19 | 4 | 1 |
| 693 | 9311696 | 203 | 697 | 13 | 2 | 0 |
| 694 | 9311697 | 286 | 1257 | 36 | 5 | 2 |
| 695 | 9311698 | 205 | 914 | 17 | 2 | 1 |
| 696 | 9311699 | 181 | 937 | 14 | 1 | 2 |
| 697 | 9311700 | 225 | 1024 | 31 | 3 | 0 |
| 698 | 9311701 | 182 | 587 | 13 | 9 | 1 |
| 699 | 9311702 | 192 | 798 | 11 | 7 | 1 |
| 700 | 9311703 | 198 | 663 | 17 | 3 | 2 |
| 701 | 9311704 | 219 | 825 | 22 | 4 | 0 |
| 702 | 9311705 | 209 | 1222 | 33 | 4 | 4 |
| 703 | 9311706 | 246 | 1024 | 23 | 4 | 2 |
| 704 | 9311707 | 214 | 844 | 37 | 7 | 1 |
| 705 | 9311708 | 202 | 869 | 22 | 18 | 0 |
| 706 | 9311709 | 224 | 1093 | 21 | 4 | 2 |
| 707 | 9311710 | 231 | 1040 | 20 | 6 | 2 |
| 708 | 9311711 | 227 | 1071 | 23 | 2 | 2 |
| 709 | 9311712 | 201 | 861 | 18 | 6 | 3 |
| 710 | 9311713 | 242 | 1010 | 24 | 3 | 3 |
| 711 | 9311714 | 232 | 1301 | 25 | 5 | 1 |
| 712 | 9311715 | 228 | 1040 | 16 | 4 | 1 |
| 713 | 9311716 | 217 | 1039 | 14 | 9 | 2 |
| 714 | 9311717 | 190 | 729 | 11 | 7 | 1 |
| 715 | 9311718 | 192 | 836 | 23 | 4 | 0 |
| 716 | 9311719 | 236 | 1079 | 27 | 6 | 1 |
| 717 | 9311720 | 201 | 726 | 17 | 4 | 1 |
| 718 | 9311721 | 201 | 829 | 17 | 6 | 1 |
| 719 | 9311722 | 205 | 954 | 23 | 7 | 1 |
| 720 | 9311723 | 163 | 573 | 11 | 2 | 0 |
| 721 | 9311724 | 250 | 1122 | 18 | 8 | 3 |
| 722 | 9311725 | 203 | 1005 | 8 | 2 | 3 |
| 723 | 9311726 | 209 | 1059 | 17 | 6 | 2 |
| 724 | 9311727 | 227 | 1145 | 37 | 2 | 1 |
| 725 | 9311728 | 218 | 1204 | 31 | 1 | 2 |
| 726 | 9311729 | 243 | 1377 | 29 | 4 | 1 |
| 727 | 9311730 | 198 | 872 | 26 | 8 | 0 |
| 728 | 9311731 | 313 | 1490 | 35 | 3 | 2 |
| 729 | 9311732 | 256 | 833 | 20 | 4 | 2 |
| 730 | 9311733 | 220 | 1282 | 37 | 6 | 1 |
| 731 | 9311734 | 204 | 944 | 28 | 2 | 3 |
| 732 | 9311735 | 217 | 971 | 26 | 13 | 2 |
| 733 | 9311736 | 219 | 864 | 16 | 6 | 1 |
| 734 | 9311737 | 250 | 1129 | 21 | 7 | 3 |
| 735 | 9311738 | 211 | 985 | 22 | 4 | 2 |
| 736 | 9311739 | 204 | 960 | 33 | 3 | 1 |
| 737 | 9311740 | 240 | 1147 | 23 | 3 | 1 |
| 738 | 9311741 | 275 | 1251 | 31 | 7 | 5 |
| 739 | 9311742 | 184 | 771 | 21 | 1 | 1 |
| 740 | 9311743 | 199 | 1044 | 12 | 5 | 0 |
| 741 | 9311744 | 248 | 1099 | 27 | 3 | 1 |
| 742 | 9311745 | 199 | 920 | 24 | 1 | 1 |
| 743 | 9311746 | 310 | 1671 | 27 | 8 | 3 |
| 744 | 9311747 | 258 | 1574 | 35 | 7 | 1 |
| 745 | 9311748 | 210 | 1069 | 28 | 5 | 3 |
| 746 | 9311749 | 189 | 956 | 20 | 4 | 1 |
| 747 | 9311750 | 227 | 837 | 24 | 1 | 2 |
| 748 | 9311751 | 190 | 597 | 13 | 8 | 1 |
| 749 | 9311752 | 303 | 1691 | 30 | 1 | 1 |
| 750 | 9311753 | 228 | 1140 | 33 | 8 | 3 |
| 751 | 9311754 | 221 | 1042 | 26 | 3 | 0 |
| 752 | 9311755 | 237 | 886 | 18 | 3 | 1 |
| 753 | 9311756 | 270 | 1185 | 28 | 5 | 3 |
| 754 | 9311757 | 268 | 1205 | 11 | 5 | 2 |
| 755 | 9311758 | 189 | 746 | 17 | 7 | 1 |
| 756 | 9311759 | 192 | 689 | 23 | 3 | 2 |
| 757 | 9311760 | 218 | 897 | 17 | 3 | 1 |
| 758 | 9311761 | 240 | 1155 | 25 | 8 | 1 |
| 759 | 9311762 | 249 | 1273 | 25 | 3 | 2 |
| 760 | 9311763 | 241 | 1311 | 26 | 0 | 1 |
| 761 | 9311764 | 217 | 889 | 24 | 7 | 2 |
| 762 | 9311765 | 255 | 1240 | 9 | 2 | 3 |
| 763 | 9311766 | 259 | 1160 | 28 | 13 | 2 |
| 764 | 9311767 | 193 | 836 | 17 | 5 | 2 |
| 765 | 9311768 | 143 | 504 | 11 | 8 | 0 |
| 766 | 9311769 | 255 | 1112 | 25 | 6 | 4 |
| 767 | 9311770 | 223 | 1103 | 25 | 4 | 1 |
| 768 | 9311771 | 244 | 950 | 17 | 5 | 2 |
| 769 | 9311772 | 223 | 961 | 23 | 6 | 2 |
| 770 | 9311773 | 169 | 760 | 10 | 3 | 1 |
| 771 | 9311774 | 266 | 1211 | 34 | 6 | 3 |
| 772 | 9311775 | 228 | 1177 | 26 | 4 | 2 |
| 773 | 9311776 | 301 | 1329 | 23 | 10 | 2 |
| 774 | 9311777 | 278 | 1657 | 31 | 1 | 1 |
| 775 | 9311778 | 171 | 674 | 12 | 8 | 0 |
| 776 | 9311779 | 168 | 653 | 12 | 2 | 0 |
| 777 | 9311780 | 241 | 1176 | 15 | 6 | 2 |
| 778 | 9311781 | 235 | 999 | 18 | 17 | 4 |
| 779 | 9311782 | 184 | 795 | 11 | 5 | 1 |
| 780 | 9311783 | 259 | 1201 | 16 | 4 | 2 |
| 781 | 9311784 | 287 | 1474 | 27 | 9 | 0 |
| 782 | 9311785 | 214 | 1000 | 20 | 2 | 2 |
| 783 | 9311786 | 210 | 1090 | 12 | 2 | 1 |
| 784 | 9311787 | 391 | 2062 | 41 | 7 | 4 |
| 785 | 9311788 | 199 | 848 | 19 | 0 | 1 |
| 786 | 9311789 | 182 | 686 | 14 | 4 | 1 |
| 787 | 9311790 | 196 | 947 | 14 | 3 | 1 |
| 788 | 9311791 | 190 | 823 | 6 | 7 | 2 |
| 789 | 9311792 | 261 | 1220 | 29 | 6 | 2 |
| 790 | 9311793 | 192 | 898 | 21 | 4 | 2 |
| 791 | 9311794 | 231 | 1040 | 22 | 7 | 0 |
| 792 | 9311795 | 270 | 1147 | 28 | 17 | 3 |
| 793 | 9311796 | 231 | 1331 | 20 | 7 | 2 |
| 794 | 9311797 | 190 | 620 | 20 | 4 | 2 |
| 795 | 9311798 | 257 | 1280 | 17 | 6 | 3 |
| 796 | 9311799 | 231 | 1013 | 18 | 2 | 1 |
| 797 | 9311800 | 224 | 810 | 23 | 6 | 1 |
| 798 | 9311801 | 240 | 727 | 16 | 5 | 1 |
| 799 | 9311802 | 230 | 1035 | 20 | 2 | 2 |
| 800 | 9311803 | 192 | 949 | 20 | 6 | 1 |
| 801 | 9311804 | 194 | 721 | 12 | 6 | 1 |
| 802 | 9311805 | 236 | 978 | 17 | 4 | 2 |
| 803 | 9311806 | 265 | 1432 | 31 | 6 | 3 |
| 804 | 9311807 | 244 | 1333 | 36 | 5 | 2 |
| 805 | 9311808 | 228 | 1122 | 18 | 6 | 2 |
| 806 | 9311809 | 237 | 944 | 23 | 11 | 0 |
| 807 | 9311810 | 180 | 813 | 20 | 11 | 3 |
| 808 | 9311811 | 162 | 651 | 8 | 2 | 0 |
| 809 | 9311812 | 210 | 732 | 16 | 5 | 3 |
| 810 | 9311813 | 194 | 810 | 18 | 2 | 2 |
| 811 | 9311814 | 276 | 1182 | 26 | 5 | 2 |
| 812 | 9311815 | 213 | 1112 | 31 | 3 | 2 |
| 813 | 9311816 | 214 | 945 | 23 | 4 | 2 |
| 814 | 9311817 | 205 | 957 | 25 | 2 | 3 |
| 815 | 9311818 | 240 | 1114 | 21 | 2 | 0 |
| 816 | 9311819 | 279 | 1422 | 26 | 6 | 2 |
| 817 | 9311820 | 167 | 543 | 6 | 3 | 0 |
| 818 | 9311821 | 286 | 1215 | 17 | 6 | 4 |
| 819 | 9311822 | 210 | 850 | 22 | 8 | 2 |
| 820 | 9311823 | 206 | 1032 | 31 | 9 | 2 |
| 821 | 9311824 | 274 | 1239 | 21 | 9 | 0 |
| 822 | 9311825 | 236 | 1295 | 30 | 3 | 1 |
| 823 | 9311826 | 220 | 896 | 18 | 4 | 0 |
| 824 | 9311827 | 242 | 1231 | 31 | 15 | 1 |
| 825 | 9311828 | 270 | 1009 | 17 | 3 | 2 |
| 826 | 9311829 | 189 | 600 | 11 | 6 | 2 |
| 827 | 9311830 | 242 | 1348 | 29 | 3 | 3 |
| 828 | 9311831 | 215 | 1032 | 22 | 6 | 1 |
| 829 | 9311832 | 238 | 1048 | 31 | 4 | 4 |
| 830 | 9311833 | 188 | 688 | 17 | 3 | 1 |
| 831 | 9311834 | 222 | 979 | 24 | 2 | 2 |
| 832 | 9311835 | 201 | 813 | 8 | 5 | 2 |
| 833 | 9311836 | 206 | 912 | 23 | 3 | 3 |
| 834 | 9311837 | 246 | 1080 | 19 | 7 | 3 |
| 835 | 9311838 | 161 | 695 | 13 | 5 | 2 |
| 836 | 9311839 | 204 | 886 | 21 | 3 | 1 |
| 837 | 9311840 | 197 | 1014 | 23 | 5 | 0 |
| 838 | 9311841 | 246 | 1172 | 22 | 11 | 1 |
| 839 | 9311842 | 260 | 1230 | 44 | 5 | 3 |
| 840 | 9311843 | 225 | 1065 | 26 | 2 | 2 |
| 841 | 9311844 | 200 | 1007 | 15 | 0 | 1 |
| 842 | 9311845 | 208 | 1059 | 15 | 6 | 4 |
| 843 | 9311846 | 247 | 1169 | 14 | 1 | 2 |
| 844 | 9311847 | 212 | 903 | 11 | 4 | 1 |
| 845 | 9311848 | 210 | 1120 | 17 | 1 | 1 |
| 846 | 9311849 | 233 | 1107 | 32 | 5 | 5 |
| 847 | 9311850 | 274 | 1241 | 15 | 2 | 4 |
| 848 | 9311851 | 175 | 746 | 12 | 3 | 2 |
| 849 | 9311852 | 218 | 980 | 21 | 4 | 2 |
| 850 | 9311853 | 217 | 1095 | 25 | 0 | 1 |
| 851 | 9311854 | 232 | 1151 | 29 | 5 | 0 |
| 852 | 9311855 | 236 | 930 | 15 | 5 | 1 |
| 853 | 9311856 | 187 | 680 | 17 | 3 | 1 |
| 854 | 9311857 | 166 | 718 | 11 | 1 | 1 |
| 855 | 9311858 | 180 | 667 | 8 | 5 | 3 |
| 856 | 9311859 | 206 | 555 | 19 | 3 | 2 |
| 857 | 9311860 | 258 | 1321 | 32 | 11 | 3 |
| 858 | 9311861 | 207 | 882 | 28 | 8 | 2 |
| 859 | 9311862 | 345 | 1550 | 37 | 9 | 2 |
| 860 | 9311863 | 193 | 619 | 17 | 9 | 2 |
| 861 | 9311864 | 232 | 1247 | 21 | 4 | 1 |
| 862 | 9311865 | 187 | 942 | 28 | 3 | 1 |
| 863 | 9311866 | 238 | 1206 | 31 | 1 | 3 |
| 864 | 9311867 | 271 | 1067 | 25 | 5 | 3 |
| 865 | 9311868 | 289 | 1575 | 19 | 4 | 2 |
| 866 | 9311869 | 191 | 798 | 18 | 5 | 2 |
| 867 | 9311870 | 191 | 628 | 17 | 9 | 2 |
| 868 | 9311871 | 196 | 1058 | 20 | 5 | 3 |
| 869 | 9311872 | 241 | 1376 | 21 | 6 | 1 |
| 870 | 9311873 | 250 | 1315 | 33 | 7 | 4 |
| 871 | 9311874 | 220 | 1029 | 21 | 5 | 2 |
| 872 | 9311875 | 207 | 1122 | 20 | 1 | 1 |
| 873 | 9311876 | 229 | 786 | 18 | 3 | 2 |
| 874 | 9311877 | 525 | 3248 | 42 | 3 | 2 |
| 875 | 9311878 | 166 | 679 | 17 | 5 | 2 |
| 876 | 9311879 | 203 | 905 | 18 | 2 | 2 |
| 877 | 9311880 | 188 | 638 | 14 | 3 | 2 |
| 878 | 9311881 | 205 | 970 | 19 | 6 | 2 |
| 879 | 9311882 | 213 | 830 | 22 | 1 | 1 |
| 880 | 9311883 | 245 | 607 | 19 | 4 | 1 |
| 881 | 9311884 | 254 | 1269 | 29 | 4 | 2 |
| 882 | 9311885 | 199 | 800 | 29 | 5 | 4 |
| 883 | 9311886 | 252 | 911 | 34 | 12 | 1 |
| 884 | 9311887 | 216 | 846 | 15 | 2 | 0 |
| 885 | 9311888 | 171 | 580 | 15 | 1 | 2 |
| 886 | 9311889 | 191 | 642 | 16 | 8 | 3 |
| 887 | 9311890 | 179 | 726 | 14 | 2 | 3 |
| 888 | 9311891 | 196 | 745 | 13 | 11 | 1 |
| 889 | 9311892 | 236 | 925 | 14 | 3 | 1 |
| 890 | 9311893 | 236 | 1258 | 43 | 2 | 3 |
| 891 | 9311894 | 219 | 882 | 26 | 4 | 1 |
| 892 | 9311895 | 202 | 947 | 22 | 7 | 2 |
| 893 | 9311896 | 246 | 1339 | 23 | 4 | 1 |
| 894 | 9311897 | 181 | 788 | 19 | 3 | 1 |
| 895 | 9311898 | 172 | 671 | 15 | 2 | 2 |
| 896 | 9311899 | 287 | 1580 | 22 | 13 | 3 |
| 897 | 9311900 | 208 | 985 | 35 | 7 | 3 |
| 898 | 9311901 | 242 | 1149 | 40 | 6 | 1 |
| 899 | 9311902 | 308 | 1511 | 32 | 4 | 2 |
| 900 | 9311903 | 247 | 1344 | 28 | 3 | 3 |
| 901 | 9311904 | 251 | 1397 | 23 | 6 | 0 |
| 902 | 9311905 | 258 | 1215 | 33 | 2 | 1 |
| 903 | 9311906 | 240 | 1206 | 32 | 1 | 2 |
| 904 | 9311907 | 227 | 1167 | 22 | 5 | 1 |
| 905 | 9311908 | 230 | 1050 | 26 | 5 | 2 |
| 906 | 9311909 | 218 | 1156 | 18 | 0 | 0 |
| 907 | 9311910 | 196 | 797 | 17 | 1 | 2 |
| 908 | 9311911 | 216 | 968 | 12 | 6 | 1 |
| 909 | 9311912 | 190 | 777 | 15 | 5 | 1 |
| 910 | 9311913 | 208 | 937 | 27 | 8 | 2 |
| 911 | 9311914 | 297 | 1213 | 28 | 5 | 2 |
| 912 | 9311915 | 177 | 639 | 22 | 3 | 2 |
| 913 | 9311916 | 277 | 1461 | 28 | 2 | 3 |
| 914 | 9311917 | 193 | 837 | 20 | 2 | 2 |
| 915 | 9311918 | 208 | 659 | 20 | 1 | 1 |
| 916 | 9311919 | 284 | 1357 | 40 | 6 | 2 |
| 917 | 9311920 | 190 | 765 | 21 | 3 | 3 |
| 918 | 9311921 | 253 | 1360 | 30 | 6 | 2 |
| 919 | 9311922 | 218 | 1055 | 20 | 4 | 0 |
| 920 | 9311923 | 230 | 1108 | 16 | 10 | 1 |
| 921 | 9311924 | 200 | 879 | 12 | 2 | 0 |
| 922 | 9311925 | 208 | 911 | 24 | 12 | 3 |
| 923 | 9311926 | 234 | 945 | 19 | 2 | 3 |
| 924 | 9311927 | 223 | 1086 | 27 | 2 | 1 |
| 925 | 9311928 | 253 | 1125 | 26 | 6 | 0 |
| 926 | 9311929 | 257 | 1267 | 21 | 3 | 1 |
| 927 | 9311930 | 195 | 967 | 18 | 4 | 2 |
| 928 | 9311931 | 226 | 888 | 14 | 7 | 2 |
| 929 | 9311932 | 204 | 829 | 18 | 2 | 1 |
| 930 | 9311933 | 242 | 1123 | 21 | 5 | 1 |
| 931 | 9311934 | 194 | 1022 | 21 | 3 | 2 |
| 932 | 9311935 | 234 | 1209 | 37 | 10 | 3 |
| 933 | 9311936 | 250 | 1305 | 27 | 5 | 2 |
| 934 | 9311937 | 171 | 678 | 7 | 3 | 1 |
| 935 | 9311938 | 290 | 1696 | 33 | 1 | 0 |
| 936 | 9311939 | 208 | 860 | 27 | 2 | 2 |
| 937 | 9311940 | 234 | 957 | 36 | 5 | 5 |
| 938 | 9311941 | 210 | 1039 | 21 | 3 | 0 |
| 939 | 9311942 | 196 | 977 | 10 | 7 | 1 |
| 940 | 9311943 | 264 | 1233 | 23 | 12 | 0 |
| 941 | 9311944 | 273 | 1376 | 26 | 7 | 0 |
| 942 | 9311945 | 227 | 1018 | 22 | 4 | 2 |
| 943 | 9311946 | 197 | 746 | 19 | 6 | 2 |
| 944 | 9311947 | 240 | 1148 | 14 | 5 | 1 |
| 945 | 9311948 | 235 | 897 | 21 | 11 | 2 |
| 946 | 9311949 | 298 | 1478 | 35 | 3 | 2 |
| 947 | 9311950 | 232 | 695 | 18 | 3 | 2 |
| 948 | 9311951 | 226 | 931 | 10 | 4 | 3 |
| 949 | 9311952 | 211 | 1030 | 27 | 3 | 1 |
| 950 | 9311953 | 193 | 875 | 16 | 5 | 2 |
| 951 | 9311954 | 267 | 1225 | 16 | 6 | 2 |
| 952 | 9311955 | 264 | 1283 | 37 | 4 | 3 |
| 953 | 9311956 | 201 | 525 | 11 | 6 | 3 |
| 954 | 9311957 | 198 | 996 | 28 | 2 | 2 |
| 955 | 9311958 | 186 | 830 | 22 | 3 | 2 |
| 956 | 9311959 | 234 | 939 | 16 | 8 | 2 |
| 957 | 9311960 | 234 | 950 | 29 | 4 | 2 |
| 958 | 9311961 | 215 | 918 | 19 | 3 | 3 |
| 959 | 9311962 | 267 | 1242 | 35 | 1 | 3 |
| 960 | 9311963 | 259 | 1202 | 34 | 3 | 3 |
| 961 | 9311964 | 217 | 1039 | 27 | 0 | 1 |
| 962 | 9311965 | 238 | 1037 | 13 | 1 | 3 |
| 963 | 9311966 | 198 | 821 | 19 | 3 | 0 |
| 964 | 9311967 | 161 | 546 | 7 | 5 | 0 |
| 965 | 9311968 | 214 | 941 | 28 | 8 | 3 |
| 966 | 9311969 | 175 | 750 | 13 | 3 | 1 |
| 967 | 9311970 | 180 | 811 | 16 | 5 | 1 |
| 968 | 9311971 | 213 | 959 | 17 | 6 | 0 |
| 969 | 9311972 | 278 | 1429 | 30 | 5 | 0 |
| 970 | 9311973 | 186 | 667 | 19 | 5 | 2 |
| 971 | 9311974 | 156 | 776 | 12 | 3 | 0 |
| 972 | 9311975 | 198 | 648 | 13 | 2 | 1 |
| 973 | 9311976 | 343 | 1593 | 40 | 6 | 2 |
| 974 | 9311977 | 304 | 1401 | 28 | 13 | 2 |
| 975 | 9311978 | 270 | 1319 | 18 | 6 | 0 |
| 976 | 9311979 | 291 | 1572 | 27 | 5 | 1 |
| 977 | 9311980 | 226 | 1118 | 26 | 10 | 1 |
| 978 | 9311981 | 200 | 762 | 22 | 3 | 3 |
| 979 | 9311982 | 202 | 863 | 20 | 2 | 2 |
| 980 | 9311983 | 256 | 1259 | 21 | 7 | 3 |
| 981 | 9311984 | 282 | 1553 | 22 | 4 | 1 |
| 982 | 9311985 | 209 | 974 | 20 | 2 | 2 |
| 983 | 9311986 | 204 | 912 | 20 | 6 | 3 |
| 984 | 9311987 | 189 | 822 | 10 | 4 | 3 |
| 985 | 9311988 | 198 | 776 | 15 | 2 | 0 |
| 986 | 9311989 | 252 | 1283 | 21 | 4 | 1 |
| 987 | 9311990 | 223 | 933 | 27 | 10 | 3 |
| 988 | 9311991 | 181 | 798 | 12 | 4 | 0 |
| 989 | 9311992 | 195 | 930 | 17 | 7 | 1 |
| 990 | 9311993 | 228 | 1011 | 21 | 8 | 1 |
| 991 | 9311994 | 258 | 1403 | 43 | 2 | 2 |
| 992 | 9311995 | 182 | 820 | 17 | 3 | 1 |
| 993 | 9311996 | 197 | 826 | 24 | 7 | 2 |
| 994 | 9311997 | 249 | 1192 | 27 | 9 | 4 |
| 995 | 9311998 | 210 | 903 | 15 | 6 | 0 |
| 996 | 9311999 | 342 | 1716 | 34 | 2 | 2 |
| 997 | 9312000 | 205 | 901 | 25 | 5 | 2 |
| 998 | 9312001 | 170 | 585 | 12 | 4 | 0 |
| 999 | 9312002 | 183 | 859 | 21 | 1 | 1 |

## Throughput

- Turns simulated: 220913 (first play of each game)
- Aggregate: 178.4 turns/s over 1238.5s wall
- Per game thread: 29.8 turns/s

## Determinism

Not checked (run with `--determinism`).

## Panics

None.
