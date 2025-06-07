use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Card(#[prop(optional)] class: Signal<String>, children: Children) -> impl IntoView {
    let class = ArcMemo::new(move |_| {
        tw_merge!(
            class.get(),
            "bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm"
        )
    });
    view! {
        <div data-slot="card" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(#[prop(optional)] class: Signal<String>, children: Children) -> impl IntoView {
    let class =  tw_merge!(
        class.get(),
        "@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6");
    view! {
        <div data-slot="card-header" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(#[prop(optional)] class: String, children: Children) -> impl IntoView {
    let class = tw_merge!(class, "leading-none font-semibold");
    view! {
        <div data-slot="card-title" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardDescription(
    #[prop(optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = ArcMemo::new(move |_| tw_merge!(class.get(), "text-muted-foreground text-sm"));
    view! {
        <div data-slot="card-description" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardContent(#[prop(optional)] class: Signal<String>, children: Children) -> impl IntoView {
    let class = ArcMemo::new(move |_| tw_merge!(class.get(), "p-6"));
    view! {
        <div data-slot="card-content" class=class>
            {children()}
        </div>
    }
}
