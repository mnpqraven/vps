use ::reqwest::header::SET_COOKIE;
use axum::{
    Extension,
    extract::{Query, State},
    http::HeaderMap,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar};
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse, reqwest, url::Url};
use proto_types::service::{LoginRequest, auth_service_client::AuthServiceClient};
use serde::{Deserialize, Serialize};
use tonic::IntoRequest;
use vps_rpc::RPC_URL;

use crate::{
    middlewares::github::GithubClient,
    utils::{error::ApiError, state::AppState},
};

pub async fn login(Extension(oauth_client): Extension<GithubClient>) -> Redirect {
    let authorize_url = make_authorize_url(&oauth_client);

    Redirect::to(authorize_url.as_str())
}

fn make_authorize_url(client: &GithubClient) -> Url {
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the user's public repos and email.
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    authorize_url
}

#[derive(Serialize, Deserialize, Debug)]
struct GithubProfile {
    login: Option<String>,
    id: u32,
    node_id: Option<String>,
    avatar_url: Option<String>,
    gravatar_id: Option<String>,
    url: Option<String>,
    followers_url: Option<String>,
    following_url: Option<String>,
    gists_url: Option<String>,
    starred_url: Option<String>,
    repos_url: Option<String>,
    events_url: Option<String>,
    received_events_url: Option<String>,
    name: Option<String>,
    email: Option<String>,
    bio: Option<String>,
    public_repos: i32,
    public_gists: i32,
    followers: i32,
    following: i32,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    // TODO: callback logic
}

/// unresolved logic
/// create_at setters
/// session checks (2nd login onwards) + rejections checks
/// callback_url logic
/// cookie setter
/// SET_COOKIE header not working
pub async fn github_callback(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    Query(query): Query<AuthRequest>,
    Extension(oauth_client): Extension<GithubClient>,
    Extension(http_client): Extension<reqwest::Client>,
) -> Result<impl IntoResponse, ApiError> {
    let token_res = oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(&http_client)
        .await;

    match token_res {
        Ok(token) => {
            let secret = token.access_token().secret();
            // TODO:
            let profile_res = state
                .outbound_api_client
                .get("https://api.github.com/user")
                .bearer_auth(secret.to_owned())
                .send()
                .await
                // TODO:
                .unwrap();
            let profile = profile_res.json::<GithubProfile>().await.unwrap();

            let week_as_secs = 86_400 * 7;

            let cookie = Cookie::build(("sid", secret.to_owned()))
                .domain("localhost:5020")
                .path("/")
                .secure(true)
                .http_only(true)
                .max_age(time::Duration::seconds(week_as_secs as i64));

            let mut client = AuthServiceClient::connect(RPC_URL).await?;
            let login_req = LoginRequest {
                user_email: profile.email.unwrap_or_default(),
                access_token: secret.to_owned(),
                max_age: week_as_secs as i64,
            };
            // TODO: unwrap
            let login_res = client
                .login(login_req.into_request())
                .await
                .unwrap()
                .into_inner()
                .success;

            let mut headers = HeaderMap::new();

            // Attach the session cookie to the response header
            let d_cookie = format!("SESSION={cookie}; SameSite=Lax; HttpOnly; Secure; Path=/");
            headers.insert(SET_COOKIE, d_cookie.parse().unwrap());

            // TODO:
            Ok((
                // jar.add(cookie),
                headers,
                // TODO: interpolation
                Redirect::to("http://localhost:5020/swagger"),
            ))
        }
        Err(err) => Err(ApiError::ParseError(err.to_string())),
    }
}
