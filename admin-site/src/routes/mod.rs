pub mod database;
pub mod not_found;
pub mod service_types;

use crate::{
    ui::primitive::card::{Card, CardContent, CardDescription, CardHeader, CardTitle},
    utils::router::RouterKey,
};
use leptos::prelude::*;
use leptos_router::components::A;
use service_types::{ServiceItem, SubService};

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let services: Vec<ServiceItem> = vec![
        ServiceItem::new("Homelab")
            .service(SubService::new("Database", RouterKey::Database))
            .service(SubService::new("NAS", RouterKey::Nas))
            .service(SubService::new("RPC Server", RouterKey::RpcServer)),
    ];

    let service_card_views = services
        .into_iter()
        .map(|card| {
            view! {
                <ServiceCard
                    title=card.title
                    description=card.description.unwrap_or_default()
                    sub_services=card.sub_services
                />
            }
        })
        .collect_view();

    view! {
        <div class="flex flex-col gap-4">
            <h1 class="text-xl">"Othi's admin panel"</h1>
            <div>{service_card_views}</div>
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
