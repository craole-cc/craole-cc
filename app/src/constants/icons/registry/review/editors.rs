use crate::constants::icons::Icon;

const DARK_INVERT: &str = "dark:invert dark:hue-rotate-180";

pub fn helix() -> Icon {
  Icon::new_local("icons/logos/helix.svg")
    .with_class(DARK_INVERT)
    .with_label("Helix")
}

pub fn typst() -> Icon {
  Icon::new_local("icons/logos/typst.svg")
    .with_class(DARK_INVERT)
    .with_label("Typst")
}

pub fn vscode() -> Icon {
  Icon::new_local("icons/logos/vscode.svg")
    .with_class(DARK_INVERT)
    .with_label("VS Code")
}

pub fn zed() -> Icon {
  Icon::new_local("icons/logos/zed.svg")
    .with_class(DARK_INVERT)
    .with_label("Zed")
}
