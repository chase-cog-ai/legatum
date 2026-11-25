# Siege starves the land: an army at War denies the tile it stands on (2026-11-25)

Rules 60 extends `economy::denied_near` (barbarian units and camps) to any land military unit at War with the tile's owner (`economy::besieges`). Nothing else was needed for the starvation loop: the denied Food flows through `city_yields`, `CityDetail::turns_to_starve` and `rules/cities.rs::process_cities`, so the city panel's existing countdown appears on its own. The denial is war-only (camping on tiles at Peace was a known exploit) and has no order and no lasting effect, so it stays clear of pillaging, which is deliberately not in the game.

The AI needed no change to answer it: `military.rs::defend` already strikes an enemy within 2 tiles of a home city (`siege_sortie_tests`, Normal and Hard).

Balance, 40 paired six-empire all-AI games (seeds 9000 to 9039, 300-turn limit), rules 59 (04bba8fd) vs rules 60: median length 253 vs 258 turns, city captures per game 13.6 vs 14.6, eliminations per game 0.85 vs 1.00, broken VP holds 20 of 58 vs 28 of 66. Games did not get shorter; captures rose about 7% (about 5% per turn played). 40 games is a small sample: read it as "no large speed-up of conquest", not as a measured effect size.

A recorded human rush does get stronger. The archived turn-35 rush (`crates/ai/tests/threat_response.rs`) replayed with the same orders now eliminates Athens and leaves the human 5 cities instead of 4, and the fixed Hard-vs-rivals tape (`hard_vs_human.rs::rival_search_is_the_old_benchmark`) wins at T198 instead of T170. Both are single tapes, pinned to the new outcome. `ranged_garrison`'s replay hash moved too; with `besieges` switched off its old pin passes, so the rule is the cause there.

Test fixture gotchas: `resolve_turn` eliminates an empire with no city (its besiegers vanish), and besiegers with no Gold disband for upkeep; give the besieger a city and Gold.
