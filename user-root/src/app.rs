use crate::routes::HomePage;
use crate::routes::not_found::NotFound;
use crate::{ui::nav_bar::NavBar, utils::hooks::use_theme::use_theme};
use leptos::prelude::*;
use leptos_meta::{Html, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                // injects a stylesheet into the document <head>
                // id=leptos means cargo-leptos will hot-reload this stylesheet
                <Stylesheet id="leptos" href="/pkg/admin-site.css" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

// @see https://github.com/leptos-rs/leptos/discussions/3399#discussioncomment-11645140
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (theme, _) = use_theme();
    let class = move || theme.get().map(|e| e.to_string()).unwrap_or_default();

    view! {
        <Html attr:lang="en" attr:class=class />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <NavBar />

            <main class="container mx-auto pt-6">
                <Routes fallback=NotFound>
                    <Route path=path!("/") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}
