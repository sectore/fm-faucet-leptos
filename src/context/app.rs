use leptos::*;

#[derive(Clone)]
pub(crate) struct AppContext {
  pub connect_str: String,
}

pub fn provide_app_context(cx: Scope) {
  // keep it empty by default
  let connect_str = dotenv::var("FM_CONNECT_STRING").unwrap_or("".to_string());

  let context = AppContext { connect_str };

  provide_context(cx, context);
}
