use crate::constants::icons::{
  Icon,
  icon,
};

const DARK_INVERT: &str = "dark:invert dark:hue-rotate-180";

pub fn rust() -> Icon {
  Icon::new_leptos(icon::FaRustBrands)
    .with_class("fill-[#D34516] dark:fill-[#F4A07C]")
    .with_tooltip("Rust programming language")
    .with_label("Rust")
}

pub fn python() -> Icon {
  Icon::new_local("icons/logos/python.svg").with_label("Python")
}

pub fn shell() -> Icon {
  Icon::new_local("icons/logos/bash.svg")
    .with_class(DARK_INVERT)
    .with_label("Shell")
}

pub fn bash() -> Icon {
  Icon::new_local("icons/logos/bash.svg")
    .with_class(DARK_INVERT)
    .with_label("Bash")
}

pub fn zig() -> Icon {
  Icon::new_local("icons/logos/zig.svg")
    .with_class(DARK_INVERT)
    .with_label("Zig")
}
