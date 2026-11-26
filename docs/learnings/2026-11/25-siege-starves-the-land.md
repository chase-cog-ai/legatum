# Siege starves the land: an army at War denies the tile it stands on (2026-11-25)

Rules 64 extends `economy::denied_near` (barbarian units and camps) to any land military unit at War with the tile's owner (`economy::besieges`). Nothing else was needed for the starvation loop: the denied Food flows through `city_yields`, `CityDetail::turns_to_starve` and `rules/cities.rs::process_cities`, so the city panel's existing countdown appears on its own. The denial is war-only (camping on tiles at Peace was a known exploit) and has no order and no lasting effect, so it stays clear of pillaging, which is deliberately not in the game.

The AI needed no change to answer it: `military.rs::defend` already strikes an enemy within 2 tiles of a home city (`siege_sortie_tests`, Normal and Hard).

Balance, 200 paired six-empire all-AI games (seeds 9000 to 9199, 300-turn limit, scripted brain), master without the rule (1f97d0e6, rules 62, breakable Walls in) vs with it (3091026d): median length 251 vs 250 turns, city captures per game 16.2 vs 16.5 (paired difference +0.25, standard error 0.61), eliminations per game 0.96 vs 1.07 (+0.11, standard error 0.08), broken VP holds 86 of 264 vs 90 of 266. None of the differences is outside noise. An earlier 40-game run on rules 59 (before Deals and breakable Walls) read 253 vs 258 turns, 13.6 vs 14.6 captures and 0.85 vs 1.00 eliminations: the same direction, and no larger once Walls have a bar.

A recorded human rush does get stronger. The archived turn-35 rush (`crates/ai/tests/threat_response.rs`) replayed with the same orders now eliminates Athens and leaves the human 5 cities instead of 4. The fixed Hard-vs-rivals tape (`hard_vs_human.rs::plain_hard_rivals_are_the_default_benchmark`) wins at T170 instead of T162; `rival_search_is_the_old_benchmark` is unchanged. Both are single tapes, pinned to the new outcome. `ranged_garrison`'s replay hash moved too; with `besieges` switched off its old pin passes, so the rule is the cause there.

Test fixture gotchas: `resolve_turn` eliminates an empire with no city (its besiegers vanish), and besiegers with no Gold disband for upkeep; give the besieger a city and Gold.
