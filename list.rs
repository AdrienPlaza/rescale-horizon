// SPDX-License-Identifier: GPL-3.0-only
//
// The mod's row in Options > Mods, with its key.
fn rescale_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::rescale_client as rescale;
    mods.push(crate::game::native_hooks::NativeMod {
        id: rescale::ID.into(),
        title: format!("Rescale {}", rescale::VERSION),
        description: rescale::DESCRIPTION.into(),
        keys: vec![[
            rescale::KEY.into(),
            "Cycle the aspect ratio".into(),
            rescale::DEFAULT_KEY.into(),
        ]],
    });
}
