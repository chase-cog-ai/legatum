# Wall ring by Walls level needs its own change check (2026-11-25)

The terrain cache (`map/terrain.ts`) redraws a tile only when its packed 32-bit signature changes, and every bit is taken (organic regions use bits 25-31). Drawing the wall ring by Walls level (`cityArt.ts::ringEraOf`) added a value with no room in the signature, so the ring's era is compared on its own against the previous `wallAt` entry. Anything else that changes a town's look must do the same, or an upgrade won't repaint until something else on the tile changes.
