use leptos::*;
use leptos_router::*;

use crate::{
  component::{join::Join, scan::Scan},
  context::provide_app_context,
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
  provide_app_context(cx);

  view! { cx,
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
