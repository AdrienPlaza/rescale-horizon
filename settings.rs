// SPDX-License-Identifier: GPL-3.0-only
//
// Rescale's page in Options > Mods: the mode, the four presets, the custom
// ratio's steps and off.
fn rescale_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::rescale_client as rescale;
    if mod_id != rescale::ID {
        return;
    }
    let settings = rescale::settings(|key| game.native_setting(rescale::ID, key));
    buttons.extend(settings.labels());
}
