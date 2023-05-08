use std::sync::Arc;

use app::{App, AppSsr};

use axum::{Extension, Router, routing::{post, get}};
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos::{view, get_configuration, log};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    let app = Router::new()
        .route("/ssr", get(vega_ssr))
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> })
        .nest_service("/pkg", ServeDir::new("target/site/pkg"))
        .fallback_service(ServeDir::new("target/site"))
        .layer(Extension(Arc::new(leptos_options)));

    log!("listening on http://{}", &addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn vega_ssr() -> axum::response::Html<String> {
    axum::response::Html(
        leptos::ssr::render_to_string(|cx| view! { cx, <AppSsr/> })
    )
}

