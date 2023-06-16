#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
  use axum::{
    routing::{get, post},
    Router,
  };
  use fm_faucet::{app::*, fallback};
  use leptos::*;
  use leptos_axum::{generate_route_list, LeptosRoutes};

  let conf = get_configuration(None).await.unwrap();
  let leptos_options = conf.leptos_options;
  let addr = &leptos_options.site_addr;
  // Generate the list of routes in your Leptos App
  let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

  simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

  // build our application with a route
  let app = Router::new()
    .route("/favicon.ico", get(fallback::file_and_error_handler))
    .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
    .fallback(fallback::file_and_error_handler)
    .with_state(leptos_options.clone())
    .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> });

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  log!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
  // a client-side main function is required for using `trunk serve`
  // prefer using `cargo leptos serve` instead
  // to run: `trunk serve --open --features ssg`
  use fm_faucet::app::*;
  use leptos::*;

  console_error_panic_hook::set_once();

  leptos::mount_to_body(move |cx| {
    // note: for testing it may be preferrable to replace this with a
    // more specific component, although leptos_router should still work
    view! { cx, <App/> }
  });
}
