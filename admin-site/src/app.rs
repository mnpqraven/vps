use crate::{
    routes::{
        database::{
            tables::{blog_tag::DatabaseTableBlogTagPage, DatabaseTablePage},
            DatabasePage,
        },
        HomePage,
    },
    ui::nav_bar::NavBar,
};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Html, MetaTags, Stylesheet, Title};
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

// TODO: migrate/move
#[derive(Debug, Clone, Copy)]
pub enum ColorMode {
    Light,
    Dark,
}

// @see https://github.com/leptos-rs/leptos/discussions/3399#discussioncomment-11645140
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let mode = RwSignal::new(ColorMode::Light);
    provide_context(mode);

    view! {
        <Html
            attr:lang="en"
            attr:class=move || match mode.get() {
                ColorMode::Light => "light",
                ColorMode::Dark => "dark",
            }
        />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <NavBar />

            <main class="container mx-auto pt-6">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/database") view=DatabasePage />
                    <Route path=path!("/database/tables") view=DatabaseTablePage />
                    <Route path=path!("/database/tables/blog_tag") view=DatabaseTableBlogTagPage />
                </Routes>
            </main>
        </Router>
    }
}
