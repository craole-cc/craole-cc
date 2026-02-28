use {
  super::Context,
  crate::prelude::*,
};

/// Provides [`Context`] to the subtree and keeps `data-theme` on
/// `<html>` in sync with the reactive signal.
///
/// Place this at the application root, wrapping all other components.
#[component]
#[must_use]
pub fn Provider(children : Children,) -> impl IntoView {
  let theme = RwSignal::new(Theme::default(),);

  #[allow(unused_variables)]
  let (hue, set_hue,) = signal(164.0_f64,);

  provide_context(Context { theme, set_hue, },);

  // Keep data-theme attribute on <html> in sync
  #[cfg(feature = "hydrate")]
  Effect::new(move |_| {
    let resolved = theme.get().resolve();
    if let Some(el,) = window()
      .and_then(|w| w.document(),)
      .and_then(|d| d.document_element(),)
    {
      let _ = el.set_attribute("data-theme", resolved,);
    }
  },);

  // Keep --hue CSS property in sync
  #[cfg(feature = "hydrate")]
  Effect::new(move |_| apply_hue_to_root(hue.get(),),);

  view! { {children()} }
}
