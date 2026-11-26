# Staged terrain must use the replay's frame

The live map can show old chunks during a staged repaint. A replay can seek backward into fog, so it never blits stale chunks. It steps its own cache between draws so background live-map work cannot advance the replay's terrain queue.
