use leptos::*;

#[derive(Clone)]
pub(crate) struct AppContext {
  pub connect_str: String,
}

pub fn provide_app_context(cx: Scope) {
  // let connect_str =
  //   std::env::var("FM_CONNECT_STRING").expect("FM_CONNECT_STRING environment variable not set");

  let connect_str = "fed1152z30ck6zctk009cn3phhktr9hjja0r8j84m7jxyglt8hvfa775tska8hw794htza7rgt6c8z9ggsqq4waen5te0xyerwt3s9cczuvf6xyurzde5974fny3frtvns4s0wvtuqszjs9h".to_string();
  let context = AppContext { connect_str };

  provide_context(cx, context);
}
