use leptos::prelude::*;
use proto_types::{blog::tag::BlogTagList, common::db::Pagination};

#[component]
pub fn DatabaseTablesPage() -> impl IntoView {
    // TODO: dynamic params
    let (pagination, _set_pagination) = signal(Pagination {
        page_index: 0,
        page_size: 15,
    });

    let async_data = Resource::new(
        // TODO: unwrap
        move || pagination.get(),
        load,
    );

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
    use proto_types::blog::tag::blog_tag_service_client::BlogTagServiceClient;
    let env = load_env::EnvSchema::load().unwrap_or_default();
    let mut client = BlogTagServiceClient::connect(env.rpc.client_url()).await?;
    let res = client
        .list(pagination)
        .await
        .map(|e| e.into_inner())
        .map_err(|_| ServerFnError::new("some bull"));
    res
}
