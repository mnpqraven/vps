pub mod create;
pub mod update;

use leptos::prelude::*;
use leptos_router::components::A;
use proto_types::{
    blog::meta::{BlogMeta, BlogMetaList},
    common::db::Pagination,
    impls::DefaultState,
};

use crate::ui::{
    back_button::BackButton,
    primitive::{
        button::Button,
        table::{ColumnDefs, Table},
    },
};

#[component]
pub fn DatabaseTableBlogPage() -> impl IntoView {
    // TODO: dynamic params
    let (pagination, set_pagination) = signal(Pagination::default_state());
    let (pending, set_pending) = signal(false);

    let async_data = Resource::new(move || pagination.get(), get_blog_metas);

    let defs = ColumnDefs::<BlogMeta>::new()
        .col("ID", |row| row.id.clone().into_any())
        .col("Title", |row| row.title.clone().into_any())
        .col("Published", |row| row.is_publish.into_any());

    let table_view = move || {
        async_data.get().map(|result| {
            let req = result.unwrap();
            let defs = defs.clone();
            view! { <Table data=req.data column_defs=defs /> }
        })
    };

    let on_prev = move |_| {
        set_pagination.update(|prev| {
            if prev.page_index >= 1 {
                prev.page_index -= 1
            }
        })
    };
    let on_next = move |_| set_pagination.update(|prev| prev.page_index += 1);

    view! {
        <div class="flex flex-col gap-4 p-4">
            <div class="flex gap-4 items-center">
                <BackButton />
                <Show when=pending>
                    <p>"Loading..."</p>
                </Show>
                <A href="/database/tables/blog/create">
                    // TODO: into() conversion
                    <Button class="ml-auto".into()>New</Button>
                </A>
            </div>

            <div class="flex gap-2 items-center">
                <Button on:click=on_prev>"prev"</Button>
                <span>Page {move || pagination.get().page_index + 1}</span>
                <Button on:click=on_next>"next"</Button>
            </div>

            <Transition set_pending fallback=move || view! { <p>"Loading initial..."</p> }>
                {table_view}
            </Transition>
        </div>
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
