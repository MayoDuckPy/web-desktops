#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::post, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::services::ServeDir;
    use web_desktops::app::*;

    simple_logger::init_with_level(log::Level::Debug).expect("Couldn't initialize logging");

    // Get env values for leptos
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let root = leptos_options.site_root.as_str();
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .nest_service("/pkg", ServeDir::new(format!("{}/pkg", root)))
        .nest_service("/scripts", ServeDir::new(format!("{}/scripts", root)))
        .leptos_routes(&leptos_options, routes, |cx| view! { cx, <App/> })
        // .fallback(file_and_error_handler)
        .with_state(leptos_options);

    log!("Listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to bind server");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
