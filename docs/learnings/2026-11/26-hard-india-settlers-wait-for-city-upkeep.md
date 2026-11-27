# Hard India settled itself into a deficit under city upkeep

2026-11-26. City upkeep (cities x (cities - 6) Gold a turn) cut Hard India's all-Hard win rate from 5.0% to 1.1% (`26-city-upkeep-catch-up.md`). `offense_ladder --faction India` now records net Gold and city upkeep at turns 20 to 150, and takes `--rule` to run a rule experiment at the whole table.

Why India: the Settler gate never looked at city upkeep, and a new Indian city remits nothing until its Revenue Office stands, so the seventh and eighth cities cost 7 and 9 Gold a turn before they pay anything. In 300 six-seat Hard games (seed 7301) the India seat founded 6.5 cities and lost 8.3 (its own and captured ones) a game. At turn 100, seats holding up to 6 cities netted +0.2 to +3.6 Gold a turn; seats with 7 netted +0.7 with 30 Gold banked, and seats with 8 or more netted -4.4 with 10 Gold banked while paying 20 a turn in city upkeep. The troop runway rules (`treasury_build_guard`, India's ten-turn troop veto) then refuse the army, and the cities fall. Expansion before turn 50 is not the cause: cities at turn 50 are the same with the rule off.

Fix (AI only): `india_settle_runway` 20 (read by Hard India only; 0 turns it off). Hard India trains no Settler while one more city's added city upkeep, taken from its planned net Gold, would leave the treasury under 20 turns of runway. Held cities plus Settlers alive or in production are counted, and the new city's own Gold is not. 300 paired six-seat Hard games (seed 7301, one India seat): India wins 3.3% to 6.7% (+3.3 pp ±2.9; 15 games won only with the gene, 5 only without), VP share 0.091 to 0.106 (+0.015 ±0.011). With the gene India founds 5.4 cities instead of 6.5, loses 6.8 instead of 8.3, fields 10.6 units instead of 9.4 at turn 150 and pays 4.1 instead of 6.3 Gold of city upkeep. The same seeds with city upkeep off for all six seats (`--rule no-city-upkeep`): 8.3% (+5.0 pp ±3.1), so the gene recovers about two thirds of what the rule cost this seat. A runway of 40 measured the same (7.0%, +3.7 pp ±3.0).

The value 20 was picked on 296 other paired games (seeds 5001 and 6001: 5.1% to 8.1%, +3.0 pp ±3.4, not significant alone) and then confirmed on seed 7301, which no arm had been tuned on.

Reproduce:

```sh
cargo build --release -p legatum-sim --example offense_ladder
target/release/examples/offense_ladder --arm off:india_settle_runway=0 --arm base \
  --arm settle40:india_settle_runway=40 --games 300 --seed 7301 --players 6 --faction India --out india.jsonl
target/release/examples/offense_ladder --arm off:india_settle_runway=0 --rule no-city-upkeep \
  --games 300 --seed 7301 --players 6 --faction India --out india-no-upkeep.jsonl
```

Lesson: a faction whose new cities pay late needs its expansion gate to price the upkeep step; the runway rules downstream only see the deficit after the city exists. India is still the weakest seat here (6.7% of a six-seat table against 16.7% par), so this is a recovery, not a balance fix.
