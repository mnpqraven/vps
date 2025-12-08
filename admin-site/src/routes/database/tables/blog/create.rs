use crate::{
    routes::database::tables::blog_tag::get_blog_tags,
    ui::{
        back_button::BackButton,
        primitive::{
            button::Button,
            form::{FormCheckbox, FormInput, FormTextarea},
        },
    },
    utils::FormMode,
};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use proto_types::{blog::root::Blog, common::db::ProtoPagination};

#[component]
pub fn BlogFormPage() -> impl IntoView {
    let params = use_params_map();
    let default_value = Resource::new(
        move || params.read().get("id"),
        |id| async move { get_blog(id).await.ok().flatten() },
    );
    let mode = Signal::derive(move || match params.read().get("id") {
        Some(_id) => FormMode::Update,
        None => FormMode::Create,
    });
    let extra_skip = Signal::derive(move || params.read().get("id").map(|_| 1_usize));
    let (_pending, set_pending) = signal(false);

    view! {
        <div class="flex flex-col gap-4">
            <BackButton extra_skip />
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
    #[prop(into)] default_value: Signal<Option<Blog>>,
) -> impl IntoView {
    let action = ServerAction::<CreateBlog>::new();
    // holds the latest *returned* value from the server
    let value = action.value();
    let error = move || {
        let v = value.get();
        match v {
            Some(Err(err)) => format!("{err}"),
            _ => String::new(),
        }
    };
    let mode_str = Signal::derive(move || mode.get().to_string());

    view! {
        <ErrorBoundary fallback=move |error| { move || format!("{:?}", error.get()) }>
            // TODO: human-readable error return
            // hide if there's no error
            <pre>{error}</pre>

            <ActionForm action>
                <div class="flex flex-col gap-4 w-fit items-start">
                    <FormInput
                        label="Title"
                        field="title"
                        {..}
                        value=default_value.get().map(|e| e.meta.clone().map(|f| f.title))
                    />
                    <TagsSelector default_value />
                    <FormTextarea
                        label="Content"
                        field="content"
                        default_value=default_value.get().map(|e| e.content.clone())
                    />
                    <FormCheckbox
                        label="Publish"
                        field="is_publish"
                        {..}
                        checked=default_value.get().map(|e| e.meta.clone().map(|f| f.is_publish))
                    />

                    // phantom
                    <input class="hidden" name="mode" value=mode_str />
                    <input
                        class="hidden"
                        name="id"
                        value=default_value.get().and_then(|e| e.meta.map(|f| f.id))
                    />

                    <Button attr:r#type="submit">{mode_str}</Button>
                </div>
            </ActionForm>
        </ErrorBoundary>
    }
}

#[server]
async fn create_blog(
    title: String,
    content: String,
    #[server(default)] tag_ids: Vec<String>,
    #[server(default)] is_publish: bool,
) -> Result<(), ServerFnError> {
    use crate::state::ctx;
    use crate::utils::router::RouterKey;
    use proto_types::blog::meta::BlogMetaShape;
    use proto_types::blog::root::{BlogShape, blog_service_client::BlogServiceClient};

    let mut rpc = BlogServiceClient::connect(ctx()?.rpc_url).await?;

    let payload = BlogShape {
        meta_shape: Some(BlogMetaShape {
            title: title.clone(),
            file_name: hyphen_filename(&title),
            is_publish,
        }),
        tag_ids, // TODO:
        file_content: content,
    };

    rpc.create(payload).await?;
    // TODO: media upload here

    leptos_axum::redirect(RouterKey::DatabaseTablesBlog.as_ref());

    Ok(())
}

#[server]
async fn get_blog(id: Option<String>) -> Result<Option<Blog>, ServerFnError> {
    use crate::state::ctx;
    use proto_types::blog::root::blog_service_client::BlogServiceClient;
    use proto_types::common::db::Id;

    if let Some(id) = id {
        let mut rpc = BlogServiceClient::connect(ctx()?.rpc_url).await?;
        let res = rpc.detail(Id { id }).await?.into_inner();
        leptos::logging::log!("{res:?}");
        return Ok(Some(res));
    }
    Ok(None)
}

#[component]
fn TagsSelector(#[prop(into)] default_value: Signal<Option<Blog>>) -> impl IntoView {
    let async_data = Resource::new(
        move || (),
        |_| {
            get_blog_tags(ProtoPagination {
                page_index: None,
                page_size: None,
                search: None,
                all: Some(true),
            })
        },
    );

    let tag_checkbox_views = move || {
        async_data.get().map(|result| {
            // TODO: unwrap
            let tags = result.unwrap().data;
            tags.into_iter()
                .enumerate()
                .map(|(i, tag)| {
                    let name = format!("tag_ids[{i}]");
                    view! {
                        <div>
                            <input
                                type="checkbox"
                                name=name
                                value=tag.id.clone()
                                // TODO: optimization
                                // probably can optimize this by omitting the clone somewhere up the tree
                                checked=move || {
                                    let ids: Vec<String> = default_value
                                        .get()
                                        .map(|e| e.tags)
                                        .unwrap_or_default()
                                        .iter()
                                        .map(|f| f.id.clone())
                                        .collect();
                                    ids.contains(&tag.id)
                                }
                            />
                            <span>{tag.label}</span>
                        </div>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <fieldset>
            <legend>"Tags"</legend>
            <Transition>{tag_checkbox_views}</Transition>
        </fieldset>
    }
}

fn hyphen_filename(filename: &str) -> String {
    use std::time::SystemTime;

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let slug = filename.to_lowercase().replace(" ", "-");
    format!("{now}_{slug}.md")
}
