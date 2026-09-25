use crate::ui::theme_toggle::ThemeToggle;
use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[component]
pub fn NavBar() -> impl IntoView {
    let location = use_location();

    view! {
        <div class="flex justify-between py-2 px-4 border-b">
            {location.pathname} <ThemeToggle />
        </div>
    }
}
