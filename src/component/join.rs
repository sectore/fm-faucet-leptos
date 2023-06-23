use js_sys::Object;
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

#[wasm_bindgen]
pub fn process_js_value_with_cast(js_value: JsValue) -> Result<String, JsValue> {
  // Attempt to cast JsValue to an Object
  if let Ok(obj) = js_value.dyn_into::<Object>() {
    // Try to get the 'data' property from the object
    if let Ok(data) = js_sys::Reflect::get(&obj, &JsValue::from_str("data")) {
      // Convert the value to a string if possible
      if let Some(data_string) = data.as_string() {
        return Ok(data_string);
      }
    }
  }

  // Return an error if the extraction fails
  Err(JsValue::from_str("Failed to extract the data property"))
}

#[component]
pub fn Join(cx: Scope) -> impl IntoView {
  let AppContext { connect_str } = expect_context::<AppContext>(cx);

  let video_ref = create_node_ref::<Video>(cx);
  let (scanned, set_scanned) = create_signal(cx, None);

  let o_scanner: StoredValue<Option<QrScanner>> = store_value(cx, None);

  let scan = move || {
    log!("scan");
    if let Some(video) = video_ref.get() {
      log!("video: {:?}", video.to_string());

      let callback = Closure::wrap(Box::new(move |result: JsValue| {
        log!("decoded qr code: {:?}", result);
        web_sys::console::log_1(&result);

        let data = match process_js_value_with_cast(result) {
          Ok(data) => format!("data: {:?}", data),
          Err(e) => format!("error: {:?}", e),
        };

        set_scanned(Some(data.clone()));
      }) as Box<dyn Fn(JsValue)>);

      // Enforce reporting detailed scan results
      // JS: {returnDetailedScanResult: true}
      // https://github.com/nimiq/qr-scanner/#2-create-a-qrscanner-instance
      let options = js_sys::Object::new();
      js_sys::Reflect::set(
        &options,
        &JsValue::from_str("returnDetailedScanResult"),
        &JsValue::from_bool(true),
      )
      .unwrap();

      let scanner = QrScanner::new(&video, callback.as_ref().unchecked_ref(), &options);
      scanner.start();
      callback.forget();

      o_scanner.set_value(Some(scanner));
    }
  };

  let cancel = move || {
    if let Some(scanner) = o_scanner.get_value() {
      scanner.stop();
      scanner.destroy();
      o_scanner.set_value(None);
    }
  };

  create_effect(cx, move |_| {
    if let Some(_) = scanned() {
      cancel();
    }
  });

  let svg: String =
    qrcode_generator::to_svg_to_string(connect_str.clone(), QrCodeEcc::Low, 400, None::<&str>)
      .unwrap();

  view! { cx,
    <Script src="https://unpkg.com/qr-scanner@1.4.2/qr-scanner.legacy.min.js"/>
    <h1 class="text-4xl text-center">"Join Federation"</h1>
    <video _ref=video_ref class="w-full h-[300px]"></video>
    <div class="text-xl">{scanned}</div>
    <button class="group relative w-full h-full" on:click=move |_| { scan() }>
      "Scan"
    </button>
    <button class="group relative w-full h-full" on:click=move |_| { cancel() }>
      "Cancel"
    </button>
    <div class="flex p-4 w-full justify-center" inner_html=svg></div>
    <div class="text-base break-words">{connect_str}</div>
  }
}
