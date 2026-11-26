# 3D map idle frames

- The 3D preview can still have visible motion with no input: the water plane covers the whole terrain, but water below unexplored fog and under dry land contributes no animated pixels. Use the frustum and the terrain/fog samples to decide whether to keep scheduling water frames, rather than the existence of the plane.
- Camera, viewport and view changes move the HTML city banners. The water clock and late unit textures don't, so those redraws can reuse the existing banner positions.
- Advance the water phase only between consecutive frames with water in view. The first frame after an inland pause resumes from the stored phase rather than adding the idle gap.
- A paused unit mixer keeps its pose without scheduling frames. Enabling motion has to wake the renderer when the model is already loaded.
