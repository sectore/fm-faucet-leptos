use leptos::*;

use fedimint_client::Client;

#[derive(Clone)]
pub(crate) struct AppContext {
  pub connect_str: String,
}

pub fn provide_app_context(cx: Scope) {
  // keep it empty by default
  let connect_str = dotenv::var("FM_CONNECT_STRING").unwrap_or("".to_string());

  log!("connect_str {}", connect_str);

  let context = AppContext { connect_str };

  // Just import the client here to have a reference in code
  // All needed logic to initialize and use it will be added later
  let client = Client::builder();

  provide_context(cx, context);
}
