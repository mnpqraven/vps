use leptos::prelude::*;

#[cfg(feature = "ssr")]
#[component]
pub fn DatabaseTablesPage() -> impl IntoView {
    use proto_types::{
        blog::tag::{
            blog_tag_service_client::BlogTagServiceClient, BlogTag, BlogTagList, BlogTagShape,
        },
        common::db::Pagination,
    };

    // TODO: dynamic params
    let (pagination, set_pagination) = signal(Pagination::default());

    // TODO: type or use #[server]
    let async_data = Resource::new(
        // TODO: unwrap
        move || pagination.get(),
        |_pagination| async {
            // TODO: unwrap
            let client = BlogTagServiceClient::connect("grpc://127.0.0.1:5005")
                .await
                .unwrap();
            // TODO: unwrap
            let res = client.list(_pagination).await.unwrap();

            Ok(res.into_inner())
        },
    );

    view! { <div>"hello DatabaseTablesPage (ssr)"</div> }
}

#[cfg(not(feature = "ssr"))]
#[component]
pub fn DatabaseTablesPage() -> impl IntoView {
    view! { <div>"hello DatabaseTablesPage (non ssr)"</div> }
}
