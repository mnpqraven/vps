use crate::ui::primitive::button::{Button, ButtonLook};
use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};

#[component]
pub fn BackButton(
    /// if we need to go back by more than 1 level in a row
    /// e.g a skip_level = 1 would take you from
    /// `/foo/bar/baz/quz` to
    /// `/foo/bar`
    #[prop(optional)]
    extra_skip: Option<usize>,
    /// optional override to a specific path
    #[prop(optional)]
    to: Option<String>,
) -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();
    let on_back = move |_e| {
        if let Some(to) = &to {
            navigate(to, Default::default());
            return;
        }

        let pathname = location.pathname.get();
        let mut chunks: Vec<&str> = pathname.split("/").collect();
        chunks.pop();
        if let Some(skip_level) = extra_skip {
            for _i in 0..skip_level {
                chunks.pop();
            }
        }

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
