use dotenv::dotenv;
use leptos::*;
use log::Level;

mod component;
mod context;

use component::App;

fn main() {
  // Load variables from .env file
  dotenv().ok();

  _ = console_log::init_with_level(Level::Debug);
  console_error_panic_hook::set_once();

  log!("csr mode - mounting to body");

  mount_to_body(move |cx| {
    view! { cx, <App/> }
  })
}
