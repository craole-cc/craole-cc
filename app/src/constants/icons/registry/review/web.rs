use crate::constants::icons::{Icon, icon};

const DARK_INVERT: &str = "dark:invert dark:hue-rotate-180";

pub fn actix() -> Icon {
  Icon::new_leptos(icon::SiActix)
    .with_class(DARK_INVERT)
    .with_label("Actix")
}

pub fn axum() -> Icon {
  Icon::new_local("icons/logos/tokio.svg")
    .with_class(DARK_INVERT)
    .with_label("Axum")
}

pub fn htmx() -> Icon {
  Icon::new_leptos(icon::SiHtmx)
    .with_class(DARK_INVERT)
    .with_label("HTMX")
}

pub fn leptos() -> Icon {
  Icon::new_local("icons/logos/leptos.ico").with_label("Leptos")
}

pub fn tailwind() -> Icon {
  Icon::new_local("icons/logos/tailwind-blue.svg").with_label("Tailwind CSS")
}
