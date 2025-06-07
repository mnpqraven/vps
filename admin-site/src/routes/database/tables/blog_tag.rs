use crate::ui::primitive::{
    button::Button,
    table::{ColumnDefs, Table},
};
use leptos::prelude::*;
use proto_types::{
    blog::tag::{BlogTag, BlogTagList},
    common::db::Pagination,
    impls::DefaultState,
};

#[component]
pub fn DatabaseTableBlogTagPage() -> impl IntoView {
    // TODO: dynamic params
    let (pagination, set_pagination) = signal(Pagination::default_state());
    let (pending, set_pending) = signal(false);

    let async_data = Resource::new(move || pagination.get(), get_blog_tags);

    let defs = ColumnDefs::<BlogTag>::new()
        .col("ID", |row| row.id.clone().into_any())
        .col("Code", |row| row.code.clone().into_any())
        .col("Label", |row| row.label.clone().into_any());

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
            <p>{move || if pending.get() { "Hang on..." } else { "Ready." }}</p>
            <Transition set_pending fallback=move || view! { <p>"Loading initial..."</p> }>
                {table_view}
            </Transition>

            <div class="flex gap-2 items-center">
                <Button on:click=on_prev>"prev"</Button>
                <span>{move || pagination.get().page_index}</span>
                <Button on:click=on_next>"next"</Button>
            </div>
        </div>
    }
}

#[server]
async fn get_blog_tags(pagination: Pagination) -> Result<BlogTagList, ServerFnError> {
    use crate::state::ctx;
    use proto_types::blog::tag::blog_tag_service_client::BlogTagServiceClient;

    let mut rpc = BlogTagServiceClient::connect(ctx()?.rpc_url).await?;

    let res = rpc
        .list(pagination)
        .await
        .map(|e| e.into_inner())
        .map_err(|status| ServerFnError::new(status.to_string()));
    res
}
