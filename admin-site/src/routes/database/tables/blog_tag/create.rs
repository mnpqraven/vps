use crate::ui::{
    back_button::BackButton,
    primitive::{button::Button, form::FormInput},
};
use leptos::prelude::*;

#[component]
pub fn CreateBlogTagPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <BackButton />
            <MetaForm />
        </div>
    }
}

#[component]
pub fn MetaForm() -> impl IntoView {
    let action = ServerAction::<CreateBlogTag>::new();
    // holds the latest *returned* value from the server
    let value = action.value();
    let error = move || {
        let v = value.get();
        match v {
            Some(Err(err)) => format!("{err}"),
            _ => String::new(),
        }
    };

    view! {
        <ErrorBoundary fallback=move |error| { move || format!("{:?}", error.get()) }>
            // TODO human-readable error return
            <pre>{error}</pre>

            <ActionForm action>
                <div class="flex flex-col gap-4 w-fit items-start">
                    <FormInput label="Code" field="code" />
                    <FormInput label="Label" field="label" />

                    <Button attr:r#type="submit">Create</Button>
                </div>
            </ActionForm>
        </ErrorBoundary>
    }
}

#[server]
async fn create_blog_tag(code: String, label: String) -> Result<(), ServerFnError> {
    use crate::state::ctx;
    use crate::utils::router::RouterKey;
    use proto_types::blog::tag::BlogTagShape;
    use proto_types::blog::tag::blog_tag_service_client::BlogTagServiceClient;
    let mut rpc = BlogTagServiceClient::connect(ctx()?.rpc_url).await?;

    rpc.create(BlogTagShape { code, label }).await?;

    leptos_axum::redirect(RouterKey::DatabaseTablesBlogTag.as_ref());
    Ok(())
}
