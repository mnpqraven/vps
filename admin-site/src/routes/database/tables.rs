use leptos::prelude::*;
use proto_types::{blog::tag::BlogTagList, common::db::Pagination, impls::DefaultState};

#[component]
pub fn DatabaseTablesPage() -> impl IntoView {
    // TODO: dynamic params
    let (pagination, _set_pagination) = signal(Pagination::default_state());

    let async_data = Resource::new(move || pagination.get(), load);

    view! {
        <div class="flex flex-col gap-4">
            <span>"hello DatabaseTablesPage (ssr)"</span>
            <Suspense fallback=move || view! { <p>"suspensing"</p> }>
                <div>
                    {move || Suspend::new(async move {
                        let t = async_data.await;
                        let formatted = format!("{t:?}");
                        view! { <p>{formatted}</p> }
                    })}
                </div>
            </Suspense>
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
