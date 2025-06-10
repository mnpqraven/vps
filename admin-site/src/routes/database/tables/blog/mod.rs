pub mod create;
pub mod update;

use crate::ui::{
    back_button::BackButton,
    primitive::{
        button::{Button, ButtonLook},
        table::{ColumnDefs, Table},
    },
};
use crate::utils::pagination::{PaginationState, use_pagination};
use leptos::prelude::*;
use leptos_router::components::A;
use proto_types::{
    blog::meta::{BlogMeta, BlogMetaList},
    common::db::Pagination,
};

#[component]
pub fn DatabaseTableBlogPage() -> impl IntoView {
    let action = ServerAction::<DeleteBlog>::new();
    provide_context(action);

    let PaginationState {
        pagination: pg,
        prev_params,
        next_params,
    } = use_pagination();
    let (pending, set_pending) = signal(false);

    let async_data = Resource::new(
        move || (pg.get(), action.version().get()),
        |(pg, _)| get_blog_metas(pg),
    );

    let defs = ColumnDefs::<BlogMeta>::new()
        .col("ID", |row| row.id.clone().into_any())
        .col("Title", |row| row.title.clone().into_any())
        .col("Published", |row| row.is_publish.into_any())
        .col("", |row| {
            let id = row.id.clone();
            view! { <TableAction id /> }.into_any()
        });

    let table_view = move || {
        async_data.get().map(|result| {
            let data = result.unwrap().data;
            let column_defs = defs.clone();
            view! { <Table data column_defs /> }
        })
    };

    view! {
        <div class="flex flex-col gap-4 p-4">
            <div class="flex gap-4 items-center">
                <BackButton />
                <A href="/database/tables/blog/create">
                    // TODO: into() conversion
                    <Button class="ml-auto".into()>New</Button>
                </A>
                <Show when=pending>
                    <p>"Loading..."</p>
                </Show>
            </div>

            <div class="flex gap-2 items-center">
                // TODO: query serialize helper fn
                <A href=move || format!("?{}", prev_params.get())>
                    <Button>"prev"</Button>
                </A>
                <span>Page {move || pg.get().page_index + 1}</span>
                <A href=move || format!("?{}", next_params.get())>
                    <Button>"next"</Button>
                </A>
            </div>

            <Transition set_pending fallback=move || view! { <p>"Loading initial..."</p> }>
                {table_view}
            </Transition>
        </div>
    }
}

#[component]
fn TableAction(id: String) -> impl IntoView {
    let action = use_context::<ServerAction<DeleteBlog>>().expect("provided delete action");

    let on_delete = move |_| {
        action.dispatch(id.clone().into());
    };

    view! {
        <div class="flex gap-2">
            <Button look=ButtonLook::Outline.into() on:click=on_delete>
                "Delete"
            </Button>
        </div>
    }
}

#[server]
async fn delete_blog(id: String) -> Result<(), ServerFnError> {
    use crate::state::ctx;
    use proto_types::blog::meta::blog_meta_service_client::BlogMetaServiceClient;
    use proto_types::common::db::Id;

    let mut rpc = BlogMetaServiceClient::connect(ctx()?.rpc_url).await?;
    match rpc.delete(Id { id }).await {
        Ok(_) => Ok(()),
        Err(status) => Err(ServerFnError::new(status.to_string())),
    }
}

#[server]
async fn get_blog_metas(pagination: Pagination) -> Result<BlogMetaList, ServerFnError> {
    use crate::state::ctx;
    use proto_types::blog::meta::blog_meta_service_client::BlogMetaServiceClient;

    let mut rpc = BlogMetaServiceClient::connect(ctx()?.rpc_url).await?;

    let res = rpc
        .list(pagination)
        .await
        .map(|e| e.into_inner())
        .map_err(|status| ServerFnError::new(status.to_string()));
    res
}
