## 2026-11-25: A big team's portraits squeezed its rivals-table name to one letter a line

- **Cause.** The rivals table's empire column is `minmax(0, 1fr)` and its portraits (`.rv-crests`) are `flex: none`, so every member adds about 16 px that the name loses. With 8 members on one team (the crowded map-layer review scene puts every seat on team 0) the portraits took ~134 px of a ~85 px column at 1000x600: "Your team" broke one letter per line (`overflow-wrap: break-word`) and the portraits ran over the VP cell.
- **Fix.** A row shows at most `MAX_CRESTS` (2) portraits; a larger team shows its first portrait and a "+N" roundel whose tooltip names the empires. The name column also has a `min-width` of a short word, so a squeeze wraps between words, never between letters.
