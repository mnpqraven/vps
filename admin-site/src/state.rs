use leptos::prelude::{use_context, ServerFnError};
use load_env::EnvSchema;

#[derive(Clone, Debug)]
pub struct AppContext {
    pub rpc_url: String,
}

impl AppContext {
    pub fn from_env(env: &EnvSchema) -> Self {
        Self {
            rpc_url: env.rpc.client_url(),
        }
    }
}

/// NOTE: can only be used in server context
pub fn ctx() -> Result<AppContext, ServerFnError> {
    use_context::<AppContext>().ok_or(ServerFnError::new("Bad env"))
}
