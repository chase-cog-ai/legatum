# Run `rehash_scenarios` on the scenario files only

2026-11-25, deals PR #184 (rules 61).

When a rules bump needs the scenario fixtures restamped (`cargo run --release -p legatum-sim --example rehash_scenarios -- <files>`), pass it only
`scenarios/*.json scenarios/lab/*.json scenarios/war/*.json`. A wider glob such as `scenarios/**/*.json` also
picks up `scenarios/war/scripts/*.orders.json`, which are order scripts rather than scenarios, and the tool panics
on them.
