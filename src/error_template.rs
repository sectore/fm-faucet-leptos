use leptos::{view, Errors, For, IntoView, RwSignal, Scope, View};

// A basic function to display errors served by the error boundaries. Feel free to do more complicated things
// here than just displaying them
pub fn error_template(cx: Scope, errors: Option<RwSignal<Errors>>) -> View {
  let Some(errors) = errors else {
        panic!("No Errors found and we expected errors!");
    };

  view! { cx,
    <h1>"Errors"</h1>
    <For
      each=errors
      key=|(key, _)| key.clone()
      view=move |cx, (_, error)| {
          let error_string = error.to_string();
          view! { cx, <p>"Error: " {error_string}</p> }
      }
    />
  }
  .into_view(cx)
}
