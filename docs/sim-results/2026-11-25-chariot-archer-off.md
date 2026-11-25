# Simulation: chariot-archer-off (2026-11-25)

1024 games, first seed 9311003, brain `scripted`, turn limit off, safety cap 1200, 4 threads, started 2026-11-25T03:13:34Z. Wall time 1640.4s.

Source checkout at run start: `4ec983d4d3a41bb88000692d19b0cffdb5308f94`; dirty: yes. Map 48×32, 8 initial seats. `first_setup` and exact binary argv are recorded in JSON. Ordinary games use seed + i (wrapping); the strategy tournament schedule is specified below. All games use a deterministic shuffle of the full faction pool, taking eight distinct factions, with any Proxy rotation described below.

Command argv: `["./target/release/legatum-sim","--players","8","--factions","Egypt,Rome,Sparta,Aztecs,Mongols,Celts,Athens,India","--rotate-factions","--difficulty","hard","--games","1024","--seed","9311003","--threads","4","--label","chariot-archer-off","--rule-off","egypt_chariot_archer"]`

## Overview

| Metric | Value |
| --- | --- |
| Games (non-panic) | 1024 of 1024 |
| Game length, turns (mean / median / min-max) | 225.8 / 221 / 145-470 |
| Win reason mix | VP hold 98.1%, Capitals 1.9% |
| Median end turn | 221 |
| Ending shares of played games (capitals / VP hold / turn limit / other) | 1.9% / 98.1% / 0.0% / 0.0% |
| Games reaching the World War era (any empire) | 1018 of 1024 (99.4%) |
| VP hold length | 10 turn ends (the reaching turn counts) |
| Holds started / broken / completed / unfinished | 1475 / 359 / 1010 / 106 |
| Hold break rate | 24.3% of started; 26.2% of resolved (broken / (broken + completed)) |
| Hold wins | 1005 |
| First reach to win, turns (mean / median / min-max) | 11.1 / 9 / 9-112 |
| Final hold start to win, turns | 9.0 / 9 / 9-9 |
| Comebacks (winner broke an earlier hold of its own) | 119 |
| Wins by a team another team beat to its target | 61 |

Game length by how the game ended (turns resolved; a stall is a safety-cap stop, never a win):

| Ended by | Games | Length mean / median / min-max |
| --- | --- | --- |
| VP hold | 1005 | 224.9 / 220 / 145-470 |
| Capitals | 19 | 275.6 / 277 / 189-395 |

Winner share by faction (wins of that faction's seats / all winning seats):

| Faction | Wins | Share of wins |
| --- | --- | --- |
| India | 95 | 9.3% |
| Celts | 142 | 13.9% |
| Mongols | 93 | 9.1% |
| Sparta | 123 | 12.0% |
| Athens | 159 | 15.5% |
| Aztecs | 133 | 13.0% |
| Egypt | 224 | 21.9% |
| Rome | 55 | 5.4% |

## Outcomes

| Outcome | Games | Share |
| --- | --- | --- |
| VP hold | 1005 | 98.1% |
| Conquest | 0 | 0.0% |
| Capitals | 19 | 1.9% |
| Turn limit | 0 | 0.0% |
| Draw | 0 | 0.0% |
| Stalled (safety cap) | 0 | 0.0% |
| Panic | 0 | 0.0% |

## Win rate by faction

Per seat: wins / seats of that faction. With 8 single-empire teams, an even game is 12.5% per seat.

| Faction | Seats | Wins | Win rate |
| --- | --- | --- | --- |
| India | 1024 | 95 | 9.3% |
| Celts | 1024 | 142 | 13.9% |
| Mongols | 1024 | 93 | 9.1% |
| Sparta | 1024 | 123 | 12.0% |
| Athens | 1024 | 159 | 15.5% |
| Aztecs | 1024 | 133 | 13.0% |
| Egypt | 1024 | 224 | 21.9% |
| Rome | 1024 | 55 | 5.4% |

## Win rate by difficulty

All initial seats in non-panicked games, including eliminated empires and cap stops. Determinism replays are not additional samples.

| Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- |
| Hard | 8192 | 1024 | 12.5% |

## Win rate by faction and difficulty

| Faction | Difficulty | Seats | Wins | Win rate |
| --- | --- | --- | --- | --- |
| India | Hard | 1024 | 95 | 9.3% |
| Celts | Hard | 1024 | 142 | 13.9% |
| Mongols | Hard | 1024 | 93 | 9.1% |
| Sparta | Hard | 1024 | 123 | 12.0% |
| Athens | Hard | 1024 | 159 | 15.5% |
| Aztecs | Hard | 1024 | 133 | 13.0% |
| Egypt | Hard | 1024 | 224 | 21.9% |
| Rome | Hard | 1024 | 55 | 5.4% |

## Faction goal progress

Per seat: how often the faction's permanent goal was reached before the game ended, and the turn its VP was first awarded (over the seats that reached it). Seats are all initial empires of non-panicked games with telemetry, including eliminated empires and cap stops.

| Faction | Seats | Reached | Goal rate | Min | P10 | Median turn | P90 | Max |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| India | 1024 | 503 | 49.1% | 58 | 84 | 126 | 194 | 344 |
| Celts | 1024 | 602 | 58.8% | 34 | 176 | 210 | 210 | 273 |
| Mongols | 1024 | 116 | 11.3% | 103 | 165 | 201 | 244 | 412 |
| Sparta | 1024 | 119 | 11.6% | 119 | 156 | 208 | 277 | 354 |
| Athens | 1024 | 301 | 29.4% | 123 | 167 | 239 | 255 | 255 |
| Aztecs | 1024 | 325 | 31.7% | 145 | 176 | 225 | 237 | 285 |
| Egypt | 1024 | 401 | 39.2% | 135 | 186 | 200 | 234 | 319 |
| Rome | 1024 | 33 | 3.2% | 127 | 142 | 197 | 265 | 328 |

## Goal cities (Rome's Colonia, Egypt's tomb)

Per seat of the faction: its goal city stood (median first turn), the seat scored it, held it at the end, a rival held it at the end, it changed hands. Rome: seats with a level-3 melee unit while the Colonia was open (median first turn) and the share of those seat-turns with no legal Colonia site on the map. Egypt: seats that started the work (median first start), work turns and pauses per starting seat.

| Faction | Seats | Stood | Turn | Scored | Held at end | Rival holds | Changed hands | Level 3 | Turn | No site | Started | Turn | Work turns | Pauses |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Egypt | 1024 | 346 | 201 | 346 | 262 | 51 | 66 | 0 | - | - | 569 | 186 | 11.6 | 0.6 |
| Rome | 1024 | 0 | - | 0 | 0 | 0 | 0 | 0 | - | - | 0 | - | - | - |

## Distributions

| Metric | Min | P10 | Median | P90 | Max |
| --- | --- | --- | --- | --- | --- |
| Observed game length (turns, including cap stops) | 145 | 182 | 221 | 275 | 470 |
| Winning team VP | 5 | 10 | 11 | 13 | 18 |
| First Soldier completed (turn, per empire) | 5 | 8 | 25 | 47 | 239 |
| First Settler completed (turn, per empire) | 5 | 12 | 14 | 42 | 252 |
| First city founded (turn, per empire) | 1 | 1 | 1 | 1 | 1 |
| Second city founded (turn, per empire) | 8 | 16 | 22 | 59 | 236 |

Safety-cap stops are censored observations, not completed game lengths.

Openings: 8192 observed empires in 1024 first plays; 0 games without opening telemetry excluded. Turn distributions include only empires that reached the milestone. Never means not reached by the end of observed play, including eliminated empires and safety-cap stops. Per-empire turns are in JSON (`games[].telemetry.openings`, keyed by player ID).

| Opening milestone | Reached empires | Never empires |
| --- | --- | --- |
| First Soldier completed | 8072 | 120 |
| First Settler completed | 7131 | 1061 |
| First city founded | 8192 | 0 |
| Second city founded | 7870 | 322 |

Final VP per empire (all seats):

| VP | Empires |
| --- | --- |
| 0 | 1979 |
| 1 | 1365 |
| 2 | 989 |
| 3 | 763 |
| 4 | 581 |
| 5 | 448 |
| 6 | 333 |
| 7 | 276 |
| 8 | 170 |
| 9 | 162 |
| 10 | 521 |
| 11 | 310 |
| 12 | 131 |
| 13 | 89 |
| 14 | 45 |
| 15 | 22 |
| 16 | 5 |
| 17 | 2 |
| 18 | 1 |

## End state

- Average game length: 225.8 turns
- Average cities per empire: 4.44
- Average techs per empire: 30.14
- Average eliminations per game: 2.43

## Strategy diversity: observed behavior (classifier v2)

1024 finished games; 8192 player seats; 1024 winning seats. Excluded 0 unfinished/panicked games and 0 games with missing labels. Winner entropy: **0.4278** (normalized over 6 categories, including zero-win categories).

Thresholds, biases and denominators: [strategy-diversity.md](../strategy-diversity.md).

| Archetype | Players | Wins | Win rate | All-player frequency | Winner frequency |
| --- | ---: | ---: | ---: | ---: | ---: |
| Rush | 3183 | 122 | 3.8% | 38.9% | 11.9% |
| Wide | 2853 | 792 | 27.8% | 34.8% | 77.3% |
| Science | 0 | 0 | n/a | 0.0% | 0.0% |
| Turtle/Wonder | 766 | 75 | 9.8% | 9.4% | 7.3% |
| Economic | 235 | 33 | 14.0% | 2.9% | 3.2% |
| Generalist | 1155 | 2 | 0.2% | 14.1% | 0.2% |

### Matchup matrix

Cell = strict ahead % / tied % (co-occurrence games). Each game has equal weight; all cross-team seat pairs within it share that weight. Rank: winning team first, then surviving empires by VP, then eliminated empires by later elimination turn. Equal ranks tie; no ID tiebreak. This is a multiplayer association, not a duel win rate.

| Row vs column | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Rush | — | 15.5% / 3.2% (1000) | n/a / n/a (0) | 18.0% / 7.8% (530) | 22.8% / 6.5% (210) | 73.0% / 3.6% (753) |
| Wide | 81.3% / 3.2% (1000) | — | n/a / n/a (0) | 60.0% / 5.9% (530) | 60.9% / 3.7% (213) | 93.8% / 1.5% (755) |
| Science | n/a / n/a (0) | n/a / n/a (0) | — | n/a / n/a (0) | n/a / n/a (0) | n/a / n/a (0) |
| Turtle/Wonder | 74.2% / 7.8% (530) | 34.1% / 5.9% (530) | n/a / n/a (0) | — | 50.4% / 4.0% (95) | 88.9% / 4.3% (382) |
| Economic | 70.7% / 6.5% (210) | 35.5% / 3.7% (213) | n/a / n/a (0) | 45.6% / 4.0% (95) | — | 92.2% / 3.6% (150) |
| Generalist | 23.3% / 3.6% (753) | 4.6% / 1.5% (755) | n/a / n/a (0) | 6.8% / 4.3% (382) | 4.2% / 3.6% (150) | — |

### Faction wins by archetype

Cell = wins / seats; win rate in JSON.

| Faction | Rush | Wide | Science | Turtle/Wonder | Economic | Generalist |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| India | 5 / 291 | 90 / 508 | 0 / 0 | 0 / 24 | 0 / 4 | 0 / 197 |
| Celts | 4 / 168 | 127 / 530 | 0 / 0 | 10 / 123 | 1 / 25 | 0 / 178 |
| Mongols | 31 / 654 | 61 / 209 | 0 / 0 | 0 / 0 | 1 / 19 | 0 / 142 |
| Sparta | 28 / 531 | 59 / 194 | 0 / 0 | 11 / 117 | 23 / 86 | 2 / 96 |
| Athens | 2 / 122 | 140 / 452 | 0 / 0 | 17 / 177 | 0 / 25 | 0 / 248 |
| Aztecs | 0 / 190 | 116 / 507 | 0 / 0 | 11 / 109 | 6 / 58 | 0 / 160 |
| Egypt | 24 / 409 | 180 / 401 | 0 / 0 | 20 / 145 | 0 / 1 | 0 / 68 |
| Rome | 28 / 818 | 19 / 52 | 0 / 0 | 6 / 71 | 2 / 17 | 0 / 66 |

Guardrail: **Fail**. Entropy minimum 0.80; at least 20 games per strategy pair. Dominant by observed score: none. Without an observed counter (all pairs covered): none. Sampled but never won: none. Unobserved: Science. Under-sampled pairs: 5.

- insufficient evidence: 5 under-sampled matchups; 1024 winning seats

- winner entropy below 0.80

## Battle and capture telemetry

1024 observed first plays; 0 records without telemetry excluded.

| Event | Total / observed games | Mean per game | Games with event / observed games |
| --- | --- | --- | --- |
| War declarations | 15741/1024 | 15.372 | 1024/1024 (100.0%) |
| Peace offers | 94274/1024 | 92.064 | 1024/1024 (100.0%) |
| Peace agreements | 12927/1024 | 12.624 | 1023/1024 (99.9%) |
| Battles | 1221493/1024 | 1192.864 | 1024/1024 (100.0%) |
| City captures | 22737/1024 | 22.204 | 1024/1024 (100.0%) |
| Settler captures | 2830/1024 | 2.764 | 926/1024 (90.4%) |
| Eliminations | 2490/1024 | 2.432 | 974/1024 (95.1%) |

## Terminal VP sources

1024 finished games. Columns are separate populations; totals across columns must not be added. Every cell is VP / population observations.

| Source | All players | All teams | Winning teams |
| --- | --- | --- | --- |
| Held Great Works | 7112/8192 | 7112/8192 | 3150/1024 |
| Faction goals | 3914/8192 | 3914/8192 | 1468/1024 |
| Era objectives | 15448/8192 | 15448/8192 | 4930/1024 |
| Capitals taken | 2911/8192 | 2911/8192 | 1809/1024 |
| Total VP | 29385/8192 | 29385/8192 | 11357/1024 |

Reconciliation failures: 0 player observations, 0 team observations.

## Turn-50 team leaders

| Cohort | Eventual leader-set wins / eligible games | Draws (included) |
| --- | --- | --- |
| Unique leader | 101/602 (16.8%) | 0 |
| Tied leaders (winner in set) | 350/422 (82.9%) | 0 |
| Any leader set | 451/1024 (44.0%) | 0 |

Excluded: 0 games ended before turn 50, 0 unfinished games, 0 snapshots without teams, 0 records without telemetry.

## Era pacing

Resolved turn each empire entered each era. `Winner vs median` is the winning team's seats' entry turn minus the median entry of the empires that entered it in the same finished game (negative: winners got there first).

| Era | Entered / seats | Fastest | p10 | Median | p90 | Slowest | Winner vs median (mean, n) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Classical | 7928/8192 | 6 | 20 | 28 | 33 | 74 | -2.5 (1024) |
| Medieval | 7674/8192 | 35 | 58 | 63 | 69 | 178 | -2.5 (1024) |
| Renaissance | 7584/8192 | 65 | 84 | 92 | 105 | 184 | -5.2 (1024) |
| Industrial | 7209/8192 | 88 | 110 | 122 | 143 | 256 | -8.4 (1024) |
| Machine Age | 6240/8192 | 115 | 138 | 153 | 175 | 277 | -10.7 (1024) |
| World War | 4819/8192 | 133 | 161 | 179 | 203 | 310 | -11.2 (1011) |
| Modern | 3015/8192 | 151 | 182 | 206 | 235 | 304 | -9.5 (891) |
| Future | 1408/8192 | 177 | 206 | 236 | 257 | 346 | -6.5 (535) |

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
| Classical | Mathematics | 7728/8192 | 44 | 301 BC | 300 BC |
| Classical | Engineering | 7704/8192 | 51 | 50 AD | n/a |
| Classical | Iron Working | 7780/8192 | 38 | 601 BC | 1200 BC |
| Classical | Code of Laws | 7685/8192 | 58 | 400 AD | 1754 BC |
| Classical | Philosophy | 7676/8192 | 63 | 590 AD | 500 BC |
| Medieval | Feudalism | 7647/8192 | 76 | 980 AD | n/a |
| Medieval | Machinery | 7656/8192 | 71 | 830 AD | n/a |
| Medieval | Chivalry | 7631/8192 | 82 | 1160 AD | n/a |
| Medieval | Education | 7601/8192 | 88 | 1340 AD | 1088 AD |
| Medieval | Crop Rotation | 7585/8192 | 91 | 1410 AD | n/a |
| Renaissance | Gunpowder | 7541/8192 | 99 | 1490 AD | n/a |
| Renaissance | Metallurgy | 7434/8192 | 110 | 1600 AD | n/a |
| Renaissance | Military Engineering | 7465/8192 | 106 | 1560 AD | n/a |
| Renaissance | Banking | 7301/8192 | 119 | 1690 AD | 1397 AD |
| Renaissance | Printing Press | 7237/8192 | 120 | 1700 AD | 1440 AD |
| Industrial | Rifling | 6573/8192 | 143 | 1816 AD | n/a |
| Industrial | Military Science | 6853/8192 | 134 | 1783 AD | n/a |
| Industrial | Ballistics | 6673/8192 | 145 | 1824 AD | n/a |
| Industrial | Industrialization | 6994/8192 | 131 | 1772 AD | 1769 AD |
| Industrial | Railways | 6248/8192 | 153 | 1854 AD | 1825 AD |
| Machine Age | Replaceable Parts | 5569/8192 | 167 | 1887 AD | 1800 AD |
| Machine Age | Combustion | 5894/8192 | 161 | 1881 AD | 1876 AD |
| Machine Age | Steel | 5568/8192 | 165 | 1885 AD | 1856 AD |
| Machine Age | Flight | 5107/8192 | 174 | 1895 AD | 1903 AD |
| Machine Age | Electricity | 4871/8192 | 179 | 1901 AD | 1879 AD |
| World War | Mass Mobilisation | 3926/8192 | 193 | 1918 AD | n/a |
| World War | Rocketry | 3621/8192 | 196 | 1922 AD | 1926 AD |
| World War | Armoured Warfare | 3373/8192 | 200 | 1928 AD | 1916 AD |
| World War | Air Power | 3209/8192 | 203 | 1932 AD | n/a |
| World War | Electronics | 4425/8192 | 185 | 1908 AD | n/a |
| Modern | Mechanised Warfare | 2094/8192 | 218 | 1957 AD | n/a |
| Modern | Composite Armour | 1864/8192 | 223 | 1969 AD | n/a |
| Modern | Guided Missiles | 1759/8192 | 228 | 1981 AD | n/a |
| Modern | Jet Engines | 1676/8192 | 230 | 1986 AD | 1939 AD |
| Modern | Green Revolution | 2524/8192 | 213 | 1947 AD | n/a |
| Future | Autonomous Systems | 862/8192 | 251 | 2027 AD | n/a |
| Future | Robotics | 657/8192 | 256 | 2032 AD | n/a |
| Future | Fusion Power | 708/8192 | 256 | 2032 AD | n/a |
| Future | Stealth Technology | 1056/8192 | 244 | 2016 AD | n/a |
| Future | Recursive AI | 627/8192 | 256 | 2032 AD | n/a |

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
| 0 | 0.00 | 87.46 |
| 1 | 74.57 | 16.97 |
| 2 | 21.79 | 5.46 |
| 3 | 5.20 | 1.50 |
| 4 | 0.97 | 0.30 |
| 5 | 0.20 | 0.08 |

## Per-game events

Each row is one observed first play; unavailable telemetry is shown as n/a. Full per-player/team score sources and turn-50 standings are in JSON.

| Game | Seed | Resolved turns | Battles | City captures | Settler captures | Eliminations |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 9311003 | 204 | 1021 | 22 | 3 | 2 |
| 1 | 9311004 | 309 | 1830 | 32 | 3 | 2 |
| 2 | 9311005 | 228 | 1057 | 18 | 4 | 1 |
| 3 | 9311006 | 321 | 1933 | 36 | 3 | 3 |
| 4 | 9311007 | 203 | 744 | 16 | 0 | 1 |
| 5 | 9311008 | 168 | 702 | 14 | 2 | 3 |
| 6 | 9311009 | 261 | 1183 | 25 | 6 | 4 |
| 7 | 9311010 | 265 | 1916 | 35 | 0 | 1 |
| 8 | 9311011 | 177 | 788 | 14 | 3 | 1 |
| 9 | 9311012 | 180 | 869 | 24 | 1 | 3 |
| 10 | 9311013 | 293 | 1982 | 29 | 2 | 2 |
| 11 | 9311014 | 302 | 2069 | 47 | 3 | 5 |
| 12 | 9311015 | 207 | 1014 | 28 | 1 | 4 |
| 13 | 9311016 | 184 | 932 | 18 | 3 | 3 |
| 14 | 9311017 | 168 | 652 | 14 | 2 | 2 |
| 15 | 9311018 | 224 | 1108 | 14 | 2 | 6 |
| 16 | 9311019 | 355 | 2039 | 33 | 10 | 2 |
| 17 | 9311020 | 241 | 1366 | 24 | 3 | 3 |
| 18 | 9311021 | 173 | 818 | 20 | 6 | 2 |
| 19 | 9311022 | 172 | 642 | 15 | 0 | 2 |
| 20 | 9311023 | 184 | 784 | 21 | 1 | 2 |
| 21 | 9311024 | 192 | 674 | 12 | 2 | 2 |
| 22 | 9311025 | 238 | 660 | 8 | 1 | 0 |
| 23 | 9311026 | 261 | 1384 | 32 | 0 | 2 |
| 24 | 9311027 | 165 | 895 | 14 | 3 | 1 |
| 25 | 9311028 | 145 | 591 | 10 | 2 | 2 |
| 26 | 9311029 | 239 | 1174 | 23 | 2 | 3 |
| 27 | 9311030 | 223 | 1039 | 31 | 3 | 4 |
| 28 | 9311031 | 219 | 1032 | 25 | 1 | 4 |
| 29 | 9311032 | 220 | 974 | 22 | 9 | 3 |
| 30 | 9311033 | 286 | 1146 | 30 | 2 | 4 |
| 31 | 9311034 | 189 | 1272 | 27 | 3 | 2 |
| 32 | 9311035 | 207 | 1192 | 33 | 1 | 4 |
| 33 | 9311036 | 231 | 1300 | 25 | 6 | 3 |
| 34 | 9311037 | 237 | 1198 | 21 | 6 | 5 |
| 35 | 9311038 | 264 | 1568 | 31 | 5 | 4 |
| 36 | 9311039 | 284 | 1683 | 35 | 3 | 1 |
| 37 | 9311040 | 224 | 1051 | 16 | 2 | 1 |
| 38 | 9311041 | 207 | 943 | 22 | 4 | 1 |
| 39 | 9311042 | 218 | 1167 | 12 | 3 | 3 |
| 40 | 9311043 | 235 | 1465 | 27 | 1 | 2 |
| 41 | 9311044 | 226 | 1252 | 22 | 2 | 1 |
| 42 | 9311045 | 218 | 953 | 27 | 3 | 1 |
| 43 | 9311046 | 247 | 1020 | 29 | 2 | 4 |
| 44 | 9311047 | 220 | 918 | 16 | 4 | 3 |
| 45 | 9311048 | 209 | 885 | 22 | 1 | 1 |
| 46 | 9311049 | 219 | 979 | 15 | 3 | 2 |
| 47 | 9311050 | 242 | 1418 | 21 | 2 | 3 |
| 48 | 9311051 | 270 | 1412 | 36 | 3 | 5 |
| 49 | 9311052 | 266 | 1909 | 33 | 1 | 2 |
| 50 | 9311053 | 221 | 1071 | 22 | 1 | 1 |
| 51 | 9311054 | 192 | 801 | 25 | 7 | 3 |
| 52 | 9311055 | 261 | 1393 | 20 | 9 | 3 |
| 53 | 9311056 | 250 | 1432 | 14 | 5 | 3 |
| 54 | 9311057 | 184 | 895 | 19 | 3 | 2 |
| 55 | 9311058 | 239 | 1561 | 28 | 3 | 4 |
| 56 | 9311059 | 250 | 1217 | 29 | 5 | 3 |
| 57 | 9311060 | 270 | 1673 | 32 | 2 | 6 |
| 58 | 9311061 | 191 | 1095 | 19 | 2 | 1 |
| 59 | 9311062 | 326 | 1680 | 25 | 0 | 2 |
| 60 | 9311063 | 201 | 666 | 16 | 2 | 1 |
| 61 | 9311064 | 238 | 1236 | 16 | 1 | 3 |
| 62 | 9311065 | 218 | 1185 | 16 | 3 | 3 |
| 63 | 9311066 | 216 | 1065 | 28 | 0 | 3 |
| 64 | 9311067 | 177 | 876 | 18 | 0 | 1 |
| 65 | 9311068 | 193 | 1085 | 16 | 4 | 1 |
| 66 | 9311069 | 261 | 1172 | 19 | 5 | 2 |
| 67 | 9311070 | 198 | 1034 | 32 | 2 | 3 |
| 68 | 9311071 | 252 | 1708 | 19 | 2 | 4 |
| 69 | 9311072 | 204 | 1311 | 21 | 0 | 4 |
| 70 | 9311073 | 264 | 1177 | 30 | 1 | 4 |
| 71 | 9311074 | 204 | 1052 | 19 | 3 | 1 |
| 72 | 9311075 | 224 | 1066 | 23 | 3 | 3 |
| 73 | 9311076 | 358 | 2503 | 39 | 0 | 2 |
| 74 | 9311077 | 259 | 1741 | 38 | 6 | 2 |
| 75 | 9311078 | 223 | 1043 | 25 | 1 | 2 |
| 76 | 9311079 | 199 | 1047 | 16 | 0 | 3 |
| 77 | 9311080 | 228 | 1400 | 26 | 3 | 3 |
| 78 | 9311081 | 232 | 1495 | 22 | 0 | 1 |
| 79 | 9311082 | 212 | 1023 | 21 | 4 | 4 |
| 80 | 9311083 | 254 | 1451 | 34 | 1 | 2 |
| 81 | 9311084 | 242 | 1322 | 17 | 1 | 2 |
| 82 | 9311085 | 212 | 1026 | 19 | 4 | 1 |
| 83 | 9311086 | 226 | 1189 | 15 | 5 | 3 |
| 84 | 9311087 | 168 | 803 | 18 | 2 | 1 |
| 85 | 9311088 | 204 | 1081 | 21 | 0 | 3 |
| 86 | 9311089 | 258 | 1739 | 32 | 3 | 1 |
| 87 | 9311090 | 191 | 1010 | 18 | 1 | 2 |
| 88 | 9311091 | 257 | 1394 | 32 | 1 | 2 |
| 89 | 9311092 | 168 | 604 | 3 | 3 | 3 |
| 90 | 9311093 | 170 | 701 | 17 | 6 | 3 |
| 91 | 9311094 | 218 | 1229 | 24 | 3 | 1 |
| 92 | 9311095 | 226 | 1106 | 13 | 3 | 2 |
| 93 | 9311096 | 215 | 846 | 18 | 3 | 4 |
| 94 | 9311097 | 221 | 1246 | 29 | 4 | 4 |
| 95 | 9311098 | 199 | 874 | 20 | 2 | 3 |
| 96 | 9311099 | 252 | 1103 | 25 | 9 | 4 |
| 97 | 9311100 | 236 | 1211 | 26 | 5 | 2 |
| 98 | 9311101 | 235 | 1256 | 19 | 6 | 1 |
| 99 | 9311102 | 201 | 1072 | 25 | 4 | 2 |
| 100 | 9311103 | 222 | 1137 | 20 | 3 | 4 |
| 101 | 9311104 | 179 | 888 | 22 | 6 | 0 |
| 102 | 9311105 | 219 | 981 | 22 | 4 | 3 |
| 103 | 9311106 | 230 | 1414 | 28 | 2 | 2 |
| 104 | 9311107 | 206 | 1208 | 22 | 2 | 3 |
| 105 | 9311108 | 157 | 597 | 13 | 2 | 0 |
| 106 | 9311109 | 226 | 1160 | 22 | 2 | 3 |
| 107 | 9311110 | 223 | 1036 | 24 | 8 | 2 |
| 108 | 9311111 | 185 | 1037 | 19 | 5 | 3 |
| 109 | 9311112 | 290 | 1717 | 23 | 4 | 3 |
| 110 | 9311113 | 193 | 776 | 7 | 1 | 1 |
| 111 | 9311114 | 261 | 1180 | 13 | 5 | 3 |
| 112 | 9311115 | 224 | 1544 | 28 | 0 | 3 |
| 113 | 9311116 | 213 | 1454 | 19 | 3 | 1 |
| 114 | 9311117 | 198 | 922 | 22 | 2 | 2 |
| 115 | 9311118 | 242 | 1462 | 27 | 3 | 3 |
| 116 | 9311119 | 344 | 2174 | 38 | 7 | 6 |
| 117 | 9311120 | 258 | 1350 | 19 | 7 | 2 |
| 118 | 9311121 | 172 | 757 | 11 | 5 | 2 |
| 119 | 9311122 | 244 | 1449 | 29 | 6 | 3 |
| 120 | 9311123 | 220 | 1369 | 17 | 0 | 1 |
| 121 | 9311124 | 242 | 1057 | 21 | 5 | 2 |
| 122 | 9311125 | 177 | 717 | 8 | 2 | 1 |
| 123 | 9311126 | 215 | 1084 | 20 | 3 | 1 |
| 124 | 9311127 | 190 | 908 | 18 | 1 | 2 |
| 125 | 9311128 | 237 | 939 | 21 | 0 | 0 |
| 126 | 9311129 | 183 | 764 | 11 | 2 | 2 |
| 127 | 9311130 | 261 | 1810 | 31 | 3 | 3 |
| 128 | 9311131 | 210 | 1136 | 18 | 0 | 2 |
| 129 | 9311132 | 179 | 1036 | 28 | 3 | 2 |
| 130 | 9311133 | 199 | 993 | 25 | 3 | 1 |
| 131 | 9311134 | 340 | 2382 | 32 | 4 | 2 |
| 132 | 9311135 | 247 | 952 | 20 | 5 | 3 |
| 133 | 9311136 | 223 | 1116 | 17 | 2 | 3 |
| 134 | 9311137 | 182 | 989 | 28 | 4 | 3 |
| 135 | 9311138 | 267 | 1399 | 24 | 7 | 5 |
| 136 | 9311139 | 173 | 761 | 13 | 0 | 2 |
| 137 | 9311140 | 200 | 1000 | 18 | 0 | 2 |
| 138 | 9311141 | 211 | 1017 | 13 | 2 | 1 |
| 139 | 9311142 | 221 | 1214 | 24 | 3 | 1 |
| 140 | 9311143 | 268 | 1359 | 23 | 2 | 4 |
| 141 | 9311144 | 187 | 955 | 21 | 2 | 3 |
| 142 | 9311145 | 212 | 899 | 12 | 5 | 2 |
| 143 | 9311146 | 219 | 1082 | 20 | 5 | 5 |
| 144 | 9311147 | 216 | 1220 | 17 | 3 | 1 |
| 145 | 9311148 | 223 | 1200 | 14 | 4 | 2 |
| 146 | 9311149 | 274 | 1289 | 25 | 0 | 2 |
| 147 | 9311150 | 257 | 1390 | 19 | 2 | 1 |
| 148 | 9311151 | 217 | 1379 | 23 | 5 | 4 |
| 149 | 9311152 | 214 | 955 | 30 | 3 | 5 |
| 150 | 9311153 | 288 | 1759 | 40 | 1 | 5 |
| 151 | 9311154 | 257 | 1518 | 46 | 2 | 3 |
| 152 | 9311155 | 177 | 783 | 23 | 2 | 0 |
| 153 | 9311156 | 169 | 797 | 17 | 4 | 2 |
| 154 | 9311157 | 241 | 1394 | 23 | 2 | 4 |
| 155 | 9311158 | 193 | 903 | 14 | 3 | 2 |
| 156 | 9311159 | 244 | 1656 | 26 | 4 | 3 |
| 157 | 9311160 | 206 | 870 | 15 | 4 | 3 |
| 158 | 9311161 | 252 | 1873 | 38 | 3 | 3 |
| 159 | 9311162 | 211 | 904 | 13 | 1 | 1 |
| 160 | 9311163 | 223 | 1390 | 25 | 3 | 3 |
| 161 | 9311164 | 180 | 869 | 20 | 2 | 2 |
| 162 | 9311165 | 211 | 1039 | 28 | 2 | 5 |
| 163 | 9311166 | 276 | 1492 | 25 | 3 | 3 |
| 164 | 9311167 | 204 | 893 | 21 | 2 | 3 |
| 165 | 9311168 | 215 | 1088 | 19 | 2 | 2 |
| 166 | 9311169 | 220 | 1201 | 23 | 2 | 2 |
| 167 | 9311170 | 214 | 1116 | 23 | 4 | 1 |
| 168 | 9311171 | 177 | 806 | 21 | 1 | 1 |
| 169 | 9311172 | 192 | 725 | 20 | 3 | 4 |
| 170 | 9311173 | 233 | 1417 | 36 | 4 | 3 |
| 171 | 9311174 | 279 | 1705 | 18 | 0 | 2 |
| 172 | 9311175 | 187 | 912 | 19 | 2 | 2 |
| 173 | 9311176 | 209 | 1149 | 18 | 1 | 3 |
| 174 | 9311177 | 224 | 986 | 24 | 0 | 2 |
| 175 | 9311178 | 238 | 1302 | 28 | 5 | 2 |
| 176 | 9311179 | 218 | 945 | 19 | 4 | 1 |
| 177 | 9311180 | 219 | 1183 | 29 | 4 | 4 |
| 178 | 9311181 | 183 | 671 | 18 | 3 | 2 |
| 179 | 9311182 | 268 | 1657 | 34 | 4 | 4 |
| 180 | 9311183 | 165 | 776 | 10 | 3 | 0 |
| 181 | 9311184 | 163 | 648 | 21 | 2 | 1 |
| 182 | 9311185 | 190 | 704 | 15 | 3 | 1 |
| 183 | 9311186 | 197 | 905 | 19 | 1 | 1 |
| 184 | 9311187 | 202 | 1010 | 30 | 5 | 3 |
| 185 | 9311188 | 222 | 1302 | 17 | 3 | 1 |
| 186 | 9311189 | 163 | 672 | 8 | 2 | 2 |
| 187 | 9311190 | 212 | 1158 | 39 | 1 | 3 |
| 188 | 9311191 | 258 | 1413 | 28 | 1 | 2 |
| 189 | 9311192 | 194 | 1027 | 19 | 3 | 1 |
| 190 | 9311193 | 222 | 1266 | 23 | 1 | 3 |
| 191 | 9311194 | 218 | 1059 | 25 | 1 | 3 |
| 192 | 9311195 | 221 | 1304 | 27 | 1 | 3 |
| 193 | 9311196 | 199 | 990 | 24 | 3 | 4 |
| 194 | 9311197 | 244 | 1396 | 35 | 10 | 2 |
| 195 | 9311198 | 221 | 1192 | 25 | 0 | 4 |
| 196 | 9311199 | 195 | 1007 | 17 | 0 | 2 |
| 197 | 9311200 | 229 | 1394 | 22 | 2 | 2 |
| 198 | 9311201 | 229 | 1256 | 32 | 8 | 5 |
| 199 | 9311202 | 185 | 791 | 12 | 2 | 4 |
| 200 | 9311203 | 199 | 1165 | 33 | 4 | 3 |
| 201 | 9311204 | 217 | 1001 | 18 | 4 | 2 |
| 202 | 9311205 | 227 | 1305 | 31 | 4 | 2 |
| 203 | 9311206 | 255 | 1415 | 24 | 0 | 3 |
| 204 | 9311207 | 269 | 1221 | 14 | 1 | 1 |
| 205 | 9311208 | 227 | 1644 | 29 | 1 | 2 |
| 206 | 9311209 | 214 | 928 | 17 | 1 | 3 |
| 207 | 9311210 | 201 | 1061 | 20 | 2 | 3 |
| 208 | 9311211 | 204 | 1170 | 18 | 1 | 3 |
| 209 | 9311212 | 268 | 1172 | 26 | 3 | 4 |
| 210 | 9311213 | 181 | 912 | 30 | 1 | 4 |
| 211 | 9311214 | 250 | 1671 | 30 | 2 | 2 |
| 212 | 9311215 | 239 | 1383 | 32 | 5 | 1 |
| 213 | 9311216 | 216 | 980 | 24 | 7 | 1 |
| 214 | 9311217 | 247 | 1010 | 23 | 3 | 3 |
| 215 | 9311218 | 332 | 1606 | 25 | 5 | 4 |
| 216 | 9311219 | 158 | 590 | 15 | 1 | 1 |
| 217 | 9311220 | 237 | 1187 | 27 | 3 | 0 |
| 218 | 9311221 | 187 | 859 | 17 | 3 | 2 |
| 219 | 9311222 | 188 | 986 | 15 | 3 | 1 |
| 220 | 9311223 | 190 | 1002 | 25 | 4 | 2 |
| 221 | 9311224 | 260 | 1468 | 37 | 0 | 2 |
| 222 | 9311225 | 256 | 1247 | 21 | 3 | 1 |
| 223 | 9311226 | 232 | 1154 | 22 | 2 | 1 |
| 224 | 9311227 | 260 | 1414 | 23 | 4 | 5 |
| 225 | 9311228 | 275 | 1379 | 31 | 0 | 4 |
| 226 | 9311229 | 233 | 937 | 31 | 4 | 5 |
| 227 | 9311230 | 162 | 632 | 15 | 3 | 1 |
| 228 | 9311231 | 260 | 1422 | 29 | 3 | 3 |
| 229 | 9311232 | 227 | 1337 | 24 | 3 | 3 |
| 230 | 9311233 | 275 | 1778 | 38 | 2 | 4 |
| 231 | 9311234 | 288 | 1270 | 23 | 1 | 4 |
| 232 | 9311235 | 224 | 1463 | 24 | 3 | 3 |
| 233 | 9311236 | 358 | 1862 | 36 | 2 | 4 |
| 234 | 9311237 | 261 | 1378 | 13 | 5 | 2 |
| 235 | 9311238 | 214 | 930 | 26 | 5 | 4 |
| 236 | 9311239 | 217 | 967 | 14 | 2 | 1 |
| 237 | 9311240 | 217 | 1027 | 24 | 5 | 3 |
| 238 | 9311241 | 236 | 1264 | 20 | 8 | 2 |
| 239 | 9311242 | 333 | 1784 | 31 | 4 | 1 |
| 240 | 9311243 | 340 | 1990 | 25 | 2 | 3 |
| 241 | 9311244 | 190 | 996 | 22 | 1 | 3 |
| 242 | 9311245 | 193 | 890 | 10 | 2 | 0 |
| 243 | 9311246 | 258 | 1821 | 26 | 5 | 3 |
| 244 | 9311247 | 228 | 1184 | 25 | 4 | 3 |
| 245 | 9311248 | 205 | 782 | 7 | 4 | 2 |
| 246 | 9311249 | 168 | 657 | 15 | 3 | 2 |
| 247 | 9311250 | 188 | 914 | 22 | 5 | 2 |
| 248 | 9311251 | 200 | 652 | 14 | 3 | 2 |
| 249 | 9311252 | 207 | 1042 | 24 | 2 | 2 |
| 250 | 9311253 | 219 | 1229 | 20 | 0 | 2 |
| 251 | 9311254 | 266 | 1412 | 41 | 2 | 3 |
| 252 | 9311255 | 204 | 1059 | 24 | 2 | 2 |
| 253 | 9311256 | 211 | 1059 | 30 | 5 | 1 |
| 254 | 9311257 | 161 | 586 | 18 | 2 | 4 |
| 255 | 9311258 | 229 | 1280 | 14 | 6 | 3 |
| 256 | 9311259 | 370 | 2356 | 27 | 3 | 3 |
| 257 | 9311260 | 249 | 1493 | 16 | 1 | 2 |
| 258 | 9311261 | 250 | 1511 | 25 | 3 | 4 |
| 259 | 9311262 | 203 | 1140 | 23 | 3 | 2 |
| 260 | 9311263 | 225 | 1144 | 26 | 3 | 4 |
| 261 | 9311264 | 197 | 1044 | 20 | 0 | 2 |
| 262 | 9311265 | 210 | 995 | 22 | 2 | 1 |
| 263 | 9311266 | 273 | 1032 | 23 | 1 | 2 |
| 264 | 9311267 | 253 | 1327 | 21 | 2 | 1 |
| 265 | 9311268 | 215 | 733 | 9 | 1 | 3 |
| 266 | 9311269 | 185 | 982 | 8 | 1 | 2 |
| 267 | 9311270 | 221 | 1089 | 24 | 1 | 2 |
| 268 | 9311271 | 254 | 1444 | 20 | 4 | 3 |
| 269 | 9311272 | 211 | 974 | 20 | 0 | 2 |
| 270 | 9311273 | 228 | 1202 | 21 | 3 | 3 |
| 271 | 9311274 | 176 | 894 | 14 | 5 | 2 |
| 272 | 9311275 | 165 | 721 | 12 | 5 | 2 |
| 273 | 9311276 | 221 | 1091 | 23 | 4 | 1 |
| 274 | 9311277 | 191 | 977 | 20 | 4 | 3 |
| 275 | 9311278 | 179 | 744 | 14 | 4 | 2 |
| 276 | 9311279 | 191 | 931 | 20 | 2 | 2 |
| 277 | 9311280 | 219 | 1016 | 15 | 2 | 2 |
| 278 | 9311281 | 229 | 1238 | 27 | 1 | 2 |
| 279 | 9311282 | 317 | 2170 | 37 | 0 | 2 |
| 280 | 9311283 | 219 | 1382 | 34 | 4 | 2 |
| 281 | 9311284 | 204 | 937 | 12 | 4 | 1 |
| 282 | 9311285 | 195 | 982 | 16 | 0 | 1 |
| 283 | 9311286 | 167 | 605 | 11 | 3 | 2 |
| 284 | 9311287 | 194 | 1064 | 26 | 7 | 0 |
| 285 | 9311288 | 297 | 1472 | 28 | 6 | 1 |
| 286 | 9311289 | 216 | 1230 | 15 | 5 | 3 |
| 287 | 9311290 | 154 | 670 | 11 | 4 | 0 |
| 288 | 9311291 | 175 | 717 | 21 | 0 | 2 |
| 289 | 9311292 | 260 | 1463 | 27 | 3 | 2 |
| 290 | 9311293 | 231 | 1285 | 13 | 8 | 4 |
| 291 | 9311294 | 248 | 1242 | 23 | 2 | 2 |
| 292 | 9311295 | 231 | 1422 | 17 | 1 | 1 |
| 293 | 9311296 | 174 | 779 | 18 | 2 | 2 |
| 294 | 9311297 | 183 | 1072 | 29 | 4 | 2 |
| 295 | 9311298 | 181 | 755 | 21 | 3 | 1 |
| 296 | 9311299 | 200 | 1063 | 16 | 4 | 2 |
| 297 | 9311300 | 245 | 1294 | 17 | 3 | 1 |
| 298 | 9311301 | 209 | 933 | 15 | 3 | 1 |
| 299 | 9311302 | 198 | 972 | 24 | 2 | 2 |
| 300 | 9311303 | 230 | 960 | 12 | 5 | 3 |
| 301 | 9311304 | 470 | 3033 | 44 | 5 | 3 |
| 302 | 9311305 | 254 | 1389 | 27 | 4 | 3 |
| 303 | 9311306 | 252 | 1268 | 25 | 2 | 4 |
| 304 | 9311307 | 185 | 784 | 19 | 1 | 3 |
| 305 | 9311308 | 176 | 980 | 16 | 3 | 2 |
| 306 | 9311309 | 250 | 1308 | 29 | 5 | 2 |
| 307 | 9311310 | 236 | 1295 | 23 | 1 | 2 |
| 308 | 9311311 | 213 | 1012 | 14 | 2 | 2 |
| 309 | 9311312 | 192 | 1031 | 23 | 2 | 3 |
| 310 | 9311313 | 204 | 994 | 22 | 0 | 3 |
| 311 | 9311314 | 317 | 1805 | 27 | 7 | 2 |
| 312 | 9311315 | 227 | 1136 | 14 | 3 | 2 |
| 313 | 9311316 | 335 | 1865 | 37 | 6 | 3 |
| 314 | 9311317 | 210 | 1075 | 25 | 4 | 2 |
| 315 | 9311318 | 226 | 1623 | 24 | 1 | 0 |
| 316 | 9311319 | 279 | 1549 | 18 | 3 | 3 |
| 317 | 9311320 | 218 | 1072 | 28 | 4 | 2 |
| 318 | 9311321 | 202 | 1058 | 21 | 2 | 2 |
| 319 | 9311322 | 199 | 915 | 23 | 3 | 5 |
| 320 | 9311323 | 178 | 961 | 24 | 5 | 2 |
| 321 | 9311324 | 200 | 934 | 11 | 3 | 3 |
| 322 | 9311325 | 205 | 977 | 12 | 4 | 0 |
| 323 | 9311326 | 196 | 977 | 20 | 0 | 3 |
| 324 | 9311327 | 282 | 1678 | 40 | 2 | 5 |
| 325 | 9311328 | 220 | 926 | 17 | 3 | 3 |
| 326 | 9311329 | 261 | 1354 | 26 | 5 | 3 |
| 327 | 9311330 | 320 | 2061 | 34 | 2 | 3 |
| 328 | 9311331 | 230 | 1303 | 22 | 1 | 3 |
| 329 | 9311332 | 295 | 1473 | 19 | 0 | 4 |
| 330 | 9311333 | 156 | 767 | 20 | 3 | 2 |
| 331 | 9311334 | 203 | 914 | 14 | 4 | 3 |
| 332 | 9311335 | 257 | 1499 | 19 | 4 | 3 |
| 333 | 9311336 | 196 | 1084 | 19 | 7 | 4 |
| 334 | 9311337 | 215 | 1196 | 26 | 4 | 3 |
| 335 | 9311338 | 208 | 1020 | 15 | 3 | 0 |
| 336 | 9311339 | 261 | 1957 | 21 | 4 | 2 |
| 337 | 9311340 | 218 | 1380 | 21 | 1 | 3 |
| 338 | 9311341 | 211 | 1210 | 13 | 3 | 2 |
| 339 | 9311342 | 243 | 1193 | 18 | 3 | 2 |
| 340 | 9311343 | 232 | 1007 | 23 | 2 | 3 |
| 341 | 9311344 | 232 | 1199 | 21 | 2 | 3 |
| 342 | 9311345 | 201 | 1076 | 20 | 1 | 4 |
| 343 | 9311346 | 206 | 971 | 17 | 2 | 3 |
| 344 | 9311347 | 322 | 2189 | 47 | 3 | 2 |
| 345 | 9311348 | 191 | 944 | 14 | 0 | 2 |
| 346 | 9311349 | 218 | 1167 | 18 | 5 | 2 |
| 347 | 9311350 | 218 | 873 | 16 | 1 | 2 |
| 348 | 9311351 | 190 | 844 | 26 | 3 | 3 |
| 349 | 9311352 | 162 | 668 | 12 | 2 | 2 |
| 350 | 9311353 | 223 | 1006 | 14 | 4 | 3 |
| 351 | 9311354 | 163 | 570 | 17 | 2 | 2 |
| 352 | 9311355 | 206 | 1001 | 25 | 0 | 4 |
| 353 | 9311356 | 251 | 1742 | 34 | 2 | 1 |
| 354 | 9311357 | 207 | 1022 | 20 | 2 | 2 |
| 355 | 9311358 | 280 | 1515 | 14 | 6 | 4 |
| 356 | 9311359 | 297 | 1574 | 51 | 8 | 4 |
| 357 | 9311360 | 243 | 1165 | 31 | 3 | 5 |
| 358 | 9311361 | 210 | 925 | 25 | 6 | 4 |
| 359 | 9311362 | 197 | 1114 | 23 | 3 | 2 |
| 360 | 9311363 | 228 | 898 | 19 | 6 | 3 |
| 361 | 9311364 | 271 | 1734 | 16 | 1 | 1 |
| 362 | 9311365 | 200 | 904 | 23 | 0 | 3 |
| 363 | 9311366 | 218 | 1223 | 18 | 3 | 4 |
| 364 | 9311367 | 242 | 1451 | 24 | 4 | 2 |
| 365 | 9311368 | 253 | 1455 | 33 | 4 | 3 |
| 366 | 9311369 | 218 | 1273 | 25 | 0 | 2 |
| 367 | 9311370 | 274 | 1397 | 37 | 6 | 4 |
| 368 | 9311371 | 214 | 1011 | 13 | 5 | 3 |
| 369 | 9311372 | 189 | 854 | 19 | 1 | 3 |
| 370 | 9311373 | 296 | 1852 | 36 | 2 | 1 |
| 371 | 9311374 | 210 | 793 | 16 | 1 | 3 |
| 372 | 9311375 | 237 | 1145 | 10 | 3 | 2 |
| 373 | 9311376 | 280 | 1606 | 43 | 7 | 4 |
| 374 | 9311377 | 274 | 2184 | 34 | 2 | 2 |
| 375 | 9311378 | 220 | 1018 | 28 | 2 | 3 |
| 376 | 9311379 | 294 | 1880 | 29 | 7 | 3 |
| 377 | 9311380 | 182 | 990 | 21 | 2 | 0 |
| 378 | 9311381 | 264 | 1498 | 38 | 5 | 3 |
| 379 | 9311382 | 232 | 1181 | 28 | 3 | 5 |
| 380 | 9311383 | 272 | 1460 | 17 | 3 | 2 |
| 381 | 9311384 | 236 | 1758 | 24 | 2 | 2 |
| 382 | 9311385 | 191 | 628 | 7 | 1 | 1 |
| 383 | 9311386 | 266 | 1343 | 24 | 4 | 4 |
| 384 | 9311387 | 254 | 1246 | 33 | 4 | 2 |
| 385 | 9311388 | 221 | 1283 | 18 | 2 | 2 |
| 386 | 9311389 | 215 | 1179 | 14 | 0 | 1 |
| 387 | 9311390 | 279 | 1417 | 22 | 3 | 2 |
| 388 | 9311391 | 229 | 1291 | 20 | 2 | 3 |
| 389 | 9311392 | 223 | 978 | 21 | 3 | 3 |
| 390 | 9311393 | 224 | 1161 | 22 | 3 | 4 |
| 391 | 9311394 | 250 | 1393 | 33 | 2 | 5 |
| 392 | 9311395 | 222 | 923 | 26 | 1 | 2 |
| 393 | 9311396 | 234 | 1214 | 14 | 2 | 0 |
| 394 | 9311397 | 272 | 1571 | 18 | 3 | 1 |
| 395 | 9311398 | 171 | 879 | 17 | 1 | 1 |
| 396 | 9311399 | 332 | 2105 | 46 | 3 | 3 |
| 397 | 9311400 | 249 | 1360 | 19 | 5 | 3 |
| 398 | 9311401 | 208 | 959 | 10 | 5 | 4 |
| 399 | 9311402 | 240 | 1266 | 20 | 2 | 2 |
| 400 | 9311403 | 277 | 1559 | 19 | 1 | 0 |
| 401 | 9311404 | 235 | 1415 | 20 | 1 | 4 |
| 402 | 9311405 | 237 | 1122 | 30 | 0 | 2 |
| 403 | 9311406 | 213 | 1254 | 27 | 5 | 3 |
| 404 | 9311407 | 212 | 985 | 19 | 1 | 1 |
| 405 | 9311408 | 243 | 1622 | 19 | 2 | 2 |
| 406 | 9311409 | 298 | 1755 | 27 | 6 | 2 |
| 407 | 9311410 | 248 | 1480 | 14 | 1 | 3 |
| 408 | 9311411 | 210 | 1165 | 21 | 0 | 4 |
| 409 | 9311412 | 186 | 940 | 15 | 2 | 1 |
| 410 | 9311413 | 216 | 1145 | 24 | 5 | 1 |
| 411 | 9311414 | 213 | 1138 | 16 | 3 | 2 |
| 412 | 9311415 | 277 | 1435 | 24 | 0 | 5 |
| 413 | 9311416 | 209 | 1214 | 24 | 2 | 5 |
| 414 | 9311417 | 294 | 1636 | 34 | 4 | 3 |
| 415 | 9311418 | 185 | 919 | 17 | 3 | 3 |
| 416 | 9311419 | 158 | 567 | 9 | 3 | 2 |
| 417 | 9311420 | 196 | 923 | 12 | 3 | 2 |
| 418 | 9311421 | 221 | 1107 | 23 | 2 | 3 |
| 419 | 9311422 | 181 | 1071 | 18 | 2 | 2 |
| 420 | 9311423 | 174 | 785 | 23 | 1 | 3 |
| 421 | 9311424 | 261 | 1394 | 24 | 2 | 1 |
| 422 | 9311425 | 246 | 1352 | 18 | 5 | 1 |
| 423 | 9311426 | 197 | 1028 | 25 | 2 | 3 |
| 424 | 9311427 | 224 | 1151 | 15 | 0 | 1 |
| 425 | 9311428 | 280 | 1766 | 28 | 0 | 3 |
| 426 | 9311429 | 209 | 1199 | 12 | 0 | 2 |
| 427 | 9311430 | 256 | 1190 | 18 | 3 | 3 |
| 428 | 9311431 | 183 | 790 | 14 | 0 | 2 |
| 429 | 9311432 | 258 | 1698 | 20 | 2 | 1 |
| 430 | 9311433 | 236 | 1539 | 33 | 3 | 2 |
| 431 | 9311434 | 239 | 1416 | 33 | 4 | 1 |
| 432 | 9311435 | 198 | 952 | 22 | 4 | 2 |
| 433 | 9311436 | 268 | 1717 | 15 | 8 | 3 |
| 434 | 9311437 | 174 | 738 | 23 | 1 | 2 |
| 435 | 9311438 | 263 | 1463 | 36 | 7 | 2 |
| 436 | 9311439 | 256 | 2079 | 40 | 2 | 1 |
| 437 | 9311440 | 214 | 1035 | 15 | 4 | 4 |
| 438 | 9311441 | 189 | 802 | 15 | 2 | 2 |
| 439 | 9311442 | 274 | 1543 | 36 | 2 | 2 |
| 440 | 9311443 | 229 | 1332 | 23 | 3 | 1 |
| 441 | 9311444 | 204 | 1060 | 24 | 2 | 4 |
| 442 | 9311445 | 190 | 979 | 21 | 1 | 1 |
| 443 | 9311446 | 191 | 897 | 18 | 1 | 2 |
| 444 | 9311447 | 198 | 1045 | 26 | 1 | 2 |
| 445 | 9311448 | 276 | 1510 | 15 | 2 | 2 |
| 446 | 9311449 | 202 | 990 | 11 | 3 | 0 |
| 447 | 9311450 | 195 | 907 | 25 | 1 | 2 |
| 448 | 9311451 | 281 | 1040 | 15 | 3 | 0 |
| 449 | 9311452 | 176 | 1215 | 16 | 3 | 3 |
| 450 | 9311453 | 242 | 1487 | 17 | 3 | 2 |
| 451 | 9311454 | 201 | 1215 | 25 | 3 | 3 |
| 452 | 9311455 | 235 | 1105 | 20 | 0 | 1 |
| 453 | 9311456 | 233 | 1539 | 22 | 0 | 3 |
| 454 | 9311457 | 199 | 1068 | 16 | 1 | 2 |
| 455 | 9311458 | 241 | 1336 | 26 | 4 | 3 |
| 456 | 9311459 | 204 | 965 | 26 | 3 | 2 |
| 457 | 9311460 | 280 | 1356 | 29 | 3 | 3 |
| 458 | 9311461 | 182 | 883 | 12 | 3 | 0 |
| 459 | 9311462 | 231 | 978 | 20 | 2 | 2 |
| 460 | 9311463 | 207 | 1163 | 21 | 4 | 1 |
| 461 | 9311464 | 279 | 1452 | 42 | 5 | 3 |
| 462 | 9311465 | 198 | 1061 | 15 | 2 | 2 |
| 463 | 9311466 | 226 | 1231 | 26 | 2 | 0 |
| 464 | 9311467 | 163 | 775 | 20 | 2 | 3 |
| 465 | 9311468 | 291 | 1690 | 40 | 6 | 3 |
| 466 | 9311469 | 240 | 1255 | 25 | 3 | 2 |
| 467 | 9311470 | 207 | 957 | 26 | 5 | 4 |
| 468 | 9311471 | 180 | 858 | 19 | 1 | 3 |
| 469 | 9311472 | 395 | 2558 | 31 | 1 | 4 |
| 470 | 9311473 | 224 | 1228 | 30 | 2 | 4 |
| 471 | 9311474 | 320 | 2132 | 39 | 1 | 2 |
| 472 | 9311475 | 213 | 1508 | 24 | 3 | 1 |
| 473 | 9311476 | 322 | 1771 | 36 | 1 | 4 |
| 474 | 9311477 | 209 | 1118 | 20 | 4 | 0 |
| 475 | 9311478 | 228 | 1136 | 25 | 3 | 3 |
| 476 | 9311479 | 254 | 1166 | 26 | 3 | 2 |
| 477 | 9311480 | 214 | 1042 | 25 | 4 | 3 |
| 478 | 9311481 | 216 | 945 | 25 | 1 | 1 |
| 479 | 9311482 | 189 | 1009 | 21 | 0 | 1 |
| 480 | 9311483 | 217 | 1313 | 32 | 1 | 2 |
| 481 | 9311484 | 214 | 921 | 19 | 2 | 4 |
| 482 | 9311485 | 246 | 1237 | 21 | 4 | 2 |
| 483 | 9311486 | 261 | 1278 | 32 | 2 | 2 |
| 484 | 9311487 | 197 | 731 | 23 | 2 | 3 |
| 485 | 9311488 | 252 | 1253 | 21 | 3 | 3 |
| 486 | 9311489 | 210 | 1198 | 26 | 1 | 5 |
| 487 | 9311490 | 216 | 1116 | 26 | 1 | 3 |
| 488 | 9311491 | 278 | 1606 | 31 | 3 | 3 |
| 489 | 9311492 | 265 | 1439 | 23 | 3 | 2 |
| 490 | 9311493 | 208 | 1105 | 15 | 2 | 3 |
| 491 | 9311494 | 214 | 1027 | 19 | 3 | 2 |
| 492 | 9311495 | 182 | 847 | 15 | 2 | 0 |
| 493 | 9311496 | 224 | 931 | 24 | 6 | 4 |
| 494 | 9311497 | 251 | 1698 | 38 | 4 | 2 |
| 495 | 9311498 | 194 | 1049 | 23 | 5 | 2 |
| 496 | 9311499 | 191 | 1055 | 17 | 1 | 2 |
| 497 | 9311500 | 232 | 1436 | 24 | 1 | 2 |
| 498 | 9311501 | 247 | 1204 | 30 | 3 | 0 |
| 499 | 9311502 | 207 | 1313 | 18 | 3 | 2 |
| 500 | 9311503 | 226 | 1172 | 19 | 6 | 0 |
| 501 | 9311504 | 220 | 1072 | 25 | 1 | 4 |
| 502 | 9311505 | 220 | 1295 | 20 | 4 | 3 |
| 503 | 9311506 | 183 | 819 | 15 | 4 | 3 |
| 504 | 9311507 | 239 | 1230 | 25 | 1 | 3 |
| 505 | 9311508 | 226 | 1059 | 11 | 3 | 2 |
| 506 | 9311509 | 239 | 1105 | 20 | 4 | 3 |
| 507 | 9311510 | 190 | 856 | 20 | 3 | 1 |
| 508 | 9311511 | 222 | 997 | 24 | 2 | 5 |
| 509 | 9311512 | 226 | 1135 | 29 | 6 | 4 |
| 510 | 9311513 | 219 | 1255 | 16 | 2 | 2 |
| 511 | 9311514 | 232 | 1164 | 29 | 3 | 1 |
| 512 | 9311515 | 255 | 1748 | 22 | 2 | 3 |
| 513 | 9311516 | 235 | 1483 | 23 | 1 | 3 |
| 514 | 9311517 | 191 | 687 | 10 | 3 | 1 |
| 515 | 9311518 | 210 | 1043 | 32 | 1 | 2 |
| 516 | 9311519 | 230 | 1010 | 38 | 1 | 2 |
| 517 | 9311520 | 234 | 1206 | 20 | 5 | 3 |
| 518 | 9311521 | 227 | 933 | 21 | 7 | 4 |
| 519 | 9311522 | 193 | 1002 | 15 | 0 | 3 |
| 520 | 9311523 | 215 | 1241 | 16 | 4 | 2 |
| 521 | 9311524 | 239 | 1161 | 23 | 2 | 3 |
| 522 | 9311525 | 277 | 1557 | 32 | 3 | 5 |
| 523 | 9311526 | 215 | 976 | 21 | 7 | 3 |
| 524 | 9311527 | 221 | 1242 | 18 | 10 | 2 |
| 525 | 9311528 | 202 | 1079 | 23 | 1 | 5 |
| 526 | 9311529 | 287 | 1812 | 29 | 2 | 1 |
| 527 | 9311530 | 201 | 1004 | 25 | 2 | 2 |
| 528 | 9311531 | 182 | 1099 | 13 | 4 | 0 |
| 529 | 9311532 | 270 | 1817 | 34 | 5 | 2 |
| 530 | 9311533 | 286 | 1589 | 23 | 0 | 3 |
| 531 | 9311534 | 266 | 1882 | 37 | 0 | 2 |
| 532 | 9311535 | 178 | 848 | 12 | 2 | 2 |
| 533 | 9311536 | 224 | 1341 | 30 | 4 | 4 |
| 534 | 9311537 | 287 | 1610 | 22 | 1 | 3 |
| 535 | 9311538 | 285 | 1473 | 23 | 2 | 2 |
| 536 | 9311539 | 254 | 1402 | 26 | 3 | 4 |
| 537 | 9311540 | 253 | 1478 | 35 | 3 | 1 |
| 538 | 9311541 | 203 | 846 | 14 | 4 | 2 |
| 539 | 9311542 | 166 | 902 | 9 | 6 | 0 |
| 540 | 9311543 | 281 | 1490 | 26 | 6 | 2 |
| 541 | 9311544 | 318 | 2097 | 22 | 0 | 2 |
| 542 | 9311545 | 235 | 1061 | 23 | 0 | 3 |
| 543 | 9311546 | 338 | 2145 | 31 | 2 | 0 |
| 544 | 9311547 | 203 | 1026 | 19 | 5 | 1 |
| 545 | 9311548 | 219 | 1176 | 26 | 3 | 5 |
| 546 | 9311549 | 209 | 1176 | 22 | 7 | 3 |
| 547 | 9311550 | 225 | 1412 | 26 | 1 | 3 |
| 548 | 9311551 | 205 | 1065 | 16 | 0 | 3 |
| 549 | 9311552 | 206 | 1249 | 15 | 9 | 2 |
| 550 | 9311553 | 160 | 745 | 16 | 1 | 0 |
| 551 | 9311554 | 230 | 1495 | 20 | 2 | 1 |
| 552 | 9311555 | 221 | 998 | 24 | 2 | 4 |
| 553 | 9311556 | 231 | 1295 | 19 | 5 | 2 |
| 554 | 9311557 | 184 | 1103 | 16 | 4 | 3 |
| 555 | 9311558 | 260 | 1186 | 22 | 3 | 4 |
| 556 | 9311559 | 394 | 2335 | 49 | 8 | 3 |
| 557 | 9311560 | 235 | 1218 | 17 | 4 | 3 |
| 558 | 9311561 | 211 | 1255 | 17 | 3 | 0 |
| 559 | 9311562 | 205 | 1192 | 21 | 2 | 3 |
| 560 | 9311563 | 282 | 1314 | 25 | 4 | 5 |
| 561 | 9311564 | 225 | 1002 | 23 | 4 | 3 |
| 562 | 9311565 | 200 | 632 | 21 | 1 | 3 |
| 563 | 9311566 | 203 | 1010 | 14 | 2 | 2 |
| 564 | 9311567 | 168 | 866 | 17 | 1 | 1 |
| 565 | 9311568 | 184 | 751 | 25 | 0 | 5 |
| 566 | 9311569 | 296 | 1748 | 42 | 3 | 2 |
| 567 | 9311570 | 214 | 902 | 12 | 4 | 3 |
| 568 | 9311571 | 208 | 1092 | 13 | 4 | 3 |
| 569 | 9311572 | 184 | 979 | 14 | 4 | 2 |
| 570 | 9311573 | 210 | 1154 | 20 | 2 | 3 |
| 571 | 9311574 | 224 | 1391 | 25 | 9 | 2 |
| 572 | 9311575 | 223 | 1385 | 19 | 1 | 1 |
| 573 | 9311576 | 249 | 1280 | 26 | 4 | 3 |
| 574 | 9311577 | 229 | 1168 | 35 | 1 | 5 |
| 575 | 9311578 | 196 | 1026 | 20 | 2 | 2 |
| 576 | 9311579 | 226 | 1287 | 21 | 5 | 1 |
| 577 | 9311580 | 179 | 904 | 13 | 4 | 2 |
| 578 | 9311581 | 238 | 1227 | 15 | 5 | 3 |
| 579 | 9311582 | 238 | 1364 | 29 | 3 | 3 |
| 580 | 9311583 | 184 | 831 | 12 | 0 | 3 |
| 581 | 9311584 | 200 | 775 | 14 | 4 | 2 |
| 582 | 9311585 | 186 | 887 | 17 | 1 | 3 |
| 583 | 9311586 | 270 | 1524 | 22 | 3 | 4 |
| 584 | 9311587 | 288 | 1896 | 32 | 4 | 1 |
| 585 | 9311588 | 219 | 1525 | 28 | 4 | 4 |
| 586 | 9311589 | 234 | 1430 | 21 | 2 | 3 |
| 587 | 9311590 | 200 | 971 | 14 | 1 | 0 |
| 588 | 9311591 | 162 | 711 | 18 | 2 | 2 |
| 589 | 9311592 | 260 | 1510 | 19 | 1 | 0 |
| 590 | 9311593 | 234 | 1263 | 15 | 1 | 1 |
| 591 | 9311594 | 225 | 1230 | 19 | 0 | 3 |
| 592 | 9311595 | 210 | 1258 | 19 | 5 | 2 |
| 593 | 9311596 | 207 | 1048 | 23 | 0 | 2 |
| 594 | 9311597 | 227 | 1020 | 21 | 3 | 2 |
| 595 | 9311598 | 284 | 1452 | 36 | 3 | 4 |
| 596 | 9311599 | 202 | 944 | 15 | 2 | 2 |
| 597 | 9311600 | 194 | 823 | 8 | 4 | 1 |
| 598 | 9311601 | 190 | 795 | 11 | 1 | 3 |
| 599 | 9311602 | 231 | 1069 | 19 | 0 | 3 |
| 600 | 9311603 | 183 | 739 | 17 | 6 | 3 |
| 601 | 9311604 | 203 | 1239 | 26 | 2 | 1 |
| 602 | 9311605 | 181 | 646 | 10 | 3 | 2 |
| 603 | 9311606 | 222 | 1288 | 27 | 2 | 2 |
| 604 | 9311607 | 178 | 1117 | 21 | 1 | 2 |
| 605 | 9311608 | 253 | 1462 | 24 | 5 | 3 |
| 606 | 9311609 | 270 | 1133 | 32 | 4 | 4 |
| 607 | 9311610 | 206 | 916 | 19 | 2 | 3 |
| 608 | 9311611 | 219 | 1405 | 25 | 1 | 2 |
| 609 | 9311612 | 267 | 1755 | 22 | 2 | 3 |
| 610 | 9311613 | 169 | 748 | 17 | 4 | 3 |
| 611 | 9311614 | 219 | 1242 | 20 | 1 | 0 |
| 612 | 9311615 | 199 | 901 | 13 | 3 | 4 |
| 613 | 9311616 | 188 | 883 | 16 | 4 | 2 |
| 614 | 9311617 | 228 | 1327 | 26 | 1 | 3 |
| 615 | 9311618 | 219 | 838 | 23 | 6 | 3 |
| 616 | 9311619 | 193 | 780 | 17 | 1 | 3 |
| 617 | 9311620 | 167 | 807 | 14 | 1 | 1 |
| 618 | 9311621 | 270 | 1861 | 16 | 1 | 1 |
| 619 | 9311622 | 153 | 709 | 14 | 1 | 2 |
| 620 | 9311623 | 241 | 1706 | 32 | 0 | 2 |
| 621 | 9311624 | 165 | 700 | 23 | 2 | 2 |
| 622 | 9311625 | 216 | 1004 | 18 | 10 | 2 |
| 623 | 9311626 | 234 | 1346 | 18 | 10 | 0 |
| 624 | 9311627 | 236 | 1142 | 14 | 4 | 3 |
| 625 | 9311628 | 255 | 1468 | 20 | 5 | 5 |
| 626 | 9311629 | 187 | 827 | 23 | 1 | 2 |
| 627 | 9311630 | 284 | 1640 | 31 | 2 | 3 |
| 628 | 9311631 | 228 | 1221 | 36 | 4 | 1 |
| 629 | 9311632 | 206 | 876 | 16 | 3 | 4 |
| 630 | 9311633 | 255 | 1367 | 33 | 0 | 4 |
| 631 | 9311634 | 255 | 1508 | 26 | 6 | 1 |
| 632 | 9311635 | 186 | 1035 | 22 | 2 | 2 |
| 633 | 9311636 | 198 | 935 | 10 | 4 | 2 |
| 634 | 9311637 | 240 | 1290 | 33 | 2 | 3 |
| 635 | 9311638 | 188 | 854 | 18 | 4 | 2 |
| 636 | 9311639 | 232 | 1378 | 22 | 4 | 3 |
| 637 | 9311640 | 243 | 1439 | 14 | 2 | 1 |
| 638 | 9311641 | 238 | 1565 | 21 | 4 | 1 |
| 639 | 9311642 | 194 | 758 | 19 | 2 | 3 |
| 640 | 9311643 | 210 | 1074 | 32 | 1 | 4 |
| 641 | 9311644 | 170 | 881 | 20 | 0 | 1 |
| 642 | 9311645 | 185 | 973 | 17 | 1 | 3 |
| 643 | 9311646 | 207 | 1064 | 18 | 7 | 2 |
| 644 | 9311647 | 205 | 914 | 16 | 3 | 1 |
| 645 | 9311648 | 185 | 846 | 15 | 5 | 1 |
| 646 | 9311649 | 189 | 974 | 17 | 1 | 2 |
| 647 | 9311650 | 184 | 836 | 19 | 1 | 3 |
| 648 | 9311651 | 311 | 1662 | 34 | 3 | 4 |
| 649 | 9311652 | 243 | 1109 | 22 | 2 | 2 |
| 650 | 9311653 | 257 | 2053 | 22 | 1 | 3 |
| 651 | 9311654 | 247 | 1323 | 24 | 3 | 4 |
| 652 | 9311655 | 186 | 665 | 6 | 2 | 1 |
| 653 | 9311656 | 199 | 1048 | 33 | 3 | 5 |
| 654 | 9311657 | 252 | 1462 | 34 | 4 | 3 |
| 655 | 9311658 | 311 | 1883 | 33 | 2 | 3 |
| 656 | 9311659 | 347 | 2070 | 31 | 1 | 2 |
| 657 | 9311660 | 200 | 886 | 12 | 5 | 2 |
| 658 | 9311661 | 239 | 1141 | 22 | 4 | 2 |
| 659 | 9311662 | 203 | 1274 | 22 | 2 | 1 |
| 660 | 9311663 | 216 | 1007 | 22 | 6 | 3 |
| 661 | 9311664 | 220 | 946 | 19 | 1 | 4 |
| 662 | 9311665 | 198 | 839 | 22 | 2 | 2 |
| 663 | 9311666 | 220 | 1757 | 29 | 2 | 4 |
| 664 | 9311667 | 247 | 1451 | 30 | 2 | 2 |
| 665 | 9311668 | 220 | 1675 | 24 | 0 | 3 |
| 666 | 9311669 | 235 | 1185 | 26 | 3 | 4 |
| 667 | 9311670 | 282 | 1530 | 18 | 2 | 5 |
| 668 | 9311671 | 223 | 1357 | 32 | 1 | 1 |
| 669 | 9311672 | 209 | 1123 | 28 | 1 | 4 |
| 670 | 9311673 | 187 | 970 | 12 | 1 | 1 |
| 671 | 9311674 | 248 | 1325 | 25 | 3 | 3 |
| 672 | 9311675 | 222 | 1076 | 11 | 4 | 3 |
| 673 | 9311676 | 183 | 1131 | 23 | 1 | 2 |
| 674 | 9311677 | 201 | 1100 | 16 | 3 | 1 |
| 675 | 9311678 | 213 | 886 | 25 | 3 | 2 |
| 676 | 9311679 | 277 | 1090 | 16 | 9 | 4 |
| 677 | 9311680 | 240 | 1287 | 27 | 5 | 3 |
| 678 | 9311681 | 172 | 781 | 18 | 1 | 3 |
| 679 | 9311682 | 240 | 1451 | 15 | 4 | 0 |
| 680 | 9311683 | 208 | 1117 | 29 | 5 | 2 |
| 681 | 9311684 | 249 | 1233 | 12 | 3 | 2 |
| 682 | 9311685 | 183 | 884 | 11 | 5 | 1 |
| 683 | 9311686 | 230 | 945 | 24 | 3 | 2 |
| 684 | 9311687 | 173 | 698 | 14 | 4 | 3 |
| 685 | 9311688 | 221 | 1229 | 27 | 2 | 2 |
| 686 | 9311689 | 237 | 1147 | 28 | 6 | 3 |
| 687 | 9311690 | 227 | 1096 | 16 | 2 | 3 |
| 688 | 9311691 | 245 | 1409 | 21 | 0 | 1 |
| 689 | 9311692 | 222 | 1388 | 29 | 0 | 3 |
| 690 | 9311693 | 226 | 1397 | 20 | 3 | 3 |
| 691 | 9311694 | 177 | 660 | 13 | 7 | 0 |
| 692 | 9311695 | 220 | 1330 | 24 | 2 | 1 |
| 693 | 9311696 | 185 | 814 | 25 | 1 | 5 |
| 694 | 9311697 | 226 | 1160 | 19 | 3 | 4 |
| 695 | 9311698 | 240 | 1350 | 24 | 1 | 2 |
| 696 | 9311699 | 328 | 2015 | 32 | 3 | 6 |
| 697 | 9311700 | 202 | 1080 | 16 | 3 | 1 |
| 698 | 9311701 | 223 | 1065 | 30 | 4 | 2 |
| 699 | 9311702 | 194 | 752 | 19 | 1 | 0 |
| 700 | 9311703 | 231 | 1431 | 31 | 3 | 3 |
| 701 | 9311704 | 239 | 1314 | 27 | 4 | 4 |
| 702 | 9311705 | 203 | 1244 | 14 | 1 | 3 |
| 703 | 9311706 | 255 | 1283 | 18 | 3 | 2 |
| 704 | 9311707 | 206 | 1120 | 15 | 3 | 1 |
| 705 | 9311708 | 211 | 1111 | 14 | 5 | 3 |
| 706 | 9311709 | 255 | 1362 | 31 | 4 | 2 |
| 707 | 9311710 | 243 | 1177 | 24 | 2 | 1 |
| 708 | 9311711 | 234 | 1192 | 31 | 1 | 3 |
| 709 | 9311712 | 200 | 915 | 19 | 2 | 1 |
| 710 | 9311713 | 243 | 1411 | 19 | 3 | 4 |
| 711 | 9311714 | 194 | 1025 | 25 | 4 | 1 |
| 712 | 9311715 | 205 | 900 | 9 | 1 | 3 |
| 713 | 9311716 | 223 | 1088 | 14 | 2 | 3 |
| 714 | 9311717 | 185 | 692 | 17 | 1 | 4 |
| 715 | 9311718 | 210 | 1185 | 27 | 0 | 3 |
| 716 | 9311719 | 200 | 781 | 26 | 2 | 3 |
| 717 | 9311720 | 165 | 672 | 13 | 3 | 3 |
| 718 | 9311721 | 242 | 1300 | 19 | 2 | 4 |
| 719 | 9311722 | 249 | 1401 | 23 | 5 | 4 |
| 720 | 9311723 | 223 | 1290 | 29 | 6 | 3 |
| 721 | 9311724 | 262 | 1642 | 28 | 3 | 2 |
| 722 | 9311725 | 194 | 1207 | 28 | 2 | 3 |
| 723 | 9311726 | 197 | 1031 | 22 | 1 | 4 |
| 724 | 9311727 | 219 | 1099 | 21 | 3 | 2 |
| 725 | 9311728 | 271 | 1496 | 37 | 0 | 5 |
| 726 | 9311729 | 348 | 1836 | 46 | 3 | 4 |
| 727 | 9311730 | 195 | 857 | 24 | 2 | 0 |
| 728 | 9311731 | 204 | 1045 | 22 | 1 | 2 |
| 729 | 9311732 | 216 | 977 | 17 | 2 | 2 |
| 730 | 9311733 | 339 | 2331 | 32 | 1 | 3 |
| 731 | 9311734 | 209 | 1085 | 33 | 7 | 3 |
| 732 | 9311735 | 203 | 1005 | 19 | 3 | 3 |
| 733 | 9311736 | 325 | 1982 | 44 | 3 | 2 |
| 734 | 9311737 | 249 | 1357 | 19 | 5 | 3 |
| 735 | 9311738 | 196 | 929 | 19 | 3 | 1 |
| 736 | 9311739 | 319 | 1694 | 31 | 2 | 4 |
| 737 | 9311740 | 190 | 783 | 19 | 4 | 2 |
| 738 | 9311741 | 272 | 1582 | 27 | 2 | 4 |
| 739 | 9311742 | 210 | 1059 | 17 | 6 | 2 |
| 740 | 9311743 | 201 | 999 | 17 | 2 | 1 |
| 741 | 9311744 | 244 | 1413 | 26 | 5 | 3 |
| 742 | 9311745 | 193 | 1088 | 27 | 1 | 3 |
| 743 | 9311746 | 238 | 1244 | 25 | 5 | 4 |
| 744 | 9311747 | 182 | 1042 | 25 | 5 | 3 |
| 745 | 9311748 | 232 | 1238 | 24 | 7 | 2 |
| 746 | 9311749 | 214 | 1038 | 29 | 1 | 3 |
| 747 | 9311750 | 191 | 1035 | 13 | 4 | 2 |
| 748 | 9311751 | 358 | 2401 | 38 | 3 | 1 |
| 749 | 9311752 | 264 | 1571 | 17 | 4 | 3 |
| 750 | 9311753 | 180 | 656 | 17 | 1 | 4 |
| 751 | 9311754 | 215 | 1331 | 12 | 4 | 1 |
| 752 | 9311755 | 190 | 939 | 17 | 3 | 1 |
| 753 | 9311756 | 207 | 1073 | 12 | 3 | 4 |
| 754 | 9311757 | 169 | 1073 | 11 | 4 | 3 |
| 755 | 9311758 | 253 | 1724 | 34 | 3 | 1 |
| 756 | 9311759 | 222 | 943 | 20 | 3 | 3 |
| 757 | 9311760 | 273 | 1417 | 23 | 2 | 2 |
| 758 | 9311761 | 216 | 1007 | 27 | 1 | 5 |
| 759 | 9311762 | 181 | 970 | 22 | 2 | 1 |
| 760 | 9311763 | 254 | 1325 | 32 | 2 | 4 |
| 761 | 9311764 | 240 | 1304 | 24 | 3 | 1 |
| 762 | 9311765 | 178 | 978 | 13 | 1 | 2 |
| 763 | 9311766 | 191 | 901 | 18 | 3 | 2 |
| 764 | 9311767 | 197 | 1227 | 18 | 4 | 4 |
| 765 | 9311768 | 263 | 1585 | 26 | 3 | 1 |
| 766 | 9311769 | 355 | 2196 | 38 | 0 | 3 |
| 767 | 9311770 | 193 | 945 | 17 | 1 | 3 |
| 768 | 9311771 | 185 | 749 | 16 | 2 | 2 |
| 769 | 9311772 | 204 | 1219 | 20 | 0 | 3 |
| 770 | 9311773 | 236 | 965 | 13 | 3 | 5 |
| 771 | 9311774 | 324 | 1587 | 30 | 1 | 6 |
| 772 | 9311775 | 178 | 784 | 23 | 2 | 3 |
| 773 | 9311776 | 160 | 745 | 10 | 2 | 3 |
| 774 | 9311777 | 235 | 1617 | 23 | 2 | 3 |
| 775 | 9311778 | 187 | 924 | 20 | 3 | 3 |
| 776 | 9311779 | 200 | 1051 | 22 | 8 | 2 |
| 777 | 9311780 | 226 | 1288 | 30 | 8 | 2 |
| 778 | 9311781 | 169 | 807 | 12 | 0 | 4 |
| 779 | 9311782 | 232 | 973 | 23 | 3 | 3 |
| 780 | 9311783 | 287 | 1771 | 27 | 0 | 2 |
| 781 | 9311784 | 258 | 1512 | 20 | 4 | 1 |
| 782 | 9311785 | 204 | 1113 | 12 | 0 | 4 |
| 783 | 9311786 | 148 | 544 | 15 | 0 | 2 |
| 784 | 9311787 | 177 | 870 | 18 | 2 | 3 |
| 785 | 9311788 | 247 | 1394 | 22 | 5 | 2 |
| 786 | 9311789 | 211 | 1195 | 30 | 2 | 4 |
| 787 | 9311790 | 235 | 1055 | 21 | 2 | 3 |
| 788 | 9311791 | 211 | 1327 | 15 | 2 | 1 |
| 789 | 9311792 | 247 | 1084 | 22 | 3 | 2 |
| 790 | 9311793 | 203 | 1053 | 25 | 3 | 4 |
| 791 | 9311794 | 245 | 1299 | 21 | 4 | 3 |
| 792 | 9311795 | 225 | 831 | 17 | 1 | 2 |
| 793 | 9311796 | 238 | 1252 | 29 | 3 | 2 |
| 794 | 9311797 | 230 | 1240 | 28 | 5 | 2 |
| 795 | 9311798 | 216 | 1225 | 20 | 2 | 2 |
| 796 | 9311799 | 209 | 896 | 20 | 1 | 4 |
| 797 | 9311800 | 220 | 1188 | 19 | 0 | 2 |
| 798 | 9311801 | 170 | 971 | 12 | 3 | 0 |
| 799 | 9311802 | 190 | 823 | 20 | 4 | 2 |
| 800 | 9311803 | 251 | 1819 | 21 | 2 | 2 |
| 801 | 9311804 | 212 | 1113 | 26 | 2 | 3 |
| 802 | 9311805 | 318 | 1787 | 20 | 1 | 1 |
| 803 | 9311806 | 234 | 1061 | 25 | 0 | 1 |
| 804 | 9311807 | 199 | 1121 | 15 | 0 | 2 |
| 805 | 9311808 | 366 | 2792 | 16 | 5 | 1 |
| 806 | 9311809 | 232 | 1154 | 25 | 3 | 1 |
| 807 | 9311810 | 186 | 948 | 17 | 1 | 2 |
| 808 | 9311811 | 254 | 1528 | 24 | 2 | 1 |
| 809 | 9311812 | 241 | 1526 | 28 | 5 | 4 |
| 810 | 9311813 | 329 | 1557 | 43 | 3 | 4 |
| 811 | 9311814 | 250 | 1358 | 9 | 2 | 0 |
| 812 | 9311815 | 197 | 925 | 18 | 2 | 1 |
| 813 | 9311816 | 258 | 1581 | 39 | 2 | 2 |
| 814 | 9311817 | 219 | 1144 | 21 | 2 | 4 |
| 815 | 9311818 | 171 | 676 | 17 | 2 | 2 |
| 816 | 9311819 | 243 | 1271 | 13 | 2 | 4 |
| 817 | 9311820 | 232 | 1088 | 20 | 6 | 2 |
| 818 | 9311821 | 231 | 1036 | 21 | 5 | 2 |
| 819 | 9311822 | 210 | 1172 | 21 | 1 | 2 |
| 820 | 9311823 | 250 | 1618 | 43 | 3 | 3 |
| 821 | 9311824 | 200 | 1100 | 29 | 1 | 4 |
| 822 | 9311825 | 221 | 1292 | 26 | 1 | 2 |
| 823 | 9311826 | 237 | 1348 | 22 | 0 | 2 |
| 824 | 9311827 | 274 | 1557 | 22 | 5 | 3 |
| 825 | 9311828 | 165 | 719 | 22 | 3 | 2 |
| 826 | 9311829 | 195 | 887 | 15 | 4 | 3 |
| 827 | 9311830 | 213 | 1199 | 12 | 0 | 3 |
| 828 | 9311831 | 231 | 1523 | 23 | 0 | 0 |
| 829 | 9311832 | 227 | 1148 | 21 | 4 | 1 |
| 830 | 9311833 | 187 | 1088 | 15 | 3 | 2 |
| 831 | 9311834 | 194 | 1023 | 21 | 2 | 1 |
| 832 | 9311835 | 220 | 1313 | 35 | 9 | 3 |
| 833 | 9311836 | 236 | 1306 | 29 | 3 | 3 |
| 834 | 9311837 | 194 | 1025 | 19 | 3 | 1 |
| 835 | 9311838 | 212 | 1085 | 15 | 4 | 2 |
| 836 | 9311839 | 214 | 1346 | 31 | 3 | 1 |
| 837 | 9311840 | 204 | 1090 | 24 | 1 | 4 |
| 838 | 9311841 | 238 | 1182 | 32 | 2 | 5 |
| 839 | 9311842 | 250 | 1228 | 28 | 2 | 3 |
| 840 | 9311843 | 201 | 1112 | 34 | 0 | 2 |
| 841 | 9311844 | 207 | 906 | 24 | 3 | 4 |
| 842 | 9311845 | 253 | 1263 | 27 | 4 | 2 |
| 843 | 9311846 | 224 | 1063 | 15 | 3 | 1 |
| 844 | 9311847 | 224 | 1410 | 21 | 0 | 1 |
| 845 | 9311848 | 197 | 1106 | 20 | 3 | 2 |
| 846 | 9311849 | 224 | 1295 | 13 | 2 | 3 |
| 847 | 9311850 | 218 | 1037 | 16 | 2 | 2 |
| 848 | 9311851 | 197 | 937 | 22 | 2 | 2 |
| 849 | 9311852 | 298 | 1901 | 20 | 3 | 1 |
| 850 | 9311853 | 224 | 1066 | 13 | 1 | 1 |
| 851 | 9311854 | 152 | 533 | 9 | 1 | 2 |
| 852 | 9311855 | 279 | 1428 | 34 | 4 | 1 |
| 853 | 9311856 | 163 | 977 | 22 | 1 | 2 |
| 854 | 9311857 | 231 | 1217 | 36 | 1 | 2 |
| 855 | 9311858 | 204 | 819 | 21 | 2 | 2 |
| 856 | 9311859 | 250 | 1143 | 21 | 3 | 2 |
| 857 | 9311860 | 205 | 1219 | 14 | 4 | 1 |
| 858 | 9311861 | 296 | 1851 | 28 | 5 | 4 |
| 859 | 9311862 | 253 | 1312 | 18 | 2 | 1 |
| 860 | 9311863 | 274 | 1529 | 20 | 4 | 2 |
| 861 | 9311864 | 231 | 1160 | 20 | 0 | 0 |
| 862 | 9311865 | 258 | 1494 | 19 | 1 | 2 |
| 863 | 9311866 | 232 | 1338 | 30 | 4 | 3 |
| 864 | 9311867 | 197 | 1178 | 18 | 4 | 2 |
| 865 | 9311868 | 252 | 1401 | 24 | 2 | 3 |
| 866 | 9311869 | 205 | 1082 | 26 | 2 | 2 |
| 867 | 9311870 | 183 | 675 | 16 | 3 | 4 |
| 868 | 9311871 | 167 | 707 | 10 | 1 | 0 |
| 869 | 9311872 | 265 | 1429 | 27 | 5 | 3 |
| 870 | 9311873 | 311 | 1932 | 44 | 7 | 3 |
| 871 | 9311874 | 175 | 837 | 13 | 3 | 0 |
| 872 | 9311875 | 180 | 756 | 10 | 2 | 1 |
| 873 | 9311876 | 310 | 1901 | 28 | 5 | 2 |
| 874 | 9311877 | 228 | 772 | 10 | 1 | 2 |
| 875 | 9311878 | 287 | 1788 | 17 | 1 | 2 |
| 876 | 9311879 | 195 | 796 | 19 | 3 | 2 |
| 877 | 9311880 | 210 | 1028 | 13 | 2 | 4 |
| 878 | 9311881 | 214 | 1031 | 14 | 1 | 2 |
| 879 | 9311882 | 230 | 1214 | 24 | 1 | 2 |
| 880 | 9311883 | 190 | 796 | 13 | 2 | 0 |
| 881 | 9311884 | 294 | 1715 | 39 | 0 | 6 |
| 882 | 9311885 | 240 | 1467 | 23 | 2 | 4 |
| 883 | 9311886 | 297 | 2017 | 27 | 4 | 3 |
| 884 | 9311887 | 249 | 1398 | 23 | 0 | 2 |
| 885 | 9311888 | 208 | 986 | 21 | 2 | 2 |
| 886 | 9311889 | 186 | 915 | 18 | 3 | 2 |
| 887 | 9311890 | 194 | 832 | 22 | 3 | 5 |
| 888 | 9311891 | 246 | 1274 | 23 | 1 | 2 |
| 889 | 9311892 | 186 | 960 | 12 | 6 | 4 |
| 890 | 9311893 | 222 | 1299 | 19 | 2 | 2 |
| 891 | 9311894 | 242 | 1590 | 35 | 1 | 3 |
| 892 | 9311895 | 167 | 801 | 20 | 3 | 2 |
| 893 | 9311896 | 256 | 1273 | 34 | 7 | 3 |
| 894 | 9311897 | 268 | 1298 | 35 | 1 | 3 |
| 895 | 9311898 | 219 | 1137 | 20 | 1 | 3 |
| 896 | 9311899 | 249 | 1120 | 20 | 1 | 4 |
| 897 | 9311900 | 189 | 890 | 18 | 3 | 3 |
| 898 | 9311901 | 205 | 881 | 18 | 2 | 2 |
| 899 | 9311902 | 239 | 1478 | 22 | 4 | 2 |
| 900 | 9311903 | 212 | 844 | 13 | 5 | 0 |
| 901 | 9311904 | 245 | 1169 | 17 | 3 | 3 |
| 902 | 9311905 | 213 | 1214 | 19 | 5 | 3 |
| 903 | 9311906 | 229 | 1321 | 22 | 1 | 3 |
| 904 | 9311907 | 170 | 898 | 19 | 6 | 2 |
| 905 | 9311908 | 249 | 1338 | 24 | 6 | 4 |
| 906 | 9311909 | 210 | 837 | 21 | 4 | 3 |
| 907 | 9311910 | 223 | 1257 | 27 | 2 | 2 |
| 908 | 9311911 | 264 | 1544 | 23 | 5 | 1 |
| 909 | 9311912 | 165 | 935 | 20 | 2 | 2 |
| 910 | 9311913 | 234 | 1321 | 16 | 2 | 2 |
| 911 | 9311914 | 292 | 1336 | 21 | 2 | 4 |
| 912 | 9311915 | 220 | 1197 | 30 | 6 | 2 |
| 913 | 9311916 | 203 | 986 | 21 | 1 | 2 |
| 914 | 9311917 | 229 | 1214 | 32 | 2 | 1 |
| 915 | 9311918 | 209 | 1244 | 17 | 2 | 2 |
| 916 | 9311919 | 194 | 1222 | 25 | 4 | 2 |
| 917 | 9311920 | 216 | 1183 | 28 | 1 | 2 |
| 918 | 9311921 | 245 | 1514 | 15 | 3 | 0 |
| 919 | 9311922 | 263 | 1717 | 28 | 3 | 3 |
| 920 | 9311923 | 219 | 1062 | 15 | 1 | 3 |
| 921 | 9311924 | 191 | 819 | 9 | 3 | 2 |
| 922 | 9311925 | 164 | 700 | 17 | 0 | 2 |
| 923 | 9311926 | 191 | 864 | 20 | 0 | 0 |
| 924 | 9311927 | 239 | 1132 | 23 | 1 | 2 |
| 925 | 9311928 | 197 | 1307 | 15 | 0 | 1 |
| 926 | 9311929 | 207 | 907 | 20 | 1 | 3 |
| 927 | 9311930 | 212 | 1077 | 23 | 6 | 4 |
| 928 | 9311931 | 286 | 1201 | 17 | 0 | 2 |
| 929 | 9311932 | 241 | 1011 | 18 | 2 | 2 |
| 930 | 9311933 | 222 | 1373 | 21 | 7 | 1 |
| 931 | 9311934 | 201 | 1024 | 20 | 1 | 3 |
| 932 | 9311935 | 228 | 1368 | 28 | 5 | 3 |
| 933 | 9311936 | 186 | 846 | 15 | 1 | 3 |
| 934 | 9311937 | 236 | 1317 | 35 | 1 | 6 |
| 935 | 9311938 | 163 | 729 | 18 | 0 | 4 |
| 936 | 9311939 | 287 | 1688 | 32 | 2 | 4 |
| 937 | 9311940 | 168 | 926 | 23 | 1 | 3 |
| 938 | 9311941 | 173 | 654 | 11 | 4 | 2 |
| 939 | 9311942 | 211 | 1069 | 17 | 2 | 0 |
| 940 | 9311943 | 204 | 1136 | 22 | 2 | 2 |
| 941 | 9311944 | 243 | 1243 | 30 | 1 | 2 |
| 942 | 9311945 | 221 | 1253 | 32 | 1 | 3 |
| 943 | 9311946 | 188 | 876 | 11 | 1 | 2 |
| 944 | 9311947 | 212 | 1212 | 12 | 6 | 1 |
| 945 | 9311948 | 233 | 975 | 21 | 0 | 4 |
| 946 | 9311949 | 246 | 1303 | 28 | 2 | 2 |
| 947 | 9311950 | 175 | 722 | 17 | 7 | 2 |
| 948 | 9311951 | 267 | 1768 | 17 | 5 | 3 |
| 949 | 9311952 | 231 | 1278 | 27 | 1 | 2 |
| 950 | 9311953 | 196 | 900 | 12 | 1 | 3 |
| 951 | 9311954 | 275 | 1192 | 23 | 1 | 3 |
| 952 | 9311955 | 183 | 929 | 21 | 3 | 4 |
| 953 | 9311956 | 194 | 1072 | 27 | 4 | 3 |
| 954 | 9311957 | 194 | 983 | 15 | 8 | 2 |
| 955 | 9311958 | 275 | 1514 | 28 | 4 | 1 |
| 956 | 9311959 | 297 | 1725 | 23 | 5 | 3 |
| 957 | 9311960 | 219 | 1061 | 30 | 3 | 3 |
| 958 | 9311961 | 222 | 1089 | 24 | 1 | 3 |
| 959 | 9311962 | 236 | 1182 | 15 | 1 | 2 |
| 960 | 9311963 | 320 | 1822 | 34 | 3 | 4 |
| 961 | 9311964 | 213 | 1163 | 13 | 4 | 3 |
| 962 | 9311965 | 223 | 949 | 22 | 2 | 5 |
| 963 | 9311966 | 226 | 1425 | 20 | 6 | 3 |
| 964 | 9311967 | 194 | 972 | 25 | 2 | 2 |
| 965 | 9311968 | 236 | 1481 | 24 | 2 | 3 |
| 966 | 9311969 | 263 | 1643 | 30 | 1 | 2 |
| 967 | 9311970 | 190 | 1020 | 17 | 2 | 1 |
| 968 | 9311971 | 211 | 1161 | 22 | 0 | 4 |
| 969 | 9311972 | 200 | 1057 | 24 | 3 | 2 |
| 970 | 9311973 | 197 | 1017 | 29 | 2 | 3 |
| 971 | 9311974 | 240 | 1807 | 12 | 2 | 2 |
| 972 | 9311975 | 247 | 918 | 20 | 0 | 2 |
| 973 | 9311976 | 272 | 1656 | 28 | 2 | 4 |
| 974 | 9311977 | 208 | 783 | 12 | 5 | 2 |
| 975 | 9311978 | 201 | 879 | 22 | 1 | 2 |
| 976 | 9311979 | 256 | 1246 | 21 | 6 | 5 |
| 977 | 9311980 | 259 | 1347 | 23 | 2 | 3 |
| 978 | 9311981 | 238 | 1577 | 26 | 6 | 2 |
| 979 | 9311982 | 192 | 973 | 26 | 6 | 2 |
| 980 | 9311983 | 225 | 1357 | 34 | 4 | 2 |
| 981 | 9311984 | 188 | 904 | 20 | 2 | 2 |
| 982 | 9311985 | 246 | 1280 | 37 | 3 | 3 |
| 983 | 9311986 | 220 | 1237 | 20 | 5 | 3 |
| 984 | 9311987 | 240 | 1486 | 30 | 2 | 5 |
| 985 | 9311988 | 213 | 1104 | 26 | 3 | 4 |
| 986 | 9311989 | 158 | 626 | 9 | 1 | 1 |
| 987 | 9311990 | 268 | 1400 | 27 | 2 | 4 |
| 988 | 9311991 | 231 | 1100 | 22 | 5 | 0 |
| 989 | 9311992 | 247 | 1347 | 30 | 2 | 4 |
| 990 | 9311993 | 265 | 1975 | 19 | 2 | 3 |
| 991 | 9311994 | 262 | 1357 | 25 | 3 | 4 |
| 992 | 9311995 | 197 | 865 | 21 | 0 | 1 |
| 993 | 9311996 | 248 | 1899 | 17 | 4 | 2 |
| 994 | 9311997 | 256 | 1195 | 21 | 4 | 3 |
| 995 | 9311998 | 183 | 715 | 21 | 1 | 3 |
| 996 | 9311999 | 204 | 892 | 16 | 2 | 1 |
| 997 | 9312000 | 213 | 814 | 20 | 2 | 3 |
| 998 | 9312001 | 233 | 850 | 7 | 0 | 1 |
| 999 | 9312002 | 206 | 981 | 18 | 2 | 3 |
| 1000 | 9312003 | 277 | 1502 | 25 | 5 | 2 |
| 1001 | 9312004 | 233 | 1362 | 15 | 2 | 3 |
| 1002 | 9312005 | 295 | 1631 | 30 | 4 | 2 |
| 1003 | 9312006 | 213 | 1140 | 20 | 3 | 3 |
| 1004 | 9312007 | 258 | 1034 | 24 | 2 | 2 |
| 1005 | 9312008 | 281 | 1670 | 38 | 3 | 2 |
| 1006 | 9312009 | 223 | 1078 | 21 | 5 | 2 |
| 1007 | 9312010 | 183 | 737 | 18 | 4 | 1 |
| 1008 | 9312011 | 178 | 737 | 23 | 2 | 4 |
| 1009 | 9312012 | 220 | 1192 | 14 | 0 | 2 |
| 1010 | 9312013 | 232 | 1090 | 19 | 4 | 2 |
| 1011 | 9312014 | 222 | 1137 | 18 | 4 | 3 |
| 1012 | 9312015 | 152 | 802 | 16 | 1 | 2 |
| 1013 | 9312016 | 197 | 1090 | 32 | 0 | 4 |
| 1014 | 9312017 | 238 | 1528 | 32 | 4 | 0 |
| 1015 | 9312018 | 173 | 876 | 12 | 5 | 1 |
| 1016 | 9312019 | 201 | 948 | 12 | 3 | 2 |
| 1017 | 9312020 | 189 | 1032 | 13 | 1 | 1 |
| 1018 | 9312021 | 330 | 1699 | 48 | 3 | 5 |
| 1019 | 9312022 | 219 | 1108 | 22 | 3 | 4 |
| 1020 | 9312023 | 264 | 1359 | 27 | 3 | 2 |
| 1021 | 9312024 | 225 | 1195 | 21 | 2 | 3 |
| 1022 | 9312025 | 248 | 828 | 7 | 6 | 4 |
| 1023 | 9312026 | 207 | 994 | 23 | 2 | 5 |

## Throughput

- Turns simulated: 231214 (first play of each game)
- Aggregate: 140.9 turns/s over 1640.4s wall
- Per game thread: 36.0 turns/s

## Determinism

Not checked (run with `--determinism`).

## Panics

None.
