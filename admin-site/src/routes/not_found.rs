use crate::ui::primitive::button::Button;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-6">
            <h1>"Page not found."</h1>
            <A href="/">
                <Button>"Go to home"</Button>
            </A>
        </div>
    }
}
