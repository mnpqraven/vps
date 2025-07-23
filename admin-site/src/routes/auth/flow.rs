#[cfg(feature = "ssr")]
pub(super) async fn dev() -> Result<(), Box<dyn std::error::Error>> {
    use load_env::schema::EnvFrontend;
    use oauth2::basic::BasicClient;
    use oauth2::reqwest;
    use oauth2::{
        AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
        RedirectUrl, Scope, TokenResponse, TokenUrl,
    };
    use tracing::info;

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
            "https://github.com/login/oauth/token".to_string(),
        )?)
        // Set the URL the user will be redirected to after the authorization process.
        .set_redirect_uri(RedirectUrl::new(callback_url.to_string())?);

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("read".to_string()))
        .add_scope(Scope::new("write".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    println!("Browse to: {}", auth_url);

    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_token`.

    let http_client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    // Now you can trade it for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new(
            "some authorization code".to_string(),
        ))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await?;

    // Unwrapping token_result will either produce a Token or a RequestTokenError.
    info!("{token_result:?}");

    Ok(())
}
