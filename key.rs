// SPDX-License-Identifier: GPL-3.0-only
//
// The mod's key: cycle 16:9, 16:10, 4:3, 5:4, custom, off. Read only while
// playing, as the mod's keybind was.
fn rescale_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::rescale_client as rescale;
    if !pressed || !game.native_on(rescale::ID) || !game.native_playing() {
        return;
    }
    let bound = game.native_mod_key(rescale::ID, rescale::KEY, rescale::DEFAULT_KEY);
    if key != bound || !rescale::press_allowed(key) {
        return;
    }
    let current = rescale::settings(|name| game.native_setting(rescale::ID, name));
    let next = current.next();
    rescale::apply(next);
    for (name, value) in next.stored() {
        game.native_set_setting(rescale::ID, name, &value);
    }
}
