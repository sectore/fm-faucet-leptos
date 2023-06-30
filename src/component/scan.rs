use js_sys::Object;
use leptos::*;

use leptos::html::Video;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[derive(Clone, Debug)]
  type QrScanner;

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
  Err(JsValue::from_str("Failed to extract the data properly"))
}

#[component]
pub fn Scan<F, G>(cx: Scope, on_cancel: F, on_scan: G) -> impl IntoView
where
  F: Fn() + 'static,
  G: Fn(String) + 'static,
{
  let video_ref = create_node_ref::<Video>(cx);
  let (scanned, set_scanned) = create_signal(cx, None);
  let (error, set_error) = create_signal(cx, None);

  let o_scanner: StoredValue<Option<QrScanner>> = store_value(cx, None);

  let scan = move || {
    if let Some(video) = video_ref.get() {
      let callback = Closure::wrap(Box::new(move |result: JsValue| {
        web_sys::console::log_1(&result);

        match process_js_value_with_cast(result) {
          Ok(data) => {
            set_scanned(Some(format!("data: {:?}", data)));
          }
          Err(e) => {
            let error_message = format!("Error: {:?}", e);
            set_error(Some(error_message));
          }
        };
      }) as Box<dyn Fn(JsValue)>);

      // Options of `QrScanner`
      // JS: {returnDetailedScanResult: true} - Enforce reporting detailed scan results
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
    on_cancel();
  };

  create_effect(cx, move |_| {
    if let Some(addr) = scanned() {
      on_scan(addr.clone());
    }
  });

  view! { cx,
    <div class="h-full w-full relative">
      <video _ref=video_ref class="w-full h-full fixed object-cover"></video>
      <Show
        when=move || error().is_some()
        fallback=|_| {
            view! { cx, "" }
        }
      >
        <div class="absolute right-4 top-4 left-4 bg-white rounded-lg text-2xl p-4 text-gray-900 text-center">
          <h1 class="text-4xl text-red-500 font-bold uppercase">"Error"</h1>
          {error()}
        </div>
      </Show>
      <div class="absolute right-0 left-0 bottom-2 flex flex-col gap-6 p-6 ">
        <button
          class="w-full text-4xl text-white bg-gray-900 p-4 uppercase rounded-xl"
          on:click=move |_| { scan() }
        >
          "Scan"
        </button>
        <button
          class="w-full text-4xl text-white bg-gray-900 p-4 uppercase rounded-xl"
          on:click=move |_| cancel()
        >
          "Cancel"
        </button>
      </div>
    </div>
  }
}
