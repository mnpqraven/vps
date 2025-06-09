use leptos::prelude::*;

use crate::ui::back_button::BackButton;

#[component]
pub fn CreateBlogPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <BackButton />
        // TODO: form
        </div>
    }
}
