use {
  super::Context,
  crate::prelude::*,
};

/// Provides [`Context`] to the subtree and keeps `data-theme` on
/// `<html>` in sync with the reactive signal.
///
/// Place this at the application root, wrapping all other components.
#[component]
pub fn Provider(children : Children,) -> impl IntoView {
  let theme = RwSignal::new({
    #[cfg(feature = "hydrate")]
    {
      Theme::from_browser_storage().unwrap_or_default()
    }
    #[cfg(not(feature = "hydrate"))]
    {
      Theme::default()
    }
  },);

  #[allow(unused_variables)]
  let (hue, set_hue,) = signal(164f64,);

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
    if let Some(storage,) = window().and_then(|w| w.local_storage().ok().flatten(),) {
      let _ = storage.set_item("craole-theme", theme.get().storage_value(),);
    }
  },);

  // Keep --hue CSS property in sync
  #[cfg(feature = "hydrate")]
  Effect::new(move |_| apply_hue_to_root(hue.get(),),);

  view! { {children()} }
}
