use crate::ui::primitive::{input::Input, textarea::Textarea};
use leptos::prelude::*;

fn use_id(field: Signal<String>) -> Signal<String> {
    Signal::derive(move || format!("form-input-{}", field()))
}

#[component]
pub fn FormInput(
    #[prop(optional, into)] label: Signal<String>,
    #[prop(into)] field: Signal<String>,
) -> impl IntoView {
    let id = use_id(field);

    view! {
        <label for=id>{label}</label>
        <Input {..} id=id name=field type="text" autocomplete="off" />
    }
}

#[component]
pub fn FormTextarea(
    #[prop(optional, into)] label: Signal<String>,
    #[prop(into)] field: Signal<String>,
) -> impl IntoView {
    let id = use_id(field);

    view! {
        <label for=id>{label}</label>
        <Textarea {..} id=id name=field type="text" autocomplete="off" />
    }
}

#[component]
pub fn FormCheckbox(
    #[prop(optional, into)] label: Signal<String>,
    #[prop(into)] field: Signal<String>,
) -> impl IntoView {
    let id = use_id(field);

    view! {
        <label for=id>{label}</label>
        <input id=id name=field type="checkbox" autocomplete="off" value="true" />
    }
}
