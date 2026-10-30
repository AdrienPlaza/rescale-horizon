// SPDX-License-Identifier: GPL-3.0-only
//
// The level's aspect, made the ratio Rescale asks for. This is the Fabric
// mod's GameRendererMixin: vanilla's projection has
// m00 = 1 / (tan(fov / 2) * aspect), and drawing at 4:3 on a 16:9 window
// is exactly what the mod's m00 change does. The hand and the HUD are drawn
// with `camera::hand_projection` and their own projection, which this never
// touches, as in the mod.
fn rescale_aspect(aspect: f32) -> f32 {
    match crate::rescale::target() {
        Some(ratio) => ratio,
        None => aspect,
    }
}
