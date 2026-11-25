# First contact plate needs no engine event (2026-11-25)

- The core records first contact (`DiplomacyState::contacts`) but emits no event for it. The app finds a new meeting by
  comparing `DiplomacyView.met` between the turn's previous and new view (`celebrate/moments.ts::meetingsForTurn`).
  This keeps the engine, saves and tapes unchanged, so games in flight are not touched, and a loaded game or a
  replayed card (no previous view) never shows a meeting again.
- `MomentLane` used `index: 3` to mean "story over" because at most three cards played. Meetings are never folded into
  "+N more", so a turn can now play more than three cards: the end marker is `END` (`Number.MAX_SAFE_INTEGER`).
- Faction sheet lines are written to the people's own player ("Your units ..."). A rival's plate turns them to the
  third person with `asTheirs` rather than keeping a second wording; a test checks every sheet line.
- Playwright in this repo pins a newer browser build than the cloud image ships; run the tour with
  `launchOptions.executablePath: '/opt/pw-browsers/chromium'` (a local config file, not committed).

**Mid-game fixtures meet strangers on their first turn.** Most e2e fixtures (researchFixture, siegeFixture,
wallsFixture, captureOrdersFixture, warPromptFixture) leave `diplomacy.contacts` empty, so the first resolved turn
meets every AI in sight and the held first-contact plate covers whatever the test waits for (24 tests in 8 specs).
`e2e/fixtures.ts` closes the plate by default (`meetings: 'close'`); a spec about first contact sets
`test.use({ meetings: 'show' })`.
