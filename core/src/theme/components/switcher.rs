use {
  super::Context,
  crate::prelude::*,
};

//╔═══════════════════════════════════════════════════════════╗
//║ Switcher                                             ║
//╚═══════════════════════════════════════════════════════════╝
// Icons come entirely from the registry (ui.rs). No SVG is defined here —
// swapping icon families is a one-line change in ui.rs.
//
// Toggle button: always shows the *outlined* icon of whichever theme is
// currently active. The button is a control, not a state indicator — outline
// keeps it visually light and consistent regardless of selection.
//
// Dropdown: all three options with labelled icons, including the monitor
// for System where its meaning is unambiguous. Active option uses filled
// icon to clearly indicate the current selection.

/// Dropdown theme switcher with a toggle button and a three-option menu.
///
/// The `class` prop is forwarded to the outer `<div>` so callers can
/// position the component without wrapper elements.
///
/// # Example
/// ```rust
/// <Switcher class="nav__theme" />
/// ```
#[component]
#[must_use]
pub fn Switcher(
  /// Extra CSS class forwarded to the outer wrapper `<div>`.
  #[prop(default = "")]
  class : &'static str,
) -> impl IntoView {
  let Context { theme, .. } = expect_context::<Context,>();
  let (open, set_open,) = signal(false,);

  // Close dropdown when anything outside it is clicked
  Effect::new(move |_| {
    if !open.get() {
      return;
    }
    let Some(doc,) = window().and_then(|w| w.document(),) else {
      return;
    };
    let handler = Closure::<dyn Fn(MouseEvent,),>::wrap(Box::new(move |_| set_open.set(false,),),);
    let _ = doc.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref(),);
    handler.forget();
  },);

  // Each option: (Theme variant, display label)
  // Icons are resolved at render time via Theme::icons() — no Icons enum needed.
  let options = [
    (Theme::System, "System",),
    (Theme::Light, "Light",),
    (Theme::Dark, "Dark",),
  ];

  view! {
    <div class=format!("theme-dropdown {class}") on:click=move |e| e.stop_propagation()>
      // ── Toggle button ───────────────────────────────────────────────────
      // Always shows the outlined icon — the button is a control, not an
      // indicator. Outlined keeps it visually lightweight at all times.
      <button
        type="button"
        class="theme-btn"
        aria-label=move || theme.get().label()
        title="Change theme"
        on:click=move |_| set_open.update(|v| *v = !*v)
      >
        {move || {
          view! { <IconRender icon=theme.get().icon() /> }
        }}
      </button>

      // ── Dropdown menu ───────────────────────────────────────────────────
      // All three options; active option is highlighted via CSS modifier.
      // Active option uses filled icon; inactive options use outlined.
      {move || {
        open
          .get()
          .then(|| {
            view! {
              <div class="theme-dropdown__menu">
                {options
                  .iter()
                  .map(|&(t, label)| {
                    let (rest, active) = t.icons();
                    let icon = if theme.get() == t { active } else { rest };

                    view! {
                      <button
                        type="button"
                        class=move || {
                          if theme.get() == t {
                            "theme-dropdown__option theme-dropdown__option--active"
                          } else {
                            "theme-dropdown__option"
                          }
                        }
                        on:click=move |_| {
                          theme.set(t);
                          set_open.set(false);
                        }
                      >
                        <IconRender icon />
                        {label}
                      </button>
                    }
                  })
                  .collect::<Vec<_>>()}
              </div>
            }
          })
      }}
    </div>
  }
}
