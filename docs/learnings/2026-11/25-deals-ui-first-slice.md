## 2026-11-25: Deals UI, first slice (Gold in offers, Give Gold to rivals, Open Borders)

- Cause: core lists every `ProposeTreaty` action with `gold: 0`, and the app matched a queued order to its action by
  JSON (`sameDiplomaticAction`), so an offer queued with Gold no longer matched its button (no "Queued", no Undo).
  Fix: a `ProposeTreaty` matches on `to` and `kind` (one offer per pair anyway). `endsOffers` counted any
  `BreakTreaty` as ending the pair's offers; closing Open Borders does not change the relation, so it now ends none.
- The app's passage mirror (`diplomacy.ts::canEnter`, behind routes, reach and the unit card) has its own list of open
  lands; a new core passage rule must be added there too or routes refuse moves core allows.
- An Open Borders badge in the empire row's badge line truncated the empire's name at 1000x600 and 1440x900; it now
  sits under the name.
- Operational: `cd X && (A) & (B) &` in one shell line runs B in the shell's original directory, not X (the `&&`
  binds only to the first group). An `npm ci` meant for one worktree reinstalled another worktree's `node_modules`.
  Put `cd` inside each backgrounded group.
