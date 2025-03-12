#![recursion_limit = "500"]
use app::*;
use serde::{Deserialize, Serialize};

pub mod app;
pub mod report;
pub mod home;
mod components;


#[cfg(feature = "ssr")]
//#[tokio::main]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use axum::{routing::get, routing::post, Router, http::Method, http::HeaderName};
 
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::{cors::{Any, CorsLayer}, services::ServeDir};

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");


    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // .allow_credentials(true)
        // allow requests from any origin
        .allow_headers({
            let mut headers: Vec<HeaderName> = vec![];
            headers.push(HeaderName::from_static("content-type"));
            headers.push(HeaderName::from_static("authorization"));
            headers.push(HeaderName::from_static("organisation"));
            headers.push(HeaderName::from_static("site_group_namespace"));
            headers
        })
        .expose_headers({
            let mut headers: Vec<HeaderName> = vec![];
            headers.push(HeaderName::from_static("www-authenticate"));
            headers.push(HeaderName::from_static("authentication-info"));
            headers
        })
        .allow_origin("http://192.168.22.8:4000".parse::<axum::http::HeaderValue>().unwrap())
        .allow_origin("http://192.168.22.8:5000".parse::<axum::http::HeaderValue>().unwrap());

        // build our application with a route
        let app = Router::new()
        
        // // Lets server functions be called externally
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .route("/api/*fn_name", get(leptos_axum::handle_server_fns))

        // .leptos_routes(&state, routes, App)
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        //.fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        
        .layer(cors);
   
    let app = app.nest_service("/videos", ServeDir::new("public/videos"));

    log::info!("listening on http://{}", &addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

// CSR-only setup
#[cfg(not(feature = "ssr"))]
fn main() {
    // use hackernews::App;
    // console_error_panic_hook::set_once();
    // leptos::mount::mount_to_body(App)
}