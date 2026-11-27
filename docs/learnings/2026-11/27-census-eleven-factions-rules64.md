# Eleven-faction census on rules 64: best/worst 19.6, Egypt 37.0% and India 1.9%

2026-11-27. Published as measured, no tuning, no rule or AI change. Reports: `docs/sim-results/2026-11-27-census-eleven-factions-rules64.md` (master `025325a4`) and `docs/sim-results/2026-11-27-census-eleven-factions-rules64-before-322.md` (master `5f98e07d`).

- **Why:** the last full-size census was rules 50 (2026-11-22, ratio 9.1, Egypt over Rome), and the city-upkeep check of 2026-11-26 ran on the runner's default pool, which has no Egypt or Rome.
- **Runner:** `legatum-sim --all-factions` seats the census design of 2026-11-22 in one run: the 11 cyclic rosters of 8 over `Faction::ALL`, one roster per 8 games, turned one seat per game. `--players` must be 8 and `--games` a multiple of 88; then every faction plays every seat position equally often. Off by default: the nine-faction seeded draw, `--factions` rosters and pinned tapes are unchanged. Before this the design took 11 runs with hand-written `--factions` rosters and `--rotate-factions`.
- **Run:** `cargo run --release -p legatum-sim -- --players 8 --difficulty hard --all-factions --games 2112 --seed 9311003 --label census-eleven-factions-rules64`. 2,112 games, seeds 9311003 to 9313114, 1,536 seats per faction, 0 panics, 0 stalls, about 31 minutes on 8 cores. Held-out seeds were not run. The seeds are not the per-roster seeds of the rules 50 census, so compare rates with it, not games.
- **Two arms, same seeds.** Current is master `025325a4`. Before is master `5f98e07d` with this change's two `crates/sim` commits cherry-picked (the report records that local commit, `d46062d3`, which is on no branch). Between the two, #322 (`egypt_peace_army`) is the only commit that changes what the AI or the rules do; #321 adds a read-only forecast field. The before arm was run twice and every results section of the two reports was identical.
- **Win rate per seat (95% Wilson interval for current; even is 12.5%):**

| Faction | Seats | Wins | Current | 95% interval | Before #322 | Change | Rules 50 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Egypt | 1,536 | 568 | 37.0% | 34.6 to 39.4 | 14.6% | +22.4 pp | 21.4% |
| Celts | 1,536 | 257 | 16.7% | 14.9 to 18.7 | 18.9% | −2.1 | 14.1% |
| Germany | 1,536 | 208 | 13.5% | 11.9 to 15.3 | 16.0% | −2.5 | 14.3% |
| Athens | 1,536 | 194 | 12.6% | 11.1 to 14.4 | 17.6% | −4.9 | 14.5% |
| Byzantium | 1,536 | 193 | 12.6% | 11.0 to 14.3 | 14.1% | −1.5 | 9.0% |
| Sparta | 1,536 | 189 | 12.3% | 10.8 to 14.0 | 15.5% | −3.2 | 12.0% |
| Aztecs | 1,536 | 159 | 10.4% | 8.9 to 12.0 | 12.6% | −2.3 | 12.8% |
| Switzerland | 1,536 | 144 | 9.4% | 8.0 to 10.9 | 13.3% | −4.0 | 10.0% |
| Mongols | 1,536 | 99 | 6.4% | 5.3 to 7.8 | 6.8% | −0.4 | 18.7% |
| Rome | 1,536 | 72 | 4.7% | 3.7 to 5.9 | 5.8% | −1.1 | 2.3% |
| India | 1,536 | 29 | 1.9% | 1.3 to 2.7 | 2.1% | −0.3 | 8.3% |

- **Ratio:** best/worst 19.6 (Egypt over India; interval 12.8 to 30.3, best's lower bound over worst's upper bound and the reverse). Pillar 8's 1.5x is not met. Before #322 it was 8.8 (Celts over India).
- **Egypt:** 14.6% to 37.0% on identical seeds, the same direction as #322's own six-seat check (14.7% to 53.3%). Every other faction's rate fell. Egypt's tomb stood in fewer seats (666 to 417 of 1,536).
- **Below even in both arms:** India (2.1%, 1.9%; the nine-faction city-upkeep check read 1.1%, so India's fall is not an artefact of that pool), Rome (5.8%, 4.7%) and the Mongols (6.8%, 6.4%; 18.7% on rules 50). Without Egypt and India the current spread is 3.6 (Celts over Rome).
- **Rome's goal:** Rome reached its faction goal in 55 of 1,536 seats (3.6%), the lowest of any faction (next lowest Switzerland, 10.9%), and its Colonia stood in 0 of 1,536 in both arms.
- **Length and endings (current):** median end turn 243 (mean 250.3, 137 to 500), against 220 on rules 50. VP hold 95.1%, capitals 4.5%, turn limit 0.4% (9 games). 99.6% of games reached the World War era.
