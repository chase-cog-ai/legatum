# Hard Sparta's conquest opening still pays under city upkeep, at about 60% of the gain

2026-11-27. The follow-up `26-sparta-conquest-opening.md` asked for: its numbers were measured on rules 62, before city upkeep (rules 63). Same runner and arms, one Sparta seat against five built-in Hard AIs, the opening genes off and on, paired seed for seed:

```sh
cargo build --release -p legatum-sim --example offense_ladder
./target/release/examples/offense_ladder \
  --arm off:sparta_rome_opening=0,sparta_opening_ready=0,sparta_opening_search=0 --arm on \
  --games 300 --seed 1001 --players 6 --faction Sparta --out sparta-opening.jsonl
```

Run at `9abb1d31` (rules 62), `0c5d2e8f` (rules 63), `5f98e07d` (rules 64) and `025325a4` (rules 64, after Hard Egypt's peace growth; the newest measured), and there again with `--seed 2001` as held-out seeds. Paired differences are on minus off with a 95% interval.

| | Wins off | Wins on | Paired | VP share off | VP share on | Paired | Game ends (on) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Rules 62, seeds 1001-1300 | 22.3% | 37.0% | +14.7 ± 6.4 | 0.183 | 0.305 | +0.122 ± 0.033 | turn 229 |
| Rules 63, seeds 1001-1300 | 16.3% | 21.7% | +5.3 ± 6.0 | 0.168 | 0.216 | +0.047 ± 0.022 | turn 267 |
| Rules 64 at `5f98e07d`, seeds 1001-1300 | 13.7% | 23.0% | +9.3 ± 5.6 | 0.166 | 0.221 | +0.055 ± 0.021 | turn 269 |
| Rules 64 at `025325a4`, seeds 1001-1300 | 9.3% | 18.3% | +9.0 ± 5.3 | 0.139 | 0.206 | +0.067 ± 0.021 | turn 264 |
| Rules 64 at `025325a4`, seeds 2001-2300 | 8.0% | 16.7% | +8.7 ± 4.8 | 0.120 | 0.187 | +0.067 ± 0.020 | turn 271 |
| Rules 64 at `025325a4`, all 600 | 8.7% | 17.5% | +8.8 ± 3.6 | 0.129 | 0.196 | +0.067 ± 0.014 | turn 267 |

The opening itself is untouched by upkeep, which starts at the seventh city: through turn 50 the rules 62, 63 and `5f98e07d` runs are the same games (declared by turn 30 in 78% of games against 33%, a capital taken by turn 50 in 71% against 15%, a rival eliminated by turn 50 in 54% against 6%, 1.41 captures by turn 50 against 0.49). At `025325a4`, 600 games: declared by turn 30 in 82% against 41%, a capital by turn 50 in 73% against 20%, a rival eliminated by turn 50 in 56% against 8%, 1.47 captures against 0.59.

What upkeep changed is the conversion of that lead into a win. With the opening on, Sparta on rules 62 ended with 10.3 cities and lost 3.3 along the way; at `025325a4` it ends with 7.0 and loses 5.2, and games run about 40 turns longer. The win gain fell from +14.7 to +8.8 points and the VP share gain from +0.12 to +0.07; both are still clear of zero on 600 games, and the opening still doubles Sparta's wins, so the opening genes stay at their defaults. The rules 63 run alone (+5.3 ± 6.0) would not have shown it; rules 63 and 64 differ only in the Upgrade path, and that is enough to reshuffle the late game seed by seed. Hard Egypt's peace growth (`egypt_peace_army`, `a0f44d9f`) took about four more points of wins from Sparta with the opening off and on alike (seeds 1001-1300: 13.7% to 9.3% off, 23.0% to 18.3% on) and left the gain where it was.

The two follow-up genes still do not earn a default (`025325a4`, seeds 1001-1300, paired against the opening on): `sparta_second_war_from=30` takes more cities by turn 50 (1.67 against 1.51) and wins -1.3 ± 3.5 points; `sparta_war_army=6` wins -0.7 ± 4.6; both together -1.7 ± 4.8. More early conquest is not what Sparta lacks under upkeep; holding and paying for what it took after turn 100 is.

`offense_ladder` prints `capital50` as the mean number of capitals taken by turn 50 (0.74 on rules 62), not the share of games with one (71%); the 74% in the 26 November entry is that mean.
