use {
  crate::prelude::*,
  leptos_meta::MetaTags,
};

//? Resolves system theme before first paint to prevent flash.
//? Runs synchronously in <head> before any rendering.
const THEME_INIT_SCRIPT : &str = "(function() {
    var theme = document.documentElement.dataset.theme;
    if (!theme || theme === 'system') {
        document.documentElement.dataset.theme =
            window.matchMedia('(prefers-color-scheme: dark)').matches
                ? 'dark'
                : 'light';
    }
})();";

pub fn shell(options : LeptosOptions,) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <AutoReload options=options.clone() />
        <HydrationScripts options />
        <MetaTags />

        // ? inner_html is required — text nodes inside view! are
        // ? HTML-escaped, so plain string content never executes
        <script inner_html=THEME_INIT_SCRIPT />
      </head>
      <body>
        <App />
      </body>
    </html>
  }
}
