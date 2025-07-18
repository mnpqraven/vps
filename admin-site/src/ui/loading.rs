use leptos::prelude::*;

#[component]
pub fn Loading<W>(when: W) -> impl IntoView
where
    W: Fn() -> bool + Send + Sync + 'static,
{
    view! {
        <Show when>
            <p>"Loading..."</p>
        </Show>
    }
}
