#[cfg(feature = "ssr")]
use worker::*;

pub mod app;
pub mod components;
pub mod constants;
pub mod server_functions;
pub mod types;
#[cfg(feature = "ssr")]
pub mod worker_helpers;

#[allow(unused_imports)]
use crate::app::*;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    use leptos::server_fn::axum::register_explicit;

    // Register all server functions
    register_explicit::<GetFeaturedProjects>();
    register_explicit::<GetFeaturedExperience>();
    register_explicit::<GetFeaturedPosts>();
    register_explicit::<GetHeroContent>();
    register_explicit::<GetAboutContent>();
}

// Re-export server functions for use in components
pub use server_functions::*;

#[cfg(feature = "ssr")]
async fn router(env: Env) -> axum::Router {
    use std::sync::Arc;

    use axum::{Extension, Router};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    register_server_functions();

    // build our application with a route
    Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .with_state(leptos_options)
        .layer(Extension(Arc::new(env))) // <- Allow leptos server functions to access Worker stuff
}

#[cfg(feature = "ssr")]
#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    use tower_service::Service;

    Ok(router(env).await.call(req).await?)
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    leptos::mount::hydrate_body(App);
}
