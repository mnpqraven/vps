use crate::app::ColorMode;
use leptos::prelude::*;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let mode = use_context::<RwSignal<ColorMode>>()
        .expect("ColorMode signal should be provided in context");

    let toggle_mode = move |_| {
        mode.update(|current| {
            *current = match current {
                ColorMode::Light => ColorMode::Dark,
                ColorMode::Dark => ColorMode::Light,
            }
        });
    };

    view! {
        <button class="theme-toggle" on:click=toggle_mode>
            {move || match mode.get() {
                ColorMode::Light => "Color mode: Light",
                ColorMode::Dark => "Color mode: Dark",
            }}
        </button>
    }
}
