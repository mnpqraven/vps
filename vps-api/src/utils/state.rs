use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    key: Key,
    /// basic rewest client to handle external api calls, e.g. github profiles etc.
    pub outbound_api_client: Client,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            key: Key::generate(),
            outbound_api_client: Client::builder()
                .user_agent("OTHI")
                .build()
                // TODO:
                .unwrap(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}
