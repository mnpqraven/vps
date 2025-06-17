use crate::{
    routes::database::tables::blog_tag::get_blog_tags,
    ui::{
        back_button::BackButton,
        primitive::{
            button::Button,
            form::{FormCheckbox, FormInput, FormTextarea},
        },
    },
};
use leptos::prelude::*;
use proto_types::common::db::Pagination;

#[component]
pub fn CreateBlogPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <BackButton />
            <MetaForm />
        </div>
    }
}

#[component]
pub fn MetaForm() -> impl IntoView {
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

    view! {
        <ErrorBoundary fallback=move |error| { move || format!("{:?}", error.get()) }>
            // TODO: human-readable error return
            <pre>{error}</pre>

            <ActionForm action>
                // TODO: tag selector
                <div class="flex flex-col gap-4 w-fit items-start">
                    <FormInput label="Title" field="title" />
                    <MultiCheckbox />
                    <FormTextarea label="Content" field="content" />
                    <FormCheckbox label="Publish" field="is_publish" />

                    <Button attr:r#type="submit">Create</Button>
                </div>
            </ActionForm>
        </ErrorBoundary>
    }
}

#[server]
async fn create_blog(
    title: String,
    content: String,
    tag_ids: Vec<String>,
    #[server(default)] is_publish: bool,
) -> Result<(), ServerFnError> {
    use crate::state::ctx;
    use crate::utils::router::RouterKey;
    use proto_types::blog::meta::BlogMetaShape;
    use proto_types::blog::root::{BlogShape, blog_service_client::BlogServiceClient};

    let mut rpc = BlogServiceClient::connect(ctx()?.rpc_url).await?;

    // TODO: get filename from title (hypenized)
    let file_name = String::from("frontend_placeholder.md");

    let payload: BlogShape = BlogShape {
        meta_shape: Some(BlogMetaShape {
            title,
            file_name,
            is_publish,
        }),
        tag_ids, // TODO:
        file_content: content,
    };

    leptos::logging::log!("{payload:?}");
    tracing::info!("{payload:?}");

    // let _ = rpc.create(payload).await?;

    leptos_axum::redirect(RouterKey::DatabaseTablesBlog.as_ref());
    Ok(())
}

// TODO: serde error
#[component]
fn MultiCheckbox() -> impl IntoView {
    let async_data = Resource::new(
        move || (),
        |_| {
            get_blog_tags(Pagination {
                page_index: 0,
                page_size: 1000,
                search: String::new(),
            })
        },
    );

    let suspend_views = move || {
        async_data.get().map(|result| {
            let tags = result.unwrap().data;
            tags.into_iter()
                .map(|tag| {
                    view! {
                        <div>
                            <input type="checkbox" name="tag_ids" value=tag.id />
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
            <Transition>{suspend_views}</Transition>
        </fieldset>
    }
}
