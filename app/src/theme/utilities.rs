use crate::prelude::*;

/// Linearise one sRGB channel (0–255) to linear light (0.0–1.0).
///
/// Applies the IEC 61966-2-1 inverse gamma curve required before any
/// colour-space matrix multiplication.
#[inline]
fn srgb_to_linear(c : f64,) -> f64 {
  let c = c / 255.0;
  if c <= 0.04045 {
    c / 12.92
  } else {
    ((c + 0.055) / 1.055).powf(2.4,)
  }
}

/// Convert sRGB (0–255 each) to an OKLCH hue in degrees, or `None` if the
/// colour is too achromatic to have a meaningful hue.
///
/// Pipeline: sRGB → gamma decode → linear RGB → OKLab
/// (Ottosson two-matrix method) → `atan2(b, a)` → OKLCH hue.
///
/// Reference: <https://bottosson.github.io/posts/oklab/>
pub fn rgb_to_oklch_hue(r : f64, g : f64, b : f64,) -> Option<f64,> {
  // 1. Remove sRGB gamma
  let r = srgb_to_linear(r,);
  let g = srgb_to_linear(g,);
  let b = srgb_to_linear(b,);

  // 2. Linear sRGB → LMS cone responses (Ottosson M1 matrix)
  let l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
  let m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
  let s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

  // 3. Perceptual compression (cube root)
  let l_ = l.cbrt();
  let m_ = m.cbrt();
  let s_ = s.cbrt();

  // 4. LMS → OKLab (Ottosson M2 matrix)
  // Only a and b are needed — they carry the chromatic information.
  let a = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
  let b_ = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;

  // 5. Chroma guard — reject near-grey pixels.
  // Vivid colours have OKLab chroma 0.1–0.37.
  // 0.03 rejects grey skies and near-black shadows without excluding
  // subtly saturated pixels like overcast seascapes.
  let chroma = (a * a + b_ * b_).sqrt();
  if chroma < 0.03 {
    return None;
  }

  // 6. OKLCH hue = atan2(b, a), normalised to 0–360°
  let hue = b_.atan2(a,).to_degrees();
  Some(if hue < 0.0 { hue + 360.0 } else { hue },)
}

/// Load an image into an off-screen canvas, sample pixels, compute the
/// circular-mean OKLCH hue, and pass it to `on_hue`.
///
/// Uses **circular mean** (sum unit vectors → `atan2`) rather than arithmetic
/// mean because hue is an angle on a circle. Averaging 350° and 10° naively
/// gives 180° when the correct answer is 0°.
#[cfg(feature = "hydrate")]
pub fn extract_hue_from_url(url : &'static str, on_hue : impl Fn(f64,)+'static,) {
  spawn_local(async move {
    let Some(window,) = web_sys::window() else {
      return;
    };
    let Some(document,) = window.document() else {
      return;
    };
    let Ok(el,) = document.create_element("canvas",) else {
      return;
    };

    let canvas = el.unchecked_into::<HtmlCanvasElement>();
    canvas.set_width(128,);
    canvas.set_height(128,);

    let ctx = match canvas.get_context("2d",).ok().flatten() {
      | Some(c,) => c.unchecked_into::<CanvasRenderingContext2d>(),
      | None => return,
    };

    let Ok(img_el,) = document.create_element("img",) else {
      return;
    };
    let img = img_el.unchecked_into::<HtmlImageElement>();
    img.set_cross_origin(Some("anonymous",),);

    let (tx, rx,) = oneshot::channel::<(),>();
    let tx_cell = Cell::new(Some(tx,),);
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

    let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(&img, 0.0, 0.0, 128.0, 128.0,);

    let Ok(image_data,) = ctx.get_image_data(0.0, 0.0, 128.0, 128.0,) else {
      return;
    };
    let data = image_data.data();
    let len = data.len();
    let (mut sum_cos, mut sum_sin, mut n,) = (0.0_f64, 0.0_f64, 0_u64,);

    // Sample every 8th pixel (32-byte stride in RGBA).
    // 128×128 / 8 = 2 048 samples — accurate and fast.
    let mut i = 0_usize;
    while i + 3 < len {
      if data[i + 3] > 128
        && let Some(hue,) =
          rgb_to_oklch_hue(data[i] as f64, data[i + 1] as f64, data[i + 2] as f64,)
      {
        let rad = hue.to_radians();
        sum_cos += rad.cos();
        sum_sin += rad.sin();
        n += 1;
      }
      i += 32;
    }

    if n == 0 {
      return;
    }

    let mean_hue = sum_sin.atan2(sum_cos,).to_degrees();
    let mean_hue = if mean_hue < 0.0 {
      mean_hue + 360.0
    } else {
      mean_hue
    };
    on_hue(mean_hue,);
  },);
}

/// Write `--hue` to the `<html>` element's inline style.
///
/// CSS `@property` + `transition: --hue 1.5s ease` handles the animation.
/// Called inside a reactive `Effect` so it re-runs whenever the hue signal
/// changes.
#[cfg(feature = "hydrate")]
pub fn apply_hue_to_root(hue : f64,) {
  if let Some(el,) = window()
    .and_then(|w| w.document(),)
    .and_then(|d| d.document_element(),)
    .map(|el| el.unchecked_into::<HtmlElement>(),)
  {
    let _ = el.style().set_property("--hue", &format!("{:.1}", hue),);
  }
}
