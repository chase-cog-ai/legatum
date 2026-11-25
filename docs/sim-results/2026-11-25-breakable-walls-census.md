# Breakable Walls (rules 62): faction census and duels, before and after (2026-11-25)

Published as measured, no tuning. Spec: Civ-style Walls (a one-time Walls bar of 100, +50 per level, that never regrows; repaired or rebuilt only with Production, never under attack).

## Setup

- BASE = master 9e8d09fd (rules 61). WALLS = the breakable-Walls branch (rules 62), same seeds.
- Census: 8 empires, all Hard, medium map, `legatum-sim --players 8 --difficulty hard --rotate-factions --factions <roster>` over the 11 cyclic rosters of Switzerland, India, Byzantium, Germany, Celts, Mongols, Sparta, Athens, Aztecs, Egypt, Rome; 24 games per roster, seed base 9311003 + 100000 per roster. 264 games per arm (192 seats per faction), a quarter of the PR 137 census, run on a 4-core container. 0 panics.
- Duels: `--players 2 --difficulty hard`, 64 games from seed 1 and 128 from seed 1001, 192 per arm.

## Win rate per seat

At 192 seats a faction's 95% interval is about ±5 points, so no single faction's change below is significant.

| Faction | BASE | WALLS | Change |
| --- | --- | --- | --- |
| Egypt | 19.8% | 18.8% | −1.0 pp |
| Germany | 16.1% | 16.1% | 0.0 |
| Mongols | 10.9% | 15.6% | +4.7 |
| Athens | 15.1% | 14.6% | −0.5 |
| Sparta | 10.4% | 13.0% | +2.6 |
| Aztecs | 14.6% | 12.0% | −2.6 |
| Switzerland | 9.9% | 11.5% | +1.6 |
| India | 10.4% | 10.9% | +0.5 |
| Rome | 7.3% | 8.9% | +1.6 |
| Celts | 12.5% | 8.3% | −4.2 |
| Byzantium | 10.4% | 7.8% | −2.6 |

Best/worst: 2.71 → 2.40. Pillar 8's 1.5x is still not met.

## Pace, endings and conquest (census)

| Metric | BASE | WALLS |
| --- | --- | --- |
| End turn, median (mean) | 220.5 (226.2) | 227 (234.6) |
| Ended by VP hold / capitals | 262 / 2 | 258 / 6 |
| Stalled (safety cap 600) | 0 | 0 |
| City captures per game | 22.29 | 24.87 |
| Captures in early wars per game | 2.50 | 2.61 |

The early rush is not harder (level I keeps the same total: a 100 bar at half damage plus 200 health equals the old 200 health at half damage). Conquest overall rises about 12%: a broken wall stays broken.

## Two-player Hard duels (192 games per arm)

| | BASE | WALLS |
| --- | --- | --- |
| Stalls (safety cap 1,200) | 4 | 4 |
| End turn, median (mean) | 198 (229.6) | 189.5 (223.9) |
| Endings | capitals 147, VP 38, elimination 3, stalled 4 | capitals 166, VP 20, elimination 2, stalled 4 |
| City captures per game | 4.97 | 4.86 |

Raw JSON not committed (`/tmp` on the run container).
