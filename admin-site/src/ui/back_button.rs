use crate::ui::primitive::button::{Button, ButtonLook};
use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};

#[component]
pub fn BackButton() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();
    let on_back = move |_e| {
        let pathname = location.pathname.get();
        let mut chunks: Vec<&str> = pathname.split("/").collect();
        // see if we need conditional guard
        chunks.pop();

        let mut next = chunks.join("/");
        if next.is_empty() {
            next.push('/');
        }
        navigate(&next, Default::default())
    };

    view! {
        <Button on:click=on_back look=ButtonLook::Secondary>
            "Back"
        </Button>
    }
}
