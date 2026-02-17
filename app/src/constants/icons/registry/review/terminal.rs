use crate::constants::icons::Icon;

const DARK_INVERT: &str = "dark:invert dark:hue-rotate-180";

pub fn powershell() -> Icon {
  Icon::new_local("icons/logos/powershell.svg")
    .with_class(DARK_INVERT)
    .with_label("PowerShell")
}

pub fn starship() -> Icon {
  Icon::new_local("icons/logos/starship.svg")
    .with_class(DARK_INVERT)
    .with_label("Starship")
}

pub fn ohmyposh() -> Icon {
  Icon::new_local("icons/logos/ohmyposh.svg")
    .with_class(DARK_INVERT)
    .with_label("Oh My Posh")
}
