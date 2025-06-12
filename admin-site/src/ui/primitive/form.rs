use leptos::{attr::any_attribute::AnyAttribute, prelude::*};
use tailwind_fuse::tw_merge;

fn use_id(field_name: Signal<String>) -> Signal<String> {
    Signal::derive(move || format!("form-input-{}", field_name()))
}

#[component]
pub fn FormInput(
    #[prop(optional, into)] label: Signal<String>,
    #[prop(into)] field: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] attr: Vec<AnyAttribute>,
) -> impl IntoView {
    let id = use_id(field);
    let class = move || tw_merge!("border", class());

    view! {
        <label for=id>{label}</label>
        <input id=id name=field class=class type="text" autocomplete="off" {..attr} />
    }
}

#[component]
pub fn FormCheckbox(
    #[prop(optional, into)] label: Signal<String>,
    #[prop(into)] field: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] attr: Vec<AnyAttribute>,
) -> impl IntoView {
    let id = use_id(field);

    view! {
        <label for=id>{label}</label>
        <input
            id=id
            name=field
            class=class
            type="checkbox"
            autocomplete="off"
            value="true"
            {..attr}
        />
    }
}
