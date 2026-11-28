# Full-game paired invasion-defense measurement

**Result: inconclusive; keep #366 open.** On 300 paired public seeds, defense-on won 48 games versus 46 with defense-off. The paired win-share difference is +0.67 percentage points, with a 95% bootstrap interval of −3.00 to +4.33 points. This does not establish nonregression. Attributed short-warning city losses fell from 147 to 135, but the paired per-game difference also includes zero. No AI behavior was changed or tuned from these results.

## Fixed comparison

- Frozen source: `1f7db8eed96d513a7b1acc70b8ad72c27458a398`, after the observer in #433 and the late-defense integration in #394.
- Public seeds **7101–7400**, 300 pairs, six-player `duel` mode (six independent Hard seats, not a two-player duel), no forced rusher, randomized distinct factions and measured seat `seed % 6`. Only the measured seat changes `invasion_defense=0` versus `1`; opponents retain shipped Hard. Other genes, maps and brain seeds match within each pair.
- Seeds 7101–7102 previously appeared in the T180 pipeline smoke. This is disclosed reuse, not held-out validation. The seed block and interpretation were committed before completing the batch; no seeds were removed or added based on outcomes.
- Explicit runner safety cap: **700 turns**. Game rules normally end at T500. All **600/600 games finished**, so no truncated games are counted as losses and all 300 pairs enter the win comparison. Mean reported end turns: 260.59 off, 258.62 on.
- The existing release runner built from the #433 worktree at `b3dfd226e49149b31700b9926fa23feb49954c6f` was reused after verifying identical `crates/`, Cargo manifests/lockfile and toolchain inputs against the frozen source. It was copied to a private target directory. SHA-256: `e9de9391bb09260a25227752336c358e9818b464f6395f064551640cd0d0ed91`. The batch ran on two threads in about 38 minutes, including the initial two-pair runtime check; its output file resumed those four rows without duplication.

## Paired effects

Intervals are descriptive 95% percentile bootstrap intervals from **20,000 resamples of paired seeds**, RNG seed 366, using NumPy 2.3.5. Assaults are never treated as independent samples. Differences are **on minus off**. A confidence interval crossing zero is inconclusive, not proof of equal performance or nonregression. No noninferiority margin was specified.

| Measure | Defense off | Defense on | Paired difference [95% interval] |
| --- | ---: | ---: | ---: |
| Wins / 300 completed games | 46 (15.33%) | 48 (16.00%) | +0.67 pp [−3.00, +4.33] |
| All city losses / game | 5.960 | 5.813 | −0.147 [−0.643, +0.290] |
| Late assaults / game | 5.333 | 5.047 | −0.287 [−0.823, +0.203] |
| Short-warning assault opportunities / game | 1.050 | 0.893 | −0.157 [−0.310, −0.010] |
| Attributed short-warning losses / game | 0.490 | 0.450 | −0.040 [−0.143, +0.060] |
| Late losses without an assault record / game | 0.097 | 0.100 | +0.003 [−0.037, +0.043] |

There were 16 seeds won only with defense-off and 18 won only with defense-on. The opportunity reduction is diagnostic, not a separate acceptance test: this table contains multiple descriptive endpoints without a multiplicity adjustment. The intervention can change whether, when and which armies attack; its cohorts are not matched attacks.

## Cohorts and censoring

The [metric definition](README.md#metric-version-1) classifies the opening turn's actual striking units, not every supporting unit or the human concept of an army. Reported assaults begin at T120 or later. First sightings survive fog/reinforcement changes; city tenure and peace delimit episodes. Some undefended or competing captures have unknown attribution.

| Cohort | Opportunities off/on | Attributed losses off/on | Other ownership end off/on | Peace off/on | Right-censored off/on |
| --- | ---: | ---: | ---: | ---: | ---: |
| Visible within two turns | 315 / 268 | 147 / 135 | 100 / 82 | 30 / 23 | 38 / 28 |
| Visible longer | 1,211 / 1,202 | 714 / 693 | 294 / 283 | 87 / 122 | 116 / 104 |
| Unseen at attack start | 74 / 44 | 22 / 14 | 13 / 11 | 14 / 8 | 25 / 11 |
| Left-censored first sighting | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 |

Late losses without an attributable assault record: **29 off / 30 on**, retained separately. All right-censored episodes: **179 off / 143 on**. Even a completed game can end with an ongoing assault; those episodes are not successful defenses. Other-ownership endings are not automatically successful defenses either.

Observed losses divided by **all opportunities** are 46.67% off versus 50.37% on for the short-warning cohort, a +3.71-point difference with paired seed-bootstrap interval [−3.54, +11.04] points. These fractions include censored opportunities in their denominators and are **not eventual loss probabilities**. The other cohorts' observed fractions are 58.96% / 57.65% (visible longer) and 29.73% / 31.82% (unseen). The JSON summary preserves fraction intervals, denominator checks and all outcome counts; empty left-censored fractions remain undefined.

## Recommendation and next step

The point estimates favor defense-on for wins and losses per game, but they do not establish the issue's “no win-share regression and fewer short-warning city losses” acceptance criteria. Keep #366 open and retain the existing policy unchanged on this evidence. Do not declare the higher conditional loss fraction a regression either: uncertainty and changed opportunity composition prevent that inference.

The next bounded task is a public-seed case audit: inspect attributed short-warning losses and the 29/30 unattributed-loss buckets, checking actual visible approach, available defenders/Gold and the orders chosen. Distinguish failure to detect a threat from a legal purchase/production delay and from the observer's attacker-attribution limitations. Any justified policy change needs a new predeclared, disjoint seed block; these diagnostic seeds are not fresh validation. This public benchmark also does not replace a recorded designer-seat gate when those inputs become available.

## Reproduce and inspect

Run against the frozen source with a newly built matching release runner, or the verified executable above. The first two pairs were a timing check; the 300-pair command resumes them by arm/seed. Never reuse an output file from a different engine or configuration.

```sh
RAYON_NUM_THREADS=2 /path/to/private/target/release/examples/rush_ladder \
  --arm baseline:invasion_defense=0 --arm candidate:invasion_defense=1 \
  --mode duel --players 6 --games 300 --seed 7101 --turns 700 \
  --out /path/to/new-full-game-output.jsonl
python3 docs/sim-results/ai-quality/visible-assaults/analyze-full-games.py \
  /path/to/new-full-game-output.jsonl /path/to/summary-directory
```

- [Compact per-game outcomes and complete cohort records](full-games-7101-7400.jsonl): 600 rows, sorted by seed/arm; unused runner diagnostics omitted, all fields required by the analysis retained.
- [Machine-readable summary](full-games-summary.json) and [analysis script](analyze-full-games.py). The script can also analyze the compact published JSONL; only its input-file hash differs from the original raw runner file. Original raw SHA-256: `41091e45100aa49750f346fc6451e0397deaf5b4494cf6b243fe510387f33826`.
- Validation: exact unique 600 arm/seed keys, matching faction/seat within pairs, metric version 1 and zero replay mismatches; synthetic equal-arm/unfinished-pair/empty-cohort checks passed. The eight observer unit tests were passed during #433; no observer or engine code changes here.

Written by ChatGPT (OpenAI; agent: chatgpt).
