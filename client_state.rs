// SPDX-License-Identifier: GPL-3.0-only
//
// Rescale's settings: the mode the key cycles and the settings page sets.
// Kept in the WebAssembly mods' files (`horizon-mods/.horizon-mod-data/
// rescale/mode`, `custom`), read once and then held. The ratio itself lives
// in horizon-client-render's `rescale` module, which the projection hook
// reads; every change here pushes it there.
//
// The cycle starts at 4:3, not at 16:9: on the 16:9 window almost everyone
// has, drawing at 16:9 is the window's own ratio, so a first press that
// landed there changed nothing on screen and looked like a mod that does not
// work. 4:3 is the ratio people install this for, and it always shows.
use std::cell::RefCell;

use horizon_client_render::rescale::{MAX_RATIO, MIN_RATIO, set_target};

/// The mod's id: its settings folder and its key's line.
pub const ID: &str = "rescale";
pub const VERSION: &str = "0.2.0";
pub const DESCRIPTION: &str = "Stretched resolution: draw the world at 4:3, 5:4, 16:10, 16:9 \
or your own ratio, while the hand and the HUD stay normal. A native port of the Rescale mod \
by TCD1234.";
/// The key's name, and its default.
pub const KEY: &str = "cycle";
pub const DEFAULT_KEY: &str = "key.keyboard.f6";

/// The ratios the key cycles through: (label, width / height). 4:3 first, so
/// the first press from off always changes what is on screen.
pub const PRESETS: [(&str, f32); 4] = [
    ("4:3", 4.0 / 3.0),
    ("5:4", 5.0 / 4.0),
    ("16:10", 16.0 / 10.0),
    ("16:9", 16.0 / 9.0),
];

/// The window's own, and the custom ratio's, mode numbers.
const OFF: u8 = 0;
const CUSTOM: u8 = 5;
/// A custom step, and how long the ratio stays on the HUD after a change.
const CUSTOM_STEP: f32 = 0.05;
const TOAST_SECONDS: f32 = 2.5;
/// How long a key press is ignored after the same key already fired: input
/// paths that report one press twice count once, and a human press is never
/// that close to the previous one.
const KEY_DEBOUNCE: f32 = 0.06;
/// What a custom ratio starts at, as the mod's own default preset.
const DEFAULT_RATIO: f32 = 16.0 / 9.0;

/// The settings, as the mod keeps them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// `OFF`, 1 to 4 for a preset, or `CUSTOM`.
    pub mode: u8,
    /// The custom ratio, the width over the height.
    pub custom: f32,
}

impl Settings {
    /// From the stored values (`get` reads one key's file); off at 16:9
    /// when they are missing.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let mode = get("mode")
            .and_then(|value| value.trim().parse::<u8>().ok())
            .filter(|mode| *mode <= CUSTOM)
            .unwrap_or(OFF);
        let custom = get("custom")
            .and_then(|value| value.trim().parse::<f32>().ok())
            .filter(|ratio| ratio.is_finite())
            .unwrap_or(DEFAULT_RATIO)
            .clamp(MIN_RATIO, MAX_RATIO);
        Self { mode, custom }
    }

    /// The ratio to draw at, `None` for the window's own.
    pub fn ratio(&self) -> Option<f32> {
        match self.mode {
            OFF => None,
            CUSTOM => Some(self.custom.clamp(MIN_RATIO, MAX_RATIO)),
            mode => Some(PRESETS[usize::from(mode - 1)].1),
        }
    }

    /// The mode as the HUD and the settings page write it.
    pub fn label(&self) -> String {
        match self.mode {
            OFF => "off".to_owned(),
            CUSTOM => format!("custom {:.2}", self.custom),
            mode => {
                let (name, ratio) = PRESETS[usize::from(mode - 1)];
                format!("{name} ({ratio:.2})")
            }
        }
    }

    /// The next mode the key cycles to: 4:3, 5:4, 16:10, 16:9, custom, off.
    pub fn next(&self) -> Self {
        let mode = if self.mode >= CUSTOM {
            OFF
        } else {
            self.mode + 1
        };
        Self { mode, ..*self }
    }

    /// The settings page's buttons, in the order `click` reads them.
    pub fn labels(&self) -> Vec<String> {
        let mut labels = vec![format!("Now: {}", self.label())];
        for (name, ratio) in PRESETS {
            labels.push(format!("Draw at {name} ({ratio:.2})"));
        }
        labels.push(format!("Use custom {:.2}", self.custom));
        labels.push("Custom -0.05".to_owned());
        labels.push("Custom +0.05".to_owned());
        labels.push("Off".to_owned());
        labels
    }

    /// A click on button `i` of `labels`.
    pub fn click(&self, i: usize) -> Self {
        let mut next = *self;
        match i {
            0 | 8 => next.mode = OFF,
            1..=4 => next.mode = i as u8,
            5 => next.mode = CUSTOM,
            6 => {
                next.custom = (next.custom - CUSTOM_STEP).clamp(MIN_RATIO, MAX_RATIO);
                next.mode = CUSTOM;
            }
            7 => {
                next.custom = (next.custom + CUSTOM_STEP).clamp(MIN_RATIO, MAX_RATIO);
                next.mode = CUSTOM;
            }
            _ => {}
        }
        next
    }

    /// What to store, key by key.
    pub fn stored(&self) -> [(&'static str, String); 2] {
        [
            ("mode", self.mode.to_string()),
            ("custom", format!("{:.4}", self.custom)),
        ]
    }
}

#[derive(Default)]
struct State {
    /// Read on the first frame or settings page, then kept.
    settings: Option<Settings>,
    /// Seconds since the first frame, for the HUD.
    clock: f32,
    /// The ratio to show on the HUD, and when it was set.
    toast: Option<(f32, String)>,
    /// The last key press that fired, and when.
    last_key: Option<(f32, String)>,
    /// The one-time "Rescale is loaded" line was shown this session.
    announced: bool,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// The settings, read once through `get`.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        if let Some(settings) = state.settings {
            return settings;
        }
        let settings = Settings::read(get);
        state.settings = Some(settings);
        settings
    })
}

/// Keeps `settings` and draws the level at their ratio, without a HUD line:
/// the frame re-applies the settings it already read.
pub fn draw(settings: Settings) {
    set_target(settings.ratio());
    STATE.with(|cell| cell.borrow_mut().settings = Some(settings));
}

/// Draws the level at the window's own ratio: the mod is off.
pub fn off() {
    set_target(None);
}

/// A change the player made: keeps it, and shows it on the HUD for a moment.
pub fn apply(settings: Settings) {
    set_target(settings.ratio());
    STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        let clock = state.clock;
        state.settings = Some(settings);
        state.toast = Some((clock, format!("Rescale: {}", settings.label())));
    });
}

/// One frame's time, for the HUD.
pub fn tick(elapsed: f32) {
    STATE.with(|cell| cell.borrow_mut().clock += elapsed);
}

/// Whether this press is the first of `key` in a while: a press reported
/// twice by the input path counts once.
pub fn press_allowed(key: &str) -> bool {
    STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        match &state.last_key {
            Some((at, last)) if last == key && state.clock - *at < KEY_DEBOUNCE => return false,
            _ => {}
        }
        let clock = state.clock;
        state.last_key = Some((clock, key.to_owned()));
        true
    })
}

/// Whether this session still has to say that the mod is loaded. True once:
/// the frame hook then writes a line in chat, which is the quickest way for
/// a player to see that the build really contains the mod.
pub fn announce_once() -> bool {
    STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        if state.announced {
            return false;
        }
        state.announced = true;
        true
    })
}

/// The ratio to show for a moment after a change, if it is still time.
pub fn toast() -> Option<String> {
    STATE.with(|cell| {
        let state = cell.borrow();
        let (at, text) = state.toast.as_ref()?;
        (state.clock - *at < TOAST_SECONDS).then(|| text.clone())
    })
}
