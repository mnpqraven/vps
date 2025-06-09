use crate::utils::hooks::use_theme::{use_theme, ColorMode};
use leptos::prelude::*;
use strum::IntoEnumIterator;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(class = "cursor-pointer")]
struct Variant {
    active: VariantActive,
}

#[derive(TwVariant)]
enum VariantActive {
    #[tw(class = "font-bold")]
    True,
    #[tw(default, class = "")]
    False,
}
impl From<bool> for VariantActive {
    fn from(value: bool) -> Self {
        match value {
            true => VariantActive::True,
            _ => VariantActive::False,
        }
    }
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (theme, set_theme) = use_theme();

    let toggle_mode = move |mode: ColorMode| {
        set_theme.set(Some(mode));
    };

    let mode_views = ColorMode::iter()
        .map(|color| {
            let class = ArcMemo::new(move |_| {
                let var = Variant {
                    active: (theme.get() == Some(color)).into(),
                };
                var.to_class()
            });
            view! {
                <span class=class on:click=move |_| toggle_mode(color)>
                    {color.to_string()}
                </span>
            }
        })
        .collect_view();

    view! { <div class="flex gap-1">{mode_views}</div> }
}
