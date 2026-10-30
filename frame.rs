// SPDX-License-Identifier: GPL-3.0-only
//
// Once a frame: read the settings the first time, then draw the level at
// their ratio (the window's own when the mod is off in Options > Mods).
fn rescale_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use crate::game::rescale_client as rescale;
    rescale::tick(elapsed.as_secs_f32());
    if !game.native_on(rescale::ID) {
        rescale::off();
        return;
    }
    let settings = rescale::settings(|key| game.native_setting(rescale::ID, key));
    rescale::draw(settings);
}
