use leptos::prelude::*;
use tailwind_fuse::*;

#[component]
pub fn Textarea(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let class = move || {
        tw_merge!(
            "flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:font-medium file:text-sm placeholder:text-muted-foreground focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
            class()
        )
    };
    view! { <textarea class=class /> }
}
