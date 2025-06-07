pub mod blog_tag;

use leptos::prelude::*;
use leptos_router::components::A;

/// general service item (database, syncthing, etc...)
struct ServiceItem {
    title: String,
    description: Option<String>,
    sub_services: Vec<SubService>,
}

/// child service/option that belongs to a service
struct SubService {
    title: String,
    href: String,
}

impl SubService {
    fn new(title: &str, href: &str) -> Self {
        Self {
            title: title.into(),
            href: href.into(),
        }
    }
}

#[component]
pub fn DatabaseTablePage() -> impl IntoView {
    let tables: Vec<ServiceItem> = vec![ServiceItem {
        title: "Tables".into(),
        description: None,
        sub_services: vec![
            SubService::new("Blog", "/database/tables/blog"),
            SubService::new("Blog Tag", "/database/tables/blog_tag"),
        ],
    }];

    let table_card_views = tables
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
            <h1 class="text-xl">"Tables"</h1>
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
    // class="rounded-md border p-2 hover:cursor-pointer"

    view! {
        <div class="p-4">
            <div class="text-lg font-semibold">{title}</div>
            <Show when=move || has_desc>
                <span>{description.clone()}</span>
            </Show>
            <div class="grid grid-cols-3 gap-2 border p-4">
                {sub_services
                    .into_iter()
                    .map(|sub_service| {
                        view! { <A href=sub_service.href>{sub_service.title}</A> }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
