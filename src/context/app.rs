use leptos::*;

#[derive(Clone)]
pub(crate) struct AppContext {
  pub connect_str: String,
}

pub fn provide_app_context(cx: Scope) {
  let connect_str =
    std::env::var("FM_CONNECT_STRING").expect("FM_CONNECT_STRING environment variable not set");

  let context = AppContext { connect_str };

  provide_context(cx, context);
}
