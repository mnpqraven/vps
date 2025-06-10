use crate::ui::back_button::BackButton;
use leptos::prelude::*;

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
    // check if the server has returned an error
    let _has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <ActionForm action>
            <div class="flex flex-col gap-4">
                <label>
                    "Title"<input class="border" type="text" name="title" autocomplete="off" />
                </label>
                <label>
                    "File name"
                    <input class="border" type="text" name="file_name" autocomplete="off" />
                </label>
                <label>
                    "Publish"
                    <input class="border" type="checkbox" name="is_publish" value="true" />
                </label>

                <input type="submit" />
            </div>
        </ActionForm>
    }
}

#[server]
async fn create_blog(
    title: String,
    file_name: String,
    #[server(default)] is_publish: bool,
) -> Result<(), ServerFnError> {
    use crate::state::ctx;
    use proto_types::blog::meta::BlogMetaShape;
    use proto_types::blog::meta::blog_meta_service_client::BlogMetaServiceClient;
    let mut rpc = BlogMetaServiceClient::connect(ctx()?.rpc_url).await?;
    let payload = BlogMetaShape {
        title,
        file_name,
        is_publish,
    };

    rpc.create(payload).await?;

    leptos_axum::redirect("/database/tables/blog");
    Ok(())
}
