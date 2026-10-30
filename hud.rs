// SPDX-License-Identifier: GPL-3.0-only
//
// The ratio, top left, for a moment after the key or the settings page
// changed it.
fn rescale_hud(game: &mut crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    use crate::game::rescale_client as rescale;
    if !game.native_on(rescale::ID) {
        return;
    }
    if let Some(text) = rescale::toast() {
        game.native_text(batch, &text, [6.0, 6.0], 0xFFFFFF, true);
    }
}
