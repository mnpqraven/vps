use leptos::prelude::*;
use proto_types::{blog::tag::BlogTagList, common::db::Pagination, impls::DefaultState};

#[component]
pub fn DatabaseTablesPage() -> impl IntoView {
    // TODO: dynamic params
    let (pagination, set_pagination) = signal(Pagination::default_state());

    let async_data = Resource::new(move || pagination.get(), load);

    let table_view = move || {
        async_data.get().map(|data| {
            let data = data.unwrap();
            view! { <BlogTagTable data pagination set_pagination /> }
        })
    };

    view! {
        <div class="flex flex-col gap-4 p-4">
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>{table_view}</Suspense>
        </div>
    }
}

#[component]
fn BlogTagTable(
    data: BlogTagList,
    pagination: ReadSignal<Pagination>,
    set_pagination: WriteSignal<Pagination>,
) -> impl IntoView {
    let row_views = data
        .data
        .into_iter()
        .map(|row| {
            view! {
                <tr>
                    <td>{row.id}</td>
                    <td>{row.code}</td>
                    <td>{row.label}</td>
                </tr>
            }
        })
        .collect_view();

    let on_prev = move |_| {
        set_pagination.update(|prev| {
            if prev.page_index >= 1 {
                prev.page_index -= 1
            }
        })
    };
    let on_next = move |_| set_pagination.update(|prev| prev.page_index += 1);

    view! {
        <div class="flex flex-col gap-4">
            <table class="border">
                <tbody>
                    <tr class="border border-b">
                        <th>"ID"</th>
                        <th>"Code"</th>
                        <th>"Label"</th>
                    </tr>
                    {row_views}
                </tbody>
            </table>

            <div class="flex gap-2 items-center">
                <button on:click=on_prev>"prev"</button>
                <span>{move || pagination.get().page_index}</span>
                <button on:click=on_next>"next"</button>
            </div>
        </div>
    }
}

#[server]
async fn load(pagination: Pagination) -> Result<BlogTagList, ServerFnError> {
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
