/// Which visual style variant of an icon to use.
///
/// Every brand in the registry implements [`From<Variant>`] for [`Icon`],
/// making this the standard way to request an icon without knowing which
/// brand module you are calling into.
///
/// Brands that ship additional non-standard styles (e.g. `FaBrands`,
/// `SiSimple`) expose a separate `Extended` enum alongside this one.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash,)]
pub enum Variant {
  /// The brand's canonical default. Resolves to `Local` for most brands,
  /// `Filled` for brands where the coloured Leptos icon is preferred
  /// (e.g. GitHub, X).
  #[default]
  Default,

  /// Local SVG asset from the `public/icons/logos/` directory.
  Local,

  /// Filled / solid Leptos icon with the brand's `--brand-*` colour applied.
  Filled,

  /// Outlined / line-weight Leptos icon with the brand's `--brand-*` colour.
  /// Falls back to `Filled` for brands that have no distinct outlined style.
  Outlined,
}
