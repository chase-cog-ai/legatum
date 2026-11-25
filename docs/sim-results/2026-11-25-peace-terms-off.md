# Simulation: terms-off (2026-11-25)

1000 games, first seed 9311003, brain `scripted`, turn limit off, safety cap 1200, 6 threads, started 2026-11-25T07:16:46Z. Wall time 1168.2s.

Source checkout at run start: `7f15d6d23a680769e3d2fb1e24a5de58de621254`; dirty: no. Map 48×32, 8 initial seats. `first_setup` and exact binary argv are recorded in JSON. Ordinary games use seed + i (wrapping); the strategy tournament schedule is specified below. All games use a deterministic shuffle of the full faction pool, taking eight distinct factions, with any Proxy rotation described below.

Command argv: `["./sim","--players","8","--difficulty","hard","--games","1000","--seed","9311003","--ai-params","off.json","--label","terms-off","--out-dir","out","--threads","6"]`

## Overview

| Metric | Value |
| --- | --- |
| Games (non-panic) | 1000 of 1000 |
| Game length, turns (mean / median / min-max) | 220.4 / 215 / 136-497 |
| Win reason mix | VP hold 98.8%, Capitals 1.2% |
| Median end turn | 215 |
| Ending shares of played games (capitals / VP hold / turn limit / other) | 1.2% / 98.8% / 0.0% / 0.0% |
| Games reaching the World War era (any empire) | 991 of 1000 (99.1%) |
| VP hold length | 10 turn ends (the reaching turn counts) |
| Holds started / broken / completed / unfinished | 1496 / 390 / 994 / 112 |
| Hold break rate | 26.1% of started; 28.2% of resolved (broken / (broken + completed)) |
| Hold wins | 988 |
| First reach to win, turns (mean / median / min-max) | 10.9 / 9 / 9-109 |
| Final hold start to win, turns | 9.0 / 9 / 9-9 |
| Comebacks (winner broke an earlier hold of its own) | 108 |
| Wins by a team another team beat to its target | 84 |

Game length by how the game ended (turns resolved; a stall is a safety-cap stop, never a win):

| Ended by | Games | Length mean / median / min-max |
| --- | --- | --- |
| VP hold | 988 | 220.2 / 215 / 136-497 |
| Capitals | 12 | 240.0 / 241 / 209-279 |

Winner share by faction (wins of that faction's seats / all winning seats):

| Faction | Wins | Share of wins |
| --- | --- | --- |
| Switzerland | 92 | 9.2% |
| India | 76 | 7.6% |
| Byzantium | 71 | 7.1% |
| Germany | 145 | 14.5% |
| Celts | 142 | 14.2% |
| Mongols | 72 | 7.2% |
| Sparta | 116 | 11.6% |
| Athens | 160 | 16.0% |
| Aztecs | 126 | 12.6% |

## Outcomes

| Outcome | Games | Share |
| --- | --- | --- |
| VP hold | 988 | 98.8% |
| Conquest | 0 | 0.0% |
| Capitals | 12 | 1.2% |
| Turn limit | 0 | 0.0% |
| Draw | 0 | 0.0% |
| Stalled (safety cap) | 0 | 0.0% |
| Panic | 0 | 0.0% |

## Win rate by faction

Per seat: wins / seats of that faction. With 8 single-empire teams, an even game is 12.5% per seat.

| Faction | Seats | Wins | Win rate |
| --- | --- | --- | --- |
| Switzerland | 879 | 92 | 10.5% |
| India | 905 | 76 | 8.4% |
| Byzantium | 885 | 71 | 8.0% |
| Germany | 900 | 145 | 16.1% |
| Celts | 880 | 142 | 16.1% |
| Mongols | 878 | 72 | 8.2% |
| Sparta | 901 | 116 | 12.9% |
| Athens | 900 | 160 | 17.8% |
| Aztecs | 872 | 126 | 14.4% |

## Win rate by difficulty

All initial seats in non-panicked games, including eliminated empires and cap stops. Determinism replays are not additional samples.

| Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- |
| Hard | 8000 | 1000 | 12.5% |

## Win rate by faction and difficulty

| Faction | Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- | --- |
| Switzerland | Hard | 879 | 92 | 10.5% |
| India | Hard | 905 | 76 | 8.4% |
| Byzantium | Hard | 885 | 71 | 8.0% |
| Germany | Hard | 900 | 145 | 16.1% |
| Celts | Hard | 880 | 142 | 16.1% |
| Mongols | Hard | 878 | 72 | 8.2% |
| Sparta | Hard | 901 | 116 | 12.9% |
| Athens | Hard | 900 | 160 | 17.8% |
| Aztecs | Hard | 872 | 126 | 14.4% |

## Faction goal progress

Per seat: how often the faction's permanent goal was reached before the game ended, and the turn its VP was first awarded (over the seats that reached it). Seats are all initial empires of non-panicked games with telemetry, including eliminated empires and cap stops.

| Faction | Seats | Reached | Goal rate | Min | P10 | Median turn | P90 | Max |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Switzerland | 879 | 201 | 22.9% | 115 | 139 | 170 | 230 | 317 |
| India | 905 | 515 | 56.9% | 45 | 87 | 123 | 195 | 281 |
| Byzantium | 885 | 712 | 80.5% | 69 | 83 | 106 | 146 | 259 |
| Germany | 900 | 337 | 37.4% | 85 | 122 | 150 | 194 | 291 |
| Celts | 880 | 551 | 62.6% | 35 | 162 | 210 | 210 | 293 |
| Mongols | 878 | 147 | 16.7% | 113 | 148 | 189 | 229 | 385 |
| Sparta | 901 | 137 | 15.2% | 131 | 147 | 198 | 254 | 342 |
| Athens | 900 | 280 | 31.1% | 119 | 154 | 225 | 255 | 255 |
| Aztecs | 872 | 304 | 34.9% | 120 | 163 | 225 | 234 | 296 |

## Distributions

| Metric | Min | P10 | Median | P90 | Max |
| --- | --- | --- | --- | --- | --- |
| Observed game length (turns, including cap stops) | 136 | 177 | 215 | 266 | 497 |
| Winning team VP | 3 | 10 | 11 | 13 | 18 |
| First Soldier completed (turn, per empire) | 6 | 8 | 36 | 71 | 262 |
| First Settler completed (turn, per empire) | 6 | 12 | 14 | 25 | 129 |
| First city founded (turn, per empire) | 1 | 1 | 1 | 1 | 1 |
| Second city founded (turn, per empire) | 10 | 16 | 19 | 39 | 214 |

Safety-cap stops are censored observations, not completed game lengths.

Openings: 8000 observed empires in 1000 first plays; 0 games without opening telemetry excluded. Turn distributions include only empires that reached the milestone. Never means not reached by the end of observed play, including eliminated empires and safety-cap stops. Per-empire turns are in JSON (`games[].telemetry.openings`, keyed by player ID).

| Opening milestone | Reached empires | Never empires |
| --- | --- | --- |
| First Soldier completed | 7906 | 94 |
| First Settler completed | 7980 | 20 |
| First city founded | 8000 | 0 |
| Second city founded | 7910 | 90 |

Final VP per empire (all seats):

| VP | Empires |
| --- | --- |
| 0 | 1530 |
| 1 | 1284 |
| 2 | 1124 |
| 3 | 779 |
| 4 | 630 |
| 5 | 525 |
| 6 | 389 |
| 7 | 294 |
| 8 | 174 |
| 9 | 162 |
| 10 | 464 |
| 11 | 329 |
| 12 | 170 |
| 13 | 81 |
| 14 | 38 |
| 15 | 15 |
| 16 | 8 |
| 17 | 3 |
| 18 | 1 |

## End state

- Average game length: 220.4 turns
- Average cities per empire: 4.69
- Average techs per empire: 32.56
- Average eliminations per game: 1.66

## Strategy diversity: observed behavior (classifier v2)

1000 finished games; 8000 player seats; 1000 winning seats. Excluded 0 unfinished/panicked games and 0 games with missing labels. Winner entropy: **0.3077** (normalized over 6 categories, including zero-win categories).

Thresholds, biases and denominators: [strategy-diversity.md](../strategy-diversity.md).

| Archetype | Players | Wins | Win rate | All-player frequency | Winner frequency |
| --- | ---: | ---: | ---: | ---: | ---: |
| Rush | 1728 | 58 | 3.4% | 21.6% | 5.8% |
| Wide | 3809 | 846 | 22.2% | 47.6% | 84.6% |
| Science | 0 | 0 | n/a | 0.0% | 0.0% |
| Turtle/Wonder | 23 | 0 | 0.0% | 0.3% | 0.0% |
| Economic | 1212 | 91 | 7.5% | 15.2% | 9.1% |
| Generalist | 1228 | 5 | 0.4% | 15.3% | 0.5% |

### Matchup matrix

Cell = strict ahead % / tied % (co-occurrence games). Each game has equal weight; all cross-team seat pairs within it share that weight. Rank: winning team first, then surviving empires by VP, then eliminated empires by later elimination turn. Equal ranks tie; no ID tiebreak. This is a multiplayer association, not a duel win rate.

| Row vs column | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Rush | — | 16.3% / 4.3% (853) | n/a / n/a (0) | 25.6% / 13.3% (15) | 17.4% / 8.8% (614) | 56.8% / 7.7% (625) |
| Wide | 79.5% / 4.3% (853) | — | n/a / n/a (0) | 81.1% / 6.7% (22) | 57.4% / 6.5% (744) | 87.9% / 3.0% (756) |
| Science | n/a / n/a (0) | n/a / n/a (0) | — | n/a / n/a (0) | n/a / n/a (0) | n/a / n/a (0) |
| Turtle/Wonder | 61.1% / 13.3% (15) | 12.1% / 6.7% (22) | n/a / n/a (0) | — | 35.6% / 18.9% (15) | 94.4% / 0.0% (15) |
| Economic | 73.8% / 8.8% (614) | 36.0% / 6.5% (744) | n/a / n/a (0) | 45.6% / 18.9% (15) | — | 87.4% / 6.3% (547) |
| Generalist | 35.5% / 7.7% (625) | 9.2% / 3.0% (756) | n/a / n/a (0) | 5.6% / 0.0% (15) | 6.3% / 6.3% (547) | — |

### Faction wins by archetype

Cell = wins / seats; win rate in JSON.

| Faction | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Switzerland | 0 / 93 | 86 / 443 | 0 / 0 | 0 / 0 | 6 / 205 | 0 / 138 |
| India | 1 / 199 | 75 / 584 | 0 / 0 | 0 / 0 | 0 / 23 | 0 / 99 |
| Byzantium | 0 / 7 | 68 / 490 | 0 / 0 | 0 / 2 | 3 / 151 | 0 / 235 |
| Germany | 3 / 139 | 136 / 464 | 0 / 0 | 0 / 4 | 6 / 113 | 0 / 180 |
| Celts | 2 / 112 | 129 / 531 | 0 / 0 | 0 / 0 | 11 / 142 | 0 / 95 |
| Mongols | 20 / 556 | 48 / 206 | 0 / 0 | 0 / 0 | 4 / 41 | 0 / 75 |
| Sparta | 26 / 430 | 55 / 182 | 0 / 0 | 0 / 15 | 31 / 173 | 4 / 101 |
| Athens | 4 / 90 | 137 / 451 | 0 / 0 | 0 / 0 | 18 / 189 | 1 / 170 |
| Aztecs | 2 / 102 | 112 / 458 | 0 / 0 | 0 / 2 | 12 / 175 | 0 / 135 |

Guardrail: **Fail**. Entropy minimum 0.80; at least 20 games per strategy pair. Dominant by observed score: none. Without an observed counter (all pairs covered): none. Sampled but never won: Turtle/Wonder. Unobserved: Science. Under-sampled pairs: 8.

- insufficient evidence: 8 under-sampled matchups; 1000 winning seats

- winner entropy below 0.80

## Battle and capture telemetry

1000 observed first plays; 0 records without telemetry excluded.

| Event | Total / observed games | Mean per game | Games with event / observed games |
| --- | --- | --- | --- |
| War declarations | 14610/1000 | 14.610 | 1000/1000 (100.0%) |
| Peace offers | 86689/1000 | 86.689 | 1000/1000 (100.0%) |
| Peace agreements | 12616/1000 | 12.616 | 1000/1000 (100.0%) |
| Battles | 992803/1000 | 992.803 | 1000/1000 (100.0%) |
| City captures | 20911/1000 | 20.911 | 1000/1000 (100.0%) |
| Settler captures | 5060/1000 | 5.060 | 980/1000 (98.0%) |
| Eliminations | 1662/1000 | 1.662 | 870/1000 (87.0%) |

## Terminal VP sources

1000 finished games. Columns are separate populations; totals across columns must not be added. Every cell is VP / population observations.

| Source | All players | All teams | Winning teams |
| --- | --- | --- | --- |
| Held Great Works | 7542/8000 | 7542/8000 | 3004/1000 |
| Faction goals | 5122/8000 | 5122/8000 | 2130/1000 |
| Era objectives | 15306/8000 | 15306/8000 | 4465/1000 |
| Capitals taken | 2592/8000 | 2592/8000 | 1534/1000 |
| Total VP | 30562/8000 | 30562/8000 | 11133/1000 |

Reconciliation failures: 0 player observations, 0 team observations.

## Turn-50 team leaders

| Cohort | Eventual leader-set wins / eligible games | Draws (included) |
| --- | --- | --- |
| Unique leader | 67/391 (17.1%) | 0 |
| Tied leaders (winner in set) | 604/609 (99.2%) | 0 |
| Any leader set | 671/1000 (67.1%) | 0 |

Excluded: 0 games ended before turn 50, 0 unfinished games, 0 snapshots without teams, 0 records without telemetry.

## Era pacing

Resolved turn each empire entered each era. `Winner vs median` is the winning team's seats' entry turn minus the median entry of the empires that entered it in the same finished game (negative: winners got there first).

| Era | Entered / seats | Fastest | p10 | Median | p90 | Slowest | Winner vs median (mean, n) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Classical | 7903/8000 | 5 | 20 | 28 | 32 | 59 | -2.2 (1000) |
| Medieval | 7820/8000 | 35 | 56 | 60 | 63 | 124 | -1.6 (1000) |
| Renaissance | 7774/8000 | 62 | 81 | 86 | 93 | 186 | -2.6 (1000) |
| Industrial | 7587/8000 | 86 | 107 | 114 | 126 | 286 | -4.3 (1000) |
| Machine Age | 7050/8000 | 111 | 136 | 145 | 160 | 256 | -6.5 (1000) |
| World War | 5670/8000 | 134 | 160 | 173 | 191 | 300 | -9.1 (983) |
| Modern | 3446/8000 | 152 | 184 | 202 | 225 | 349 | -8.9 (830) |
| Future | 1462/8000 | 179 | 207 | 233 | 256 | 444 | -7.0 (472) |

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
| Classical | Mathematics | 7840/8000 | 43 | 351 BC | 300 BC |
| Classical | Engineering | 7831/8000 | 49 | 51 BC | n/a |
| Classical | Iron Working | 7857/8000 | 37 | 651 BC | 1200 BC |
| Classical | Code of Laws | 7825/8000 | 57 | 350 AD | 1754 BC |
| Classical | Philosophy | 7821/8000 | 58 | 400 AD | 500 BC |
| Medieval | Feudalism | 7803/8000 | 73 | 890 AD | n/a |
| Medieval | Machinery | 7815/8000 | 67 | 710 AD | n/a |
| Medieval | Chivalry | 6928/8000 | 77 | 1010 AD | n/a |
| Medieval | Education | 7784/8000 | 82 | 1160 AD | 1088 AD |
| Medieval | Crop Rotation | 7774/8000 | 85 | 1250 AD | n/a |
| Renaissance | Gunpowder | 7754/8000 | 92 | 1420 AD | n/a |
| Renaissance | Metallurgy | 7705/8000 | 102 | 1520 AD | n/a |
| Renaissance | Military Engineering | 7722/8000 | 100 | 1500 AD | n/a |
| Renaissance | Banking | 7630/8000 | 111 | 1610 AD | 1397 AD |
| Renaissance | Printing Press | 7601/8000 | 113 | 1630 AD | 1440 AD |
| Industrial | Rifling | 7251/8000 | 135 | 1787 AD | n/a |
| Industrial | Military Science | 7397/8000 | 128 | 1761 AD | n/a |
| Industrial | Ballistics | 7300/8000 | 137 | 1794 AD | n/a |
| Industrial | Industrialization | 7488/8000 | 122 | 1720 AD | 1769 AD |
| Industrial | Railways | 7056/8000 | 145 | 1824 AD | 1825 AD |
| Machine Age | Replaceable Parts | 6512/8000 | 159 | 1876 AD | 1800 AD |
| Machine Age | Combustion | 6803/8000 | 153 | 1854 AD | 1876 AD |
| Machine Age | Steel | 6488/8000 | 158 | 1872 AD | 1856 AD |
| Machine Age | Flight | 5974/8000 | 167 | 1887 AD | 1903 AD |
| Machine Age | Electricity | 5736/8000 | 173 | 1894 AD | 1879 AD |
| World War | Mass Mobilisation | 4558/8000 | 188 | 1911 AD | n/a |
| World War | Rocketry | 4241/8000 | 192 | 1916 AD | 1926 AD |
| World War | Armoured Warfare | 3924/8000 | 197 | 1924 AD | 1916 AD |
| World War | Air Power | 3692/8000 | 200 | 1928 AD | n/a |
| World War | Electronics | 5168/8000 | 180 | 1902 AD | n/a |
| Modern | Mechanised Warfare | 2298/8000 | 217 | 1954 AD | n/a |
| Modern | Composite Armour | 1965/8000 | 221 | 1964 AD | n/a |
| Modern | Guided Missiles | 1793/8000 | 227 | 1978 AD | n/a |
| Modern | Jet Engines | 1731/8000 | 228 | 1981 AD | 1939 AD |
| Modern | Green Revolution | 2847/8000 | 210 | 1942 AD | n/a |
| Future | Autonomous Systems | 840/8000 | 251 | 2027 AD | n/a |
| Future | Robotics | 642/8000 | 256 | 2032 AD | n/a |
| Future | Fusion Power | 685/8000 | 256 | 2032 AD | n/a |
| Future | Stealth Technology | 1058/8000 | 243 | 2014 AD | n/a |
| Future | Recursive AI | 605/8000 | 256 | 2032 AD | n/a |

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
| 0 | 0.00 | 74.90 |
| 1 | 57.40 | 16.62 |
| 2 | 18.14 | 4.95 |
| 3 | 4.32 | 1.24 |
| 4 | 0.79 | 0.26 |
| 5 | 0.18 | 0.07 |

## Per-game events

Each row is one observed first play; unavailable telemetry is shown as n/a. Full per-player/team score sources and turn-50 standings are in JSON.

| Game | Seed | Resolved turns | Battles | City captures | Settler captures | Eliminations |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 9311003 | 247 | 1077 | 30 | 7 | 1 |
| 1 | 9311004 | 205 | 718 | 19 | 8 | 1 |
| 2 | 9311005 | 248 | 1090 | 28 | 5 | 2 |
| 3 | 9311006 | 285 | 1516 | 33 | 7 | 0 |
| 4 | 9311007 | 206 | 884 | 24 | 6 | 3 |
| 5 | 9311008 | 256 | 1495 | 30 | 3 | 2 |
| 6 | 9311009 | 228 | 1210 | 22 | 5 | 0 |
| 7 | 9311010 | 196 | 840 | 25 | 6 | 2 |
| 8 | 9311011 | 194 | 644 | 13 | 5 | 1 |
| 9 | 9311012 | 259 | 1262 | 28 | 5 | 4 |
| 10 | 9311013 | 203 | 1139 | 25 | 12 | 2 |
| 11 | 9311014 | 246 | 1163 | 35 | 7 | 5 |
| 12 | 9311015 | 185 | 724 | 11 | 8 | 1 |
| 13 | 9311016 | 237 | 1190 | 35 | 6 | 1 |
| 14 | 9311017 | 219 | 1026 | 19 | 3 | 2 |
| 15 | 9311018 | 169 | 491 | 9 | 8 | 0 |
| 16 | 9311019 | 251 | 1086 | 15 | 6 | 2 |
| 17 | 9311020 | 177 | 737 | 9 | 8 | 0 |
| 18 | 9311021 | 190 | 929 | 14 | 10 | 2 |
| 19 | 9311022 | 161 | 573 | 8 | 2 | 0 |
| 20 | 9311023 | 297 | 1632 | 27 | 11 | 1 |
| 21 | 9311024 | 319 | 1616 | 35 | 1 | 4 |
| 22 | 9311025 | 222 | 1099 | 22 | 15 | 1 |
| 23 | 9311026 | 222 | 1056 | 18 | 0 | 3 |
| 24 | 9311027 | 416 | 1966 | 23 | 3 | 1 |
| 25 | 9311028 | 193 | 950 | 28 | 5 | 0 |
| 26 | 9311029 | 159 | 453 | 9 | 10 | 1 |
| 27 | 9311030 | 204 | 697 | 21 | 3 | 2 |
| 28 | 9311031 | 198 | 987 | 19 | 6 | 1 |
| 29 | 9311032 | 231 | 1273 | 25 | 6 | 0 |
| 30 | 9311033 | 221 | 1030 | 21 | 8 | 3 |
| 31 | 9311034 | 256 | 1456 | 19 | 4 | 1 |
| 32 | 9311035 | 193 | 905 | 27 | 8 | 2 |
| 33 | 9311036 | 175 | 699 | 12 | 11 | 0 |
| 34 | 9311037 | 195 | 910 | 20 | 4 | 1 |
| 35 | 9311038 | 201 | 982 | 20 | 4 | 1 |
| 36 | 9311039 | 186 | 886 | 32 | 4 | 1 |
| 37 | 9311040 | 177 | 811 | 10 | 2 | 0 |
| 38 | 9311041 | 261 | 1156 | 24 | 4 | 2 |
| 39 | 9311042 | 228 | 1003 | 21 | 3 | 1 |
| 40 | 9311043 | 233 | 1056 | 31 | 3 | 4 |
| 41 | 9311044 | 168 | 693 | 10 | 5 | 1 |
| 42 | 9311045 | 222 | 1081 | 23 | 3 | 2 |
| 43 | 9311046 | 197 | 881 | 14 | 3 | 3 |
| 44 | 9311047 | 206 | 1091 | 19 | 5 | 2 |
| 45 | 9311048 | 198 | 760 | 8 | 15 | 2 |
| 46 | 9311049 | 221 | 1092 | 22 | 5 | 2 |
| 47 | 9311050 | 243 | 1113 | 28 | 5 | 2 |
| 48 | 9311051 | 223 | 1031 | 8 | 6 | 2 |
| 49 | 9311052 | 220 | 1062 | 13 | 6 | 1 |
| 50 | 9311053 | 219 | 933 | 25 | 3 | 0 |
| 51 | 9311054 | 242 | 893 | 20 | 5 | 3 |
| 52 | 9311055 | 170 | 718 | 10 | 5 | 1 |
| 53 | 9311056 | 203 | 1088 | 15 | 2 | 2 |
| 54 | 9311057 | 240 | 1032 | 19 | 7 | 2 |
| 55 | 9311058 | 256 | 1395 | 24 | 10 | 3 |
| 56 | 9311059 | 215 | 999 | 21 | 4 | 3 |
| 57 | 9311060 | 274 | 1229 | 16 | 5 | 1 |
| 58 | 9311061 | 186 | 811 | 15 | 2 | 1 |
| 59 | 9311062 | 185 | 896 | 20 | 3 | 0 |
| 60 | 9311063 | 157 | 642 | 10 | 4 | 0 |
| 61 | 9311064 | 220 | 1031 | 17 | 4 | 4 |
| 62 | 9311065 | 231 | 1154 | 28 | 4 | 2 |
| 63 | 9311066 | 289 | 1637 | 21 | 3 | 1 |
| 64 | 9311067 | 198 | 946 | 19 | 6 | 2 |
| 65 | 9311068 | 191 | 705 | 21 | 6 | 2 |
| 66 | 9311069 | 255 | 1236 | 34 | 3 | 3 |
| 67 | 9311070 | 189 | 946 | 18 | 3 | 1 |
| 68 | 9311071 | 137 | 504 | 10 | 1 | 2 |
| 69 | 9311072 | 180 | 795 | 19 | 3 | 2 |
| 70 | 9311073 | 245 | 1268 | 18 | 3 | 1 |
| 71 | 9311074 | 273 | 1313 | 38 | 13 | 2 |
| 72 | 9311075 | 204 | 908 | 25 | 10 | 1 |
| 73 | 9311076 | 262 | 1124 | 30 | 0 | 2 |
| 74 | 9311077 | 204 | 1053 | 28 | 6 | 2 |
| 75 | 9311078 | 197 | 726 | 14 | 3 | 2 |
| 76 | 9311079 | 221 | 948 | 19 | 2 | 1 |
| 77 | 9311080 | 210 | 824 | 12 | 5 | 1 |
| 78 | 9311081 | 216 | 853 | 21 | 6 | 2 |
| 79 | 9311082 | 283 | 1668 | 28 | 8 | 2 |
| 80 | 9311083 | 259 | 916 | 15 | 11 | 1 |
| 81 | 9311084 | 250 | 1071 | 24 | 8 | 3 |
| 82 | 9311085 | 201 | 921 | 22 | 5 | 2 |
| 83 | 9311086 | 167 | 618 | 11 | 6 | 0 |
| 84 | 9311087 | 181 | 616 | 14 | 7 | 1 |
| 85 | 9311088 | 303 | 1607 | 43 | 3 | 2 |
| 86 | 9311089 | 206 | 807 | 24 | 2 | 3 |
| 87 | 9311090 | 208 | 978 | 21 | 10 | 2 |
| 88 | 9311091 | 246 | 1251 | 33 | 10 | 3 |
| 89 | 9311092 | 198 | 960 | 17 | 7 | 0 |
| 90 | 9311093 | 274 | 1139 | 30 | 17 | 1 |
| 91 | 9311094 | 185 | 717 | 29 | 6 | 1 |
| 92 | 9311095 | 273 | 1581 | 27 | 4 | 1 |
| 93 | 9311096 | 221 | 1059 | 31 | 2 | 1 |
| 94 | 9311097 | 224 | 1132 | 31 | 1 | 3 |
| 95 | 9311098 | 177 | 766 | 12 | 7 | 1 |
| 96 | 9311099 | 189 | 853 | 35 | 1 | 0 |
| 97 | 9311100 | 198 | 772 | 20 | 5 | 1 |
| 98 | 9311101 | 198 | 870 | 21 | 2 | 0 |
| 99 | 9311102 | 193 | 905 | 18 | 8 | 0 |
| 100 | 9311103 | 210 | 767 | 10 | 5 | 2 |
| 101 | 9311104 | 185 | 960 | 24 | 1 | 1 |
| 102 | 9311105 | 209 | 1211 | 43 | 6 | 3 |
| 103 | 9311106 | 175 | 464 | 12 | 1 | 0 |
| 104 | 9311107 | 209 | 896 | 23 | 2 | 1 |
| 105 | 9311108 | 198 | 548 | 12 | 5 | 2 |
| 106 | 9311109 | 217 | 939 | 20 | 3 | 1 |
| 107 | 9311110 | 163 | 691 | 13 | 7 | 1 |
| 108 | 9311111 | 181 | 690 | 16 | 4 | 2 |
| 109 | 9311112 | 178 | 724 | 18 | 2 | 0 |
| 110 | 9311113 | 253 | 1430 | 19 | 4 | 1 |
| 111 | 9311114 | 295 | 1176 | 39 | 12 | 3 |
| 112 | 9311115 | 176 | 717 | 14 | 2 | 0 |
| 113 | 9311116 | 214 | 1099 | 26 | 4 | 2 |
| 114 | 9311117 | 199 | 660 | 18 | 3 | 2 |
| 115 | 9311118 | 291 | 1362 | 19 | 2 | 3 |
| 116 | 9311119 | 254 | 1086 | 22 | 3 | 1 |
| 117 | 9311120 | 188 | 776 | 17 | 9 | 0 |
| 118 | 9311121 | 245 | 1013 | 19 | 3 | 3 |
| 119 | 9311122 | 172 | 780 | 24 | 6 | 2 |
| 120 | 9311123 | 208 | 990 | 21 | 3 | 3 |
| 121 | 9311124 | 238 | 961 | 22 | 6 | 2 |
| 122 | 9311125 | 238 | 688 | 15 | 2 | 3 |
| 123 | 9311126 | 209 | 768 | 16 | 11 | 1 |
| 124 | 9311127 | 246 | 1146 | 33 | 6 | 2 |
| 125 | 9311128 | 213 | 943 | 16 | 12 | 1 |
| 126 | 9311129 | 213 | 932 | 16 | 2 | 1 |
| 127 | 9311130 | 246 | 1156 | 11 | 6 | 1 |
| 128 | 9311131 | 175 | 920 | 15 | 6 | 0 |
| 129 | 9311132 | 185 | 704 | 13 | 3 | 3 |
| 130 | 9311133 | 156 | 626 | 10 | 3 | 1 |
| 131 | 9311134 | 210 | 716 | 17 | 7 | 1 |
| 132 | 9311135 | 157 | 637 | 12 | 2 | 1 |
| 133 | 9311136 | 231 | 1029 | 21 | 5 | 1 |
| 134 | 9311137 | 228 | 902 | 20 | 9 | 2 |
| 135 | 9311138 | 181 | 761 | 17 | 7 | 1 |
| 136 | 9311139 | 194 | 715 | 21 | 7 | 1 |
| 137 | 9311140 | 227 | 1010 | 24 | 3 | 1 |
| 138 | 9311141 | 173 | 699 | 9 | 3 | 0 |
| 139 | 9311142 | 193 | 874 | 16 | 4 | 0 |
| 140 | 9311143 | 245 | 1101 | 13 | 2 | 3 |
| 141 | 9311144 | 200 | 715 | 16 | 5 | 1 |
| 142 | 9311145 | 199 | 777 | 13 | 6 | 1 |
| 143 | 9311146 | 238 | 1238 | 28 | 6 | 2 |
| 144 | 9311147 | 372 | 2036 | 50 | 5 | 2 |
| 145 | 9311148 | 257 | 1001 | 34 | 1 | 3 |
| 146 | 9311149 | 190 | 930 | 15 | 8 | 4 |
| 147 | 9311150 | 237 | 947 | 23 | 7 | 3 |
| 148 | 9311151 | 231 | 1014 | 30 | 5 | 1 |
| 149 | 9311152 | 234 | 1099 | 25 | 7 | 2 |
| 150 | 9311153 | 165 | 685 | 10 | 4 | 1 |
| 151 | 9311154 | 222 | 1137 | 30 | 6 | 2 |
| 152 | 9311155 | 262 | 1361 | 21 | 5 | 2 |
| 153 | 9311156 | 253 | 1164 | 25 | 5 | 1 |
| 154 | 9311157 | 241 | 1344 | 26 | 5 | 0 |
| 155 | 9311158 | 198 | 986 | 16 | 4 | 3 |
| 156 | 9311159 | 298 | 1541 | 20 | 5 | 1 |
| 157 | 9311160 | 191 | 660 | 13 | 9 | 1 |
| 158 | 9311161 | 203 | 692 | 18 | 6 | 1 |
| 159 | 9311162 | 239 | 1161 | 25 | 7 | 1 |
| 160 | 9311163 | 222 | 886 | 28 | 7 | 1 |
| 161 | 9311164 | 209 | 936 | 31 | 0 | 3 |
| 162 | 9311165 | 224 | 844 | 23 | 6 | 3 |
| 163 | 9311166 | 206 | 754 | 9 | 6 | 4 |
| 164 | 9311167 | 168 | 535 | 12 | 2 | 1 |
| 165 | 9311168 | 237 | 1126 | 18 | 7 | 3 |
| 166 | 9311169 | 228 | 996 | 18 | 5 | 1 |
| 167 | 9311170 | 204 | 887 | 21 | 4 | 3 |
| 168 | 9311171 | 209 | 885 | 20 | 6 | 1 |
| 169 | 9311172 | 215 | 903 | 13 | 4 | 2 |
| 170 | 9311173 | 182 | 953 | 11 | 4 | 1 |
| 171 | 9311174 | 213 | 932 | 21 | 4 | 4 |
| 172 | 9311175 | 215 | 703 | 12 | 1 | 0 |
| 173 | 9311176 | 248 | 1195 | 27 | 4 | 1 |
| 174 | 9311177 | 158 | 623 | 15 | 0 | 3 |
| 175 | 9311178 | 255 | 1519 | 34 | 7 | 0 |
| 176 | 9311179 | 207 | 912 | 15 | 7 | 2 |
| 177 | 9311180 | 207 | 896 | 22 | 1 | 3 |
| 178 | 9311181 | 238 | 904 | 27 | 4 | 1 |
| 179 | 9311182 | 191 | 843 | 13 | 2 | 0 |
| 180 | 9311183 | 173 | 659 | 11 | 8 | 2 |
| 181 | 9311184 | 192 | 798 | 24 | 4 | 3 |
| 182 | 9311185 | 190 | 799 | 28 | 5 | 3 |
| 183 | 9311186 | 256 | 1249 | 29 | 10 | 2 |
| 184 | 9311187 | 242 | 992 | 15 | 9 | 1 |
| 185 | 9311188 | 242 | 1128 | 24 | 9 | 2 |
| 186 | 9311189 | 209 | 979 | 12 | 8 | 1 |
| 187 | 9311190 | 229 | 1054 | 25 | 4 | 0 |
| 188 | 9311191 | 205 | 862 | 26 | 3 | 1 |
| 189 | 9311192 | 178 | 622 | 15 | 2 | 2 |
| 190 | 9311193 | 195 | 757 | 19 | 1 | 1 |
| 191 | 9311194 | 203 | 990 | 15 | 4 | 3 |
| 192 | 9311195 | 175 | 794 | 21 | 4 | 2 |
| 193 | 9311196 | 227 | 842 | 27 | 2 | 3 |
| 194 | 9311197 | 285 | 1603 | 27 | 8 | 3 |
| 195 | 9311198 | 245 | 1112 | 24 | 14 | 3 |
| 196 | 9311199 | 251 | 1217 | 27 | 8 | 3 |
| 197 | 9311200 | 232 | 1125 | 17 | 3 | 0 |
| 198 | 9311201 | 181 | 628 | 8 | 5 | 1 |
| 199 | 9311202 | 315 | 1665 | 23 | 7 | 2 |
| 200 | 9311203 | 204 | 897 | 10 | 4 | 1 |
| 201 | 9311204 | 246 | 1251 | 22 | 12 | 4 |
| 202 | 9311205 | 262 | 1325 | 38 | 4 | 2 |
| 203 | 9311206 | 202 | 808 | 19 | 7 | 2 |
| 204 | 9311207 | 263 | 1374 | 24 | 8 | 3 |
| 205 | 9311208 | 211 | 1020 | 27 | 7 | 2 |
| 206 | 9311209 | 202 | 851 | 15 | 4 | 1 |
| 207 | 9311210 | 267 | 1536 | 16 | 11 | 3 |
| 208 | 9311211 | 242 | 1237 | 24 | 3 | 2 |
| 209 | 9311212 | 168 | 566 | 18 | 3 | 1 |
| 210 | 9311213 | 316 | 1680 | 32 | 2 | 1 |
| 211 | 9311214 | 180 | 754 | 14 | 8 | 3 |
| 212 | 9311215 | 214 | 1148 | 13 | 3 | 0 |
| 213 | 9311216 | 215 | 815 | 26 | 2 | 1 |
| 214 | 9311217 | 204 | 929 | 18 | 5 | 1 |
| 215 | 9311218 | 224 | 907 | 20 | 5 | 2 |
| 216 | 9311219 | 250 | 1466 | 29 | 1 | 1 |
| 217 | 9311220 | 206 | 701 | 26 | 6 | 2 |
| 218 | 9311221 | 233 | 823 | 26 | 9 | 1 |
| 219 | 9311222 | 203 | 989 | 26 | 4 | 3 |
| 220 | 9311223 | 254 | 1222 | 36 | 5 | 3 |
| 221 | 9311224 | 279 | 1599 | 26 | 14 | 2 |
| 222 | 9311225 | 275 | 1663 | 30 | 6 | 2 |
| 223 | 9311226 | 225 | 1109 | 37 | 4 | 2 |
| 224 | 9311227 | 201 | 668 | 15 | 7 | 3 |
| 225 | 9311228 | 279 | 1413 | 27 | 6 | 3 |
| 226 | 9311229 | 232 | 1157 | 10 | 2 | 1 |
| 227 | 9311230 | 201 | 758 | 10 | 3 | 1 |
| 228 | 9311231 | 279 | 1109 | 34 | 2 | 2 |
| 229 | 9311232 | 287 | 1535 | 39 | 5 | 3 |
| 230 | 9311233 | 202 | 876 | 13 | 3 | 1 |
| 231 | 9311234 | 297 | 1326 | 44 | 5 | 5 |
| 232 | 9311235 | 242 | 1172 | 33 | 3 | 3 |
| 233 | 9311236 | 234 | 1143 | 22 | 3 | 1 |
| 234 | 9311237 | 274 | 1886 | 25 | 1 | 1 |
| 235 | 9311238 | 175 | 608 | 15 | 2 | 2 |
| 236 | 9311239 | 221 | 786 | 33 | 5 | 3 |
| 237 | 9311240 | 193 | 811 | 23 | 4 | 3 |
| 238 | 9311241 | 183 | 785 | 18 | 7 | 2 |
| 239 | 9311242 | 266 | 1255 | 18 | 5 | 0 |
| 240 | 9311243 | 203 | 641 | 21 | 5 | 2 |
| 241 | 9311244 | 233 | 1245 | 17 | 7 | 4 |
| 242 | 9311245 | 154 | 482 | 9 | 3 | 2 |
| 243 | 9311246 | 179 | 832 | 22 | 5 | 2 |
| 244 | 9311247 | 187 | 886 | 14 | 12 | 3 |
| 245 | 9311248 | 220 | 965 | 23 | 9 | 1 |
| 246 | 9311249 | 169 | 572 | 18 | 4 | 0 |
| 247 | 9311250 | 184 | 597 | 15 | 7 | 0 |
| 248 | 9311251 | 179 | 686 | 18 | 8 | 2 |
| 249 | 9311252 | 235 | 865 | 27 | 9 | 4 |
| 250 | 9311253 | 191 | 765 | 11 | 8 | 0 |
| 251 | 9311254 | 254 | 1256 | 25 | 5 | 3 |
| 252 | 9311255 | 185 | 1085 | 19 | 4 | 1 |
| 253 | 9311256 | 216 | 1214 | 16 | 4 | 0 |
| 254 | 9311257 | 165 | 652 | 18 | 5 | 1 |
| 255 | 9311258 | 210 | 790 | 18 | 4 | 3 |
| 256 | 9311259 | 282 | 1200 | 23 | 1 | 2 |
| 257 | 9311260 | 230 | 876 | 6 | 5 | 2 |
| 258 | 9311261 | 258 | 1661 | 33 | 4 | 1 |
| 259 | 9311262 | 206 | 867 | 16 | 2 | 3 |
| 260 | 9311263 | 286 | 1487 | 27 | 6 | 2 |
| 261 | 9311264 | 218 | 854 | 23 | 3 | 1 |
| 262 | 9311265 | 216 | 919 | 23 | 4 | 1 |
| 263 | 9311266 | 194 | 842 | 9 | 4 | 2 |
| 264 | 9311267 | 283 | 1291 | 33 | 8 | 1 |
| 265 | 9311268 | 234 | 1152 | 14 | 3 | 0 |
| 266 | 9311269 | 223 | 951 | 22 | 4 | 3 |
| 267 | 9311270 | 248 | 1276 | 13 | 10 | 1 |
| 268 | 9311271 | 265 | 808 | 12 | 8 | 3 |
| 269 | 9311272 | 215 | 858 | 20 | 14 | 1 |
| 270 | 9311273 | 211 | 1040 | 16 | 4 | 0 |
| 271 | 9311274 | 293 | 1298 | 38 | 8 | 2 |
| 272 | 9311275 | 273 | 857 | 28 | 15 | 4 |
| 273 | 9311276 | 167 | 657 | 11 | 5 | 0 |
| 274 | 9311277 | 222 | 1017 | 9 | 2 | 0 |
| 275 | 9311278 | 201 | 879 | 12 | 6 | 1 |
| 276 | 9311279 | 289 | 1249 | 30 | 3 | 1 |
| 277 | 9311280 | 213 | 899 | 24 | 4 | 2 |
| 278 | 9311281 | 237 | 846 | 18 | 1 | 3 |
| 279 | 9311282 | 226 | 901 | 13 | 5 | 1 |
| 280 | 9311283 | 169 | 672 | 9 | 0 | 2 |
| 281 | 9311284 | 177 | 522 | 15 | 3 | 2 |
| 282 | 9311285 | 214 | 1071 | 26 | 4 | 1 |
| 283 | 9311286 | 206 | 960 | 18 | 4 | 1 |
| 284 | 9311287 | 242 | 1175 | 22 | 3 | 2 |
| 285 | 9311288 | 227 | 949 | 15 | 7 | 0 |
| 286 | 9311289 | 248 | 1400 | 38 | 4 | 2 |
| 287 | 9311290 | 197 | 729 | 15 | 1 | 0 |
| 288 | 9311291 | 216 | 909 | 20 | 2 | 2 |
| 289 | 9311292 | 210 | 946 | 20 | 1 | 3 |
| 290 | 9311293 | 201 | 1053 | 22 | 6 | 1 |
| 291 | 9311294 | 156 | 571 | 6 | 5 | 2 |
| 292 | 9311295 | 232 | 1019 | 16 | 7 | 2 |
| 293 | 9311296 | 246 | 830 | 17 | 9 | 4 |
| 294 | 9311297 | 191 | 763 | 22 | 4 | 2 |
| 295 | 9311298 | 187 | 788 | 18 | 5 | 1 |
| 296 | 9311299 | 176 | 766 | 15 | 7 | 2 |
| 297 | 9311300 | 231 | 986 | 22 | 11 | 1 |
| 298 | 9311301 | 176 | 686 | 9 | 5 | 1 |
| 299 | 9311302 | 210 | 888 | 23 | 4 | 2 |
| 300 | 9311303 | 247 | 1074 | 15 | 6 | 2 |
| 301 | 9311304 | 168 | 489 | 9 | 2 | 1 |
| 302 | 9311305 | 215 | 792 | 18 | 3 | 1 |
| 303 | 9311306 | 177 | 613 | 12 | 4 | 1 |
| 304 | 9311307 | 230 | 1091 | 20 | 4 | 1 |
| 305 | 9311308 | 229 | 1032 | 22 | 8 | 3 |
| 306 | 9311309 | 231 | 1039 | 19 | 7 | 2 |
| 307 | 9311310 | 201 | 1014 | 18 | 0 | 1 |
| 308 | 9311311 | 165 | 787 | 12 | 6 | 1 |
| 309 | 9311312 | 181 | 900 | 19 | 8 | 3 |
| 310 | 9311313 | 266 | 1088 | 22 | 11 | 4 |
| 311 | 9311314 | 247 | 1160 | 27 | 7 | 1 |
| 312 | 9311315 | 214 | 1027 | 24 | 3 | 1 |
| 313 | 9311316 | 248 | 1003 | 27 | 1 | 2 |
| 314 | 9311317 | 205 | 887 | 17 | 2 | 1 |
| 315 | 9311318 | 256 | 1347 | 27 | 8 | 3 |
| 316 | 9311319 | 210 | 920 | 23 | 7 | 3 |
| 317 | 9311320 | 205 | 1045 | 24 | 6 | 1 |
| 318 | 9311321 | 180 | 676 | 14 | 5 | 1 |
| 319 | 9311322 | 229 | 1017 | 17 | 14 | 1 |
| 320 | 9311323 | 197 | 890 | 24 | 4 | 1 |
| 321 | 9311324 | 223 | 923 | 26 | 6 | 3 |
| 322 | 9311325 | 312 | 1561 | 21 | 6 | 1 |
| 323 | 9311326 | 247 | 1371 | 20 | 7 | 2 |
| 324 | 9311327 | 213 | 878 | 24 | 2 | 1 |
| 325 | 9311328 | 244 | 1117 | 25 | 12 | 1 |
| 326 | 9311329 | 193 | 601 | 16 | 3 | 2 |
| 327 | 9311330 | 188 | 803 | 20 | 8 | 2 |
| 328 | 9311331 | 220 | 1071 | 23 | 2 | 1 |
| 329 | 9311332 | 201 | 712 | 16 | 3 | 1 |
| 330 | 9311333 | 194 | 849 | 23 | 6 | 2 |
| 331 | 9311334 | 165 | 482 | 18 | 1 | 2 |
| 332 | 9311335 | 194 | 904 | 14 | 6 | 1 |
| 333 | 9311336 | 224 | 1257 | 23 | 5 | 2 |
| 334 | 9311337 | 179 | 781 | 15 | 4 | 1 |
| 335 | 9311338 | 194 | 941 | 18 | 6 | 2 |
| 336 | 9311339 | 263 | 1172 | 31 | 13 | 4 |
| 337 | 9311340 | 185 | 698 | 14 | 5 | 2 |
| 338 | 9311341 | 253 | 1111 | 28 | 4 | 2 |
| 339 | 9311342 | 205 | 633 | 16 | 0 | 1 |
| 340 | 9311343 | 222 | 1180 | 20 | 14 | 1 |
| 341 | 9311344 | 206 | 511 | 9 | 11 | 1 |
| 342 | 9311345 | 229 | 1325 | 29 | 5 | 1 |
| 343 | 9311346 | 270 | 1470 | 34 | 15 | 2 |
| 344 | 9311347 | 237 | 1028 | 21 | 4 | 2 |
| 345 | 9311348 | 199 | 788 | 20 | 6 | 1 |
| 346 | 9311349 | 236 | 912 | 15 | 9 | 0 |
| 347 | 9311350 | 296 | 1469 | 27 | 2 | 1 |
| 348 | 9311351 | 264 | 1092 | 18 | 1 | 3 |
| 349 | 9311352 | 218 | 730 | 19 | 9 | 1 |
| 350 | 9311353 | 245 | 1437 | 23 | 1 | 1 |
| 351 | 9311354 | 215 | 1030 | 27 | 6 | 1 |
| 352 | 9311355 | 185 | 858 | 12 | 2 | 1 |
| 353 | 9311356 | 258 | 1329 | 27 | 9 | 2 |
| 354 | 9311357 | 204 | 999 | 28 | 8 | 2 |
| 355 | 9311358 | 185 | 645 | 20 | 6 | 2 |
| 356 | 9311359 | 175 | 635 | 7 | 5 | 1 |
| 357 | 9311360 | 194 | 809 | 15 | 6 | 1 |
| 358 | 9311361 | 200 | 941 | 14 | 3 | 1 |
| 359 | 9311362 | 227 | 1033 | 25 | 9 | 2 |
| 360 | 9311363 | 218 | 980 | 16 | 6 | 0 |
| 361 | 9311364 | 239 | 999 | 20 | 8 | 2 |
| 362 | 9311365 | 251 | 1148 | 26 | 17 | 2 |
| 363 | 9311366 | 255 | 1215 | 29 | 7 | 2 |
| 364 | 9311367 | 215 | 901 | 15 | 4 | 2 |
| 365 | 9311368 | 228 | 1165 | 21 | 2 | 2 |
| 366 | 9311369 | 208 | 909 | 16 | 8 | 0 |
| 367 | 9311370 | 235 | 905 | 19 | 3 | 2 |
| 368 | 9311371 | 203 | 803 | 13 | 9 | 2 |
| 369 | 9311372 | 214 | 988 | 23 | 3 | 2 |
| 370 | 9311373 | 263 | 931 | 27 | 4 | 3 |
| 371 | 9311374 | 281 | 1344 | 29 | 7 | 2 |
| 372 | 9311375 | 210 | 834 | 23 | 6 | 2 |
| 373 | 9311376 | 202 | 849 | 15 | 1 | 3 |
| 374 | 9311377 | 178 | 783 | 16 | 3 | 0 |
| 375 | 9311378 | 252 | 1108 | 25 | 3 | 2 |
| 376 | 9311379 | 247 | 993 | 15 | 2 | 1 |
| 377 | 9311380 | 225 | 968 | 21 | 0 | 1 |
| 378 | 9311381 | 239 | 1116 | 21 | 3 | 2 |
| 379 | 9311382 | 327 | 1714 | 42 | 8 | 5 |
| 380 | 9311383 | 314 | 1614 | 21 | 1 | 2 |
| 381 | 9311384 | 168 | 757 | 11 | 6 | 0 |
| 382 | 9311385 | 267 | 1008 | 18 | 12 | 1 |
| 383 | 9311386 | 151 | 458 | 11 | 2 | 1 |
| 384 | 9311387 | 177 | 857 | 5 | 9 | 2 |
| 385 | 9311388 | 171 | 652 | 7 | 7 | 2 |
| 386 | 9311389 | 215 | 812 | 17 | 8 | 1 |
| 387 | 9311390 | 219 | 979 | 21 | 2 | 2 |
| 388 | 9311391 | 247 | 1057 | 20 | 5 | 2 |
| 389 | 9311392 | 187 | 783 | 29 | 3 | 1 |
| 390 | 9311393 | 159 | 586 | 13 | 3 | 0 |
| 391 | 9311394 | 194 | 792 | 13 | 7 | 1 |
| 392 | 9311395 | 169 | 633 | 18 | 7 | 2 |
| 393 | 9311396 | 306 | 1272 | 35 | 4 | 1 |
| 394 | 9311397 | 169 | 744 | 15 | 7 | 1 |
| 395 | 9311398 | 155 | 692 | 16 | 1 | 0 |
| 396 | 9311399 | 240 | 1193 | 19 | 2 | 1 |
| 397 | 9311400 | 271 | 1047 | 11 | 4 | 1 |
| 398 | 9311401 | 363 | 2216 | 32 | 7 | 3 |
| 399 | 9311402 | 216 | 962 | 29 | 8 | 3 |
| 400 | 9311403 | 205 | 939 | 20 | 2 | 1 |
| 401 | 9311404 | 252 | 1115 | 17 | 3 | 0 |
| 402 | 9311405 | 201 | 868 | 21 | 2 | 2 |
| 403 | 9311406 | 210 | 794 | 10 | 7 | 1 |
| 404 | 9311407 | 192 | 618 | 18 | 3 | 2 |
| 405 | 9311408 | 226 | 1176 | 20 | 2 | 2 |
| 406 | 9311409 | 297 | 1936 | 26 | 8 | 1 |
| 407 | 9311410 | 187 | 731 | 16 | 9 | 2 |
| 408 | 9311411 | 204 | 568 | 16 | 3 | 1 |
| 409 | 9311412 | 219 | 1101 | 25 | 5 | 5 |
| 410 | 9311413 | 251 | 1006 | 22 | 5 | 2 |
| 411 | 9311414 | 176 | 843 | 13 | 5 | 1 |
| 412 | 9311415 | 206 | 932 | 18 | 8 | 2 |
| 413 | 9311416 | 226 | 1117 | 26 | 4 | 1 |
| 414 | 9311417 | 191 | 871 | 16 | 5 | 1 |
| 415 | 9311418 | 180 | 769 | 20 | 5 | 1 |
| 416 | 9311419 | 232 | 976 | 24 | 5 | 2 |
| 417 | 9311420 | 231 | 1096 | 46 | 10 | 3 |
| 418 | 9311421 | 247 | 978 | 20 | 1 | 3 |
| 419 | 9311422 | 201 | 638 | 14 | 5 | 0 |
| 420 | 9311423 | 326 | 2211 | 42 | 1 | 2 |
| 421 | 9311424 | 239 | 1203 | 19 | 6 | 2 |
| 422 | 9311425 | 258 | 792 | 13 | 8 | 0 |
| 423 | 9311426 | 190 | 707 | 22 | 4 | 1 |
| 424 | 9311427 | 223 | 996 | 19 | 7 | 3 |
| 425 | 9311428 | 151 | 612 | 12 | 4 | 2 |
| 426 | 9311429 | 237 | 961 | 31 | 6 | 1 |
| 427 | 9311430 | 179 | 744 | 19 | 1 | 0 |
| 428 | 9311431 | 175 | 748 | 12 | 3 | 3 |
| 429 | 9311432 | 203 | 960 | 26 | 3 | 3 |
| 430 | 9311433 | 199 | 533 | 11 | 3 | 1 |
| 431 | 9311434 | 178 | 786 | 21 | 6 | 2 |
| 432 | 9311435 | 226 | 939 | 20 | 5 | 0 |
| 433 | 9311436 | 185 | 882 | 16 | 7 | 2 |
| 434 | 9311437 | 281 | 1176 | 29 | 6 | 2 |
| 435 | 9311438 | 173 | 667 | 15 | 5 | 4 |
| 436 | 9311439 | 211 | 1107 | 13 | 4 | 1 |
| 437 | 9311440 | 228 | 850 | 18 | 8 | 2 |
| 438 | 9311441 | 156 | 595 | 6 | 6 | 0 |
| 439 | 9311442 | 217 | 1061 | 13 | 3 | 1 |
| 440 | 9311443 | 189 | 979 | 15 | 4 | 0 |
| 441 | 9311444 | 176 | 726 | 10 | 3 | 0 |
| 442 | 9311445 | 222 | 1125 | 14 | 6 | 1 |
| 443 | 9311446 | 214 | 968 | 22 | 2 | 2 |
| 444 | 9311447 | 221 | 912 | 13 | 3 | 0 |
| 445 | 9311448 | 226 | 1219 | 29 | 5 | 2 |
| 446 | 9311449 | 225 | 1095 | 17 | 2 | 1 |
| 447 | 9311450 | 244 | 1228 | 22 | 5 | 2 |
| 448 | 9311451 | 205 | 1055 | 23 | 2 | 1 |
| 449 | 9311452 | 226 | 1138 | 37 | 7 | 0 |
| 450 | 9311453 | 175 | 643 | 7 | 4 | 2 |
| 451 | 9311454 | 256 | 1237 | 27 | 5 | 2 |
| 452 | 9311455 | 224 | 985 | 19 | 4 | 1 |
| 453 | 9311456 | 208 | 882 | 20 | 3 | 2 |
| 454 | 9311457 | 245 | 1190 | 25 | 7 | 2 |
| 455 | 9311458 | 234 | 920 | 29 | 7 | 3 |
| 456 | 9311459 | 236 | 830 | 12 | 15 | 3 |
| 457 | 9311460 | 210 | 1044 | 23 | 3 | 0 |
| 458 | 9311461 | 280 | 1200 | 27 | 4 | 2 |
| 459 | 9311462 | 248 | 1183 | 35 | 9 | 4 |
| 460 | 9311463 | 212 | 934 | 21 | 6 | 3 |
| 461 | 9311464 | 188 | 907 | 26 | 5 | 0 |
| 462 | 9311465 | 166 | 726 | 14 | 2 | 1 |
| 463 | 9311466 | 209 | 931 | 20 | 4 | 2 |
| 464 | 9311467 | 157 | 452 | 8 | 5 | 2 |
| 465 | 9311468 | 172 | 729 | 24 | 2 | 2 |
| 466 | 9311469 | 197 | 864 | 18 | 2 | 2 |
| 467 | 9311470 | 196 | 861 | 8 | 12 | 0 |
| 468 | 9311471 | 214 | 1213 | 21 | 6 | 1 |
| 469 | 9311472 | 221 | 1118 | 21 | 9 | 2 |
| 470 | 9311473 | 205 | 793 | 19 | 3 | 2 |
| 471 | 9311474 | 238 | 983 | 20 | 3 | 1 |
| 472 | 9311475 | 183 | 585 | 12 | 5 | 3 |
| 473 | 9311476 | 183 | 716 | 13 | 4 | 0 |
| 474 | 9311477 | 257 | 1131 | 17 | 6 | 2 |
| 475 | 9311478 | 191 | 952 | 15 | 5 | 1 |
| 476 | 9311479 | 215 | 902 | 17 | 6 | 1 |
| 477 | 9311480 | 192 | 713 | 21 | 0 | 3 |
| 478 | 9311481 | 249 | 736 | 21 | 2 | 3 |
| 479 | 9311482 | 176 | 639 | 12 | 2 | 3 |
| 480 | 9311483 | 238 | 1373 | 16 | 4 | 0 |
| 481 | 9311484 | 173 | 620 | 8 | 2 | 1 |
| 482 | 9311485 | 238 | 1069 | 23 | 2 | 2 |
| 483 | 9311486 | 186 | 768 | 20 | 6 | 0 |
| 484 | 9311487 | 169 | 505 | 11 | 2 | 0 |
| 485 | 9311488 | 227 | 956 | 15 | 10 | 2 |
| 486 | 9311489 | 215 | 840 | 15 | 12 | 1 |
| 487 | 9311490 | 256 | 1176 | 22 | 3 | 0 |
| 488 | 9311491 | 235 | 1036 | 27 | 9 | 3 |
| 489 | 9311492 | 221 | 958 | 22 | 4 | 2 |
| 490 | 9311493 | 214 | 1038 | 16 | 3 | 1 |
| 491 | 9311494 | 168 | 701 | 11 | 5 | 1 |
| 492 | 9311495 | 199 | 873 | 21 | 6 | 1 |
| 493 | 9311496 | 272 | 1222 | 28 | 6 | 2 |
| 494 | 9311497 | 202 | 737 | 18 | 1 | 1 |
| 495 | 9311498 | 225 | 1136 | 18 | 2 | 4 |
| 496 | 9311499 | 188 | 839 | 22 | 8 | 2 |
| 497 | 9311500 | 196 | 820 | 24 | 7 | 1 |
| 498 | 9311501 | 243 | 956 | 28 | 8 | 2 |
| 499 | 9311502 | 209 | 754 | 17 | 0 | 2 |
| 500 | 9311503 | 222 | 1027 | 13 | 7 | 1 |
| 501 | 9311504 | 198 | 804 | 18 | 5 | 1 |
| 502 | 9311505 | 186 | 1011 | 13 | 1 | 3 |
| 503 | 9311506 | 194 | 874 | 22 | 8 | 2 |
| 504 | 9311507 | 259 | 1204 | 23 | 5 | 3 |
| 505 | 9311508 | 175 | 728 | 8 | 0 | 1 |
| 506 | 9311509 | 287 | 1600 | 36 | 4 | 3 |
| 507 | 9311510 | 225 | 1262 | 28 | 1 | 2 |
| 508 | 9311511 | 216 | 731 | 19 | 8 | 2 |
| 509 | 9311512 | 203 | 903 | 15 | 7 | 4 |
| 510 | 9311513 | 207 | 699 | 11 | 5 | 2 |
| 511 | 9311514 | 193 | 583 | 14 | 3 | 0 |
| 512 | 9311515 | 212 | 907 | 18 | 7 | 2 |
| 513 | 9311516 | 224 | 953 | 19 | 3 | 1 |
| 514 | 9311517 | 231 | 1095 | 15 | 10 | 2 |
| 515 | 9311518 | 219 | 1128 | 16 | 6 | 2 |
| 516 | 9311519 | 188 | 806 | 17 | 13 | 1 |
| 517 | 9311520 | 369 | 1911 | 31 | 14 | 2 |
| 518 | 9311521 | 180 | 471 | 12 | 7 | 1 |
| 519 | 9311522 | 177 | 761 | 17 | 5 | 1 |
| 520 | 9311523 | 189 | 884 | 12 | 4 | 1 |
| 521 | 9311524 | 241 | 963 | 29 | 4 | 1 |
| 522 | 9311525 | 187 | 808 | 7 | 5 | 1 |
| 523 | 9311526 | 290 | 1568 | 18 | 4 | 2 |
| 524 | 9311527 | 206 | 1101 | 16 | 9 | 2 |
| 525 | 9311528 | 210 | 923 | 13 | 4 | 2 |
| 526 | 9311529 | 340 | 1888 | 38 | 6 | 2 |
| 527 | 9311530 | 189 | 765 | 7 | 7 | 2 |
| 528 | 9311531 | 204 | 934 | 12 | 3 | 1 |
| 529 | 9311532 | 162 | 674 | 15 | 4 | 1 |
| 530 | 9311533 | 334 | 1664 | 21 | 2 | 3 |
| 531 | 9311534 | 252 | 1073 | 17 | 1 | 1 |
| 532 | 9311535 | 219 | 1054 | 30 | 5 | 2 |
| 533 | 9311536 | 194 | 801 | 19 | 4 | 4 |
| 534 | 9311537 | 218 | 1029 | 25 | 1 | 2 |
| 535 | 9311538 | 316 | 1746 | 16 | 2 | 2 |
| 536 | 9311539 | 226 | 1172 | 34 | 4 | 4 |
| 537 | 9311540 | 253 | 1315 | 29 | 4 | 1 |
| 538 | 9311541 | 269 | 1379 | 20 | 10 | 3 |
| 539 | 9311542 | 228 | 861 | 18 | 7 | 2 |
| 540 | 9311543 | 184 | 632 | 13 | 6 | 0 |
| 541 | 9311544 | 235 | 1136 | 21 | 3 | 1 |
| 542 | 9311545 | 218 | 966 | 13 | 6 | 2 |
| 543 | 9311546 | 207 | 783 | 23 | 4 | 3 |
| 544 | 9311547 | 190 | 731 | 18 | 0 | 1 |
| 545 | 9311548 | 183 | 775 | 11 | 4 | 1 |
| 546 | 9311549 | 273 | 1478 | 24 | 10 | 1 |
| 547 | 9311550 | 237 | 1105 | 18 | 9 | 5 |
| 548 | 9311551 | 225 | 897 | 17 | 5 | 0 |
| 549 | 9311552 | 280 | 1489 | 27 | 4 | 2 |
| 550 | 9311553 | 197 | 972 | 29 | 4 | 1 |
| 551 | 9311554 | 192 | 804 | 22 | 6 | 4 |
| 552 | 9311555 | 251 | 1191 | 16 | 6 | 2 |
| 553 | 9311556 | 221 | 887 | 11 | 9 | 1 |
| 554 | 9311557 | 204 | 572 | 18 | 5 | 3 |
| 555 | 9311558 | 210 | 805 | 17 | 4 | 2 |
| 556 | 9311559 | 245 | 1287 | 47 | 9 | 2 |
| 557 | 9311560 | 178 | 671 | 18 | 5 | 2 |
| 558 | 9311561 | 225 | 976 | 22 | 4 | 2 |
| 559 | 9311562 | 280 | 1725 | 32 | 4 | 0 |
| 560 | 9311563 | 230 | 996 | 24 | 5 | 2 |
| 561 | 9311564 | 232 | 1159 | 25 | 5 | 2 |
| 562 | 9311565 | 157 | 442 | 6 | 7 | 0 |
| 563 | 9311566 | 298 | 1643 | 31 | 5 | 2 |
| 564 | 9311567 | 182 | 731 | 10 | 1 | 1 |
| 565 | 9311568 | 265 | 1004 | 15 | 7 | 3 |
| 566 | 9311569 | 255 | 1234 | 24 | 2 | 2 |
| 567 | 9311570 | 170 | 628 | 18 | 2 | 1 |
| 568 | 9311571 | 243 | 1233 | 20 | 6 | 2 |
| 569 | 9311572 | 280 | 1194 | 29 | 2 | 2 |
| 570 | 9311573 | 237 | 1124 | 25 | 3 | 2 |
| 571 | 9311574 | 206 | 871 | 22 | 8 | 2 |
| 572 | 9311575 | 206 | 1037 | 25 | 1 | 2 |
| 573 | 9311576 | 260 | 1431 | 22 | 8 | 2 |
| 574 | 9311577 | 271 | 1290 | 28 | 5 | 2 |
| 575 | 9311578 | 249 | 1287 | 28 | 3 | 2 |
| 576 | 9311579 | 211 | 1038 | 26 | 6 | 1 |
| 577 | 9311580 | 254 | 1450 | 23 | 5 | 1 |
| 578 | 9311581 | 222 | 1036 | 16 | 3 | 1 |
| 579 | 9311582 | 234 | 996 | 24 | 3 | 1 |
| 580 | 9311583 | 227 | 1087 | 32 | 6 | 2 |
| 581 | 9311584 | 213 | 973 | 19 | 6 | 2 |
| 582 | 9311585 | 269 | 1720 | 44 | 3 | 1 |
| 583 | 9311586 | 183 | 613 | 16 | 4 | 2 |
| 584 | 9311587 | 210 | 871 | 23 | 5 | 0 |
| 585 | 9311588 | 317 | 1708 | 35 | 3 | 3 |
| 586 | 9311589 | 323 | 1828 | 32 | 2 | 1 |
| 587 | 9311590 | 211 | 912 | 19 | 2 | 2 |
| 588 | 9311591 | 166 | 440 | 8 | 7 | 1 |
| 589 | 9311592 | 299 | 1373 | 27 | 7 | 3 |
| 590 | 9311593 | 165 | 520 | 4 | 3 | 2 |
| 591 | 9311594 | 205 | 958 | 16 | 5 | 2 |
| 592 | 9311595 | 306 | 1626 | 30 | 7 | 3 |
| 593 | 9311596 | 216 | 929 | 14 | 5 | 1 |
| 594 | 9311597 | 178 | 721 | 21 | 6 | 3 |
| 595 | 9311598 | 497 | 2738 | 59 | 6 | 3 |
| 596 | 9311599 | 217 | 1048 | 29 | 8 | 1 |
| 597 | 9311600 | 260 | 1198 | 22 | 14 | 2 |
| 598 | 9311601 | 211 | 1010 | 20 | 3 | 2 |
| 599 | 9311602 | 194 | 785 | 16 | 4 | 3 |
| 600 | 9311603 | 216 | 830 | 13 | 9 | 1 |
| 601 | 9311604 | 249 | 1129 | 26 | 0 | 3 |
| 602 | 9311605 | 284 | 1382 | 24 | 8 | 1 |
| 603 | 9311606 | 214 | 948 | 22 | 7 | 0 |
| 604 | 9311607 | 192 | 821 | 17 | 7 | 4 |
| 605 | 9311608 | 269 | 878 | 20 | 8 | 1 |
| 606 | 9311609 | 271 | 1292 | 27 | 6 | 0 |
| 607 | 9311610 | 180 | 801 | 18 | 2 | 1 |
| 608 | 9311611 | 180 | 733 | 15 | 7 | 2 |
| 609 | 9311612 | 218 | 1075 | 15 | 3 | 0 |
| 610 | 9311613 | 198 | 945 | 30 | 2 | 2 |
| 611 | 9311614 | 238 | 1111 | 30 | 2 | 1 |
| 612 | 9311615 | 216 | 971 | 26 | 4 | 1 |
| 613 | 9311616 | 259 | 1119 | 23 | 4 | 2 |
| 614 | 9311617 | 200 | 997 | 15 | 6 | 1 |
| 615 | 9311618 | 195 | 821 | 11 | 3 | 2 |
| 616 | 9311619 | 264 | 1209 | 22 | 5 | 3 |
| 617 | 9311620 | 168 | 693 | 18 | 1 | 1 |
| 618 | 9311621 | 188 | 842 | 28 | 2 | 2 |
| 619 | 9311622 | 180 | 861 | 21 | 5 | 1 |
| 620 | 9311623 | 237 | 1367 | 27 | 3 | 3 |
| 621 | 9311624 | 210 | 1019 | 20 | 2 | 0 |
| 622 | 9311625 | 197 | 900 | 12 | 3 | 0 |
| 623 | 9311626 | 287 | 1408 | 30 | 11 | 2 |
| 624 | 9311627 | 219 | 923 | 28 | 5 | 3 |
| 625 | 9311628 | 185 | 941 | 16 | 1 | 3 |
| 626 | 9311629 | 224 | 1031 | 19 | 2 | 2 |
| 627 | 9311630 | 293 | 1398 | 17 | 5 | 2 |
| 628 | 9311631 | 242 | 1111 | 22 | 8 | 3 |
| 629 | 9311632 | 267 | 1196 | 34 | 11 | 2 |
| 630 | 9311633 | 254 | 1208 | 22 | 2 | 3 |
| 631 | 9311634 | 175 | 573 | 11 | 12 | 0 |
| 632 | 9311635 | 259 | 1269 | 45 | 7 | 1 |
| 633 | 9311636 | 203 | 1028 | 21 | 10 | 3 |
| 634 | 9311637 | 212 | 1004 | 22 | 2 | 2 |
| 635 | 9311638 | 256 | 1198 | 24 | 2 | 2 |
| 636 | 9311639 | 152 | 582 | 8 | 7 | 1 |
| 637 | 9311640 | 198 | 779 | 22 | 6 | 3 |
| 638 | 9311641 | 273 | 1572 | 28 | 1 | 2 |
| 639 | 9311642 | 243 | 1290 | 31 | 4 | 2 |
| 640 | 9311643 | 212 | 1165 | 15 | 2 | 1 |
| 641 | 9311644 | 256 | 1367 | 19 | 13 | 1 |
| 642 | 9311645 | 208 | 780 | 16 | 4 | 0 |
| 643 | 9311646 | 212 | 838 | 13 | 6 | 1 |
| 644 | 9311647 | 210 | 1005 | 12 | 4 | 0 |
| 645 | 9311648 | 249 | 1302 | 25 | 8 | 1 |
| 646 | 9311649 | 255 | 1253 | 36 | 8 | 1 |
| 647 | 9311650 | 193 | 733 | 16 | 1 | 2 |
| 648 | 9311651 | 259 | 1402 | 34 | 5 | 1 |
| 649 | 9311652 | 177 | 708 | 10 | 1 | 1 |
| 650 | 9311653 | 225 | 1098 | 26 | 6 | 3 |
| 651 | 9311654 | 382 | 1847 | 41 | 6 | 1 |
| 652 | 9311655 | 225 | 1033 | 22 | 4 | 3 |
| 653 | 9311656 | 231 | 946 | 30 | 5 | 3 |
| 654 | 9311657 | 196 | 864 | 19 | 3 | 0 |
| 655 | 9311658 | 210 | 933 | 15 | 4 | 3 |
| 656 | 9311659 | 201 | 953 | 23 | 5 | 2 |
| 657 | 9311660 | 176 | 740 | 12 | 10 | 0 |
| 658 | 9311661 | 280 | 1020 | 19 | 11 | 2 |
| 659 | 9311662 | 196 | 838 | 17 | 8 | 2 |
| 660 | 9311663 | 182 | 698 | 16 | 4 | 1 |
| 661 | 9311664 | 241 | 1066 | 20 | 4 | 1 |
| 662 | 9311665 | 206 | 870 | 23 | 2 | 2 |
| 663 | 9311666 | 294 | 1676 | 23 | 8 | 3 |
| 664 | 9311667 | 215 | 1075 | 23 | 7 | 0 |
| 665 | 9311668 | 263 | 1204 | 17 | 3 | 2 |
| 666 | 9311669 | 162 | 796 | 29 | 1 | 2 |
| 667 | 9311670 | 231 | 1255 | 28 | 2 | 2 |
| 668 | 9311671 | 234 | 946 | 33 | 4 | 2 |
| 669 | 9311672 | 229 | 853 | 22 | 4 | 1 |
| 670 | 9311673 | 157 | 431 | 14 | 3 | 2 |
| 671 | 9311674 | 220 | 1004 | 30 | 2 | 2 |
| 672 | 9311675 | 189 | 643 | 16 | 4 | 0 |
| 673 | 9311676 | 248 | 1263 | 28 | 4 | 1 |
| 674 | 9311677 | 192 | 598 | 14 | 2 | 1 |
| 675 | 9311678 | 204 | 1003 | 23 | 1 | 2 |
| 676 | 9311679 | 216 | 878 | 24 | 6 | 3 |
| 677 | 9311680 | 226 | 1101 | 34 | 6 | 1 |
| 678 | 9311681 | 169 | 677 | 16 | 5 | 0 |
| 679 | 9311682 | 180 | 488 | 16 | 5 | 2 |
| 680 | 9311683 | 155 | 533 | 11 | 2 | 0 |
| 681 | 9311684 | 216 | 1030 | 25 | 2 | 1 |
| 682 | 9311685 | 246 | 1211 | 18 | 4 | 1 |
| 683 | 9311686 | 220 | 891 | 27 | 4 | 2 |
| 684 | 9311687 | 252 | 1000 | 24 | 16 | 2 |
| 685 | 9311688 | 178 | 570 | 10 | 3 | 1 |
| 686 | 9311689 | 227 | 1076 | 18 | 7 | 2 |
| 687 | 9311690 | 192 | 796 | 22 | 3 | 2 |
| 688 | 9311691 | 183 | 782 | 22 | 8 | 2 |
| 689 | 9311692 | 244 | 1050 | 19 | 8 | 1 |
| 690 | 9311693 | 179 | 760 | 14 | 4 | 1 |
| 691 | 9311694 | 198 | 822 | 25 | 2 | 3 |
| 692 | 9311695 | 208 | 1252 | 26 | 5 | 2 |
| 693 | 9311696 | 173 | 656 | 12 | 1 | 0 |
| 694 | 9311697 | 214 | 906 | 29 | 7 | 1 |
| 695 | 9311698 | 228 | 809 | 16 | 4 | 0 |
| 696 | 9311699 | 201 | 926 | 22 | 2 | 1 |
| 697 | 9311700 | 183 | 671 | 14 | 0 | 0 |
| 698 | 9311701 | 193 | 591 | 11 | 8 | 1 |
| 699 | 9311702 | 182 | 940 | 17 | 6 | 1 |
| 700 | 9311703 | 378 | 1912 | 33 | 6 | 1 |
| 701 | 9311704 | 188 | 701 | 14 | 10 | 0 |
| 702 | 9311705 | 175 | 963 | 24 | 10 | 1 |
| 703 | 9311706 | 291 | 1304 | 26 | 9 | 3 |
| 704 | 9311707 | 184 | 673 | 23 | 6 | 0 |
| 705 | 9311708 | 231 | 982 | 31 | 10 | 3 |
| 706 | 9311709 | 211 | 857 | 17 | 9 | 3 |
| 707 | 9311710 | 230 | 1051 | 18 | 6 | 3 |
| 708 | 9311711 | 267 | 1460 | 32 | 5 | 3 |
| 709 | 9311712 | 202 | 793 | 14 | 7 | 1 |
| 710 | 9311713 | 246 | 1007 | 28 | 5 | 4 |
| 711 | 9311714 | 208 | 1158 | 27 | 3 | 2 |
| 712 | 9311715 | 241 | 1310 | 37 | 3 | 2 |
| 713 | 9311716 | 184 | 707 | 17 | 2 | 0 |
| 714 | 9311717 | 259 | 1380 | 21 | 7 | 3 |
| 715 | 9311718 | 219 | 1049 | 36 | 5 | 1 |
| 716 | 9311719 | 258 | 1198 | 36 | 7 | 1 |
| 717 | 9311720 | 219 | 790 | 18 | 4 | 3 |
| 718 | 9311721 | 189 | 541 | 7 | 4 | 0 |
| 719 | 9311722 | 241 | 1248 | 28 | 9 | 4 |
| 720 | 9311723 | 194 | 948 | 25 | 4 | 2 |
| 721 | 9311724 | 198 | 807 | 22 | 7 | 1 |
| 722 | 9311725 | 173 | 734 | 9 | 2 | 1 |
| 723 | 9311726 | 194 | 776 | 18 | 5 | 2 |
| 724 | 9311727 | 257 | 1371 | 24 | 6 | 3 |
| 725 | 9311728 | 205 | 1019 | 24 | 7 | 2 |
| 726 | 9311729 | 225 | 912 | 17 | 4 | 0 |
| 727 | 9311730 | 191 | 601 | 17 | 5 | 1 |
| 728 | 9311731 | 192 | 842 | 23 | 2 | 1 |
| 729 | 9311732 | 205 | 725 | 11 | 4 | 1 |
| 730 | 9311733 | 218 | 1113 | 24 | 6 | 1 |
| 731 | 9311734 | 274 | 1236 | 31 | 2 | 2 |
| 732 | 9311735 | 207 | 678 | 17 | 14 | 2 |
| 733 | 9311736 | 233 | 1195 | 27 | 7 | 1 |
| 734 | 9311737 | 295 | 1494 | 33 | 4 | 1 |
| 735 | 9311738 | 231 | 1005 | 29 | 6 | 3 |
| 736 | 9311739 | 219 | 1028 | 34 | 2 | 2 |
| 737 | 9311740 | 212 | 1150 | 19 | 3 | 2 |
| 738 | 9311741 | 221 | 1066 | 25 | 9 | 1 |
| 739 | 9311742 | 177 | 815 | 19 | 2 | 0 |
| 740 | 9311743 | 266 | 1818 | 18 | 6 | 2 |
| 741 | 9311744 | 202 | 957 | 22 | 2 | 2 |
| 742 | 9311745 | 190 | 855 | 17 | 2 | 0 |
| 743 | 9311746 | 311 | 1579 | 24 | 9 | 2 |
| 744 | 9311747 | 204 | 1171 | 27 | 4 | 3 |
| 745 | 9311748 | 189 | 909 | 28 | 3 | 3 |
| 746 | 9311749 | 176 | 818 | 17 | 4 | 0 |
| 747 | 9311750 | 243 | 1127 | 23 | 2 | 1 |
| 748 | 9311751 | 228 | 899 | 19 | 11 | 1 |
| 749 | 9311752 | 201 | 1040 | 20 | 5 | 1 |
| 750 | 9311753 | 208 | 1082 | 30 | 6 | 3 |
| 751 | 9311754 | 233 | 1059 | 22 | 5 | 3 |
| 752 | 9311755 | 172 | 574 | 24 | 3 | 4 |
| 753 | 9311756 | 216 | 892 | 31 | 8 | 4 |
| 754 | 9311757 | 219 | 931 | 15 | 5 | 1 |
| 755 | 9311758 | 191 | 707 | 24 | 2 | 1 |
| 756 | 9311759 | 242 | 1052 | 20 | 4 | 2 |
| 757 | 9311760 | 229 | 904 | 25 | 5 | 2 |
| 758 | 9311761 | 196 | 979 | 27 | 8 | 1 |
| 759 | 9311762 | 274 | 1591 | 25 | 3 | 1 |
| 760 | 9311763 | 180 | 886 | 9 | 1 | 2 |
| 761 | 9311764 | 254 | 975 | 28 | 7 | 4 |
| 762 | 9311765 | 180 | 818 | 7 | 3 | 4 |
| 763 | 9311766 | 162 | 507 | 15 | 4 | 1 |
| 764 | 9311767 | 203 | 780 | 19 | 4 | 1 |
| 765 | 9311768 | 136 | 408 | 6 | 10 | 1 |
| 766 | 9311769 | 210 | 850 | 18 | 5 | 2 |
| 767 | 9311770 | 273 | 1417 | 40 | 8 | 2 |
| 768 | 9311771 | 224 | 805 | 8 | 5 | 1 |
| 769 | 9311772 | 181 | 738 | 23 | 6 | 1 |
| 770 | 9311773 | 199 | 971 | 17 | 4 | 1 |
| 771 | 9311774 | 263 | 1184 | 41 | 5 | 4 |
| 772 | 9311775 | 308 | 1620 | 37 | 10 | 1 |
| 773 | 9311776 | 260 | 1225 | 22 | 11 | 1 |
| 774 | 9311777 | 365 | 2031 | 47 | 1 | 2 |
| 775 | 9311778 | 228 | 710 | 12 | 8 | 0 |
| 776 | 9311779 | 202 | 992 | 27 | 1 | 3 |
| 777 | 9311780 | 234 | 1144 | 14 | 7 | 3 |
| 778 | 9311781 | 214 | 783 | 9 | 12 | 2 |
| 779 | 9311782 | 168 | 676 | 13 | 1 | 2 |
| 780 | 9311783 | 201 | 702 | 10 | 3 | 0 |
| 781 | 9311784 | 152 | 571 | 10 | 3 | 0 |
| 782 | 9311785 | 220 | 1030 | 23 | 3 | 1 |
| 783 | 9311786 | 264 | 1201 | 29 | 4 | 0 |
| 784 | 9311787 | 189 | 700 | 15 | 1 | 2 |
| 785 | 9311788 | 224 | 951 | 32 | 3 | 3 |
| 786 | 9311789 | 192 | 659 | 13 | 4 | 1 |
| 787 | 9311790 | 261 | 1311 | 18 | 6 | 1 |
| 788 | 9311791 | 241 | 1040 | 6 | 5 | 0 |
| 789 | 9311792 | 215 | 1125 | 21 | 7 | 2 |
| 790 | 9311793 | 211 | 786 | 25 | 8 | 5 |
| 791 | 9311794 | 235 | 1064 | 31 | 2 | 3 |
| 792 | 9311795 | 281 | 1488 | 18 | 3 | 3 |
| 793 | 9311796 | 221 | 1213 | 24 | 4 | 1 |
| 794 | 9311797 | 193 | 672 | 23 | 4 | 4 |
| 795 | 9311798 | 210 | 1074 | 16 | 3 | 0 |
| 796 | 9311799 | 244 | 1056 | 20 | 6 | 2 |
| 797 | 9311800 | 260 | 931 | 30 | 8 | 4 |
| 798 | 9311801 | 248 | 1149 | 23 | 7 | 1 |
| 799 | 9311802 | 234 | 1319 | 19 | 4 | 3 |
| 800 | 9311803 | 214 | 1104 | 24 | 5 | 2 |
| 801 | 9311804 | 302 | 1754 | 29 | 3 | 1 |
| 802 | 9311805 | 169 | 601 | 5 | 4 | 0 |
| 803 | 9311806 | 259 | 1634 | 31 | 5 | 2 |
| 804 | 9311807 | 252 | 1236 | 34 | 4 | 1 |
| 805 | 9311808 | 206 | 1007 | 16 | 7 | 3 |
| 806 | 9311809 | 260 | 1291 | 23 | 12 | 1 |
| 807 | 9311810 | 205 | 933 | 24 | 12 | 2 |
| 808 | 9311811 | 172 | 639 | 10 | 2 | 0 |
| 809 | 9311812 | 213 | 757 | 17 | 9 | 2 |
| 810 | 9311813 | 208 | 841 | 12 | 3 | 2 |
| 811 | 9311814 | 278 | 1138 | 24 | 5 | 2 |
| 812 | 9311815 | 232 | 1250 | 31 | 2 | 2 |
| 813 | 9311816 | 265 | 1414 | 31 | 2 | 3 |
| 814 | 9311817 | 146 | 503 | 9 | 2 | 0 |
| 815 | 9311818 | 205 | 832 | 13 | 4 | 1 |
| 816 | 9311819 | 366 | 2217 | 45 | 3 | 1 |
| 817 | 9311820 | 223 | 822 | 12 | 4 | 2 |
| 818 | 9311821 | 208 | 843 | 11 | 4 | 1 |
| 819 | 9311822 | 258 | 1110 | 27 | 7 | 1 |
| 820 | 9311823 | 230 | 1472 | 29 | 10 | 0 |
| 821 | 9311824 | 212 | 727 | 17 | 3 | 1 |
| 822 | 9311825 | 175 | 819 | 15 | 3 | 1 |
| 823 | 9311826 | 224 | 1264 | 33 | 1 | 2 |
| 824 | 9311827 | 229 | 1110 | 31 | 8 | 2 |
| 825 | 9311828 | 208 | 812 | 27 | 3 | 2 |
| 826 | 9311829 | 190 | 666 | 7 | 6 | 2 |
| 827 | 9311830 | 233 | 1275 | 29 | 1 | 1 |
| 828 | 9311831 | 193 | 841 | 18 | 5 | 1 |
| 829 | 9311832 | 172 | 652 | 13 | 4 | 1 |
| 830 | 9311833 | 241 | 1055 | 31 | 4 | 2 |
| 831 | 9311834 | 196 | 782 | 22 | 5 | 3 |
| 832 | 9311835 | 257 | 1139 | 28 | 4 | 2 |
| 833 | 9311836 | 217 | 954 | 17 | 1 | 2 |
| 834 | 9311837 | 222 | 971 | 26 | 6 | 3 |
| 835 | 9311838 | 169 | 745 | 15 | 8 | 1 |
| 836 | 9311839 | 223 | 1077 | 31 | 3 | 2 |
| 837 | 9311840 | 256 | 1392 | 31 | 8 | 3 |
| 838 | 9311841 | 232 | 988 | 11 | 13 | 0 |
| 839 | 9311842 | 225 | 1231 | 30 | 1 | 3 |
| 840 | 9311843 | 226 | 1042 | 37 | 5 | 4 |
| 841 | 9311844 | 222 | 1047 | 22 | 2 | 2 |
| 842 | 9311845 | 233 | 1187 | 26 | 6 | 3 |
| 843 | 9311846 | 209 | 899 | 16 | 4 | 4 |
| 844 | 9311847 | 223 | 1185 | 17 | 3 | 4 |
| 845 | 9311848 | 323 | 1840 | 43 | 1 | 1 |
| 846 | 9311849 | 221 | 949 | 33 | 4 | 6 |
| 847 | 9311850 | 187 | 761 | 9 | 2 | 1 |
| 848 | 9311851 | 183 | 818 | 18 | 3 | 2 |
| 849 | 9311852 | 218 | 902 | 19 | 3 | 0 |
| 850 | 9311853 | 204 | 798 | 17 | 2 | 3 |
| 851 | 9311854 | 268 | 1332 | 36 | 6 | 1 |
| 852 | 9311855 | 237 | 982 | 13 | 5 | 1 |
| 853 | 9311856 | 240 | 1054 | 22 | 3 | 2 |
| 854 | 9311857 | 218 | 957 | 18 | 0 | 2 |
| 855 | 9311858 | 200 | 710 | 13 | 8 | 4 |
| 856 | 9311859 | 230 | 1003 | 15 | 2 | 2 |
| 857 | 9311860 | 245 | 1313 | 26 | 9 | 0 |
| 858 | 9311861 | 272 | 1373 | 30 | 7 | 1 |
| 859 | 9311862 | 324 | 1617 | 33 | 7 | 2 |
| 860 | 9311863 | 184 | 699 | 18 | 7 | 1 |
| 861 | 9311864 | 253 | 1311 | 23 | 3 | 0 |
| 862 | 9311865 | 203 | 765 | 16 | 3 | 1 |
| 863 | 9311866 | 235 | 1188 | 14 | 4 | 1 |
| 864 | 9311867 | 178 | 737 | 10 | 0 | 2 |
| 865 | 9311868 | 241 | 1161 | 30 | 5 | 2 |
| 866 | 9311869 | 207 | 810 | 20 | 3 | 1 |
| 867 | 9311870 | 180 | 542 | 17 | 11 | 3 |
| 868 | 9311871 | 215 | 1002 | 15 | 3 | 1 |
| 869 | 9311872 | 317 | 1818 | 34 | 6 | 2 |
| 870 | 9311873 | 224 | 1392 | 26 | 4 | 1 |
| 871 | 9311874 | 196 | 830 | 20 | 1 | 2 |
| 872 | 9311875 | 279 | 1315 | 33 | 10 | 3 |
| 873 | 9311876 | 214 | 826 | 20 | 4 | 2 |
| 874 | 9311877 | 254 | 1371 | 32 | 1 | 3 |
| 875 | 9311878 | 240 | 1229 | 27 | 4 | 2 |
| 876 | 9311879 | 207 | 971 | 25 | 2 | 1 |
| 877 | 9311880 | 253 | 1122 | 18 | 3 | 2 |
| 878 | 9311881 | 208 | 802 | 23 | 5 | 2 |
| 879 | 9311882 | 216 | 929 | 31 | 1 | 3 |
| 880 | 9311883 | 241 | 884 | 19 | 4 | 2 |
| 881 | 9311884 | 199 | 787 | 19 | 5 | 1 |
| 882 | 9311885 | 206 | 986 | 32 | 6 | 2 |
| 883 | 9311886 | 244 | 1180 | 24 | 10 | 1 |
| 884 | 9311887 | 209 | 920 | 15 | 5 | 1 |
| 885 | 9311888 | 191 | 715 | 19 | 2 | 2 |
| 886 | 9311889 | 186 | 796 | 20 | 5 | 3 |
| 887 | 9311890 | 209 | 907 | 26 | 2 | 3 |
| 888 | 9311891 | 258 | 1336 | 21 | 8 | 2 |
| 889 | 9311892 | 222 | 971 | 19 | 6 | 3 |
| 890 | 9311893 | 205 | 1174 | 33 | 2 | 1 |
| 891 | 9311894 | 199 | 596 | 17 | 5 | 2 |
| 892 | 9311895 | 205 | 896 | 21 | 4 | 2 |
| 893 | 9311896 | 204 | 1016 | 15 | 4 | 0 |
| 894 | 9311897 | 198 | 893 | 17 | 6 | 2 |
| 895 | 9311898 | 197 | 607 | 17 | 3 | 2 |
| 896 | 9311899 | 206 | 891 | 18 | 7 | 2 |
| 897 | 9311900 | 205 | 953 | 28 | 10 | 2 |
| 898 | 9311901 | 224 | 978 | 17 | 5 | 0 |
| 899 | 9311902 | 342 | 1804 | 36 | 7 | 3 |
| 900 | 9311903 | 241 | 1341 | 28 | 2 | 2 |
| 901 | 9311904 | 232 | 1053 | 20 | 3 | 2 |
| 902 | 9311905 | 397 | 2165 | 32 | 1 | 3 |
| 903 | 9311906 | 233 | 1163 | 28 | 3 | 2 |
| 904 | 9311907 | 155 | 505 | 11 | 6 | 1 |
| 905 | 9311908 | 237 | 1396 | 13 | 3 | 1 |
| 906 | 9311909 | 190 | 971 | 14 | 0 | 0 |
| 907 | 9311910 | 200 | 745 | 16 | 3 | 2 |
| 908 | 9311911 | 229 | 1149 | 20 | 6 | 1 |
| 909 | 9311912 | 220 | 988 | 26 | 6 | 3 |
| 910 | 9311913 | 230 | 1116 | 32 | 9 | 2 |
| 911 | 9311914 | 239 | 1062 | 19 | 7 | 2 |
| 912 | 9311915 | 175 | 816 | 19 | 3 | 2 |
| 913 | 9311916 | 194 | 886 | 16 | 2 | 1 |
| 914 | 9311917 | 212 | 821 | 19 | 0 | 3 |
| 915 | 9311918 | 243 | 884 | 23 | 1 | 2 |
| 916 | 9311919 | 184 | 634 | 29 | 6 | 2 |
| 917 | 9311920 | 185 | 805 | 27 | 1 | 3 |
| 918 | 9311921 | 212 | 1139 | 26 | 6 | 1 |
| 919 | 9311922 | 210 | 983 | 22 | 8 | 1 |
| 920 | 9311923 | 191 | 1043 | 26 | 5 | 1 |
| 921 | 9311924 | 194 | 738 | 13 | 4 | 1 |
| 922 | 9311925 | 219 | 730 | 21 | 13 | 2 |
| 923 | 9311926 | 250 | 1001 | 17 | 7 | 3 |
| 924 | 9311927 | 264 | 1547 | 29 | 3 | 0 |
| 925 | 9311928 | 187 | 932 | 13 | 5 | 0 |
| 926 | 9311929 | 237 | 1185 | 25 | 6 | 3 |
| 927 | 9311930 | 229 | 1207 | 18 | 7 | 3 |
| 928 | 9311931 | 169 | 498 | 8 | 8 | 1 |
| 929 | 9311932 | 265 | 1197 | 32 | 4 | 2 |
| 930 | 9311933 | 169 | 597 | 6 | 0 | 1 |
| 931 | 9311934 | 197 | 796 | 16 | 3 | 2 |
| 932 | 9311935 | 236 | 1150 | 28 | 6 | 1 |
| 933 | 9311936 | 196 | 762 | 15 | 2 | 1 |
| 934 | 9311937 | 185 | 792 | 10 | 3 | 1 |
| 935 | 9311938 | 203 | 859 | 15 | 1 | 0 |
| 936 | 9311939 | 207 | 1000 | 24 | 6 | 1 |
| 937 | 9311940 | 200 | 904 | 27 | 2 | 2 |
| 938 | 9311941 | 185 | 978 | 20 | 6 | 0 |
| 939 | 9311942 | 198 | 931 | 14 | 6 | 1 |
| 940 | 9311943 | 247 | 1065 | 33 | 7 | 1 |
| 941 | 9311944 | 221 | 1067 | 22 | 10 | 1 |
| 942 | 9311945 | 213 | 875 | 22 | 3 | 3 |
| 943 | 9311946 | 166 | 569 | 17 | 6 | 1 |
| 944 | 9311947 | 203 | 849 | 16 | 6 | 0 |
| 945 | 9311948 | 233 | 724 | 16 | 7 | 2 |
| 946 | 9311949 | 252 | 1429 | 16 | 7 | 2 |
| 947 | 9311950 | 230 | 740 | 17 | 3 | 4 |
| 948 | 9311951 | 234 | 1194 | 14 | 3 | 3 |
| 949 | 9311952 | 213 | 893 | 20 | 4 | 0 |
| 950 | 9311953 | 234 | 1203 | 21 | 10 | 2 |
| 951 | 9311954 | 228 | 1089 | 20 | 14 | 3 |
| 952 | 9311955 | 191 | 990 | 22 | 3 | 0 |
| 953 | 9311956 | 218 | 695 | 19 | 5 | 3 |
| 954 | 9311957 | 197 | 973 | 26 | 3 | 2 |
| 955 | 9311958 | 187 | 930 | 16 | 3 | 1 |
| 956 | 9311959 | 205 | 750 | 17 | 8 | 1 |
| 957 | 9311960 | 239 | 1072 | 30 | 4 | 2 |
| 958 | 9311961 | 182 | 758 | 27 | 2 | 3 |
| 959 | 9311962 | 286 | 1283 | 34 | 8 | 2 |
| 960 | 9311963 | 195 | 859 | 22 | 4 | 3 |
| 961 | 9311964 | 216 | 832 | 26 | 1 | 1 |
| 962 | 9311965 | 211 | 882 | 27 | 2 | 3 |
| 963 | 9311966 | 255 | 1202 | 31 | 2 | 2 |
| 964 | 9311967 | 160 | 529 | 5 | 10 | 2 |
| 965 | 9311968 | 169 | 669 | 12 | 8 | 3 |
| 966 | 9311969 | 163 | 621 | 9 | 3 | 0 |
| 967 | 9311970 | 252 | 1170 | 32 | 3 | 3 |
| 968 | 9311971 | 224 | 942 | 26 | 8 | 2 |
| 969 | 9311972 | 230 | 973 | 14 | 4 | 2 |
| 970 | 9311973 | 196 | 636 | 12 | 6 | 2 |
| 971 | 9311974 | 147 | 659 | 8 | 2 | 0 |
| 972 | 9311975 | 199 | 676 | 16 | 4 | 1 |
| 973 | 9311976 | 260 | 1103 | 32 | 3 | 2 |
| 974 | 9311977 | 262 | 1086 | 22 | 7 | 1 |
| 975 | 9311978 | 241 | 1115 | 22 | 7 | 2 |
| 976 | 9311979 | 232 | 1112 | 26 | 2 | 1 |
| 977 | 9311980 | 187 | 1024 | 16 | 5 | 0 |
| 978 | 9311981 | 203 | 879 | 20 | 1 | 2 |
| 979 | 9311982 | 194 | 806 | 19 | 4 | 2 |
| 980 | 9311983 | 248 | 1447 | 17 | 4 | 1 |
| 981 | 9311984 | 216 | 928 | 13 | 5 | 2 |
| 982 | 9311985 | 222 | 947 | 28 | 2 | 3 |
| 983 | 9311986 | 218 | 926 | 35 | 7 | 3 |
| 984 | 9311987 | 210 | 1072 | 16 | 4 | 3 |
| 985 | 9311988 | 189 | 988 | 16 | 2 | 0 |
| 986 | 9311989 | 215 | 951 | 21 | 4 | 0 |
| 987 | 9311990 | 245 | 1228 | 27 | 4 | 3 |
| 988 | 9311991 | 231 | 1099 | 28 | 5 | 2 |
| 989 | 9311992 | 189 | 899 | 17 | 2 | 0 |
| 990 | 9311993 | 225 | 1058 | 18 | 6 | 1 |
| 991 | 9311994 | 222 | 1065 | 31 | 5 | 1 |
| 992 | 9311995 | 231 | 1144 | 11 | 4 | 0 |
| 993 | 9311996 | 184 | 824 | 11 | 6 | 1 |
| 994 | 9311997 | 264 | 1258 | 25 | 8 | 2 |
| 995 | 9311998 | 173 | 726 | 19 | 7 | 1 |
| 996 | 9311999 | 202 | 779 | 19 | 4 | 1 |
| 997 | 9312000 | 183 | 674 | 19 | 8 | 2 |
| 998 | 9312001 | 228 | 1066 | 26 | 6 | 0 |
| 999 | 9312002 | 183 | 826 | 18 | 0 | 1 |

## Throughput

- Turns simulated: 220445 (first play of each game)
- Aggregate: 188.7 turns/s over 1168.2s wall
- Per game thread: 31.7 turns/s

## Determinism

Not checked (run with `--determinism`).

## Panics

None.
