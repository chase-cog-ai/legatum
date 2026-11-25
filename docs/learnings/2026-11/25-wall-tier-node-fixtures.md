## 2026-11-25: Node fixtures must not import wall artwork

- The Playwright fixture engine imports the siege module directly in Node. Importing the wall-tier helper from `map/cityArt.ts` brought in Vite's `import.meta.glob` and failed before any browser could open.
- Keep the wall-tier helper in `combat.ts`, which the siege fixture already imports. The renderer and combat projections can share the same visibility check without bringing artwork into the fixture engine.
