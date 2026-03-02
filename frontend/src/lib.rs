#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  _ = console_log::init_with_level(log::Level::Error,);
  console_error_panic_hook::set_once();

  leptos::mount::hydrate_body(core::App,);
}
