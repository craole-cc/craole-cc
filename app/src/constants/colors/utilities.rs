//╔═══════════════════════════════════════════════════════════╗
//║ SVG Filter Styles                                         ║
//╚═══════════════════════════════════════════════════════════╝
/// Renders a hard-black SVG as mid-grey. Use for icon rest/idle state.
///
/// `brightness(0)` collapses all pixels to black, `invert(0.35)` lifts to ~35% grey.
///
/// ```rust
/// use app::constants::colors::GREY_FROM_BLACK;
/// assert!(GREY_FROM_BLACK.contains("brightness(0)"));
/// assert!(GREY_FROM_BLACK.contains("invert(0.35)"));
/// ```
pub const GREY_FROM_BLACK: &str = "filter: brightness(0) invert(0.35);";

/// Renders a hard-black SVG as bright white. Use for icon hover state.
///
/// Pair with [`GREY_FROM_BLACK`] on the same icon: rest = grey, hover = white.
///
/// ```rust
/// use app::constants::colors::WHITE_FROM_BLACK;
/// assert!(WHITE_FROM_BLACK.contains("brightness(0)"));
/// assert!(WHITE_FROM_BLACK.contains("invert(1)"));
/// ```
pub const WHITE_FROM_BLACK: &str = "filter: brightness(0) invert(1);";

/// Dims a solid-colour SVG to match the grey tone of [`GREY_FROM_BLACK`].
///
/// ```rust
/// use app::constants::colors::DIM_COLOUR;
/// assert!(DIM_COLOUR.contains("opacity"));
/// ```
pub const DIM_COLOUR: &str = "opacity: 0.6;";

pub const DARK_INVERT: &str = "dark:invert dark:hue-rotate-180";
