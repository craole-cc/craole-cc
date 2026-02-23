use {
  super::{
    LeptosIcon,
    Source,
  },
  crate::prelude::*,
  std::{
    collections::HashMap,
    sync::{
      LazyLock,
      Mutex,
    },
  },
};

static FILL_CLASS_CACHE : LazyLock<Mutex<HashMap<(&'static str, &'static str,), &'static str,>,>,> =
  LazyLock::new(|| Mutex::new(HashMap::new(),),);

#[component]
pub fn Render(icon : Icon, #[prop(optional, into)] class : Option<String,>,) -> impl IntoView {
  let final_class = match class {
    | Some(extra,) => format!("{} {}", icon.class, extra),
    | None => icon.class.to_string(),
  };

  match icon.source {
    | Source::Leptos(ico,) => view! { <LeptosIcon icon=ico attr:class=final_class /> }.into_any(),
    | Source::Local(src,) => view! {
      <img
        src=src
        class=final_class
        alt=icon.label
        loading="lazy"
        onerror="this.style.display='none'"
      />
    }
    .into_any(),
    | Source::Empty => view! { <span /> }.into_any(),
  }
}

/// Helper to create fill class with color
pub fn fill_class(light : &'static str, dark : &'static str,) -> &'static str {
  let mut cache = FILL_CLASS_CACHE.lock().unwrap();
  if let Some(&cached,) = cache.get(&(light, dark,),) {
    return cached;
  }
  let s = Box::leak(format!("fill-[{light}] dark:fill-[{dark}]").into_boxed_str(),);
  cache.insert((light, dark,), s,);
  s
}
