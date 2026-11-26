# 3D map idle frames

- The 3D preview can still have visible motion with no input: the water plane covers the whole terrain, but water below unexplored fog and under dry land contributes no animated pixels. Use the frustum and the terrain/fog samples to decide whether to keep scheduling water frames, rather than the existence of the plane.
- The canvas overlay needs its own invalidation: camera and view changes affect both canvases, while late icon/font loads, pickers and route dashes can redraw just the marks. Water and model idle loops don't move those anchors.
- Turn playback moves the 3D units and their marks together. Draw both through the cleanup frame that restores positions and removes projectiles and fallen figures; detached ghost mixers must not keep scheduling frames.
- Advance the water phase only between consecutive frames with water in view. The first frame after an inland pause resumes from the stored phase rather than adding the idle gap.
- A paused unit mixer keeps its pose without scheduling frames. Enabling motion has to wake the renderer when the model is already loaded.
