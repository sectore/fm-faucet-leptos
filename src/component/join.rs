use leptos::*;

use crate::context::AppContext;

use qrcode_generator::QrCodeEcc;

#[component]
pub fn Join(cx: Scope) -> impl IntoView {
  let AppContext { connect_str } = expect_context::<AppContext>(cx);

  let svg: String =
    qrcode_generator::to_svg_to_string(connect_str.clone(), QrCodeEcc::Low, 400, None::<&str>)
      .unwrap();

  view! { cx,
    <h1 class="text-4xl text-center">"Join Federation"</h1>
    <div class="flex p-4 w-full justify-center" inner_html=svg></div>
    <div class="text-base break-words">{connect_str}</div>
  }
}
