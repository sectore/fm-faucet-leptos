use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
  component::{join::Join, scan::Scan},
  context::provide_app_context,
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context(cx);
  provide_app_context(cx);

  view! { cx,
    <Stylesheet id="leptos" href="/pkg/fm_faucet.css"/>
    <Title text="Ferdimint Helper"/>
    <Script src="https://unpkg.com/qr-scanner@1.4.2/qr-scanner.legacy.min.js"/>
    <Router>
      <main class="h-[100dvh]">
        <Routes>
          <Route
            path=""
            view=|cx| {
                view! { cx, <Join/> }
            }
          />
          <Route
            path="/scan"
            view=move |cx| {
                let navigate = use_navigate(cx);
                let go_to_home = move || {
                    _ = navigate("/", Default::default());
                };
                let navigate2 = use_navigate(cx);
                let store_addr = move |addr: String| {
                    log!("addr: {}", & addr);
                    _ = navigate2("/", Default::default());
                };
                view! { cx,
                  <Scan
                    on_cancel=move || {
                        go_to_home();
                    }
                    on_scan=move |a| {
                        store_addr(a);
                    }
                  />
                }
            }
          />
        </Routes>
      </main>
    </Router>
  }
}
