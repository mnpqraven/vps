use crate::ui::primitive::button::Button;
use leptos::prelude::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    let action = ServerAction::<Signin>::new();

    view! {
        <div class="flex flex-col justify-center">
            <Button on:click=move |_e| {
                action.dispatch_local(Signin {});
            }>Sign in with Github</Button>
        </div>
    }
}

#[server]
async fn signin() -> Result<(), ServerFnError> {
    let res = super::flow::dev().await;

    Ok(())
}
