# Supply sold by the turn: what kept it from being a way to farm the AI (2026-11-25)

Rules 61 adds per-turn supply trades (`crates/core/src/supply_trade.rs`, AI in `crates/ai/src/supply_trade.rs`).

- **A trade is a change to `resources::supplied`, not a transfer.** Supply is not a stockpile, so the trade is a
  standing record that both sides' supply totals read. Everything that already asks "is there spare supply?"
  (builds, upgrades, idle users, the AI's supply value) then works for traded supply with no new code.
- **Check spare supply twice.** The seller's spare is checked when the offer is made and again at acceptance, in
  seat order, so two buyers accepting in one turn cannot both get the same supply.
- **War cuts the trade before combat, not at end of turn.** `end_broken` runs right after surprise wars are
  declared and re-runs `update_supply`, so an attacker cannot buy a rival's Iron, declare a surprise war and fight
  that turn with the rival's supply.
- **The AI never values a trade by Gold alone.** A human can always offer more Gold than supply is worth to arm
  for a war on the seller. The AI refuses an army larger than its own, its diplomatic target and a team near
  victory at any price, keeps one supply spare, and ends the sale the turn the buyer outgrows it.
- **The AI buys only for users that are idle now.** A human cannot sell it supply it has no use for as a Gold
  drain: the bid limit is zero without an idle user, capped at twice the fair price and a quarter of income.
- **Test fixtures need contact.** `rules::cities::tests::test_game` has no diplomacy state (everyone is at War);
  build `DiplomacyState::new` and `meet_all` before testing anything that needs Peace.
