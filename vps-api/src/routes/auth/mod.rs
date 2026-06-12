use std::time::Duration;

use axum::{
    Extension,
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar};
use chrono::Local;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse, reqwest, url::Url};
use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
}
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
            dbg!(&token, secret);

            let week_as_secs = 86_400 * 7;
            let max_age = Local::now().naive_local() + Duration::from_secs(week_as_secs);

            let cookie = Cookie::build(("sid", token.access_token().secret().to_owned()))
                .domain(".app.localhost")
                .path("/")
                .secure(true)
                .http_only(true)
                .max_age(time::Duration::seconds(week_as_secs as i64));

            // TODO:
            //         // add to session table
            //         let db = database::get_db().await?;

            //         sqlx::query(
            //             "INSERT INTO oauth_user (email) VALUES ($1) ON CONFLICT (email) DO NOTHING",
            //         )
            //         .bind(profile.email.clone())
            //         .execute(&state.db)
            //         .await?;

            //         sqlx::query(
            //             "INSERT INTO session (user_id, session_id, expires_at) VALUES (
            // (SELECT ID FROM USERS WHERE email = $1 LIMIT 1),
            //  $2, $3)
            // ON CONFLICT (user_id) DO UPDATE SET
            // session_id = excluded.session_id,
            // expires_at = excluded.expires_at",
            //         )
            //         .bind(profile.email)
            //         .bind(token.access_token().secret().to_owned())
            //         .bind(max_age)
            //         .execute(&state.db)
            //         .await?;

            Ok((jar.add(cookie), Redirect::to("/protected")))
        }
        Err(err) => Err(ApiError::ParseError(err.to_string())),
    }
}
