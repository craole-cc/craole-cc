use crate::prelude::*;

/// Shared reactive context provided by [`Provider`].
///
/// Inject with `provide_context` at the app root and read anywhere
/// via `expect_context::<Context>()`.
#[derive(Clone, Copy,)]
pub struct Context {
  /// The currently selected [`Theme`] variant.
  pub theme :   RwSignal<Theme,>,
  /// Write end of the dynamic accent hue signal (0–360°, OKLCH).
  pub set_hue : WriteSignal<f64,>,
}
