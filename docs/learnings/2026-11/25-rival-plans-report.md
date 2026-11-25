# 2026-11-25: the post-game "Rival plans" tab reads the brains' own logs

- The AI already keeps what the tab needs: `OpponentMemory` (saved as `ai_opponent_memory`) holds each
  seat's plan switches, campaign starts and ends, and threat reactions, all bounded. `Game::ai_timeline`
  only reshapes them, so the report shows what a rival decided, not a motive guessed from its orders.
- It is refused until `state.finished`: the logs are hidden information while the game is live.
- A campaign target is often razed by the end of the game, and `state.cities` then has no name for it.
  The session now keeps the names of cities that stop existing (`razed_city_names` in the save, absent
  when empty), so old saves load unchanged and say "a city since razed".
- Default-difficulty seats write few plan events; a fixture that shows the tab needs Hard seats and a
  real opponent in the human seat (`write_rival_plans_fixture` plays it with the Hard script).
- GameOver tests that picked the report's tabs by position broke when a tab was added; pick by `data-tab`.
