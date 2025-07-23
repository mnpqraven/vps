use crate::ui::primitive::button::Button;
use leptos::prelude::*;
use leptos::server::codee::string::FromToStringCodec;
use leptos::task::spawn_local;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;
use leptos_use::use_cookie;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div class="flex flex-col justify-center">
            <Button on:click=move |_e| {
                spawn_local(async {
                    signin().await.unwrap();
                });
            }>Sign in with Github</Button>
        </div>
    }
}

#[derive(Clone, Params, PartialEq)]
struct OauthCallbackFromProviderQuery {
    code: Option<String>,
    state: Option<String>,
}

#[component]
pub fn AuthCallbackPage() -> impl IntoView {
    let query_map = use_query::<OauthCallbackFromProviderQuery>();
    let token = Resource::new(
        move || query_map.read().as_ref().ok().map(|e| e.code.clone()),
        |code| process_external_returns(code.flatten()),
    );
    let (_pending, set_pending) = signal(false);

    view! {
        <div>
            <Transition set_pending>
                {move || { token.get().map(|value| format!("{value:?}")) }}
            </Transition>
        </div>
    }
}

#[server]
async fn signin() -> Result<(), ServerFnError> {
    let url = super::flow::oauth_url().await.unwrap();
    leptos_axum::redirect(&url);

    Ok(())
}

#[server]
async fn process_external_returns(code: Option<String>) -> Result<(), ServerFnError> {
    match super::flow::exchange_code_for_token(code).await.unwrap() {
        Some(access_token) => {
            let (_, set) = use_cookie::<String, FromToStringCodec>("access_token");
            set(access_token.secret());
            Ok(())
        }
        None => unimplemented!("couldnt exchange to valid access token case"),
    }
}
