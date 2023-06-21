use leptos::*;
use leptos_meta::*;

use leptos::html::Video;

use crate::context::AppContext;

use qrcode_generator::QrCodeEcc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[derive(Clone, Debug)]
  type QrScanner;

  // #[wasm_bindgen(constructor, js_class = default)]
  #[wasm_bindgen(constructor)]
  fn new(
    video_elem: &web_sys::HtmlVideoElement,
    callback: &js_sys::Function,
    options: &JsValue,
  ) -> QrScanner;

  #[wasm_bindgen(method, js_name = start)]
  fn start(this: &QrScanner);
  #[wasm_bindgen(method, js_name = stop)]
  fn stop(this: &QrScanner);
  #[wasm_bindgen(method, js_name = destroy)]
  fn destroy(this: &QrScanner);
}

#[component]
pub fn Join(cx: Scope) -> impl IntoView {
  let AppContext { connect_str } = expect_context::<AppContext>(cx);

  let video_ref = create_node_ref::<Video>(cx);

  let o_scanner: StoredValue<Option<QrScanner>> = store_value(cx, None);

  let scan = move |_| {
    log!("scan");
    if let Some(video) = video_ref.get() {
      log!("video: {:?}", video.to_string());
      let callback = Closure::wrap(Box::new(|result: JsValue| {
        log!("decoded qr code: {:?}", result);
      }) as Box<dyn FnMut(JsValue)>);

      let options = js_sys::Object::new();
      js_sys::Reflect::set(
        &options,
        &JsValue::from_str("returnDetailedScanResult"),
        &JsValue::from_bool(true),
      )
      .unwrap();

      let scanner = QrScanner::new(&video, callback.as_ref().unchecked_ref(), &options);
      scanner.start();

      o_scanner.set_value(Some(scanner));
    }
  };

  let cancel = move |_| {
    if let Some(scanner) = o_scanner.get_value() {
      scanner.stop();
      scanner.destroy();
      o_scanner.set_value(None);
    }
  };

  let svg: String =
    qrcode_generator::to_svg_to_string(connect_str.clone(), QrCodeEcc::Low, 400, None::<&str>)
      .unwrap();

  view! { cx,
    // loading sources of qr-scanner from unpkg.com
    <Script src="https://unpkg.com/qr-scanner@1.4.2/qr-scanner.legacy.min.js"/>
    <h1 class="text-4xl text-center">"Join Federation"</h1>
      <video _ref=video_ref class="w-full h-[300px]"></video>
      <button class="group relative w-full h-full" on:click=scan>"Scan"</button>
      <button class="group relative w-full h-full" on:click=cancel>"Cancel"</button>

    <div class="flex p-4 w-full justify-center" inner_html=svg></div>
    <div class="text-base break-words">{connect_str}</div>
  }
}
