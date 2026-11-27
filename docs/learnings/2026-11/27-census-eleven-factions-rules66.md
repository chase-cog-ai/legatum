# Eleven-faction census on rules 66: best/worst 8.4, Egypt 34.0% and Rome 4.0%

2026-11-27. Published as measured, no tuning, no rule or AI change. Reports: `docs/sim-results/2026-11-27-census-eleven-factions-rules66.md` (master `82a0e544`, rules 66), and the two rules 64 arms run earlier the same day on the same seeds: `docs/sim-results/2026-11-27-census-eleven-factions-rules64.md` (master `025325a4`) and `docs/sim-results/2026-11-27-census-eleven-factions-rules64-before-322.md` (master `5f98e07d`).

- **Why:** the last full-size census was rules 50 (2026-11-22, ratio 9.1, Egypt over Rome), and the city-upkeep check of 2026-11-26 ran on the runner's default pool, which has no Egypt or Rome.
- **Runner:** `legatum-sim --all-factions` seats the census design of 2026-11-22 in one run: the 11 cyclic rosters of 8 over `Faction::ALL`, one roster per 8 games, turned one seat per game. `--players` must be 8 and `--games` a multiple of 88; then every faction plays every seat position equally often. Off by default: the nine-faction seeded draw, `--factions` rosters and pinned tapes are unchanged. Before this the design took 11 runs with hand-written `--factions` rosters and `--rotate-factions`.
- **Run:** `cargo run --release -p legatum-sim -- --players 8 --difficulty hard --all-factions --games 2112 --seed 9311003 --label census-eleven-factions-rules66`. 2,112 games, seeds 9311003 to 9313114, 1,536 seats per faction, 0 panics, 0 stalls, about 35 minutes on 8 cores. Held-out seeds were not run. The seeds are not the per-roster seeds of the rules 50 census, so compare rates with it, not games.
- **Three arms, same seeds.** Rules 66 is master `82a0e544`. The rules 64 arms are master `025325a4` (after #322, `egypt_peace_army`) and master `5f98e07d` (before it) with this change's two `crates/sim` commits cherry-picked (the report records that local commit, `d46062d3`, which is on no branch). Between the two rules 64 arms #322 is the only commit that changes what the AI or the rules do. Between `025325a4` and `82a0e544` several commits touch the rules or the AI (India's Revenue Offices #327, rules 65; Hard Sparta's chained opening #326; Byzantium's Buy a city #305, rules 66 among them), so the rules 66 changes are not attributed to one commit here. The before-#322 arm was run twice and every results section of the two reports was identical.
- **Win rate per seat (95% Wilson interval for rules 66; even is 12.5%; Change is rules 66 against rules 64 `025325a4`):**

| Faction | Seats | Wins | Rules 66 (`82a0e544`) | 95% interval | Rules 64 (`025325a4`) | Change | Rules 64 before #322 (`5f98e07d`) | Rules 50 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Egypt | 1,536 | 523 | 34.0% | 31.7 to 36.5 | 37.0% | −2.9 pp | 14.6% | 21.4% |
| Celts | 1,536 | 244 | 15.9% | 14.1 to 17.8 | 16.7% | −0.8 pp | 18.9% | 14.1% |
| Germany | 1,536 | 206 | 13.4% | 11.8 to 15.2 | 13.5% | −0.1 pp | 16.0% | 14.3% |
| Athens | 1,536 | 193 | 12.6% | 11.0 to 14.3 | 12.6% | −0.1 pp | 17.6% | 14.5% |
| Sparta | 1,536 | 189 | 12.3% | 10.8 to 14.0 | 12.3% | +0.0 pp | 15.5% | 12.0% |
| Byzantium | 1,536 | 173 | 11.3% | 9.8 to 12.9 | 12.6% | −1.3 pp | 14.1% | 9.0% |
| India | 1,536 | 155 | 10.1% | 8.7 to 11.7 | 1.9% | +8.2 pp | 2.1% | 8.3% |
| Aztecs | 1,536 | 151 | 9.8% | 8.4 to 11.4 | 10.4% | −0.5 pp | 12.6% | 12.8% |
| Switzerland | 1,536 | 122 | 7.9% | 6.7 to 9.4 | 9.4% | −1.4 pp | 13.3% | 10.0% |
| Mongols | 1,536 | 92 | 6.0% | 4.9 to 7.3 | 6.4% | −0.5 pp | 6.8% | 18.7% |
| Rome | 1,536 | 62 | 4.0% | 3.2 to 5.1 | 4.7% | −0.7 pp | 5.8% | 2.3% |

- **Ratio:** best/worst 8.4 (Egypt over Rome; interval 6.2 to 11.5, best's lower bound over worst's upper bound and the reverse). Pillar 8's 1.5x is not met. On rules 64 it was 19.6 (Egypt 37.0% over India 1.9%), and 8.8 before #322 (Celts 18.9% over India 2.1%). Without Egypt the rules 66 spread is 3.9 (Celts over Rome).
- **India:** 1.9% to 10.1% on identical seeds, the direction #327 (Revenue Offices waive upkeep) aimed for; India is no longer the worst faction, and reached its goal in 41.8% of seats.
- **Egypt:** 14.6% before #322, 37.0% after it, 34.0% on rules 66: still 2.1 times the next faction (Celts, 15.9%). Its tomb stood in 475 of 1,536 seats (666 before #322, 417 after).
- **Below even in all three arms:** Rome (5.8%, 4.7%, 4.0%) and the Mongols (6.8%, 6.4%, 6.0%; 18.7% on rules 50). Switzerland fell to 7.9% (13.3% before #322).
- **Rome's goal:** Rome reached its faction goal in 53 of 1,536 seats (3.5%), the lowest of any faction (next lowest Switzerland, 8.3%), and its Colonia stood in 0 of 1,536 in all three arms.
- **Length and endings (rules 66):** median end turn 245 (mean 252.1, 132 to 500), against 243 on rules 64 and 220 on rules 50. VP hold 94.3%, capitals 5.3%, turn limit 0.3% (7 games), draw 0.1%. 99.4% of games reached the World War era.
