# Say "health", never "HP" (2026-11-25)

A first-time player watching the attack forecast did not know what "HP" meant. The forecast switched to "health" in PR 157; this change sweeps every other screen (unit and city panels, tile card, siege alerts, Air wing, turn report, Guide, faction sheets, tech and building cards) and the two core strings a player sees (the "Siege against city health" breakdown row and the Roman founder refusal).

- The copy lint now bans `\bHP\b`, `HEALTH` and "hit points" in rendered text and in source string literals (`app/src/test/copyRules.ts`), so the source scan over `crates/core` also checks test and assert messages. They were changed too rather than excepted: cheaper than an exception list, and no assert matches on them.
- The old rule was the opposite ("city health" was banned, the glossary said "city HP"). The glossary in `legatum-ui` now has "health" and "city health" as the canonical terms.
- Code identifiers (`hp`, `CITY_HP`, `HpMark`, CSS classes such as `.hp-delta`) keep their names: only player text changed.
- A longer word can break a tight row. The siege card's `minmax(0, 1fr) auto` grid squeezed "120 /200 health" under "−110 health last turn" in the 1000x600 sidebar with a unit selected. The fix was a wrapping flex row with nowrap parts, and dropping the repeated "health" from the loss text. Check renames at the narrowest width a string shows.
