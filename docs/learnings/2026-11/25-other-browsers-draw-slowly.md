## 2026-11-25: Safari and Firefox draw the map 10 to 20 times slower; we tune for Chrome only

- **Seen in:** 36 players' games of 30 turns or more uploaded on 2026-11-24/25. Fifteen report no long tasks at
  all, which means a browser without long-task timing (Safari, Firefox). Nine of those have a per-turn map draw
  mean of 7 to 42 ms; Chrome-family games sit at 0.7 to 2.6 ms. The worst (185 turns) had 15% of frames over 50 ms.
- **Not the cause:** the JS blur fallback for browsers without canvas filters. Chromium with `?blur=js` on a
  turn-40 save draws no slower than with the filter.
- **Decision (user, 2026-11-25):** solve performance for Chrome and browsers built on it, and tell the others.
  The title screen footer says "Runs best in Chrome" (`app/src/platform/browser.ts`); on an iPhone or iPad,
  where every browser is Safari underneath, it says "Runs best in Chrome on a computer".
- **Reading telemetry:** a game whose `long_task_ms` is 0 on every turn is not a smooth game, it is a browser
  that cannot count long tasks. Judge those by `slow_frames` and `map_draw_mean_ms10`, and do not mix them into
  Chrome numbers.
