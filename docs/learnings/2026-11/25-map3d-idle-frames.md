# 3D map idle frames

- The 3D preview can still have visible motion with no input: the water plane covers the whole terrain, but water below unexplored fog and under dry land contributes no animated pixels. Use the frustum and the terrain/fog samples to decide whether to keep scheduling water frames, rather than the existence of the plane.
- Camera, viewport and view changes move the HTML city banners. The water clock and late unit textures don't, so those redraws can reuse the existing banner positions.
