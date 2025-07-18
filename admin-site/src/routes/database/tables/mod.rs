pub mod blog;
pub mod blog_tag;

use crate::{
    routes::service_types::{ServiceItem, SubService},
    ui::{
        back_button::BackButton,
        primitive::card::{Card, CardContent, CardDescription, CardHeader, CardTitle},
    },
    utils::router::RouterKey,
};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn DatabaseTablePage() -> impl IntoView {
    let tables: Vec<ServiceItem> = vec![ServiceItem {
        title: "Tables".into(),
        description: None,
        sub_services: vec![
            SubService::new("Blog", RouterKey::DatabaseTablesBlog),
            SubService::new("Blog Tag", RouterKey::DatabaseTablesBlogTag),
        ],
    }];

    let table_card_views = tables
        .into_iter()
        .map(
            |ServiceItem {
                 title,
                 description,
                 sub_services,
             }| {
                let description = description.unwrap_or_default();
                view! { <ServiceCard title description sub_services /> }
            },
        )
        .collect_view();

    view! {
        <div class="flex flex-col gap-4">
            <div class="flex items-center gap-2">
                <BackButton />
                <h1 class="text-xl">"Tables"</h1>
            </div>
            <div>{table_card_views}</div>
        </div>
    }
}

#[component]
fn ServiceCard(
    title: String,
    #[prop(optional)] description: String,
    sub_services: Vec<SubService>,
) -> impl IntoView {
    let has_desc = !description.is_empty();
    let sub_service_views = sub_services
        .into_iter()
        .map(|sub_service| {
            view! {
                <A href=sub_service.href>
                    <div class="border rounded-xl p-4">{sub_service.title}</div>
                </A>
            }
        })
        .collect_view();

    view! {
        <Card>
            <CardHeader>
                <CardTitle>{title}</CardTitle>
                <Show when=move || has_desc>
                    <CardDescription clone:description>{description}</CardDescription>
                </Show>
            </CardHeader>
            <CardContent class="grid grid-cols-3 gap-2".into()>{sub_service_views}</CardContent>
        </Card>
    }
}
