use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;

#[derive(Clone)]
pub struct AppState {
    key: Key,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            key: Key::generate(),
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
