use std::time::SystemTime;

use crate::{
    ui::{
        back_button::BackButton,
        primitive::{button::Button, form::FormInput},
    },
    utils::FormMode,
};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use proto_types::blog::tag::BlogTag;

#[component]
pub fn BlogTagFormPage() -> impl IntoView {
    let params = use_params_map();
    let default_value = Resource::new(
        move || params.read().get("id"),
        |id| async move { get_blog_tag(id).await.ok().flatten() },
    );
    let mode = Signal::derive(move || match params.read().get("id") {
        Some(_id) => FormMode::Update,
        None => FormMode::Create,
    });
    let (_pending, set_pending) = signal(false);

    view! {
        <div class="flex flex-col gap-4">
            <BackButton extra_skip=1 />
            <Transition set_pending>
                {move || {
                    default_value
                        .get()
                        .map(|default_value| {
                            view! { <MetaForm default_value mode /> }
                        })
                }}
            </Transition>
        </div>
    }
}

#[component]
pub fn MetaForm(
    #[prop(into)] mode: Signal<FormMode>,
    default_value: Option<BlogTag>,
) -> impl IntoView {
    let form_action = ServerAction::<BlogTagFormAction>::new();
    // holds the latest *returned* value from the server
    let value = form_action.value();
    let error = Signal::derive(move || match value.get() {
        Some(Err(err)) => format!("{err}"),
        _ => String::new(),
    });
    let mode_str = Signal::derive(move || mode.get().to_string());

    view! {
        <ErrorBoundary fallback=move |error| { move || format!("{:?}", error.get()) }>
            // TODO human-readable error return
            <pre>{error}</pre>

            <ActionForm action=form_action>
                <div class="flex flex-col gap-4 w-fit items-start">
                    <FormInput
                        label="Code"
                        field="code"
                        {..}
                        value=default_value.as_ref().map(|e| e.code.clone())
                    />
                    <FormInput
                        label="Label"
                        field="label"
                        {..}
                        value=default_value.as_ref().map(|e| e.label.clone())
                    />

                    // phantom divs
                    <input class="hidden" name="mode" value=mode_str />
                    <input class="hidden" name="id" value=default_value.map(|e| e.id) />

                    <Button attr:r#type="submit">{mode_str}</Button>
                </div>
            </ActionForm>
        </ErrorBoundary>
    }
}

#[server]
async fn blog_tag_form_action(
    mode: FormMode,
    id: Option<String>,
    code: String,
    label: String,
) -> Result<(), ServerFnError> {
    use crate::state::ctx;
    use crate::utils::router::RouterKey;
    use proto_types::blog::tag::BlogTagShape;
    use proto_types::blog::tag::blog_tag_service_client::BlogTagServiceClient;
    use proto_types::common::db::Id;

    let mut rpc = BlogTagServiceClient::connect(ctx()?.rpc_url).await?;

    match mode {
        FormMode::Create => {
            rpc.create(BlogTagShape { code, label }).await?;
        }
        FormMode::Update => {
            let id = id.expect("should always have id provided in update fn");
            let prev = rpc.get_by_id(Id { id }).await?.into_inner();
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let next = BlogTag {
                code,
                label,
                updated_at: now as i64,
                ..prev
            };
            rpc.update(next).await?;
        }
    }

    leptos_axum::redirect(RouterKey::DatabaseTablesBlogTag.as_ref());
    Ok(())
}

#[server]
async fn get_blog_tag(id: Option<String>) -> Result<Option<BlogTag>, ServerFnError> {
    use crate::state::ctx;
    use proto_types::blog::tag::blog_tag_service_client::BlogTagServiceClient;
    use proto_types::common::db::Id;

    if let Some(id) = id {
        let mut rpc = BlogTagServiceClient::connect(ctx()?.rpc_url).await?;
        let res = rpc.get_by_id(Id { id }).await?.into_inner();
        return Ok(Some(res));
    }
    Ok(None)
}
