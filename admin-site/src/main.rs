#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use admin_site::{app::*, state::AppContext};
    use axum::Router;
    use leptos::{config::get_configuration, logging::log, prelude::provide_context};
    use leptos_axum::{generate_route_list, LeptosRoutes};

    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // load env variables
    let env = load_env::schema::EnvSchema::load().unwrap_or_default();
    // global app state
    let app_ctx = AppContext::from_env(&env);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(app_ctx.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
