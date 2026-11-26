# 3D map idle frames

- The 3D preview can still have visible motion with no input: the water plane covers the whole terrain, but water below unexplored fog and under dry land contributes no animated pixels. Use the frustum and the terrain/fog samples to decide whether to keep scheduling water frames, rather than the existence of the plane.
- The canvas overlay needs its own invalidation: camera and view changes affect both canvases, while late icon/font loads, pickers and route dashes can redraw just the marks. Water and model idle loops don't move those anchors.
- Turn playback moves the 3D units and their marks together. Draw both through the cleanup frame that restores positions and removes projectiles and fallen figures; detached ghost mixers must not keep scheduling frames.
- After a move, easing the facing back to rest needs WebGL frames even without a model mixer or an active turn; it does not move the marks. Render the last straightened pose before going idle.
- Advance the water phase only between consecutive frames with water in view. The first frame after an inland pause resumes from the stored phase rather than adding the idle gap.
- A paused unit mixer keeps its pose without scheduling frames. Enabling motion has to wake the renderer when the model is already loaded.
- After #312, `MapWater` owns the flat plane and its wet, revealed bounds. Tree wind is separate: check the current tree LOD's instance bounds (including shader sway) on view changes and keep animating only while the trees may be visible and motion is enabled. Late tree geometry and shared textures need to request a redraw even in a still view.
- The terrain worker invalidates the scene and marks together and completes its ready promise after that scheduled draw. `MapPost` consumes the current color and depth without temporal state; the offscreen draw and fullscreen finish happen in one scheduled frame, and neither pass needs a follow-up.
