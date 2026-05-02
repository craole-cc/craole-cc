use super::_prelude::*;

pub fn use_spotlight_keyboard(set_open: WriteSignal<bool>) {
  #[cfg(feature = "hydrate")]
  Effect::new(move |_| {
    let handler = Closure::<dyn Fn(KeyboardEvent)>::wrap(Box::new(move |e: KeyboardEvent| {
      if (e.meta_key() || e.ctrl_key()) && e.key() == "k" {
        e.prevent_default();
        set_open.update(|v| *v = !*v);
      }
      if e.key() == "Escape" {
        set_open.set(false);
      }
    }));
    if let Some(win) = window() {
      let cb = handler.as_ref().unchecked_ref::<js_sys::Function>().clone();
      let _ = win.add_event_listener_with_callback("keydown", &cb);
    }
    handler.forget();
  });
}

pub fn use_focus_on_open(
  open: ReadSignal<bool>,
  set_query: WriteSignal<String>,
  input_ref: NodeRef<LeptosInput>,
) {
  Effect::new(move |_| {
    if !open.get() {
      return;
    }
    set_query.set(String::new());
    #[cfg(feature = "hydrate")]
    if let Some(win) = window() {
      let cb = Closure::<dyn Fn()>::wrap(Box::new(move || {
        if let Some(el) = input_ref.get() {
          let _ = el.focus();
        }
      }));
      let _ =
        win.set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 30);
      cb.forget();
    }
  });
}
