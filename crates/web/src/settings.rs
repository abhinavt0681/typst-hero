//! Per-browser preferences (localStorage), e.g. Vim mode.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Vim keybindings in the editor.
    pub vim: bool,
    /// Relative line numbers (Vim's `relativenumber`).
    pub relative_numbers: bool,
}

#[derive(Clone, Copy)]
pub struct SettingsStore(RwSignal<Settings>);

impl SettingsStore {
    pub fn vim(&self) -> Signal<bool> {
        let s = self.0;
        Signal::derive(move || s.with(|s| s.vim))
    }

    pub fn relative_numbers(&self) -> Signal<bool> {
        let s = self.0;
        Signal::derive(move || s.with(|s| s.relative_numbers))
    }

    pub fn update(&self, f: impl FnOnce(&mut Settings)) {
        self.0.update(f);
        #[cfg(feature = "hydrate")]
        storage::save(&self.0.get_untracked());
    }
}

/// Call once at the top of the app. Defaults on the server and during
/// hydration; saved settings are applied right after.
pub fn provide_settings() {
    let settings = RwSignal::new(Settings::default());
    provide_context(SettingsStore(settings));
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        if let Some(saved) = storage::load() {
            settings.set(saved);
        }
    });
}

pub fn use_settings() -> SettingsStore {
    expect_context::<SettingsStore>()
}

#[cfg(feature = "hydrate")]
mod storage {
    use super::Settings;

    const KEY: &str = "typst-hero/settings/v1";

    fn local() -> Option<web_sys::Storage> {
        web_sys::window()?.local_storage().ok()?
    }

    pub fn load() -> Option<Settings> {
        serde_json::from_str(&local()?.get_item(KEY).ok()??).ok()
    }

    pub fn save(settings: &Settings) {
        if let (Some(storage), Ok(json)) = (local(), serde_json::to_string(settings)) {
            let _ = storage.set_item(KEY, &json);
        }
    }
}
