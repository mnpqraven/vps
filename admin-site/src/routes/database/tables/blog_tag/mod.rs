use crate::ui::back_button::BackButton;
use crate::ui::loading::Loading;
use crate::ui::primitive::{
    button::{Button, ButtonLook},
    table::{ColumnDefs, Table},
};
use crate::utils::pagination::PaginationButton;
use crate::utils::pagination::{PaginationDirection, PaginationState, use_pagination};
use crate::utils::router::RouterKey;
use leptos::prelude::*;
use leptos_router::components::A;
use proto_types::{
    blog::tag::{BlogTag, BlogTagList},
    common::db::Pagination,
};

pub mod create;

#[component]
pub fn DatabaseTableBlogTagPage() -> impl IntoView {
    let action = ServerAction::<DeleteTag>::new();
    provide_context(action);

    let PaginationState { pagination, .. } = use_pagination();
    let (pending, set_pending) = signal(false);

    let async_data = Resource::new(
        move || (pagination.get(), action.version().get()),
        |(pg, _)| get_blog_tags(pg),
    );

    let column_defs = ColumnDefs::<BlogTag>::new()
        .col("ID", |row| row.id.clone().into_any())
        .col("Code", |row| row.code.clone().into_any())
        .col("Label", |row| row.label.clone().into_any())
        .col("", |row| {
            let id = row.id.clone();
            view! { <TableAction id /> }.into_any()
        });

    let table_view = move || {
        async_data.get().map(|result| {
            let data = result.unwrap().data;
            view! { <Table data column_defs=column_defs.clone() /> }
        })
    };

    view! {
        <div class="flex flex-col gap-4 p-4">
            <div class="flex gap-4 items-center">
                <BackButton />
                <A href=RouterKey::DatabaseTablesBlogTagCreate.as_ref()>
                    <Button class="ml-auto">New</Button>
                </A>
                <Loading when=pending />
            </div>

            <div class="flex gap-2 items-center">
                <PaginationButton pagination direction=PaginationDirection::Prev />
                <span>Page {move || pagination.get().page_index + 1}</span>
                <PaginationButton pagination direction=PaginationDirection::Next />
            </div>

            <Transition set_pending>{table_view}</Transition>
        </div>
    }
}

#[component]
fn TableAction(id: String) -> impl IntoView {
    let action = use_context::<ServerAction<DeleteTag>>().expect("provided delete action");

    let on_delete = move |_| {
        action.dispatch(id.clone().into());
    };

    view! {
        <div class="flex gap-2">
            <Button look=ButtonLook::Outline on:click=on_delete>
                "Delete"
            </Button>
        </div>
    }
}

#[server]
async fn delete_tag(id: String) -> Result<(), ServerFnError> {
    use crate::state::ctx;
    use proto_types::blog::tag::blog_tag_service_client::BlogTagServiceClient;
    use proto_types::common::db::Id;

    let mut rpc = BlogTagServiceClient::connect(ctx()?.rpc_url).await?;
    match rpc.delete(Id { id }).await {
        Ok(_) => Ok(()),
        Err(status) => Err(ServerFnError::new(status.to_string())),
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
