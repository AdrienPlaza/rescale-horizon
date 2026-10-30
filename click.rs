// SPDX-License-Identifier: GPL-3.0-only
//
// A click on Rescale's settings page: change the mode, draw at once and
// keep it.
fn rescale_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::rescale_client as rescale;
    if mod_id != rescale::ID {
        return;
    }
    let current = rescale::settings(|key| game.native_setting(rescale::ID, key));
    let next = current.click(button);
    rescale::apply(next);
    for (name, value) in next.stored() {
        game.native_set_setting(rescale::ID, name, &value);
    }
}
