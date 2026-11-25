## 2026-11-22: The art masters have a public archive

- **Where:** the 66 game art masters in `tools/art/sources-manifest.json` (unit `*-source.png`, `people-<faction>.png`, the title key art) are public-read at `https://pub-fb5c281dd12d4ea4bbd38d657cdeb907.r2.dev`, laid out by repository path. Set `LEGATUM_ART_SOURCES_URL` to that address for `tools/art/fetch-sources.py`.
- **What is not there:** the manifest's `docs/design` and `docs/playtests` files are kept in a private archive (playtest material stays out of public storage), so `fetch-sources.py --all` reports them as missing against the public address. Fetch game art by path.
- **Closed gap (2026-11-25):** `app/src/assets/art/random-choice.png` was missing from the archive until 2026-11-25, when the original Codex output (found by its manifest sha256 among the Mac's generated images) was uploaded; every game art entry now fetches.
