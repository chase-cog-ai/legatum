## 2026-11-24: Hovering the map with a founder selected sent the whole view to the engine per pointer move

- **Symptom:** after the turn-131 terrain fix (#85) a 132-turn Rome game still logged about 10 long tasks per turn
  of roughly 70 ms from turn 110 on, on a map of only 49 units and 30 cities.
- **Cause:** `MapCanvas` asked `foundCityBlocker(view, hoveredTile)` on every render while a unit that can found
  a city was selected. For Rome that is every melee unit. Each call did `JSON.stringify(view)` (over 1 MB late),
  copied it into WASM and parsed the whole `PlayerView` there to answer one tile: 50 to 150 ms per pointer move.
  Replaying the player's tape to turn 115 in headless Chromium with a profiler showed 72 long tasks in 20 s of
  hovering with a melee unit selected, none idle, none panning.
- **Fix:** `Game.site_blockers(view)` answers every tile in one call; `siteBlocker()` keeps the table in a
  `WeakMap` keyed on the view object. One parse per view at most, then array lookups.
- **Rule:** a call that takes the view as JSON costs a full view parse. Never make one per pointer move, per
  tile or per candidate; ask once per view and keep the answer with the view. Views are replaced, never edited
  in place, so a `WeakMap<PlayerView, …>` is a safe cache (a test that edits a view in place must clone it).
- **Telemetry:** `frame_times` rows now carry `long_draw_ms`, `long_terrain_ms` and `long_engine_ms`, so the next
  stall of this kind names itself as engine calls without a replay.
- **Method that found it:** replay the uploaded tape to a late turn at the game's engine commit
  (`crates/wasm/examples`-style loader), load the save in a headless build, and profile one player action per
  phase (idle, select, hover, city, pan, End Turn). The long-task count per phase points at the action.
