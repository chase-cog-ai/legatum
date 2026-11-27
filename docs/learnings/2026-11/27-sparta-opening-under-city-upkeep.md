# Hard Sparta's conquest opening still pays under city upkeep, at about two thirds of the gain

2026-11-27. The follow-up `26-sparta-conquest-opening.md` asked for: its numbers were measured on rules 62, before city upkeep (rules 63). Same runner and arms, one Sparta seat against five built-in Hard AIs, the opening genes off and on, paired seed for seed:

```sh
cargo build --release -p legatum-sim --example offense_ladder
./target/release/examples/offense_ladder \
  --arm off:sparta_rome_opening=0,sparta_opening_ready=0,sparta_opening_search=0 --arm on \
  --games 300 --seed 1001 --players 6 --faction Sparta --out sparta-opening.jsonl
```

Run at `9abb1d31` (rules 62), `0c5d2e8f` (rules 63) and `5f98e07d` (rules 64, master), and on master again with `--seed 2001` as held-out seeds. Paired differences are on minus off with a 95% interval.

| | Wins off | Wins on | Paired | VP share off | VP share on | Paired | Game ends (on) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Rules 62, seeds 1001-1300 | 22.3% | 37.0% | +14.7 ± 6.4 | 0.183 | 0.305 | +0.122 ± 0.033 | turn 229 |
| Rules 63, seeds 1001-1300 | 16.3% | 21.7% | +5.3 ± 6.0 | 0.168 | 0.216 | +0.047 ± 0.022 | turn 267 |
| Rules 64, seeds 1001-1300 | 13.7% | 23.0% | +9.3 ± 5.6 | 0.166 | 0.221 | +0.055 ± 0.021 | turn 269 |
| Rules 64, seeds 2001-2300 | 11.3% | 21.0% | +9.7 ± 5.6 | 0.137 | 0.207 | +0.070 ± 0.023 | turn 273 |
| Rules 64, all 600 | 12.5% | 22.0% | +9.5 ± 4.0 | 0.152 | 0.214 | +0.063 ± 0.016 | turn 271 |

The opening itself is untouched by upkeep, which starts at the seventh city: through turn 50 the rules 62, 63 and 64 runs on seeds 1001-1300 are the same games (declared by turn 30 in 78% of games against 33%, a capital taken by turn 50 in 71% against 15%, a rival eliminated by turn 50 in 54% against 6%, 1.41 captures by turn 50 against 0.49). The held-out seeds agree (78% against 35%, 69% against 18%, 58% against 7%).

What upkeep changed is the conversion of that lead into a win. With the opening on, Sparta on rules 62 ended with 10.3 cities and lost 3.3 along the way; on rules 64 it ends with 8.0 and loses 5.0, and games run about 40 turns longer. The win gain fell from +14.7 to +9.5 points and the VP share gain from +0.12 to +0.06; both are still clear of zero on 600 games, so the opening genes stay at their defaults. The rules 63 run alone (+5.3 ± 6.0) would not have shown it; rules 63 and 64 differ only in the Upgrade path, and that is enough to reshuffle the late game seed by seed.

The two follow-up genes still do not earn a default (rules 64, seeds 1001-1300, paired against the opening on): `sparta_second_war_from=30` wins -0.7 ± 3.6 points with more captures by turn 50 (1.57 against 1.41); `sparta_war_army=6` wins -5.7 ± 4.9; both together -4.0 ± 5.8. More early conquest is not what Sparta lacks under upkeep; holding and paying for what it took after turn 100 is.

`offense_ladder` prints `capital50` as the mean number of capitals taken by turn 50 (0.74), not the share of games with one (71%); the 74% in the 26 November entry is that mean.
