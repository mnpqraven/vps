pub mod database;

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

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let services: Vec<ServiceItem> = vec![ServiceItem {
        title: "Database".into(),
        description: None,
        sub_services: vec![
            SubService::new("Tables", "/database/tables"),
            SubService::new("Health", "/database/health"),
        ],
    }];

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
