use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{component::join::*, context::provide_app_context};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context(cx);
  provide_app_context(cx);

  view! { cx,
    <Stylesheet id="leptos" href="/pkg/fm_faucet.css"/>
    <Title text="Ferdimint Helper"/>
    <Router>
      <main class="p-20">
        <Routes>
          <Route
            path=""
            view=|cx| {
                view! { cx, <Join/> }
            }
          />
        </Routes>
      </main>
    </Router>
  }
}
