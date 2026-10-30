// SPDX-License-Identifier: GPL-3.0-only
//
// Rescale: the ratio the level is drawn at, shared by both crates. The
// client's hooks set it (Options > Mods, the key, the first frame) and the
// projection hook in gfx/view.rs reads it. An atomic, so the read costs
// nothing on the render thread and no lock is ever taken in a frame.
use std::sync::atomic::{AtomicU32, Ordering};

/// The ratio, as `f32` bits; 0 is "the window's own", which no ratio equals
/// (they are clamped to at least `MIN_RATIO`).
static RATIO: AtomicU32 = AtomicU32::new(0);

/// The smallest and largest ratio a player can set.
pub const MIN_RATIO: f32 = 0.5;
pub const MAX_RATIO: f32 = 3.0;

/// The ratio the level is drawn at, `None` for the window's own.
pub fn target() -> Option<f32> {
    let bits = RATIO.load(Ordering::Relaxed);
    if bits == 0 {
        None
    } else {
        Some(f32::from_bits(bits))
    }
}

/// Draws the level at `ratio`, `None` for the window's own.
pub fn set_target(ratio: Option<f32>) {
    let bits = match ratio {
        Some(ratio) => ratio.clamp(MIN_RATIO, MAX_RATIO).to_bits(),
        None => 0,
    };
    RATIO.store(bits, Ordering::Relaxed);
}
