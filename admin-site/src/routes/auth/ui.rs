use crate::ui::primitive::button::Button;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div class="flex flex-col justify-center">
            <Button on:click=move |_e| {
                spawn_local(async {
                    let _ = signin().await;
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
    let token = Resource::new_blocking(
        move || query_map.read().as_ref().ok().map(|e| e.code.clone()),
        |code| translate_callback_code(code.flatten()),
    );
    let (_pending, set_pending) = signal(false);

    view! {
        <div>
            <Transition set_pending>
                {move || { token.get().map(|value| value.is_ok()) }}
            </Transition>
        </div>
    }
}

/// redirects to the SSO login url
#[server]
async fn signin() -> Result<(), ServerFnError> {
    let url = super::flow::oauth_url().await?;
    leptos_axum::redirect(&url);

    Ok(())
}

/// converts the returned code from Oauth SSO to access token and save it
/// in the cookie
#[server]
async fn translate_callback_code(code: Option<String>) -> Result<(), ServerFnError> {
    use axum::http::HeaderValue;
    use leptos_axum::ResponseOptions;
    use reqwest::header;

    let response = expect_context::<ResponseOptions>();

    match super::flow::exchange_code_for_token(code).await? {
        Some(access_token) => {
            let access_token = access_token.secret().to_string();

            // @see: https://github.com/leptos-rs/leptos/discussions/3486
            let header_value =
                format!("access_token={access_token}; Path=/; SameSite=Strict; Secure;");
            let header_value = HeaderValue::from_str(&header_value)?;
            response.insert_header(header::SET_COOKIE, header_value);

            leptos_axum::redirect("/");

            Ok(())
        }
        None => unimplemented!("couldnt exchange to valid access token case"),
    }
}
