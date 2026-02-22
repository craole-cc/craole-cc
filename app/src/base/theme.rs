use {
  leptos::prelude::*,
  wasm_bindgen::{
    JsCast,
    closure::Closure,
  },
  wasm_bindgen_futures::spawn_local,
  web_sys::{
    HtmlCanvasElement,
    HtmlImageElement,
  },
};

//╔═══════════════════════════════════════════════════════════╗
//║ Configuration                                             ║
//╚═══════════════════════════════════════════════════════════╝

#[derive(Clone, Copy, PartialEq, Eq, Default,)]
pub enum Theme {
  #[default]
  System,
  Light,
  Dark,
}

impl Theme {
  pub fn next(self,) -> Self {
    match self {
      | Theme::System => Theme::Light,
      | Theme::Light => Theme::Dark,
      | Theme::Dark => Theme::System,
    }
  }

  pub fn label(self,) -> &'static str {
    match self {
      | Theme::System => "System theme",
      | Theme::Light => "Light theme",
      | Theme::Dark => "Dark theme",
    }
  }

  pub fn next_label(self,) -> &'static str {
    match self {
      | Theme::System => "Switch to light theme",
      | Theme::Light => "Switch to dark theme",
      | Theme::Dark => "Switch to system theme",
    }
  }

  pub fn resolve(self,) -> &'static str {
    match self {
      | Theme::Light => "light",
      | Theme::Dark => "dark",
      | Theme::System => {
        #[cfg(feature = "hydrate")]
        {
          let prefers_dark = web_sys::window()
            .and_then(|w| {
              w.match_media("(prefers-color-scheme: dark)",)
                .ok()
                .flatten()
            },)
            .map(|mql : web_sys::MediaQueryList| mql.matches(),)
            .unwrap_or(false,);
          if prefers_dark { "dark" } else { "light" }
        }
        #[cfg(not(feature = "hydrate"))]
        "dark"
      }
    }
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Context                                                   ║
//╚═══════════════════════════════════════════════════════════╝

/// Global theme context — holds the mode signal AND the hue setter.
/// Any component can call set_hue() to shift the entire site palette.
#[derive(Clone, Copy,)]
pub struct ThemeContext {
  pub theme :   RwSignal<Theme,>,
  pub set_hue : WriteSignal<f64,>,
}

//╔═══════════════════════════════════════════════════════════╗
//║ Color Utilities                                           ║
//╚═══════════════════════════════════════════════════════════╝

/// RGB (0–255 each) → hue angle in degrees, or None if too grey.
pub fn rgb_to_hue(r : f64, g : f64, b : f64,) -> Option<f64,> {
  let (r, g, b,) = (r / 255.0, g / 255.0, b / 255.0,);
  let max = r.max(g,).max(b,);
  let min = r.min(g,).min(b,);
  let delta = max - min;

  if max > 0.0 && (delta / max) < 0.10 {
    return None; // near-achromatic — skip
  }

  let h = if max == r {
    60.0 * (((g - b) / delta) % 6.0)
  } else if max == g {
    60.0 * (((b - r) / delta) + 2.0)
  } else {
    60.0 * (((r - g) / delta) + 4.0)
  };

  Some(if h < 0.0 { h + 360.0 } else { h },)
}

/// Load a URL into an off-screen canvas, average the pixels,
/// derive a hue and pass it to `on_hue`. Safe to call repeatedly.
pub fn extract_hue_from_url(url : &'static str, on_hue : impl Fn(f64,)+'static,) {
  spawn_local(async move {
    let Some(window,) = web_sys::window() else {
      return;
    };
    let Some(document,) = window.document() else {
      return;
    };

    // Tiny canvas — 64×64 is plenty for colour averaging
    let Ok(el,) = document.create_element("canvas",) else {
      return;
    };
    let canvas = el.unchecked_into::<HtmlCanvasElement>();
    canvas.set_width(64,);
    canvas.set_height(64,);

    let ctx = match canvas.get_context("2d",).ok().flatten() {
      | Some(c,) => c.unchecked_into::<web_sys::CanvasRenderingContext2d>(),
      | None => return,
    };

    // Load with crossOrigin so getImageData() isn't CORS-blocked
    let Ok(img_el,) = document.create_element("img",) else {
      return;
    };
    let img = img_el.unchecked_into::<HtmlImageElement>();
    img.set_cross_origin(Some("anonymous",),);

    // One-shot channel: resolve when onload fires
    let (tx, rx,) = futures::channel::oneshot::channel::<(),>();
    let tx_cell = std::cell::Cell::new(Some(tx,),);
    let on_load = Closure::once(Box::new(move || {
      if let Some(s,) = tx_cell.take() {
        let _ = s.send((),);
      }
    },) as Box<dyn FnOnce(),>,);

    img.set_onload(Some(on_load.as_ref().unchecked_ref(),),);
    img.set_src(url,);
    on_load.forget();

    if rx.await.is_err() {
      return;
    }

    let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(&img, 0.0, 0.0, 64.0, 64.0,);

    let Ok(image_data,) = ctx.get_image_data(0.0, 0.0, 64.0, 64.0,) else {
      return;
    };
    let data = image_data.data();
    let len = data.len();
    let (mut rs, mut gs, mut bs, mut n,) = (0u64, 0u64, 0u64, 0u64,);

    let mut i = 0usize;
    while i + 3 < len {
      let a = data[i + 3] as u64;
      if a > 128 {
        rs += data[i] as u64;
        gs += data[i + 1] as u64;
        bs += data[i + 2] as u64;
        n += 1;
      }
      i += 16; // sample every 4th pixel
    }

    if n == 0 {
      return;
    }

    if let Some(hue,) = rgb_to_hue(
      rs as f64 / n as f64,
      gs as f64 / n as f64,
      bs as f64 / n as f64,
    ) {
      on_hue(hue,);
    }
  },);
}

/// Write --hue to the <html> element's inline style.
/// @property in CSS makes this animatable — no JS loop needed.
pub fn apply_hue_to_root(hue : f64,) {
  if let Some(el,) = web_sys::window()
    .and_then(|w| w.document(),)
    .and_then(|d| d.document_element(),)
    .map(|el| el.unchecked_into::<web_sys::HtmlElement>(),)
  {
    let _ = el.style().set_property("--hue", &format!("{:.1}", hue),);
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Provider                                                  ║
//╚═══════════════════════════════════════════════════════════╝

#[component]
pub fn ThemeProvider(children : Children,) -> impl IntoView {
  let theme = RwSignal::new(Theme::default(),);
  let (hue, set_hue,) = signal(164.0_f64,); // default teal — matches @property initial-value

  provide_context(ThemeContext { theme, set_hue, },);

  // data-theme → <html> on every mode change
  Effect::new(move |_| {
    let resolved = theme.get().resolve();
    if let Some(el,) = web_sys::window()
      .and_then(|w| w.document(),)
      .and_then(|d| d.document_element(),)
    {
      let _ = el.set_attribute("data-theme", resolved,);
    }
  },);

  // --hue → <html> inline style on every hue signal change.
  // CSS @property + transition: --hue 1.5s ease handles the animation.
  Effect::new(move |_| apply_hue_to_root(hue.get(),),);

  view! { {children()} }
}

//╔═══════════════════════════════════════════════════════════╗
//║ SVG Icons                                                 ║
//╚═══════════════════════════════════════════════════════════╝

#[component]
fn IconSystem() -> impl IntoView {
  view! {
    <svg xmlns="http://www.w3.org/2000/svg" fill="none"
      viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round"
        d="M9 17.25v1.007a3 3 0 0 1-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0 1
      15 18.257V17.25m6-12V15a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0
      1 3 15V5.25m18 0A2.25 2.25 0 0 0 18.75 3H5.25A2.25 2.25 0 0 0 3
      5.25m18 0H3" />
    </svg>
  }
}

#[component]
fn IconLight() -> impl IntoView {
  view! {
    <svg xmlns="http://www.w3.org/2000/svg" fill="none"
      viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round"
        d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591
      -1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773
      L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z" />
    </svg>
  }
}

#[component]
fn IconDark() -> impl IntoView {
  view! {
    <svg xmlns="http://www.w3.org/2000/svg" fill="none"
      viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round"
        d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75
      -9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635
      7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z" />
    </svg>
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ ThemeSwitcher component                                   ║
//╚═══════════════════════════════════════════════════════════╝

#[component]
pub fn ThemeSwitcher(#[prop(default = "")] class : &'static str,) -> impl IntoView {
  let ThemeContext { theme, .. } = expect_context::<ThemeContext,>();

  let icon = move || match theme.get() {
    | Theme::System => view! { <IconSystem /> }.into_any(),
    | Theme::Light => view! { <IconLight  /> }.into_any(),
    | Theme::Dark => view! { <IconDark   /> }.into_any(),
  };

  view! {
    <button
      type="button"
      class=format!("theme-btn {class}")
      aria-label=move || theme.get().label()
      title=move || theme.get().next_label()
      on:click=move |_| theme.update(|t| *t = t.next())
    >
      {icon}
    </button>
  }
}
