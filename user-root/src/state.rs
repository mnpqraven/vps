use leptos::prelude::{ServerFnError, use_context};
use load_env::{EnvSchema, schema::EnvSchemaRpc};

#[derive(Clone, Debug)]
pub struct AppContext {
    pub rpc_env: EnvSchemaRpc,
}

impl AppContext {
    pub fn from_env(env: &EnvSchema) -> Self {
        Self {
            rpc_env: env.rpc.clone(),
        }
    }
}

/// NOTE: can only be used in server context
pub fn ctx() -> Result<AppContext, ServerFnError> {
    use_context::<AppContext>().ok_or(ServerFnError::new("Bad env"))
}
