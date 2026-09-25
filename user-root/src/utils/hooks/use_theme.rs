use leptos::prelude::*;
use leptos::server::codee::string::FromToStringCodec;
use leptos_use::use_cookie;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};

#[derive(
    Clone,
    Copy,
    Default,
    Serialize,
    Deserialize,
    EnumString,
    strum::Display,
    EnumIter,
    PartialEq,
    Debug,
)]
#[strum(serialize_all = "lowercase")]
pub enum ColorMode {
    Light,
    Dark,
    #[default]
    System,
}

impl ColorMode {
    pub fn next(&self) -> Self {
        match self {
            ColorMode::Light => ColorMode::Dark,
            ColorMode::Dark => ColorMode::System,
            ColorMode::System => ColorMode::Light,
        }
    }
}

pub fn use_theme() -> (Signal<Option<ColorMode>>, WriteSignal<Option<ColorMode>>) {
    let (theme, set_theme) = use_cookie::<ColorMode, FromToStringCodec>("color_mode");

    (theme, set_theme)
}

#[cfg(test)]
mod tests {
    use super::ColorMode;

    #[test]
    fn correct_theme_cookie_name() {
        let mode = ColorMode::Light;
        let mode_str = mode.to_string();
        assert_eq!(mode_str, "light");
    }
}
