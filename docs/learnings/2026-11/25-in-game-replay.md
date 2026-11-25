## 2026-11-25: In-game replay uses the order tape, not the game report

- The game report's history has totals and a chronicle, but not a historical map. The owner recording has the effective orders and a hash after every turn. Run those orders through the engine without running the AI.
- Display the order tape's resolved turn, not the next planning turn in the reconstructed state. A partial recording starts before its first recorded turn. Neither a partial tape nor a rewind recovers turns that aren't in the tape.
- The local replay builds bounded checkpoints in a worker after the player opens it. Its map and turning points use the player's sight; it never reads an online guest's connection or the live order controls.
