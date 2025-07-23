use leptos::prelude::ServerFnError;
use leptos::server;
use oauth2::basic::{BasicClient, BasicErrorResponseType, BasicTokenType};
use oauth2::*;
use tracing::info;

#[cfg(feature = "ssr")]
pub(super) fn http_client_for_token() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build")
}

#[cfg(feature = "ssr")]
pub(super) async fn oauth_url() -> Result<String, Box<dyn std::error::Error>> {
    let client = github_client()?;

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read".to_string()))
        .url();

    Ok(auth_url.to_string())
}

type GithubClientType = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

#[cfg(feature = "ssr")]
pub fn github_client() -> Result<GithubClientType, Box<dyn std::error::Error>> {
    use load_env::schema::EnvFrontend;

    let dot = load_env::EnvSchema::load();
    if dot.is_err() {
        panic!("bad env");
    }
    let dot = dot.unwrap();
    let dot = dot.frontend.get("admin-site");
    if dot.is_none() {
        panic!("bad env");
    }
    let dot = dot.unwrap();
    let EnvFrontend {
        client_id,
        client_secret,
        callback_url,
    } = dot;

    // Create an OAuth2 client by specifying the client ID, client secret,
    // authorization URL and token URL.
    let client = BasicClient::new(ClientId::new(client_id.to_string()))
        .set_client_secret(ClientSecret::new(client_secret.to_string()))
        .set_auth_uri(AuthUrl::new(
            "https://github.com/login/oauth/authorize".to_string(),
        )?)
        .set_token_uri(TokenUrl::new(
            "https://github.com/login/oauth/access_token".to_string(),
        )?)
        // Set the URL the user will be redirected to after the authorization process.
        .set_redirect_uri(RedirectUrl::new(callback_url.to_string())?);
    Ok(client)
}

#[server]
pub(super) async fn exchange_code_for_token(
    code: Option<String>,
) -> Result<Option<AccessToken>, ServerFnError> {
    use super::flow::github_client;
    use oauth2::AuthorizationCode;

    if let Some(code) = code {
        // Once the user has been redirected to the redirect URL, you'll have access to the
        // authorization code. For security reasons, your code should verify that the `state`
        // parameter returned by the server matches `csrf_token`.

        // Now you can trade it for an access token.
        let token_result = github_client()
            .unwrap()
            .exchange_code(AuthorizationCode::new(code))
            .request_async(&http_client_for_token())
            .await?;
        let access_token = token_result.access_token();
        return Ok(Some(access_token.clone()));
    }

    Ok(None)
}
