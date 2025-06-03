#[cfg(feature = "ssr")]
use proto_types::{
    blog::tag::blog_tag_service_client::BlogTagServiceClient, common::db::Pagination,
};

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
struct Paging {
    page_index: i32,
    page_size: i32,
}

#[component]
pub fn DatabaseTablesPage() -> impl IntoView {
    // TODO: dynamic params
    let (pagination, _set_pagination) = signal(0);

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
async fn load(index: i32) -> Result<i32, ServerFnError> {
    // TODO: better types
    let env = load_env::EnvSchema::load().unwrap_or_default();
    let mut client = BlogTagServiceClient::connect(env.rpc.client_url()).await?;
    let res = client
        .list(Pagination {
            page_index: index,
            page_size: 10,
        })
        .await
        .map(|e| e.into_inner().total)
        .map_err(|_| ServerFnError::new("some bull"));
    res
}
